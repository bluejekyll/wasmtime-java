use std::convert::TryFrom;
use std::mem;
use std::ops::Deref;

use anyhow::{anyhow, ensure, Context, Error};
use log::{debug, warn};
use wasmtime::{AsContextMut, Caller, Extern, Func, Instance, Memory, Store, Val};
use wasmtime_jni_exports::{ALLOC_EXPORT, DEALLOC_EXPORT, MEMORY_EXPORT};

use crate::{
    ty::{WasmAllocated, WasmSlice},
    wasm_state::JavaState,
};

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
    pub fn from_caller(caller: &mut Caller<JavaState>) -> Option<Self> {
        let memory = caller
            .get_export(MEMORY_EXPORT)
            .and_then(Extern::into_memory);
        let alloc = caller.get_export(ALLOC_EXPORT).and_then(Extern::into_func);
        let dealloc = caller
            .get_export(DEALLOC_EXPORT)
            .and_then(Extern::into_func);

        Self::from(memory, alloc, dealloc)
    }

    pub fn from_instance(instance: &Instance, mut store: impl AsContextMut) -> Option<Self> {
        Self::from(
            instance.get_memory(&mut store, MEMORY_EXPORT),
            instance.get_func(&mut store, ALLOC_EXPORT),
            instance.get_func(&mut store, DEALLOC_EXPORT),
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
    pub unsafe fn as_mut<'a>(
        &self,
        wasm_slice: WasmSlice,
        store: &'a mut impl AsContextMut,
    ) -> &'a mut [u8] {
        debug!("data ptr: {}", wasm_slice.ptr());

        &mut self.memory.data_mut(store.as_context_mut())[wasm_slice.ptr() as usize..]
            [..wasm_slice.len() as usize]
    }

    /// Allocates size bytes in the Wasm Memory context, returns the offset into the Memory region
    pub unsafe fn alloc_size(
        &self,
        size: usize,
        store: impl AsContextMut,
    ) -> Result<WasmSliceWrapper<'_>, Error> {
        let len = size as i32;
        let mut ptr = [Val::null(); 1];
        self.alloc.call(store, &[Val::I32(len)], &mut ptr)?;

        let ptr = ptr
            .get(0)
            .and_then(|v| v.i32())
            .ok_or_else(|| anyhow!("i32 was not returned from the alloc"))?;

        debug!("Allocated offset {} len {}", ptr, len);

        let wasm_slice = WasmSlice::new(ptr, len);
        Ok(WasmSliceWrapper::new(self, wasm_slice))
    }

    /// Allocates the bytes from the src bytes
    pub fn alloc_bytes(
        &self,
        src: &[u8],
        mut store: impl AsContextMut,
    ) -> Result<WasmSliceWrapper<'_>, Error> {
        let mem_base = self.memory.data_ptr(&mut store) as usize;
        let mem_size = self.memory.size(&mut store) as usize * MEM_SEGMENT_SIZE;

        // TODO: check if we need to grow the underlying memory with `grow`, etc.
        ensure!(
            mem_size > src.len(),
            "memory is {} need {} more",
            mem_size,
            src.len()
        );

        // get target memory location and then copy into the function
        let wasm_slice = unsafe { self.alloc_size(src.len(), &mut store)? };
        let mem_bytes = unsafe { self.as_mut(wasm_slice.wasm_slice, &mut store) };
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
    pub fn dealloc_bytes(
        &self,
        slice: WasmSlice,
        store: &mut Store<JavaState>,
    ) -> Result<(), Error> {
        let ptr = slice.ptr();
        let len = slice.len();
        let mut no_result = [Val::null(); 0];
        self.dealloc
            .call(store, &[Val::I32(ptr), Val::I32(len)], &mut no_result)
            .with_context(|| anyhow!("failed to deallocate bytes"))?;

        debug!("Deallocated offset {} len {}", ptr, len);
        Ok(())
    }

    pub fn alloc<T: Sized>(
        &self,
        store: &mut Store<JavaState>,
    ) -> Result<WasmSliceWrapper<'_>, Error> {
        let wasm_slice = unsafe { self.alloc_size(mem::size_of::<T>(), &mut *store)? };

        // zero out the memory...
        for b in unsafe { wasm_slice.as_mut(&mut *store) } {
            *b = 0;
        }

        debug!(
            "stored {} at {:x?}",
            std::any::type_name::<T>(),
            wasm_slice.wasm_slice().ptr()
        );
        Ok(wasm_slice)
    }

    #[allow(clippy::mut_from_ref)]
    pub unsafe fn obj_as_mut<T: Sized, S: AsContextMut>(&self, ptr: i32, store: S) -> &mut T {
        debug_assert!(ptr > 0);
        let ptr_to_mem = self.memory.data_ptr(store).add(ptr as usize);
        debug!("dereffing {:x?} from offset {:x?}", ptr_to_mem, ptr);

        &mut *(ptr_to_mem as *mut T)
    }

    #[allow(unused)]
    pub unsafe fn dealloc<T: Sized>(
        &self,
        ptr: i32,
        store: &mut Store<JavaState>,
    ) -> Result<(), Error> {
        debug_assert!(ptr > 0);
        let len = i32::try_from(mem::size_of::<T>())?;
        let wasm_slice = WasmSlice::new(ptr, len);

        self.dealloc_bytes(wasm_slice, store)
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
    pub unsafe fn as_mut<'a>(&'a self, store: &'a mut Store<JavaState>) -> &'a mut [u8] {
        self.wasm_alloc.as_mut(self.wasm_slice, store)
    }

    #[allow(clippy::mut_from_ref)]
    pub unsafe fn obj_as_mut<T: Sized>(&self, store: &mut Store<JavaState>) -> &mut T {
        self.wasm_alloc.obj_as_mut(self.wasm_slice.ptr(), store)
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
