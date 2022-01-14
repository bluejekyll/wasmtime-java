wit_bindgen_rust::export!("tests/math-wit/src/math.wit");

struct Math;

pub use math::Math as MathExport;

impl MathExport for Math {
    fn add_i32(a: i32, b: i32) -> i32 {
        a + b
    }

    fn add_u32(a: u32, b: u32) -> u32 {
        a + b
    }

    fn add_i64(a: i64, b: i64) -> i64 {
        a + b
    }

    fn add_u64(a: u64, b: u64) -> u64 {
        a + b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_i32() {
        assert_eq!(Math::add_i32(1, 2), 3);
    }

    #[test]
    fn test_add_u32() {
        assert_eq!(Math::add_u32(1, 2), 3);
    }

    #[test]
    fn test_add_i64() {
        assert_eq!(Math::add_i64(1, 2), 3);
    }

    #[test]
    fn test_add_u64() {
        assert_eq!(Math::add_u64(1, 2), 3);
    }
}
