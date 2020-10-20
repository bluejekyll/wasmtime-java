use wasmtime_jni_exports::WasmSlice;

// needed for exports to wasmtime-jni
pub use wasmtime_jni_exports;

#[link(wasm_import_module = "test")]
extern "C" {
    fn hello_to_java(data: *const u8, len: u32);
}

#[no_mangle]
pub extern "C" fn say_hello_to_java() {
    let hello = "Hello Java!";

    let bytes = hello.as_bytes();
    unsafe { hello_to_java(bytes.as_ptr(), bytes.len() as u32) }
}

#[no_mangle]
pub unsafe extern "C" fn print_bytes(slice: WasmSlice) {
    println!(
        "slices::print_bytes: ptr: {:x?} len: {}",
        slice.ptr, slice.len
    );

    let data: &[u8] = slice.as_bytes();
    println!("slices::print_bytes: received bytes {:x?}", data);
}

#[no_mangle]
pub unsafe extern "C" fn reverse_bytes(slice: WasmSlice, slice_ref: &mut WasmSlice) {
    println!(
        "slices::reverse_bytes: ptr: {:x?} len: {}",
        slice.ptr, slice.len
    );

    let data: &[u8] = slice.as_bytes();
    println!("slices::reverse_bytes: received bytes {:x?}", data);

    let mut reversed: Vec<u8> = Vec::with_capacity(data.len());
    for b in data.iter().rev() {
        reversed.push(*b);
    }

    let reversed = reversed.into_boxed_slice();
    let reversed = WasmSlice::from(reversed);

    // assign the return value
    *slice_ref = reversed;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_bytes() {
        let bytes = &[0u8, 1, 2] as &[u8];
        unsafe { print_bytes(bytes.as_ptr(), bytes.len() as u32) };
    }
}
