use image::{self, GenericImageView};
use std::{
    env,
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
};

fn main() {
    let path = PathBuf::from("output.x");
    x_to_png(path)
}

fn png_to_x(path: PathBuf) {
    let img = image::open(path).expect("Failed to open image");
    let mut str = String::new();
    let mut last_line = 0;

    img.pixels().for_each(|pixel| {
        let hex_color = format!("{:02X}{:02X}{:02X}", pixel.2[0], pixel.2[1], pixel.2[2]);

        if last_line != pixel.1 {
            str.push_str("\n");
            last_line = pixel.1;
        }
        str.push_str(&hex_color.replace("#", ""));
    });

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("output.x")
        .expect("Failed to create file");

    file.write_all(str.as_bytes())
        .expect("Failed to write to file");

    println!("File created successfully");
}

fn x_to_png(path: PathBuf) {
    let content = fs::read_to_string(path).expect("Failed to read file");
    let mut img = image::ImageBuffer::new(1920, 1080);

    let mut x = 0;
    let mut y = 0;

    for i in 0..content.len() {
        let color = image::Rgb([
            u8::from_str_radix(&content[i..i + 2], 16).unwrap(),
            u8::from_str_radix(&content[i + 2..i + 4], 16).unwrap(),
            u8::from_str_radix(&content[i + 4..i + 6], 16).unwrap(),
        ]);

        img.put_pixel(x, y, color);

        x += 1;
        if x == 1920 {
            x = 0;
            y += 1;
        }
    }

    img.save("output.png").expect("Failed to save image");

    println!("Image created successfully");
}
