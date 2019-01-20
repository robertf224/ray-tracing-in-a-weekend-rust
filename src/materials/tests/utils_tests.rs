use crate::vec3::Vec3;
use crate::materials::utils;

#[test]
fn refraction() {
    let vector = Vec3::new(1.0, -1.0, 0.0);
    let normal = Vec3::new(0.0, 1.0, 0.0);
    let outside_refractive_index = 1.0;
    let inside_refractive_index = 1.5;

    let refracted = utils::refract(vector, outside_refractive_index, normal, inside_refractive_index);
    let refraction_matches_expectation = match refracted {
        Some(refracted) => (refracted.x - 0.4714).abs() < 0.01 && (refracted.y + 0.8819).abs() < 0.01,
        None => false
    };
    assert!(refraction_matches_expectation);
}
