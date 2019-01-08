mod vec3;
mod ray;
mod hitable;
mod sphere;
mod camera;

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::hitable::{ Hitable };
use crate::camera::Camera;
use std::f64;

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;
    println!("P3 {} {} 255", nx, ny);

    let anchor = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let camera = Camera::new(origin, anchor, horizontal, vertical);

    let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    let ground = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0);
    let world: Vec<Box<Hitable>> = vec![Box::new(sphere), Box::new(ground)];

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            for s in 0..ns {
                let u = (i as f64 + s as f64 / ns as f64) / (nx as f64);
                let v = (j as f64 + s as f64 / ns as f64) / (ny as f64);
                let r = camera.ray(u, v);
                pixel_color = pixel_color + color(r, &world);
            }
            pixel_color = pixel_color.scale(1.0 / ns as f64);
            // Gamma correction (?)
            pixel_color.x = pixel_color.x.sqrt();
            pixel_color.y = pixel_color.y.sqrt();
            pixel_color.z = pixel_color.z.sqrt();

            let ir = (pixel_color.x * 255.0) as u32;
            let ig = (pixel_color.y * 255.0) as u32;
            let ib = (pixel_color.z * 255.0) as u32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}

fn color(ray: Ray, world: &Vec<Box<Hitable>>) -> Vec3 {
    let hit = hitable::closest(world, ray, 0.001, f64::MAX);
    if let Some(hit) = hit {
        let tangent_unit_sphere_center = hit.point + hit.normal;
        let target = tangent_unit_sphere_center + sphere::random_point_in_unit_sphere();
        return color(Ray::new(hit.point, target - hit.point), world).scale(0.5);
    } else {
        // Background
        // Get unit vector so -1.0 < y < 1.0
        let unit_direction = ray.direction.unit();
        // Some algebra so that 0.0 < t < 1.0
        let t = 0.5 * (unit_direction.y + 1.0);
        
        let white = Vec3::new(1.0, 1.0, 1.0);
        let blue = Vec3::new(0.5, 0.7, 1.0);
        return white.scale(1.0 - t) + blue.scale(t);
    }
}
