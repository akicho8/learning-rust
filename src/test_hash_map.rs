// -*- compile-command: "cargo test -- --nocapture" -*-

#[cfg(test)]
mod tests {
    #[test]
    fn test_case1() {
        use std::collections::HashMap;
        let mut v1 = HashMap::new();
        v1.insert(String::from("key1"), 10);
        v1.insert(String::from("key2"), 20);
        assert_eq!(10, v1["key1"]);
        // let key1 = String::from("key1");
        // assert_eq!(10, v1.get(&key1));
        v1.entry(String::from("key3")).or_insert(30);
        println!("{:?}", v1);

    }
}
