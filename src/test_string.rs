// -*- compile-command: "cargo test -- --nocapture" -*-
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
    #[test]
    fn test_string_from() {
        let mut s1 = String::from("foo"); // let mut s1 = "foo"; と書いた場合は push_str できない。なんで？
        s1.push_str("bar");
        assert_eq!("foobar", s1);

        // let s1 = String::from("foo");
        // let s2 = s1; // s1 を s2 にコピーしたあと s1 を使うとエラーになる
        // println!("s2 -> {}", s1);
    }
    #[test]
    fn test_string_clone() {
        let s1 = String::from("foo");
        let s2 = s1.clone();    // 実体をコピーしている
        assert_eq!(s1, s2);
    }
    #[test]
    fn test_string_len() {
        assert_eq!(3, "foo".len());
        assert_eq!(3, String::from("foo").len());
    }
    #[test]
    fn test_string_as_bytes() {
        // let s1 = String::from("foo");
        // assert_eq!(3, s1.as_bytes());
    }
    #[test]
    fn test_string_clear() {
        let mut s1 = String::from("foo");
        s1.clear();
        assert_eq!("", s1);
    }
    #[test]
    fn test_string_slice() {
        let s1 = "foobar";
        assert_eq!("fo", &s1[0..2]); // .. は ... と考えて 0...2 が ruby の 0..1 になる
        assert_eq!("ba", &s1[3..5]);
    }
}
