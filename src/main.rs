use image::{GenericImageView, Pixel, imageops::FilterType};
use std::{env, fs, fs::File, io::Write};
use termion;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (width, height) = termion::terminal_size().unwrap();
    let nheight = (height as u32 * 2) - 6;
    let nwidth = width as u32 - 4;

    let raw_img = image::open(&args[1]).unwrap();
    let img = raw_img.resize(nwidth, nheight, FilterType::Triangle);

    let ave_img = raw_img.resize_exact(1, 1, FilterType::Triangle);
    let ave_pixel = ave_img.pixels().last().unwrap();

    let ave_red = ave_pixel.2[0] as u32;
    let ave_green = ave_pixel.2[1] as u32;
    let ave_blue = ave_pixel.2[2] as u32;

    let ave_color = ave_red + ave_green + ave_blue;
    let ave_color_str = format!("\x1b[38;2;{};{};{}m", ave_red, ave_green, ave_blue);

    let charset = String::from("   ...,,;:clodxkO0KXNWM");
    let mut file = File::create(&args[2]).unwrap();

    write!(&mut file, "\x1b[1m{}┌", ave_color_str).unwrap();
    for _ in 0..img.width() + 2 { write!(&mut file, "─").unwrap(); }
    writeln!(&mut file, "┐").unwrap();

    let mut prev_color = ave_color;

    for pixel in img.pixels() {
        if pixel.1 % 2 != 0 { continue; }
        if pixel.0 == 0 { write!(&mut file, "│ ").unwrap(); }

        let luma = pixel.2.to_luma().0[0] as f32;
        let charset_max = (charset.len() - 1) as f32;
        let char = ((luma / 255.0) * charset_max).round() as usize;

        let red = pixel.2[0] as u32;
        let green = pixel.2[1] as u32;
        let blue = pixel.2[2] as u32;

        let color = red + green + blue;
        if color != prev_color {
            write!(&mut file, "\x1b[38;2;{};{};{}m", red, green, blue).unwrap();
        }

        write!(&mut file, "{}", &charset[char..char + 1]).unwrap();
        prev_color = color;

        if pixel.0 == img.width() - 1 {
            writeln!(&mut file, "{} │", ave_color_str).unwrap();
            prev_color = ave_color;
        }
    }

    write!(&mut file, "└").unwrap();
    for _ in 0..img.width() + 2 { write!(&mut file, "─").unwrap(); }
    writeln!(&mut file, "┘\x1b[0m").unwrap();

    print!("{}", fs::read_to_string(&args[2]).unwrap());
}
