use crate::{ray::Ray, vector::Point, Vector};

pub struct Hit {
    pub hit_point: Point,
    pub normal: Vector,
    pub time: f64,
}

impl Hit {
    pub fn new(hit_point: Point, normal: Vector, time: f64) -> Self {
        Hit {
            hit_point,
            normal,
            time,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}
