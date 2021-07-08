// -*- compile-command: "cargo test -- --nocapture" -*-

mod mod1 {
    pub fn func1() -> u32 {
        1
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_module_case1() {
        assert_eq!(1, crate::test_module::mod1::func1()); // どゆこと？？？ なんで test_module が入る？
    }
}
