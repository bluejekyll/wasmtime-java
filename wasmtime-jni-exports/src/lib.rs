use std::alloc::{self, Layout};

/// Allocates size in bytes of `memory`, offset to area returned.
///
/// # Returns
///
/// Offset from start of `memory` export in WASM to the region, or 0 if unable to allocate.
#[no_mangle]
pub unsafe extern "C" fn __alloc_bytes(size: u32) -> i32 {
    let layout = Layout::array::<u8>(size as usize).expect("u8 should definitely have a layout");
    let ptr = alloc::alloc(layout) as i32;

    // useful for debugging
    //println!("allocated {} at {}", size, ptr);
    ptr
}

/// Frees ptr from `memory` in WASM
#[no_mangle]
pub unsafe extern "C" fn __dealloc_bytes(ptr: u32, size: u32) {
    let layout = Layout::array::<u8>(size as usize).expect("u8 should definitely have a layout");
    alloc::dealloc(ptr as *mut u8, layout);
}
