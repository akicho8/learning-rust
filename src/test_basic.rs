// -*- compile-command: "cargo test -- --nocapture" -*-

#[cfg(test)]
mod tests {
    // use super::*;
    #[test]
    fn test_tuple() {
        let foo = ("a", 123, "c");
        assert_eq!("a", foo.0);
        assert_eq!(123, foo.1);
        assert_eq!("c", foo.2);

        let (x, y, z) = foo;
        assert_eq!("a", x);
        assert_eq!(123, y);
        assert_eq!("c", z);
    }
    #[test]
    fn test_scalar() {
        let v: i8 = 127;
        assert_eq!(127, v);
        assert_eq!(255, 0xff);
        assert_eq!(255, 0xFF);
        assert_eq!(8, 0o10);
        assert_eq!(15, 0b11_11);
        assert_eq!(0x41, b'A');

        // float
        let v: f32 = 10.0 / 3.0;
        println!("{}", v);
        let v: f64 = 10.0 / 3.0;
        println!("{}", v);

    }
}
