extern crate sdl2;

use sdl2::event::Event;
use sdl2::image::{LoadTexture, INIT_JPG, INIT_PNG};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::env;
use std::thread;
use std::time::Duration;

pub fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: cargo run /path/to/image.(png|jpg)")
    }
    let png = &args[1];

    // SDLの初期化
    let sdl_context = sdl2::init().unwrap();
    // Videoサブシステムの取得
    let video_subsystem = sdl_context.video().unwrap();
    // sdl2_imageの初期化
    let _image_context = sdl2::image::init(INIT_PNG | INIT_JPG).unwrap();

    // Widnowの作成
    let window = video_subsystem
        .window("hello", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    // Canvasの作成
    let mut canvas = window.into_canvas().build().unwrap();
    // テクスチャの初期化
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture(png).unwrap();

    // ゲームループ
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        // Canvasのクリア(塗りつぶし)
        canvas.set_draw_color(Color::RGB(200, 200, 200));
        canvas.clear();
        // テクスチャの描画
        canvas.copy(&texture, None, None).expect("Render failed");
        // スクリーンの更新
        canvas.present();

        // イベント処理
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        // TODO: ゲーム処理
    }
}











// // use std::io;
// // use std::cmp::Ordering;
// // use rand::Rng;
// 
// mod test_basic;
// mod test_string;
// 
// fn main() {
//     const FOO: u32 = 100;
// 
//     let mut x = 1;
//     println!("{}", x);
//     x = 2;
//     println!("{}", x);
//     println!("{}", FOO);
// 
//     // println!("foobar::my_func1 -> {}", foobar::my_func1(1, 2));
// 
//     // let secret_number = rand::thread_rng().gen_range(1, 101);
//     // println!("--> {}", secret_number);
//     //
//     // loop {
//     //     let mut guess = String::new();
//     //     io::stdin().read_line(&mut guess)
//     //         .expect("失敗");
//     //
//     //     // let guess: u32 = guess.trim().parse()
//     //     //     .expect("Please type a number!");
//     //
//     //     let guess: u32 = match guess.trim().parse() {
//     //         Ok(num) => num,
//     //         Err(_) => continue,
//     //     };
//     //
//     //     println!("入力値: {}", guess);
//     //
//     //     match guess.cmp(&secret_number) {
//     //         Ordering::Less => println!("<"),
//     //         Ordering::Greater => println!(">"),
//     //         Ordering::Equal => {
//     //             println!("==");
//     //             break;
//     //         },
//     //     }
//     // }
// 
// }
// 
// // pub fn add_two(a: i32) -> i32 {
// //     a + 2
// // }
// //
// // #[cfg(test)]
// // mod tests {
// //     use super::*;
// //
// //     #[test]
// //     fn add_two_and_two() {
// //         assert_eq!(4, add_two(2));
// //     }
// //
// //     #[test]
// //     fn add_three_and_two() {
// //         assert_eq!(5, add_two(3));
// //     }
// //
// //     #[test]
// //     fn one_hundred() {
// //         assert_eq!(102, add_two(100));
// //     }
// // }
