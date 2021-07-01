pub fn my_func1(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_func1() {
        assert_eq!(3, my_func1(1, 2));
    }
}
