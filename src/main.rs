use clap::Parser;
use image::{self, GenericImageView};
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // folder to search
    #[clap(short, long)]
    path: String,

    // what to search for
    #[clap(short, long)]
    target: String,
}

fn main() {
    let args = Args::parse();
    let path = PathBuf::from(&args.path);
    let target = PathBuf::from(&args.target);

    if path.is_dir() {
        let files = fs::read_dir(path).expect("Failed to read directory");
        files.for_each(|file| {
            let file = file.expect("Failed to read file");
            let path = file.path();
            let file_name = path.file_name().unwrap().to_str().unwrap();
            if file_name.contains(&args.target) {
                println!("{}", file_name);
            }
        });
    } else {
        let file_name = path.file_name().unwrap().to_str().unwrap();
        if file_name.ends_with(".png") {
            println!("{}", path.to_str().unwrap());
            println!("{}", target.to_str().unwrap());
            png_to_x(path, target);
        } else if file_name.ends_with(".x") {
            x_to_png(path, target);
        } else {
            println!("Unsupported file type");
        }
    }

    println!("Done");
}

fn png_to_x(path: PathBuf, target: PathBuf) {
    let img = image::open(path).expect("Failed to open image");
    let mut str = String::new();
    let mut last_line = 0;

    img.pixels().for_each(|pixel| {
        let hex_color = format!("{:02X}{:02X}{:02X}", pixel.2[0], pixel.2[1], pixel.2[2]);

        if last_line != pixel.1 {
            str.push_str("\n");
            last_line = pixel.1;
        }
        str.push_str(&hex_color);
    });

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(target.to_str().unwrap())
        .expect("Failed to create file");

    file.write_all(str.as_bytes())
        .expect("Failed to write to file");

    println!("File created successfully");
}

fn x_to_png(path: PathBuf, target: PathBuf) {
    /*
    example x file:
    0000003400006800009C0000D00000
    0034003434016834029C3403D03404
    0068003468026868049C6806D06808
    009C00349C03689C069C9C09D09C0C
    00D00034D00468D0089CD00CD0D010
     */

    let content = fs::read_to_string(path).expect("Failed to read file");
    let width = content.lines().next().unwrap().len() / 6;
    let height = content.lines().count();
    let mut img = image::ImageBuffer::new(width as u32, height as u32);

    for (y, line) in content.lines().enumerate() {
        for (x, hex) in line.chars().collect::<Vec<char>>().chunks(6).enumerate() {
            let hex = hex.iter().collect::<String>();
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
            img.put_pixel(x as u32, y as u32, image::Rgb([r, g, b]));
        }
    }

    img.save(target.to_str().unwrap()).expect("Failed to save image");
    println!("Image created successfully");
}
