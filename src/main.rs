mod vec3;
mod ray;

use crate::vec3::Vec3;
use crate::ray::Ray;

fn main() {
    let nx = 200;
    let ny = 100;
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let camera = Vec3::new(0.0, 0.0, 0.0);
    println!("P3 {} {} 255", nx, ny);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = (i as f64) / (nx as f64);
            let v = (j as f64) / (ny as f64);
            let r = Ray::new(camera, lower_left_corner + horizontal.scale(u) + vertical.scale(v));
            let color = color(r);
            let ir = (color.x * 255.0) as u32;
            let ig = (color.y * 255.0) as u32;
            let ib = (color.z * 255.0) as u32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}

fn color(ray: Ray) -> Vec3 {
    if hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, ray) {
        return Vec3::new(1.0, 0.0, 0.0);
    }

    // Get unit vector so -1.0 < y < 1.0
    let unit_direction = ray.direction.unit();
    // Some algebra so that 0.0 < t < 1.0
    let t = 0.5 * (unit_direction.y + 1.0);
    
    let white = Vec3::new(1.0, 1.0, 1.0);
    let blue = Vec3::new(0.5, 0.7, 1.0);
    return white.scale(1.0 - t) + blue.scale(t);
}

fn hit_sphere(center: Vec3, radius: f64, ray: Ray) -> bool {
    let a = ray.direction.dot(ray.direction);
    let b = 2.0 * ray.direction.dot(ray.origin - center);
    let c = (ray.origin - center).dot(ray.origin - center) - radius*radius;
    let discriminant = b*b - 4.0*a*c;
    return discriminant > 0.0;
}
