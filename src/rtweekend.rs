use crate::ray::Vec3;

// idk if this is needed but might aswell

pub const INFINITY: f32 = std::f32::INFINITY;
pub const PI: f32 = std::f32::consts::PI;

// idk if i need inline but oh well

#[inline(always)]
pub fn degrees_to_radians(degrees: &f32) -> f32 {
    degrees * PI / 180.0
}

#[inline(always)]
pub fn random_f32_range(min: f32, max: f32) -> f32 {
    (max - min) * fastrand::f32() + min
}

#[inline(always)]
pub fn random_vec3() -> Vec3 {
    Vec3::new(fastrand::f32(), fastrand::f32(), fastrand::f32())
}
#[inline(always)]
pub fn random_vec3_range(min: f32, max: f32) -> Vec3 {
    Vec3::new(
        random_f32_range(min, max),
        random_f32_range(min, max),
        random_f32_range(min, max),
    )
}

#[inline(always)]
pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p: Vec3 = random_vec3_range(-1.0, 1.0);
        if p.norm_squared() >= 1.0 {
            continue;
        };
        return p;
    }
}

#[inline(always)]
pub fn random_unit_vec3() -> Vec3 {
    random_in_unit_sphere().normalize()
}

#[inline(always)]
pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let in_unit_sphere: Vec3 = random_in_unit_sphere();
    if in_unit_sphere.dot(&normal) > 0.0 {
        return in_unit_sphere;
    } else {
        return -in_unit_sphere;
    }
}

#[inline(always)]
pub fn vec3_near_zero(vec: &Vec3) -> bool {
    const S: f32 = 1e-8;
    (vec.x.abs() < S) && (vec.y.abs() < S) && (vec.z.abs() < S)
}

#[inline(always)]
pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}

#[inline(always)]
pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}
