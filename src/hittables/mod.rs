mod sphere;

use std::rc::Rc;

use crate::{materials::Material, ray::Ray, vector::Point, Vector};
pub use sphere::*;

pub struct Hit {
    pub hit_point: Point,
    pub normal: Vector,
    pub time: f64,
    pub front_face: bool,
    pub material: Rc<dyn Material>,
}

impl Hit {
    pub fn new(hit_point: Point, normal: Vector, time: f64, material: Rc<dyn Material>) -> Self {
        Hit {
            hit_point,
            normal,
            time,
            front_face: true,
            material: material,
        }
    }

    pub fn with_face_normal(
        hit_point: Point,
        time: f64,
        ray: &Ray,
        material: Rc<dyn Material>,
        outward_normal: Vector,
    ) -> Self {
        let front_face: bool = ray.direction.dot(&outward_normal) < 0.0;

        Hit {
            hit_point,
            normal: if front_face {
                outward_normal
            } else {
                -outward_normal
            },
            time,
            front_face,
            material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}
