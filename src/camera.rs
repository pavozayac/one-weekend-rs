use image::{ImageResult, Rgb, RgbImage};

use crate::{
    hit::Hittable,
    ray::Ray,
    vector::{Point, Vector},
    Color,
};

fn write_color(img: &mut RgbImage, x: u32, y: u32, color: Color) {
    let color = color * 255.99;
    let rgb = Rgb::from([color.x as u8, color.y as u8, color.z as u8]);
    img.put_pixel(x, y, rgb);
}

#[derive(Default)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub imw: u32,
    pub camera_center: Point,
    imh: u32,
    focal_length: f64,
    vh: f64,
    vw: f64,
    vu: Vector,
    vv: Vector,
    pdu: Vector,
    pdv: Vector,
    viewport_upper_left: Vector,
    pixel_upper_left: Vector,
}

impl Camera {
    fn initialize(&mut self) {
        self.aspect_ratio = 16.0 / 9.0;
        self.imw = 1920;
        self.imh = (self.imw as f64 / self.aspect_ratio).clamp(1.0, f64::MAX) as u32;

        self.focal_length = 1.0;
        self.vh = 2.0;
        self.vw = self.vh * (self.imw as f64 / self.imh as f64);

        self.vu = Vector::new(self.vw, 0.0, 0.0);
        self.vv = Vector::new(0.0, -self.vh, 0.0);

        let pdu = self.vu / self.imw as f64;
        let pdv = self.vv / self.imh as f64;

        self.viewport_upper_left = self.camera_center
            - Vector::new(0.0, 0.0, self.focal_length)
            - self.vu / 2.0
            - self.vv / 2.0;

        self.pixel_upper_left = self.viewport_upper_left + 0.5 * (pdu + pdv);
    }

    fn ray_color(ray: &Ray, world: &Vec<&dyn Hittable>) -> Color {
        for object in world.into_iter() {
            if let Some(hit) = object.hit(ray, 0.0, f64::MAX) {
                return 0.5
                    * Color::new(hit.normal.x + 1.0, hit.normal.y + 1.0, hit.normal.z + 1.0);
            }
        }

        let unit = ray.direction.unit();
        let a = 0.5 * (unit.y + 1.0);

        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    pub fn render(&mut self, world: &Vec<&dyn Hittable>) -> ImageResult<()> {
        self.initialize();

        let mut img = RgbImage::new(self.imw, self.imw);

        for i in 0..self.imw {
            println!("Lines left: {}", self.imh - i);
            for j in 0..self.imw {
                let pixel_center =
                    self.pixel_upper_left + self.pdu * j as f64 + self.pdv * i as f64;
                let ray_dir = pixel_center - self.camera_center;

                let r = Ray::new(self.camera_center, ray_dir);

                write_color(&mut img, j, i, Self::ray_color(&r, &world));
            }
        }

        img.save("test.png")?;

        Ok(())
    }
}
