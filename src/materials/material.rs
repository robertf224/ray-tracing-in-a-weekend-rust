use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::hitable::Hit;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Scatter {
    pub scattered_ray: Ray,
    pub attenuation: Vec3 // Strength retention of each color after scatter
}

pub trait Material {
    fn scatter(&self, ray: Ray, hit: &Hit) -> Option<Scatter>;
}
