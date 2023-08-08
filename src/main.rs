mod vector;

use std::error::Error;

use image::{self, io::Reader, Pixel, Rgb, RgbImage};

type Color = vector::Vector;

fn write_color(img: &mut RgbImage, x: u32, y: u32, color: Color) {
    let rgb = Rgb::from([color.x as u8, color.y as u8, color.z as u8]);
    img.put_pixel(x, y, rgb);
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut img = RgbImage::new(256, 256);

    for i in 0..256 {
        println!("Lines left: {}", 256 - i);
        for j in 0..256 {
            let r = j;
            let g = i;
            let b = 0;

            let color = Color::from([j, i, 0]);
            write_color(&mut img, j, i, color);
        }
    }

    img.save("test.png")?;

    Ok(())
}
