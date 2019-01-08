mod vec3;
mod ray;
mod hitable;
mod sphere;

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::hitable::{ Hitable };
use std::f64;

fn main() {
    let nx = 200;
    let ny = 100;
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let camera = Vec3::new(0.0, 0.0, 0.0);
    println!("P3 {} {} 255", nx, ny);

    let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    let ground = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0);
    let world: Vec<Box<Hitable>> = vec![Box::new(sphere), Box::new(ground)];

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = (i as f64) / (nx as f64);
            let v = (j as f64) / (ny as f64);
            let r = Ray::new(camera, lower_left_corner + horizontal.scale(u) + vertical.scale(v));

            let color = color(r, &world);
            let ir = (color.x * 255.0) as u32;
            let ig = (color.y * 255.0) as u32;
            let ib = (color.z * 255.0) as u32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}

fn color(ray: Ray, world: &Vec<Box<Hitable>>) -> Vec3 {
    let hit = hitable::closest(world, ray, 0.0, f64::MAX);
    if let Some(hit) = hit {
        // Some algebra so that, for normal, 0.0 < x,y,z < 1.0
        return (Vec3::new(1.0, 1.0, 1.0) + hit.normal).scale(0.5);
    } else {
        // Get unit vector so -1.0 < y < 1.0
        let unit_direction = ray.direction.unit();
        // Some algebra so that 0.0 < t < 1.0
        let t = 0.5 * (unit_direction.y + 1.0);
        
        let white = Vec3::new(1.0, 1.0, 1.0);
        let blue = Vec3::new(0.5, 0.7, 1.0);
        return white.scale(1.0 - t) + blue.scale(t);
    }
}
