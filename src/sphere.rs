use rand::Rng;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hitable::{ Hitable, Hit };

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere { center, radius }
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
            return Some(Hit { t: solution_one, point, normal });
        }

        let solution_two = (-b + discriminant.sqrt()) / (2.0 * a);
        if t_min < solution_two && solution_two < t_max {
            let point = ray.point(solution_two);
            let normal = (point - self.center).unit();
            return Some(Hit { t: solution_two, point, normal });
        }
        
        return None;
    }
}

pub fn random_point_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let point = Vec3::new(rng.gen(), rng.gen(), rng.gen()).scale(2.0) - Vec3::new(1.0, 1.0, 1.0);
        if point.length() < 1.0 {
            return point;
        }
    }
}

