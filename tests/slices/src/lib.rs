use wasmtime_jni_exports::WasmSlice;

// needed for exports to wasmtime-jni
pub use wasmtime_jni_exports;

// These functions are declared in Java and use the Linker to associate them to the Module Instance.
#[link(wasm_import_module = "test")]
extern "C" {
    fn hello_to_java(data_ptr: i32, data_len: i32);
    fn reverse_bytes_java(data_ptr: i32, data_len: i32, result: &mut WasmSlice);
}

#[no_mangle]
pub extern "C" fn say_hello_to_java() {
    let hello = "Hello Java!";

    let bytes = hello.as_bytes();
    unsafe { hello_to_java(bytes.as_ptr() as i32, bytes.len() as i32) }
}

/// # Safety
///
/// This relies on an external method having properly allocated the WasmSlice before calling this method.
#[no_mangle]
pub unsafe extern "C" fn print_bytes(slice_ptr: i32, slice_len: i32) {
    let slice = WasmSlice {
        ptr: slice_ptr,
        len: slice_len,
    };
    println!(
        "slices::print_bytes: ptr: {:x?} len: {}",
        slice.ptr, slice.len
    );

    let data: &[u8] = slice.as_bytes();
    println!("slices::print_bytes: received bytes {:x?}", data);
}

/// # Safety
///
/// This relies on an external method having properly allocated the WasmSlice before calling this method.
#[no_mangle]
pub unsafe extern "C" fn reverse_bytes(slice_ptr: i32, slice_len: i32, slice_ref: &mut WasmSlice) {
    let slice = WasmSlice {
        ptr: slice_ptr,
        len: slice_len,
    };
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

/// # Safety
/// Assumes that the data input is properly allocated slice, and the result has an allocated WasmSlice object at the pointer.
#[no_mangle]
pub unsafe extern "C" fn reverse_bytes_in_java(
    data_ptr: i32,
    data_len: i32,
    result: &mut WasmSlice,
) {
    let data = WasmSlice {
        ptr: data_ptr,
        len: data_len,
    };
    println!("slices::reverse_bytes_in_java: {:?}", data);
    reverse_bytes_java(data.ptr, data.len, result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_bytes() {
        let bytes = &[0u8, 1, 2] as &[u8];
        unsafe { print_bytes(bytes.as_ptr() as i32, bytes.len() as i32) };
    }
}
