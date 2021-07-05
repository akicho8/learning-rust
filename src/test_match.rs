// -*- compile-command: "cargo test -- --nocapture" -*-
// use std::cmp::PartialEq;
#[cfg(test)]
mod tests {
    #[test]
    fn test_match() {
        enum Foo { A, B }
        let v1 = match Foo::A {
            Foo::A => 100,
            Foo::B => 200,
        };
        assert_eq!(100, v1);
        let _ = Foo::B;
    }
}
