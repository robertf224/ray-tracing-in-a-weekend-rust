use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hitable::{ Hitable, Hit };
use crate::materials::material::Material;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Box<Material>
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Box<Material>) -> Sphere {
        Sphere { center, radius, material }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(ray.origin - self.center);
        let c = (ray.origin - self.center).dot(ray.origin - self.center) - self.radius*self.radius;
        let discriminant = b*b - 4.0*a*c;
        if discriminant < 0.0 {
            return None;
        }

        let solution_one = (-b - discriminant.sqrt()) / (2.0 * a);
        if t_min < solution_one && solution_one < t_max {
            let point = ray.point(solution_one);
            let normal = (point - self.center).unit();
            return Some(Hit { t: solution_one, point, normal, material: &self.material });
        }

        let solution_two = (-b + discriminant.sqrt()) / (2.0 * a);
        if t_min < solution_two && solution_two < t_max {
            let point = ray.point(solution_two);
            let normal = (point - self.center).unit();
            return Some(Hit { t: solution_two, point, normal, material: &self.material });
        }
        
        return None;
    }
}


