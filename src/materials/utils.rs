use rand::Rng;
use crate::vec3::Vec3;

pub fn reflect(vector: Vec3, normal: Vec3) -> Vec3 {
    return vector - normal.scale(vector.dot(normal)).scale(2.0);
}

pub fn refract(vector: Vec3, outside_refractive_index: f64, normal: Vec3, inside_refractive_index: f64) -> Option<Vec3> {
    let unit_vector = vector.unit();
    let unit_normal = normal.unit();

    // Convert variables to symbols that match derivation
    let d = unit_vector;
    let N = unit_normal;
    let n = outside_refractive_index;
    let np = inside_refractive_index;

    let cos_theta = -d.dot(N);

    let discriminant = 1.0 - (n / np).powi(2) * (1.0 - cos_theta.powi(2));
    if discriminant < 0.0 {
        return None;
    }

    let dp = d.scale(n / np) + N.scale((n / np) * cos_theta - discriminant.sqrt());
    return Some(dp);
}

pub fn schlick(vector: Vec3, outside_refractive_index: f64, normal: Vec3, inside_refractive_index: f64) -> f64 {
    let unit_vector = vector.unit();
    let unit_normal = normal.unit();

    // Convert variables to symbols that match derivation
    let d = unit_vector;
    let N = unit_normal;
    let n = outside_refractive_index;
    let np = inside_refractive_index;
    let R0 = ((n - np) / (n + np)).powi(2);

    let cos_theta = -d.dot(N);
    if n <= np {
        return R0 + (1.0 - R0) * (1.0 - cos_theta).powi(5);
    } else {
        let cos_theta_p = (1.0 - (n / np).powi(2) + (n / np).powi(2) * cos_theta.powi(2)).sqrt();
        return R0 + (1.0 - R0) * (1.0 - cos_theta_p).powi(5);
    }
}

pub fn random_point_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let point = Vec3::new(rng.gen(), rng.gen(), rng.gen()).scale(2.0) - Vec3::new(1.0, 1.0, 1.0);
        if point.length() < 1.0 {
            return point;
        }
    }
}