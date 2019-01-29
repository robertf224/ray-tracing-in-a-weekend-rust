use std::f64;
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
    pub fn new(look_from: Vec3, look_at: Vec3, vup: Vec3, fov_theta: f64, aspect: f64) -> Camera {
        let theta = fov_theta * f64::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (look_from - look_at).unit();
        let u = (vup.cross(w)).unit();
        let v = (w.cross(u)).unit();
        let anchor = look_from - w - u.scale(half_width) - v.scale(half_height);
        let horizontal = u.scale(2.0 * half_width);
        let vertical = v.scale(2.0 * half_height);
        Camera { origin: look_from, anchor, horizontal, vertical }
    }

    pub fn ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.anchor + self.horizontal.scale(u) + self.vertical.scale(v) - self.origin)
    }
}