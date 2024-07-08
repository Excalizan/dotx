use clap::Parser;
use image::{self, GenericImageView};
use std::{
    fs::File,
    io::{prelude::*, Write},
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

    println!("Processing file: {:?}", path);
    println!("Target file: {:?}", target);

    if path.is_dir() {
        println!("Detected directory");
        println!("Only files with .png or .x extension will be processed");
    } else {
        let file_name = path.file_name().unwrap().to_str().unwrap();
        if file_name.ends_with(".png") {
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
    // create a binary file
    let mut file = File::create(target).expect("Failed to create file");

    let img = image::open(path).expect("Failed to open image");
    let (width, height) = img.dimensions();
    let mut content = Vec::<u8>::new();

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let r = pixel[0];
            let g = pixel[1];
            let b = pixel[2];
            content.push(r);
            content.push(g);
            content.push(b);
        }
        content.push(0x0A);
    }

    file.write_all(&content).expect("Failed to write to file");
    println!("File created successfully");
}

fn x_to_png(path: PathBuf, target: PathBuf) {
    let mut file = File::open(path).expect("Failed to open file");
    let mut content = Vec::<u8>::new();
    file.read_to_end(&mut content).expect("Failed to read file");

    let mut width = 0;
    let mut height = 0;

    let mut pixels = Vec::<u8>::new();
    let mut row = Vec::<u8>::new();

    for byte in content {
        if byte == 0x0A {
            height += 1;
            width = row.len() / 3;
            pixels.append(&mut row);
        } else {
            row.push(byte);
        }
    }

    let img = image::ImageBuffer::from_fn(width as u32, height as u32, |x, y| {
        let index = (y * width as u32 + x) as usize * 3;
        image::Rgb([pixels[index], pixels[index + 1], pixels[index + 2]])
    });

    img.save(target).expect("Failed to save image");
    println!("Image created successfully");
}

fn _make_test_image(width: u32, height: u32) {
    let mut img = image::ImageBuffer::new(width, height);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let r = (x * 255 / width) as u8;
        let g = (y * 255 / height) as u8;
        let b = 128;
        *pixel = image::Rgb([r, g, b]);
    }
    img.save("test.png").expect("Failed to save image");
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_png_to_x() {
        let path = PathBuf::from("test.png");
        let target = PathBuf::from("test.x");
        png_to_x(path, target);
        assert!(PathBuf::from("test.x").exists());
    }

    #[test]
    fn test_x_to_png() {
        let path = PathBuf::from("test.x");
        let target = PathBuf::from("test2.png");
        x_to_png(path, target);
        assert!(PathBuf::from("test2.png").exists());
    }

    #[test]
    fn test_correct_transformation() {
        let image1 = image::open("test.png").expect("Failed to open image");
        let image2 = image::open("test2.png").expect("Failed to open image");
        assert_eq!(image1.dimensions(), image2.dimensions());
        for y in 0..image1.height() {
            for x in 0..image1.width() {
                let pixel1 = image1.get_pixel(x, y);
                let pixel2 = image2.get_pixel(x, y);
                assert_eq!(pixel1, pixel2);
            }
        }
    }
}
