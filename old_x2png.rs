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

    img.save(target.to_str().unwrap())
        .expect("Failed to save image");
    println!("Image created successfully");