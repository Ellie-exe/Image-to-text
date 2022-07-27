use image::{GenericImageView, Pixel, imageops::FilterType};
use std::{env, fs::File, io::Write};
use termion;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut add_color = false;
    let mut output_file = false;
    let mut path = "";

    for i in 0..args.len() {
        let arg = &args[i];
        if !arg.starts_with('-') { continue; }

        match arg.chars().last().unwrap() {
            'c' => { add_color = true; },
            'o' => {
                output_file = true;
                path = &args[i + 1];
            },

            _ => {}
        }
    }

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

    let mut string = String::from("\x1b[1m");
    if add_color { string += &ave_color_str; }

    string += "┌";
    for _ in 0..img.width() + 2 { string += "─"; }
    string += "┐\n";

    let mut prev_color = ave_color;

    for pixel in img.pixels() {
        if pixel.1 % 2 != 0 { continue; }
        if pixel.0 == 0 { string += "│ "; }

        let luma = pixel.2.to_luma().0[0] as f32;
        let charset_max = (charset.len() - 1) as f32;
        let char = ((luma / 255.0) * charset_max).round() as usize;

        let red = pixel.2[0] as u32;
        let green = pixel.2[1] as u32;
        let blue = pixel.2[2] as u32;

        let color = red + green + blue;
        if add_color && color != prev_color {
            string += &format!("\x1b[38;2;{};{};{}m", red, green, blue);
        }

        string += &format!("{}", &charset[char..char + 1]);
        prev_color = color;

        if pixel.0 == img.width() - 1 {
            if add_color { string += &ave_color_str; }
            string += " │\n";
            prev_color = ave_color;
        }
    }

    string += "└";
    for _ in 0..img.width() + 2 { string += "─"; }
    string += "┘\x1b[0m\n";

    print!("{}", string);

    if output_file {
        let mut file = File::create(path).unwrap();
        write!(&mut file, "{}", string).unwrap();
    }
}
