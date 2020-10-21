use std::alloc::{self, Layout};
use std::slice;

pub const MEMORY_EXPORT: &str = "memory";
pub const ALLOC_EXPORT: &str = "__alloc_bytes";
pub const DEALLOC_EXPORT: &str = "__dealloc_bytes";

/// Allocates size in bytes of `memory`, offset to area returned.
///
/// # Returns
///
/// Offset from start of `memory` export in WASM to the region, or 0 if unable to allocate.
///
/// # Safety
///
/// This will allocate a byte array in the WASM module. To refer to this memory externally, there is an
///  exported memory section. These bytes are only valid for the life of the Store or until the Memory is
///  resized.
#[no_mangle]
pub unsafe extern "C" fn __alloc_bytes(size: u32) -> i32 {
    let layout = Layout::array::<u8>(size as usize).expect("u8 should definitely have a layout");
    let ptr = alloc::alloc(layout) as i32;

    debug_assert_ne!(0, ptr);
    // useful for debugging
    //println!("allocated {} at {}", size, ptr);
    ptr
}

/// Frees ptr from `memory` in WASM
///
/// # Safety
///
/// Must be a pointer to data allocated with the __alloc_bytes
#[no_mangle]
pub unsafe extern "C" fn __dealloc_bytes(ptr: u32, size: u32) {
    let layout = Layout::array::<u8>(size as usize).expect("u8 should definitely have a layout");
    alloc::dealloc(ptr as *mut u8, layout);
}

/// A WasmSlice is an offset into the local `memory` of the WASM module instance.
///
/// It is only valid in the context of a `memory` contiguous region and a module's associated `Store`
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct WasmSlice {
    pub ptr: i32,
    pub len: i32,
}

impl WasmSlice {
    /// # Safety
    /// This relies on the ptr and len being accurate for the current memory environment. Inside a WASM runtime for example.
    #[inline]
    pub unsafe fn as_bytes(&self) -> &[u8] {
        let ptr = self.ptr as *const u8;
        slice::from_raw_parts(ptr, self.len as usize)
    }
}

impl From<Vec<u8>> for WasmSlice {
    #[inline]
    fn from(bytes: Vec<u8>) -> Self {
        let boxed_slice = bytes.into_boxed_slice();
        WasmSlice::from(boxed_slice)
    }
}

impl From<Box<[u8]>> for WasmSlice {
    #[inline]
    fn from(bytes: Box<[u8]>) -> Self {
        let len = bytes.len() as i32;
        let ptr = Box::into_raw(bytes) as *mut u8 as i32;

        // helpful for debugging
        // println!("storing Box<[u8]> at {} len {}", ptr, len);

        Self { ptr, len }
    }
}
