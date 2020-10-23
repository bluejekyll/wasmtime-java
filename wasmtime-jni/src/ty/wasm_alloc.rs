use std::mem;
use std::ops::Deref;

use anyhow::{anyhow, ensure, Context, Error};
use log::{debug, warn};
use wasmtime::{Caller, Extern, Func, Instance, Memory, Val};
use wasmtime_jni_exports::{ALLOC_EXPORT, DEALLOC_EXPORT, MEMORY_EXPORT};

use crate::ty::WasmSlice;

const MEM_SEGMENT_SIZE: usize = 64 * 1024;

/// Allocator that can allocate and deallocate to and from a WASM module.
///
/// This assumes the existence of `memory` Memory as well as `__alloc_bytes` and `__dealloc_bytes` Funcs
///   are exported from the module.
pub(crate) struct WasmAlloc {
    memory: Memory,
    alloc: Func,
    dealloc: Func,
}

impl WasmAlloc {
    pub fn from_caller(caller: &Caller) -> Option<Self> {
        let memory = caller
            .get_export(MEMORY_EXPORT)
            .and_then(Extern::into_memory);
        let alloc = caller.get_export(ALLOC_EXPORT).and_then(Extern::into_func);
        let dealloc = caller
            .get_export(DEALLOC_EXPORT)
            .and_then(Extern::into_func);

        Self::from(memory, alloc, dealloc)
    }

    pub fn from_instance(instance: &Instance) -> Option<Self> {
        Self::from(
            instance.get_memory(MEMORY_EXPORT),
            instance.get_func(ALLOC_EXPORT),
            instance.get_func(DEALLOC_EXPORT),
        )
    }

    fn from(memory: Option<Memory>, alloc: Option<Func>, dealloc: Option<Func>) -> Option<Self> {
        Some(Self {
            memory: memory?,
            alloc: alloc?,
            dealloc: dealloc?,
        })
    }

    /// Safety, the returned array is uninitialized
    #[allow(clippy::mut_from_ref)]
    pub unsafe fn as_mut(&self, wasm_slice: WasmSlice) -> &mut [u8] {
        debug!("data ptr: {}", wasm_slice.ptr);

        &mut self.memory.data_unchecked_mut()[wasm_slice.ptr as usize..][..wasm_slice.len as usize]
    }

    /// Allocates size bytes in the Wasm Memory context, returns the offset into the Memory region
    pub unsafe fn alloc_size(&self, size: usize) -> Result<WasmSliceWrapper<'_>, Error> {
        let len = size as i32;
        let ptr = self
            .alloc
            .call(&[Val::I32(len)])?
            .get(0)
            .and_then(|v| v.i32())
            .ok_or_else(|| anyhow!("i32 was not returned from the alloc"))?;

        debug!("Allocated offset {} len {}", ptr, len);

        let wasm_slice = WasmSlice { ptr, len };
        Ok(WasmSliceWrapper::new(self, wasm_slice))
    }

    /// Allocates the bytes from the src bytes
    pub fn alloc_bytes(&self, src: &[u8]) -> Result<WasmSliceWrapper<'_>, Error> {
        let mem_base = self.memory.data_ptr() as usize;
        let mem_size = self.memory.size() as usize * MEM_SEGMENT_SIZE;
        ensure!(
            mem_size > src.len(),
            "memory is {} need {} more",
            mem_size,
            src.len()
        );

        // get target memor location and then copy into the function
        let wasm_slice = unsafe { self.alloc_size(src.len())? };
        let mem_bytes = unsafe { self.as_mut(wasm_slice.wasm_slice) };
        mem_bytes.copy_from_slice(src);

        debug!(
            "copied bytes into mem: {:x?}, mem_base: {:x?} mem_bytes: {:x?}",
            mem_bytes,
            mem_base,
            mem_bytes.as_ptr(),
        );

        Ok(wasm_slice)
    }

    /// Deallocate the WasmSlice
    pub fn dealloc_bytes(&self, slice: WasmSlice) -> Result<(), Error> {
        let WasmSlice { ptr, len } = slice;
        self.dealloc
            .call(&[Val::I32(ptr), Val::I32(len)])
            .with_context(|| anyhow!("failed to deallocate bytes"))?;

        debug!("Deallocated offset {} len {}", ptr, len);
        Ok(())
    }

    pub fn alloc<T: Sized>(&self) -> Result<WasmSliceWrapper<'_>, Error> {
        let wasm_slice = unsafe { self.alloc_size(mem::size_of::<T>())? };

        // zero out the memory...
        for b in unsafe { wasm_slice.as_mut() } {
            *b = 0;
        }

        debug!(
            "stored {} at {:x?}",
            std::any::type_name::<T>(),
            wasm_slice.wasm_slice().ptr
        );
        Ok(wasm_slice)
    }

    #[allow(clippy::mut_from_ref)]
    pub unsafe fn obj_as_mut<T: Sized>(&self, ptr: i32) -> &mut T {
        debug_assert!(ptr > 0);
        let ptr_to_mem = self.memory.data_ptr().add(ptr as usize);
        debug!("dereffing {:x?} from offset {:x?}", ptr_to_mem, ptr);

        &mut *(ptr_to_mem as *mut T)
    }

    #[allow(unused)]
    pub fn dealloc<T: Sized>(&self, ptr: i32) -> Result<(), Error> {
        debug_assert!(ptr > 0);
        let len = mem::size_of::<T>() as i32;
        let wasm_slice = WasmSlice { ptr, len };

        self.dealloc_bytes(wasm_slice)
    }
}

/// This is use to free memory after a function call
pub(crate) struct WasmSliceWrapper<'w> {
    wasm_alloc: &'w WasmAlloc,
    wasm_slice: WasmSlice,
}

impl<'w> WasmSliceWrapper<'w> {
    pub fn new(wasm_alloc: &'w WasmAlloc, wasm_slice: WasmSlice) -> Self {
        Self {
            wasm_alloc,
            wasm_slice,
        }
    }

    /// Safety, the returned array is uninitialized
    #[allow(clippy::mut_from_ref)]
    pub unsafe fn as_mut(&self) -> &mut [u8] {
        self.wasm_alloc.as_mut(self.wasm_slice)
    }

    #[allow(clippy::mut_from_ref)]
    pub unsafe fn obj_as_mut<T: Sized>(&self) -> &mut T {
        self.wasm_alloc.obj_as_mut(self.wasm_slice.ptr)
    }

    /// Copy out the WasmSlice, careful, the lifetime of this is really tied to the memory lifetime backing the WasmAlloc
    pub fn wasm_slice(&self) -> WasmSlice {
        self.wasm_slice
    }
}

impl<'w> Deref for WasmSliceWrapper<'w> {
    type Target = WasmSlice;

    fn deref(&self) -> &WasmSlice {
        &self.wasm_slice
    }
}

impl<'w> Drop for WasmSliceWrapper<'w> {
    fn drop(&mut self) {
        if let Err(err) = self.wasm_alloc.dealloc_bytes(self.wasm_slice) {
            warn!("Error deallocating bytes: {}", err);
        }
    }
}
