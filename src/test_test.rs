#[cfg(test)]
mod tests {
    #[test]
    #[should_panic]
    fn test_case1() {
        panic!();
    }

    #[test]
    #[should_panic(expected = "msg1")]
    fn test_case2() {
        panic!("msg1");
    }
}
