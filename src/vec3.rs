use std::ops::Add;
use std::ops::Sub;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn length(&self) -> f64 {
        self.dot(*self).sqrt()
    }

    pub fn dot(&self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x - other.z,
            z: self.x * other.y - self.y * other.x
        }
    }

    pub fn scale(&self, scale: f64) -> Vec3 {
        Vec3 {
            x: self.x * scale,
            y: self.y * scale,
            z: self.z * scale
        }
    }

    pub fn unit(&self) -> Vec3 {
        let scale = 1.0 / self.length();
        return self.scale(scale)
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        self + other.scale(-1.0)
    }
}