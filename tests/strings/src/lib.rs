use wasmtime_jni_exports::{Owned, WasmAllocated, WasmSlice};

/// test imports from Java
#[link(wasm_import_module = "test")]
extern "C" {
    // Ownership, the response should be freed by the caller
    fn say_hello_to_java(data_ptr: i32, data_len: i32, response: &mut Owned<WasmSlice>);
}

/// Greetings
///
/// # Safety
/// Passed in WasmSlice is owned by caller
#[no_mangle]
pub unsafe extern "C" fn greet(name_ptr: i32, name_len: i32) {
    let name = WasmSlice::borrowed(name_ptr, &name_len);
    let name = name.from_utf8_lossy();

    println!("Hello, {}!", name);
}

/// # Safety
/// Passed in WasmSlice is owned by caller
#[no_mangle]
pub unsafe extern "C" fn say_hello_to(
    name_ptr: i32,
    name_len: i32,
    response: &mut Owned<WasmSlice>,
) {
    let name = WasmSlice::borrowed(name_ptr, &name_len);
    let name = name.from_utf8_lossy();

    let hello_to = format!("Hello, {}!", name);
    println!("{}", hello_to);
    let hello_to: Owned<WasmSlice> = hello_to.into(); // make this a heap allocated str (this makes capacity == len)

    assert_eq!(
        hello_to.ptr() as *mut u8,
        hello_to.as_bytes() as *const [u8] as *mut u8
    );

    response.replace(hello_to);
}

/// # Safety
/// Passed in WasmSlice is owned by caller
#[no_mangle]
pub unsafe extern "C" fn say_hello_in_java(
    data_ptr: i32,
    data_len: i32,
    response: &mut Owned<WasmSlice>,
) {
    // Technically after this call we "own" the response, but we pass that directly into the next method
    //  which will then own freeing the memory (which is technically handled by the wasmtime-jni bindings)
    say_hello_to_java(data_ptr, data_len, response)
}
