use wasmtime_jni_exports::WasmSlice;

// test imports from Java
#[link(wasm_import_module = "test")]
extern "C" {
    fn say_hello_to_java(data: WasmSlice, response: &mut WasmSlice);
}

/// Greetings
///
/// # Safety
/// Passed in WasmSlice is owned by caller
#[no_mangle]
pub unsafe extern "C" fn greet(name: WasmSlice) {
    let name = name.as_bytes();
    let name = String::from_utf8_lossy(name);

    println!("Hello, {}!", name);
}

/// # Safety
/// Passed in WasmSlice is owned by caller
#[no_mangle]
pub unsafe extern "C" fn say_hello_to(name: WasmSlice, response: &mut WasmSlice) {
    let name = name.as_bytes();
    let name = String::from_utf8_lossy(name);

    let hello_to = format!("Hello, {}!", name);
    let hello_to = hello_to.into_boxed_str(); // make this a heap allocated str (this makes capacity == len)

    assert_eq!(
        hello_to.as_ptr(),
        hello_to.as_bytes() as *const [u8] as *const u8
    );

    // need to drop ownership of this value
    let len = hello_to.len();
    let ptr = Box::into_raw(hello_to);

    *response = WasmSlice {
        ptr: ptr as *const u8 as i32,
        len: len as i32,
    };
}

/// # Safety
/// Passed in WasmSlice is owned by caller
#[no_mangle]
pub unsafe extern "C" fn say_hello_in_java(name: WasmSlice, response: &mut WasmSlice) {
    say_hello_to_java(name, response)
}
