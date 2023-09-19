use image::{ImageResult, Rgb, RgbImage};
use rand::{thread_rng, Rng};

const MAX_DEPTH: u32 = 50;

use crate::{
    hit::Hittable,
    interval::Interval,
    ray::Ray,
    vector::{Point, Vector},
    Color,
};

fn write_color(img: &mut RgbImage, x: u32, y: u32, color: Color, samples_pp: u32) {
    let scale: f64 = 1.0 / samples_pp as f64;
    let color: Color = scale * color;

    let intensity: Interval = Interval::new(0.0, 0.9999);

    let rgb = Rgb::from([
        (256.0 * intensity.clamp(color.x)) as u8,
        (256.0 * intensity.clamp(color.y)) as u8,
        (256.0 * intensity.clamp(color.z)) as u8,
    ]);

    img.put_pixel(x, y, rgb);
}

#[derive(Default)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub imw: u32,
    pub camera_center: Point,
    pub samples_pp: u32,
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
        // self.aspect_ratio = 16.0 / 9.0;
        // self.imw = 1920;
        self.samples_pp = 100;
        self.imh = (self.imw as f64 / self.aspect_ratio).clamp(1.0, f64::MAX) as u32;

        self.focal_length = 1.0;
        self.vh = 2.0;
        self.vw = self.vh * (self.imw as f64 / self.imh as f64);

        self.vu = Vector::new(self.vw, 0.0, 0.0);
        self.vv = Vector::new(0.0, -self.vh, 0.0);

        self.pdu = self.vu / self.imw as f64;
        self.pdv = self.vv / self.imh as f64;

        self.viewport_upper_left = self.camera_center
            - Vector::new(0.0, 0.0, self.focal_length)
            - self.vu / 2.0
            - self.vv / 2.0;

        self.pixel_upper_left = self.viewport_upper_left + 0.5 * (self.pdu + self.pdv);
    }

    fn ray_color(ray: &Ray, world: &Vec<&dyn Hittable>, depth: u32) -> Color {
        if depth > MAX_DEPTH {
            return Color::new(0.0, 0.0, 0.0);
        }

        for object in world.into_iter() {
            if let Some(hit) = object.hit(ray, 0.001, f64::INFINITY) {
                // let dir: Vector = Vector::random_on_hemisphere(&hit.normal);
                let dir: Vector = hit.normal + Vector::random_unit();

                return 0.5 * Self::ray_color(&Ray::new(hit.hit_point, dir), world, depth + 1);
            }
        }

        let unit = ray.direction.unit();
        let a = 0.5 * (unit.y + 1.0);

        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    fn generate_ray(&self, x: u32, y: u32) -> Ray {
        let p_center: Point = self.pixel_upper_left + (x as f64 * self.pdu + y as f64 * self.pdv);

        let mut rng = thread_rng();

        let sample_center: Point =
            p_center + rng.gen_range(-0.5..0.5) * self.pdu + rng.gen_range(-0.5..0.5) * self.pdv;

        let direction = sample_center - self.camera_center;

        Ray::new(self.camera_center, direction)
    }

    pub fn render(&mut self, world: &Vec<&dyn Hittable>) -> ImageResult<()> {
        self.initialize();

        let mut img = RgbImage::new(self.imw, self.imh);

        for i in 0..self.imh {
            for j in 0..self.imw {
                let mut color_sum = Color::default();

                for _ in 0..self.samples_pp {
                    color_sum += Self::ray_color(&self.generate_ray(j, i), &world, 0);
                }

                write_color(&mut img, j, i, color_sum, self.samples_pp);
            }
        }

        img.save("test.png")?;

        Ok(())
    }
}
