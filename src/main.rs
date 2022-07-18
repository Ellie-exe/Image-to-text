use image::{GenericImageView, Pixel, imageops::FilterType};
use std::env;
use termion;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (width, height) = termion::terminal_size().unwrap();
    let nheight = (height as u32 * 2) - 6;
    let nwidth = width as u32 - 4;

    println!("\nProcessing image...\x1b[2A\x1b[?25l");

    let raw_img = image::open(&args[1]).unwrap();
    let img = raw_img.resize(nwidth, nheight, FilterType::Triangle);

    let ave_img = raw_img.resize_exact(1, 1, FilterType::Triangle);
    let ave_pixel = ave_img.pixels().last().unwrap();
    let ave_rgb_str = format!("{};{};{}", ave_pixel.2[0], ave_pixel.2[1], ave_pixel.2[2]);

    let charset = String::from("   ...,,;:clodxkO0KXNWM");

    print!("\x1b[1;38;2;{}m┌", ave_rgb_str);
    for _ in 0..img.width() + 2 { print!("─"); }
    println!("┐\x1b[0m");

    for pixel in img.pixels() {
        if pixel.1 % 2 != 0 { continue; }
        if pixel.0 == 0 { print!("\x1b[1;38;2;{}m│ \x1b[0m", ave_rgb_str); }

        let luma = pixel.2.to_luma().0[0] as f32;
        let charset_max = (charset.len() - 1) as f32;
        let char = ((luma / 255.0) * charset_max).round() as usize;

        let rgb_str = format!("{};{};{}", pixel.2[0], pixel.2[1], pixel.2[2]);
        print!("\x1b[1;38;2;{}m{}\x1b[0m", rgb_str, &charset[char..char + 1]);

        if pixel.0 == img.width() - 1 { print!("\x1b[1;38;2;{}m │\x1b[0m\n", ave_rgb_str); }
    }

    print!("\x1b[1;38;2;{}m└", ave_rgb_str);
    for _ in 0..img.width() + 2 { print!("─"); }
    println!("┘\x1b[0m\x1b[?25h");
}
