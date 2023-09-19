mod camera;
mod hit;
mod interval;
mod ray;
mod sphere;
mod vector;

use camera::Camera;
use hit::Hittable;
use sphere::Sphere;
use vector::{Point, Vector};

pub type Color = vector::Vector;

fn main() {
    let mut world: Vec<&dyn Hittable> = Vec::new();
    let s = Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5);
    let s2 = Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0);

    world.push(&s);
    world.push(&s2);

    let mut cam: Camera = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.imw = 1920;
    cam.camera_center = Point::new(0.0, 0.0, 0.0);

    cam.render(&world).unwrap();
}
