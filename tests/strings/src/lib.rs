use std::slice;

#[no_mangle]
pub extern "C" fn greet(name_ptr: *const u8, name_len: u32) {
    let name = unsafe { slice::from_raw_parts(name_ptr, name_len as usize) };
    let name = String::from_utf8_lossy(name);
    println!("Hello, {}!", name);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        //       assert_eq!(greet("world"), "Hello, world!");
    }
}
