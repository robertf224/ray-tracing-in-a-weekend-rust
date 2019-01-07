mod vec3;

use crate::vec3::Vec3;

fn main() {
    let nx = 100;
    let ny = 100;
    println!("P3 {} {} 255", nx, ny);
    for i in 0..nx {
        for j in 0..ny {
            let color = Vec3::new((i as f64) / (nx as f64), (j as f64) / (ny as f64), 0.2);
            let ir = (color.x * 255.0) as u32;
            let ig = (color.y * 255.0) as u32;
            let ib = (color.z * 255.0) as u32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
