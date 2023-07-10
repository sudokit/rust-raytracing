use std::rc::Rc;

use crate::{
    hittable::{HitRecord, Hittable},
    ray::{Point3, Ray, Vec3},
};

pub struct Sphere {
    center: Point3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = ray.origin - self.center;
        let a: f32 = ray.dir.norm_squared();
        let half_b: f32 = oc.dot(&ray.dir);
        let c: f32 = oc.norm_squared() - self.radius * self.radius;

        let disc: f32 = half_b * half_b - a * c; // this is the thing inside the square root when solving a quadratic equation
        if disc < 0.0 {
            return false;
        }
        let sqrtd: f32 = disc.sqrt();

        let mut root: f32 = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, &outward_normal);

        true
    }
}
