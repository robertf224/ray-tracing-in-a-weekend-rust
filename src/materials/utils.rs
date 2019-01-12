use rand::Rng;
use crate::vec3::Vec3;

pub fn reflect(vector: Vec3, normal: Vec3) -> Vec3 {
    return vector - normal.scale(vector.dot(normal)).scale(2.0);
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