// pub fn my_func2(a: i32, b: i32) -> i32 {
//     a + b
// }

#[cfg(test)]
mod tests {
    // use super::*;
    #[test]
    fn test_string_parse() {
        assert_eq!(123, "123".parse().expect("expect_message"));
    }
}
