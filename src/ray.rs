// use crate::vec::{Point3, Vec3};
use nalgebra::Vector3;

pub type Vec3 = Vector3<f32>;
pub type Point3 = Vector3<f32>; // 3D point
pub type Color = Vector3<f32>; // RGB color

pub struct Ray {
    pub origin: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3) -> Self {
        Self { origin, dir }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        // self.origin + self.dir * t
        self.origin + self.dir * t
    }

    pub fn default() -> Self {
        Self {
            origin: Point3::new(0.0, 0.0, 0.0),
            dir: Vec3::new(0.0, 0.0, 0.0),
        }
    }
}
