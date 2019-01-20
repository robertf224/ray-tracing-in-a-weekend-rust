use rand::Rng;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::hitable::Hit;
use crate::materials::material::{ Material, Scatter };
use crate::materials::utils;

pub struct Glass {
    refraction_index: f64
}

impl Glass {
    pub fn new(refraction_index: f64) -> Glass {
        Glass { refraction_index }
    }
}

impl Material for Glass {
    fn scatter(&self, ray: Ray, hit: &Hit) -> Option<Scatter> {
        let inside_refractive_index: f64;
        let outside_refractive_index: f64;
        let outward_normal: Vec3;
        if ray.direction.dot(hit.normal) > 0.0 {
            // Internal ray
            outside_refractive_index = self.refraction_index;
            inside_refractive_index = 1.0;
            outward_normal = hit.normal.scale(-1.0);
        } else {
            // External ray
            outside_refractive_index = 1.0;
            inside_refractive_index = self.refraction_index;
            outward_normal = hit.normal;
        }

        let scattered_ray: Ray;
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let refracted = utils::refract(ray.direction, outside_refractive_index, outward_normal, inside_refractive_index);
        if let Some(refracted) = refracted {
            let reflection_probability = utils::schlick(ray.direction, outside_refractive_index, outward_normal, inside_refractive_index);
            if reflection_probability > rand::thread_rng().gen() {
                let reflected = utils::reflect(ray.direction.unit(), hit.normal);
                scattered_ray = Ray::new(hit.point, reflected);
            } else {
                scattered_ray = Ray::new(hit.point, refracted);
            }
        } else {
            // Total internal reflection
            let reflected = utils::reflect(ray.direction.unit(), hit.normal);
            scattered_ray = Ray::new(hit.point, reflected);
        }
        return Some(Scatter { scattered_ray, attenuation } );
    }
}
