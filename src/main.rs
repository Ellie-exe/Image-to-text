use image::{GenericImageView, Pixel};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let img = image::open(&args[1]).unwrap();
    let charset = String::from(" .:-=+*#%@");

    for pixel in img.pixels() {
        if pixel.1 % 2 != 0 { continue; }
        if pixel.0 == 0 { println!(); }

        let luma = pixel.2.to_luma().0[0] as f32;
        let charset_max = (charset.len() - 1) as f32;
        let char = ((luma / 255.0) * charset_max).round() as usize;

        let rgb_str = format!("{};{};{}", pixel.2[0], pixel.2[1], pixel.2[2]);
        print!("\x1b[1;38;2;{}m{}\x1b[0m", rgb_str, &charset[char..char + 1]);
    }

    println!("\n");
}
