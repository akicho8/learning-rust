// use std::io;
// use std::cmp::Ordering;
// use rand::Rng;

mod test_basic;
mod test_string;

fn main() {
    const FOO: u32 = 100;

    let mut x = 1;
    println!("{}", x);
    x = 2;
    println!("{}", x);
    println!("{}", FOO);

    // println!("foobar::my_func1 -> {}", foobar::my_func1(1, 2));

    // let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("--> {}", secret_number);
    //
    // loop {
    //     let mut guess = String::new();
    //     io::stdin().read_line(&mut guess)
    //         .expect("失敗");
    //
    //     // let guess: u32 = guess.trim().parse()
    //     //     .expect("Please type a number!");
    //
    //     let guess: u32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };
    //
    //     println!("入力値: {}", guess);
    //
    //     match guess.cmp(&secret_number) {
    //         Ordering::Less => println!("<"),
    //         Ordering::Greater => println!(">"),
    //         Ordering::Equal => {
    //             println!("==");
    //             break;
    //         },
    //     }
    // }

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
