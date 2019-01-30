use std::f64;
use rand::Rng;
use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Camera {
    // Eye
    origin: Vec3,
    lens_radius: f64,
    u: Vec3, v: Vec3,

    // Screen
    anchor: Vec3,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, vup: Vec3, fov_theta: f64, aspect: f64, aperture: f64, focus_dist: f64) -> Camera {
        let theta = fov_theta * f64::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (look_from - look_at).unit();
        let u = (vup.cross(w)).unit();
        let v = (w.cross(u)).unit();
        let anchor = look_from - w.scale(focus_dist) - u.scale(half_width * focus_dist) - v.scale(half_height * focus_dist);
        let horizontal = u.scale(2.0 * half_width * focus_dist);
        let vertical = v.scale(2.0 * half_height * focus_dist);
        Camera { origin: look_from, anchor, horizontal, vertical, lens_radius: aperture / 2.0, u, v }
    }

    pub fn ray(&self, s: f64, t: f64) -> Ray {
        let rd = random_point_in_unit_circle().scale(self.lens_radius);
        let effective_origin = self.origin + self.u.scale(rd.x) + self.v.scale(rd.y);
        Ray::new(effective_origin, self.anchor + self.horizontal.scale(s) + self.vertical.scale(t) - effective_origin)
    }
}

fn random_point_in_unit_circle() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let point = Vec3::new(rng.gen(), rng.gen(), 0.0).scale(2.0) - Vec3::new(1.0, 1.0, 0.0);
        if point.length() < 1.0 {
            return point;
        }
    }
}