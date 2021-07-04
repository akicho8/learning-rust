// -*- compile-command: "cargo test -- --nocapture" -*-
// use std::cmp::PartialEq;

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
    #[test]
    fn test_array() {
        assert_eq!("b", ["a", "b", "c"][1]);
    }
    #[test]
    fn test_let() {
        let v2 = 1;
        let v1 = {
            let v2 = 100;
            v2 + 23            // 最後に ; をつけると謎のエラーがでる
        };
        assert_eq!(123, v1);
        assert_eq!(1, v2);
    }
    #[test]
    fn test_if() {
        let v1 = if true {
            1
        } else {                // = if の場合は else がないとエラーになる！？
            2
        };
        assert_eq!(1, v1);

        if true {
            let v1 = 100;
            assert_eq!(100, v1);
        }
    }
    #[test]
    fn test_iter() {
        for e in ["a", "b"].iter() {
            println!("--> {}", e);
        }
    }
    #[test]
    fn test_enum() {
        enum FOO1  { A, B } // 従来の単なる enum
        let v1: FOO1 = FOO1::A;
        let _v2: FOO1 = FOO1::B;
        // assert_eq!(FOO1::A, _v1); ← うごかない

        // enum のそれぞれが異なる構造体みたいになれる
        enum FOO2 {
            A(String),
            B(u32, u32),
        }
        FOO2::A(String::from("a"));
        FOO2::B(1, 2);

        // enum も struct と同じように impl でメソッドを追加できる
        impl FOO1 {
            fn func1(&self) -> u32 {
                123
            }
        }
        assert_eq!(123, v1.func1());
    }
    #[test]
    fn test_option() {
        let _ = Some(1);           // Some だと型がわかるのでそのまま指定できる
        let _: Option<i32> = None; // None の場合は型がわからないので自分で指定しないといけない 
    }
}
