use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Hit {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3
}

pub trait Hitable {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}

pub fn closest(objects: &Vec<Box<Hitable>>, ray: Ray, t_min: f64, t_max: f64) -> Option<Hit> {
    let mut closest: Option<Hit> = None;
    for object in objects.iter() {
        let hit = object.hit(ray, t_min, match closest { None => t_max, Some(hit) => hit.t });
        if let Some(hit) = hit {
            closest = Some(hit);
        }
    }
    return closest;
}