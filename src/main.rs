mod camera;
mod hittables;
mod interval;
mod materials;
mod ray;
mod vector;

use std::rc::Rc;

use camera::Camera;
use hittables::{Hittable, Sphere};
use materials::{Diffuse, Mirror};
use vector::{Point, Vector};

pub type Color = vector::Vector;

fn main() {
    let diff: Rc<Diffuse> = Rc::new(Diffuse::new(Color::new(0.5, 0.5, 0.5)));
    let reddish: Rc<Diffuse> = Rc::new(Diffuse::new(Color::new(1.0, 0.2, 0.2)));
    let mirror: Rc<Mirror> = Rc::new(Mirror::new());

    let mut world: Vec<&dyn Hittable> = Vec::new();
    let s = Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5, mirror.clone());
    let s1 = Sphere::new(Point::new(2.0, 0.0, 0.0), 0.5, reddish.clone());
    let s2 = Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0, diff.clone());

    world.push(&s);
    world.push(&s1);
    world.push(&s2);

    let mut cam: Camera = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.imw = 800;
    cam.camera_center = Point::new(0.0, 0.0, 0.0);

    cam.render(&world).unwrap();
}
