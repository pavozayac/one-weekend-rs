use crate::{hittables::Hit, ray::Ray, Color};
mod lambertian_diffuse;
pub use lambertian_diffuse::*;

pub struct ScatterEffect {
    pub scattered_ray: Ray,
    pub attenuation: Color,
}

pub trait Material {
    fn scatter(&self, incoming: &Ray, hit: &Hit) -> Option<ScatterEffect>;
}
