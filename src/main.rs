mod camera;
mod hittables;
mod interval;
mod materials;
mod ray;
mod vector;

use std::rc::Rc;

use camera::Camera;
use hittables::{Hittable, Sphere};
use materials::Diffuse;
use vector::{Point, Vector};

pub type Color = vector::Vector;

fn main() {
    let diff: Rc<Diffuse> = Rc::new(Diffuse::new(Color::new(0.5, 0.5, 0.5)));

    let mut world: Vec<&dyn Hittable> = Vec::new();
    let s = Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5, diff.clone());
    let s2 = Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0, diff.clone());

    world.push(&s);
    world.push(&s2);

    let mut cam: Camera = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.imw = 800;
    cam.camera_center = Point::new(0.0, 0.0, 0.0);

    cam.render(&world).unwrap();
}
