// -*- compile-command: "cargo test -- --nocapture" -*-

#[cfg(test)]
mod tests {
    #[test]
    fn test_struct_case1() {
        #[derive(Debug)]
        struct User {
            name: String,
            score: u64,
        }
        let user1 = User {
            name: String::from("alice"),
            score: 100,
        };
        assert_eq!("alice", user1.name);
        assert_eq!(100, user1.score);

        let name = String::from("alice");
        let user2 = User { name, ..user1 }; // { name: name, **user1 } の省略形
        assert_eq!("alice", user2.name);
        assert_eq!(100, user2.score);

        println!("{:?}", user1);  // ":?" で inspect する (さらに構造体の前に #[derive(Debug)] を定義する必要がある)
        println!("{:#?}", user1); // "#?" は複数行になって見やすい

        impl User {
            fn foo(&self) -> u64 {
                self.score * 2
            }
        }
        assert_eq!(200, user1.foo());

        impl User {
            // クラスメソッドは &self 不要
            fn create1(name: String) -> User {
                User { name, score: 300 }
            }
        }
        assert_eq!("alice", User::create1(String::from("alice")).name);

    }
}
