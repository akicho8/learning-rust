// -*- compile-command: "cargo test -- --nocapture" -*-

#[cfg(test)]
mod tests {
    #[test]
    fn test_vec_case1() {
        let v1: Vec<i32> = Vec::new();
        assert_eq!(0, v1.len());

        let v1 = vec![10, 20];
        assert_eq!(2, v1.len());

        let v2: &i32 = &v1[1];
        assert_eq!(20, *v2);

        let _v2: Option<&i32> = v1.get(1);
        // assert_eq!(20, _v2); なんでうごかない？
    }
}
