pub extern "C" fn add_i32(a: i32, b: i32) -> i32 {
    a + b
}

pub extern "C" fn add_u32(a: u32, b: u32) -> u32 {
    a + b
}

pub extern "C" fn add_i64(a: i64, b: i64) -> i64 {
    a + b
}

pub extern "C" fn add_u64(a: u64, b: u64) -> u64 {
    a + b
}

pub extern "C" fn add_f32(a: f32, b: f32) -> f32 {
    a + b
}

pub extern "C" fn add_f64(a: f64, b: f64) -> f64 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_i32() {
        assert_eq!(add_i32(1, 2), 3);
    }

    #[test]
    fn test_add_u32() {
        assert_eq!(add_u32(1, 2), 3);
    }

    #[test]
    fn test_add_i64() {
        assert_eq!(add_i64(1, 2), 3);
    }

    #[test]
    fn test_add_u64() {
        assert_eq!(add_u64(1, 2), 3);
    }

    #[test]
    fn test_add_f32() {
        assert_eq!(add_f32(1.1, 2.2), 3.3);
    }

    #[test]
    fn test_add_f64() {
        assert_eq!(add_f64(1.1, 2.2), 3.3);
    }
}
