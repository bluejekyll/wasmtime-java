use wasmtime_jni_exports::{Owned, WasmString};

/// test imports from Java
#[link(wasm_import_module = "test")]
extern "C" {
    // Ownership, the response should be freed by the caller
    fn say_hello_to_java(data_ptr: i32, data_len: i32, response: &mut Owned<WasmString>);
}

/// Greetings
///
/// # Safety
/// Passed in WasmSlice is owned by caller
#[no_mangle]
pub unsafe extern "C" fn greet(name_ptr: i32, name_len: i32) {
    let name = WasmString::borrowed(&name_ptr, name_len);
    let name = name.from_utf8_lossy();

    println!("Hello, {}!", name);
}

/// # Safety
/// Passed in WasmSlice is owned by caller
#[no_mangle]
pub unsafe extern "C" fn say_hello_to(
    name_ptr: i32,
    name_len: i32,
    response: &mut Owned<WasmString>,
) {
    let name = WasmString::borrowed(&name_ptr, name_len);
    let name = name.from_utf8_lossy();

    let hello_to = format!("Hello, {}!", name);
    let hello_to = hello_to.into_boxed_str(); // make this a heap allocated str (this makes capacity == len)

    assert_eq!(
        hello_to.as_ptr(),
        hello_to.as_bytes() as *const [u8] as *const u8
    );

    // need to drop ownership of this value
    let len = hello_to.len();
    let ptr = Box::into_raw(hello_to);

    *response = WasmString::owned(ptr as *const u8 as i32, len as i32);
}

/// # Safety
/// Passed in WasmSlice is owned by caller
#[no_mangle]
pub unsafe extern "C" fn say_hello_in_java(
    data_ptr: i32,
    data_len: i32,
    response: &mut Owned<WasmString>,
) {
    // Technically after this call we "own" the response, but we pass that directly into the next method
    //  which will then own freeing the memory (which is technically handled by the wasmtime-jni bindings)
    say_hello_to_java(data_ptr, data_len, response)
}
