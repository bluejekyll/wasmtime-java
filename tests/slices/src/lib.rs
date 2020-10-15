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
pub extern "C" fn print_bytes(data: *const u8, len: u32) {
    println!("SLICES: ptr: {:x?} len: {}", data, len);

    //let data: &[u8] = unsafe { slice::from_raw_parts(data, len as usize) };
    println!("SLICES: received bytes {:x?}", data);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let bytes = &[0u8, 1, 2] as &[u8];
        print_bytes(bytes.as_ptr(), bytes.len() as u32);
    }
}
