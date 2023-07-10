use std::fmt;
use std::ops::{Add, Div, Mul, Sub, SubAssign};
use std::ops::{AddAssign, DivAssign, MulAssign, Neg};

// #[derive(Debug, Copy, Clone)]
// pub struct Vec3 {
//     e: [f32; 3],
// }

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn with_values(x: f32, y: f32, z: f32) -> Self {
        // Vec3 { e: [e0, e1, e2] }
        Self { x, y, z }
    }

    // pub fn x(&self) -> f32 {
    //     self.e[0]
    // }

    // pub fn y(&self) -> f32 {
    //     self.e[1]
    // }

    // pub fn z(&self) -> f32 {
    //     self.e[2]
    // }

    pub fn length_squared(&self) -> f32 {
        // self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
        self.x * self.x + self.y * self.y * self.z * self.z
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
}
impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self::with_values(-self.x, -self.y, -self.z)
    }
}

// AddAssign for Vec3
impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

// SubAssign for Vec3
impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

// MulAssign for Vec3
impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

// DivAssign for Vec3
impl DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, rhs: Vec3) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

// AddAssign for f32
impl AddAssign<f32> for Vec3 {
    fn add_assign(&mut self, rhs: f32) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}

// SubAssign for f32
impl SubAssign<f32> for Vec3 {
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
    }
}

// MulAssign for f32
impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, t: f32) {
        self.x *= t;
        self.y *= t;
        self.z *= t;
    }
}

// DivAssign for f32
impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, t: f32) {
        *self *= 1.0 / t;
    }
}

pub type Point3 = Vec3; // 3D point
pub type Color = Vec3; // RGB color

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

// add for Vec3
impl Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
// sub for Vec3
impl Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
// mul for Vec3
impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}
// div for Vec3
impl Div<Vec3> for Vec3 {
    type Output = Self;

    fn div(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

// add for f32
impl Add<f32> for Vec3 {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}
// sub for f32
impl Sub<f32> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}
// mul for f32
impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
// div for f32
impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

// IMPORTANT: this just became so annyoing when there was a f32 * vec3 so i implement these:

// add for Vec3
impl Add<Vec3> for f32 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self + rhs.x,
            y: self + rhs.y,
            z: self + rhs.z,
        }
    }
}
// sub for Vec3
impl Sub<Vec3> for f32 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self - rhs.x,
            y: self - rhs.y,
            z: self - rhs.z,
        }
    }
}
// mul for Vec3
impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}
// div for Vec3
impl Div<Vec3> for f32 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self / rhs.x,
            y: self / rhs.y,
            z: self / rhs.z,
        }
    }
}

// impl Add<Vec3> for Vec3 {
//     type Output = Self;

// impl Add<Vec3> for Vec3 {
//     type Output = Self;

//     fn add(self, other: Self) -> Self {
//         Self {
//             x: self.x + other.x,
//             y: self.y + other.y,
//             z: self.z + other.z,
//         }
//     }
// }

// impl Sub<Vec3> for Vec3 {
//     type Output = Self;

//     fn sub(self, other: Self) -> Self {
//         Self::with_values(self.x - other.x, self.y - other.y, self.z - other.z)
//     }
// }

// impl Mul<Vec3> for Vec3 {
//     type Output = Self;

//     fn mul(self, other: Self) -> Vec3 {
//         Self::with_values(self.x * other.x, self.y * other.y, self.z * other.z)
//     }
// }

// impl Mul<f32> for Vec3 {
//     type Output = Self;

//     fn mul(self, t: f32) -> Self {
//         Self::with_values(self.x * t, self.y * t, self.y * t)
//     }
// }

// impl Div<f32> for Self {
//     type Output = Self;

//     fn div(self, t: f32) -> Self {
//         self * (1.0 / t)
//     }
// }

pub fn dot(u: Vec3, v: Vec3) -> f32 {
    // u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
    u.x * v.x + u.y * v.y + u.z * v.z
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    // Vec3::with_values(
    //     u.e[1] * v.e[2] - u.e[2] * v.e[1],
    //     u.e[2] * v.e[0] - u.e[0] * v.e[2],
    //     u.e[0] * v.e[1] - u.e[1] * v.e[0],
    // )
    Vec3 {
        x: u.y * v.z - u.z * v.y,
        y: u.z * v.x - u.x * v.z,
        z: u.x * v.y - u.y * v.x,
    }
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}
