use crate::{ray::Ray, Color};

use super::Material;

pub struct Mirror {}

impl Mirror {
    pub fn new() -> Self {
        Mirror {}
    }
}

impl Material for Mirror {
    fn scatter(
        &self,
        incoming: &crate::ray::Ray,
        hit: &crate::hittables::Hit,
    ) -> Option<super::ScatterEffect> {
        Some(super::ScatterEffect {
            scattered_ray: Ray::new(hit.hit_point, incoming.direction.reflect(&hit.normal)),
            attenuation: Color::new(1.0, 1.0, 1.0),
        })
    }
}
