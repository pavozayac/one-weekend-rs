use crate::{ray::Ray, vector::Point, Vector};

pub struct Hit {
    pub hit_point: Point,
    pub normal: Vector,
    pub time: f64,
    pub front_face: bool,
}

impl Hit {
    pub fn new(hit_point: Point, normal: Vector, time: f64) -> Self {
        Hit {
            hit_point,
            normal,
            time,
            front_face: true,
        }
    }

    pub fn with_face_normal(
        hit_point: Point,
        time: f64,
        ray: &Ray,
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
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}
