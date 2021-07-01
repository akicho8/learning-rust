pub fn my_add(a, b) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_add() {
        assert_eq!(e, my_add(1, 2));
    }
}
