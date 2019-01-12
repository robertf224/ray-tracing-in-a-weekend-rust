use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::hitable::Hit;
use crate::materials::material::{ Material, Scatter };
use crate::materials::utils;

pub struct Diffuse {
    albedo: Vec3,
}

impl Diffuse {
    pub fn new(albedo: Vec3) -> Diffuse {
        Diffuse { albedo }
    }
}

impl Material for Diffuse {
    fn scatter(&self, _ray: Ray, hit: &Hit) -> Option<Scatter> {
        let tangent_unit_sphere_center = hit.point + hit.normal;
        let target = tangent_unit_sphere_center + utils::random_point_in_unit_sphere();
        let scattered_ray = Ray::new(hit.point, target - hit.point);
        return Some(Scatter {
            scattered_ray,
            attenuation: self.albedo
        });
    }
}
