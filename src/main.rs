mod vec3;
mod ray;
mod hitable;
mod sphere;
mod camera;
pub mod materials;

use indicatif::ProgressBar;
use std::f64;
use rand::Rng;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::hitable::{ Hitable };
use crate::camera::Camera;
use crate::materials::material::Material;
use crate::materials::matte::Matte;
use crate::materials::metal::Metal;
use crate::materials::glass::Glass;

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;
    println!("P3 {} {} 255", nx, ny);

    let origin = Vec3::new(-2.0, 2.0, 1.0);
    let focus = Vec3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let camera = Camera::new(origin, focus, vup, 90.0, (nx / ny) as f64);

    let matte_sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Box::new(Matte::new(Vec3::new(0.8, 0.3, 0.3))));
    let ground = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Box::new(Matte::new(Vec3::new(0.8, 0.8, 0.0))));
    let metal_sphere = Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, Box::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.3)));
    let glass_sphere = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, Box::new(Glass::new(1.5)));
    let world: Vec<Box<Hitable>> = vec![Box::new(matte_sphere), Box::new(ground), Box::new(metal_sphere), Box::new(glass_sphere)];

    let progress = ProgressBar::new(nx * ny);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            for s in 0..ns {
                let u = (i as f64 + s as f64 / ns as f64) / (nx as f64);
                let v = (j as f64 + s as f64 / ns as f64) / (ny as f64);
                let r = camera.ray(u, v);
                pixel_color = pixel_color + color(r, &world, 0);
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
            progress.inc(1);
        }
    }

    progress.finish();
}

fn color(ray: Ray, world: &Vec<Box<Hitable>>, depth: u32) -> Vec3 {
    let hit = hitable::closest(world, ray, 0.001, f64::MAX);
    if let Some(hit) = hit {
        if depth > 50 {
            return Vec3::new(0.0, 0.0, 0.0);
        }

        let scatter = hit.material.scatter(ray, &hit);
        return match scatter {
            Some(scatter) => color(scatter.scattered_ray, world, depth + 1).product(scatter.attenuation),
            None => Vec3::new(0.0, 0.0, 0.0),
        };
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

fn random_world() -> Vec<Box<Hitable>> {
    let ground = Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Box::new(Matte::new(Vec3::new(0.5, 0.5, 0.5))));
    let mut world: Vec<Box<Hitable>> = vec![Box::new(ground)];

    let mut rng = rand::thread_rng();
    let radius = 0.2;
    for i in 1..12 {
        for j in 1..12 {
            let x = rng.gen::<f64>() * (i as f64);
            let z = rng.gen::<f64>() * (j as f64);
            let center = Vec3::new(x, 0.2, z);

            let material_choice = rng.gen_range(1, 4);
            let material: Box<Material> = match material_choice {
                1 => Box::new(Matte::new(Vec3::new(rng.gen::<f64>()*rng.gen::<f64>(), rng.gen::<f64>()*rng.gen::<f64>(), rng.gen::<f64>()*rng.gen::<f64>()))),
                2 => Box::new(Metal::new(Vec3::new(0.5*(1.0 + rng.gen::<f64>()), 0.5*(1.0 + rng.gen::<f64>()), 0.5*(1.0 + rng.gen::<f64>())), 0.5*rng.gen::<f64>())),
                3 => Box::new(Glass::new(1.5)),
                _ => Box::new(Glass::new(1.5)),
            };

            let sphere = Sphere::new(center, radius, material);
            world.push(Box::new(sphere));            
        }
    }

    let big_matte_sphere = Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, Box::new(Matte::new(Vec3::new(0.4, 0.2, 0.1))));
    let big_metal_sphere = Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, Box::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)));
    let big_glass_sphere = Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Box::new(Glass::new(1.5)));

    world.push(Box::new(big_matte_sphere));
    world.push(Box::new(big_metal_sphere));
    world.push(Box::new(big_glass_sphere));

    return world;
}
