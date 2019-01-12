use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::hitable::Hit;
use crate::materials::material::{ Material, Scatter };
use crate::materials::utils;

pub struct Metal {
    albedo: Vec3,
    fuzz: f64
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Metal {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: Ray, hit: &Hit) -> Option<Scatter> {
        let reflected = utils::reflect(ray.direction.unit(), hit.normal);
        let scattered_ray = Ray::new(hit.point, reflected + utils::random_point_in_unit_sphere().scale(self.fuzz));
        return match scattered_ray.direction.dot(hit.normal) > 0.0 {
            true => Some(Scatter {
                scattered_ray,
                attenuation: self.albedo
            }),
            false => None
        };
    }
}