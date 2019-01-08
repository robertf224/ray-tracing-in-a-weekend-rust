use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Camera {
    // Eye
    origin: Vec3,

    // Screen
    anchor: Vec3,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    pub fn new(origin: Vec3, anchor: Vec3, horizontal: Vec3, vertical: Vec3) -> Camera {
        Camera { anchor, horizontal, vertical, origin }
    }

    pub fn ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.anchor + self.horizontal.scale(u) + self.vertical.scale(v) - self.origin)
    }
}