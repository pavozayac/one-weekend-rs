mod hit;
mod interval;
mod ray;
mod sphere;
mod vector;

use std::error::Error;

use hit::Hittable;
use image::{self, io::Reader, Pixel, Rgb, RgbImage};
use ray::Ray;
use sphere::Sphere;
use vector::{Point, Vector};

type Color = vector::Vector;

fn write_color(img: &mut RgbImage, x: u32, y: u32, color: Color) {
    let color = color * 255.99;
    let rgb = Rgb::from([color.x as u8, color.y as u8, color.z as u8]);
    img.put_pixel(x, y, rgb);
}

fn ray_color(ray: &Ray, world: &Vec<&dyn Hittable>) -> Color {
    for object in world.into_iter() {
        if let Some(hit) = object.hit(ray, 0.0, f64::MAX) {
            return 0.5 * Color::new(hit.normal.x + 1.0, hit.normal.y + 1.0, hit.normal.z + 1.0);
        }
    }

    let unit = ray.direction.unit();
    let a = 0.5 * (unit.y + 1.0);

    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn main() -> Result<(), Box<dyn Error>> {
    let aspect_ratio: f64 = 16.0 / 9.0;
    let imw: u32 = 1920;
    let imh: u32 = (imw as f64 / aspect_ratio).clamp(1.0, f64::MAX) as u32;

    let focal_length: f64 = 1.0;
    let vh = 2.0;
    let vw = vh * (imw as f64 / imh as f64);
    let camera_center = Point::new(0.0, 0.0, 0.0);

    let vu = Vector::new(vw, 0.0, 0.0);
    let vv = Vector::new(0.0, -vh, 0.0);

    let pdu = vu / imw as f64;
    let pdv = vv / imh as f64;

    let viewport_upper_left =
        camera_center - Vector::new(0.0, 0.0, focal_length) - vu / 2.0 - vv / 2.0;

    let pixel_upper_left = viewport_upper_left + 0.5 * (pdu + pdv);

    let mut img = RgbImage::new(imw, imh);

    let mut world: Vec<&dyn Hittable> = Vec::new();
    let s = Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5);
    let s2 = Sphere::new(Point::new(0.0, -100.0, -1.0), 100.0);

    world.push(&s);
    world.push(&s2);

    for i in 0..imh {
        println!("Lines left: {}", imh - i);
        for j in 0..imw {
            let pixel_center = pixel_upper_left + pdu * j as f64 + pdv * i as f64;
            let ray_dir = pixel_center - camera_center;

            let r = Ray::new(camera_center, ray_dir);

            write_color(&mut img, j, i, ray_color(&r, &world));
        }
    }

    img.save("test.png")?;

    Ok(())
}
