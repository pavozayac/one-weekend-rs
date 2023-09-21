use crate::{hittables::Hit, ray::Ray, vector::Vector, Color};

use super::{Material, ScatterEffect};

pub struct Diffuse {
    pub albedo: Color,
}

impl Diffuse {
    pub fn new(albedo: Color) -> Self {
        Diffuse { albedo }
    }
}

impl Material for Diffuse {
    fn scatter(&self, incoming: &crate::ray::Ray, hit: &Hit) -> Option<super::ScatterEffect> {
        let dir: Vector = hit.normal + Vector::random_unit();

        Some(ScatterEffect {
            scattered_ray: Ray::new(hit.hit_point, dir),
            attenuation: self.albedo,
        })
    }
}
