// -*- compile-command: "cargo test -- --nocapture" -*-

#[cfg(test)]
mod tests {
    #[test]
    fn test_hash_map() {
        use std::collections::HashMap;
        let mut v1 = HashMap::new();
        v1.insert(String::from("key1"), 10);
        v1.insert(String::from("key2"), 20);
        assert_eq!(10, v1["key1"]);
    }
}
