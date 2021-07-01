// use std::io;
// use rand::Rng;

mod foobar;

fn main() {
    // let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("--> ");
    // let mut guess = String::new();
    // // io::stdin().read_line(&mut guess).expect("失敗");
    // println!("入力値: {}", guess)
    println!("foobar::my_func1 -> {}", foobar::my_func1(1, 2))
}

// pub fn add_two(a: i32) -> i32 {
//     a + 2
// }
// 
// #[cfg(test)]
// mod tests {
//     use super::*;
// 
//     #[test]
//     fn add_two_and_two() {
//         assert_eq!(4, add_two(2));
//     }
// 
//     #[test]
//     fn add_three_and_two() {
//         assert_eq!(5, add_two(3));
//     }
// 
//     #[test]
//     fn one_hundred() {
//         assert_eq!(102, add_two(100));
//     }
// }
