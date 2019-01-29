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
    let nx = 1200;
    let ny = 800;
    let ns = 100;
    println!("P3 {} {} 255", nx, ny);

    let origin = Vec3::new(13.0, 2.0, 3.0);
    let focus = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(origin, focus, vup, 20.0, (nx / ny) as f64, aperture, dist_to_focus);

    let world = random_world();

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
    for i in -10..10 {
        for j in -10..10 {
            let x = rng.gen::<f64>() * (i as f64);
            let z = rng.gen::<f64>() * (j as f64);
            let center = Vec3::new(x, 0.2, z);

            let material_choice: f64 = rng.gen();
            let material: Box<Material>;
            if material_choice < 0.8 {
                material = Box::new(Matte::new(Vec3::new(rng.gen::<f64>()*rng.gen::<f64>(), rng.gen::<f64>()*rng.gen::<f64>(), rng.gen::<f64>()*rng.gen::<f64>())));
            } else if material_choice < 0.95 {
                material = Box::new(Metal::new(Vec3::new(0.5*(1.0 + rng.gen::<f64>()), 0.5*(1.0 + rng.gen::<f64>()), 0.5*(1.0 + rng.gen::<f64>())), 0.5*rng.gen::<f64>()));
            } else {
                material = Box::new(Glass::new(1.5));
            }

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
