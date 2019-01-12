use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::materials::material::Material;

pub struct Hit<'a> {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: &'a Box<Material>
}

pub trait Hitable {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}

pub fn closest(objects: &Vec<Box<Hitable>>, ray: Ray, t_min: f64, t_max: f64) -> Option<Hit> {
    let mut closest: Option<Hit> = None;
    let mut closest_distance = t_max;
    for object in objects.iter() {
        let hit = object.hit(ray, t_min, closest_distance);
        if let Some(hit) = hit {
            closest_distance = hit.t;
            closest = Some(hit);
        }
    }
    return closest;
}