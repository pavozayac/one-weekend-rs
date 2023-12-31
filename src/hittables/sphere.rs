use std::rc::Rc;

use crate::{
    materials::Material,
    vector::{Point, Vector},
};

use super::{Hit, Hittable};

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, material: Rc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            None
        } else {
            let t = (-half_b - discriminant.sqrt()) / a;

            if t > t_max || t < t_min {
                None
            } else {
                let outward_normal: Vector = (ray.at(t) - self.center).unit();

                Some(Hit::with_face_normal(
                    ray.at(t),
                    t,
                    &ray,
                    self.material.clone(),
                    outward_normal,
                ))
            }
        }
    }
}
