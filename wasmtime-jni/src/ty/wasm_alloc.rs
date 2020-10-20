use std::mem;

use anyhow::{anyhow, ensure, Error};
use log::debug;
use wasmtime::{Caller, Extern, Func, Instance, Memory, Val};
use wasmtime_jni_exports::{ALLOC_EXPORT, MEMORY_EXPORT};

use crate::ty::WasmSlice;

const MEM_SEGMENT_SIZE: usize = 64 * 1024;

pub(crate) struct WasmAlloc {
    memory: Memory,
    allocator: Func,
}

impl WasmAlloc {
    pub fn from_caller(caller: &Caller) -> Option<Self> {
        let memory = caller
            .get_export(MEMORY_EXPORT)
            .and_then(Extern::into_memory);
        let alloc = caller.get_export(ALLOC_EXPORT).and_then(Extern::into_func);

        Self::from(memory, alloc)
    }

    pub fn from_instance(instance: &Instance) -> Option<Self> {
        Self::from(
            instance.get_memory(MEMORY_EXPORT),
            instance.get_func(ALLOC_EXPORT),
        )
    }

    fn from(memory: Option<Memory>, allocator: Option<Func>) -> Option<Self> {
        Some(Self {
            memory: memory?,
            allocator: allocator?,
        })
    }

    pub unsafe fn as_mut(&self, wasm_slice: WasmSlice) -> &mut [u8] {
        debug!("data ptr: {}", wasm_slice.ptr);

        &mut self.memory.data_unchecked_mut()[wasm_slice.ptr as usize..][..wasm_slice.len as usize]
    }

    /// Allocates size bytes in the Wasm Memory context, returns the offset into the Memory region
    fn alloc_size(&self, size: usize) -> Result<WasmSlice, Error> {
        let len = size as i32;
        let ptr = self
            .allocator
            .call(&[Val::I32(len)])?
            .get(0)
            .and_then(|v| v.i32())
            .ok_or_else(|| anyhow!("i32 was not returned from the allocator"))?;

        debug!("Allocated offset {} len {}", ptr, len);

        Ok(WasmSlice { ptr, len })
    }

    /// Allocates the bytes from the src bytes
    pub fn alloc_bytes(&self, src: &[u8]) -> Result<WasmSlice, Error> {
        let mem_size = self.memory.size() as usize * MEM_SEGMENT_SIZE;
        ensure!(
            mem_size > src.len(),
            "memory is {} need {} more",
            mem_size,
            src.len()
        );

        // get target memor location and then copy into the function
        let wasm_slice = self.alloc_size(src.len())?;
        let mem_bytes = unsafe { self.as_mut(wasm_slice) };
        mem_bytes.copy_from_slice(src);

        debug!(
            "copied bytes into mem: {:x?}, mem_base: {:x?} mem_bytes: {:x?}",
            mem_bytes,
            self.memory.data_ptr(),
            mem_bytes.as_ptr(),
        );

        Ok(wasm_slice)
    }

    pub fn alloc<T: Sized>(&self) -> Result<i32, Error> {
        let wasm_slice = self.alloc_size(mem::size_of::<T>())?;

        // zero out the memory...
        for b in unsafe { self.as_mut(wasm_slice) } {
            *b = 0;
        }

        debug!(
            "stored {} at {:x?}",
            std::any::type_name::<T>(),
            wasm_slice.ptr
        );
        Ok(wasm_slice.ptr)
    }

    pub unsafe fn obj_as_mut<'a, T: Sized>(&'a self, ptr: i32) -> &'a mut T {
        debug_assert!(ptr > 0);
        let ptr_to_mem = self.memory.data_ptr().add(ptr as usize);
        debug!("dereffing {:x?} from offset {:x?}", ptr_to_mem, ptr);

        &mut *(ptr_to_mem as *mut T)
    }
}
