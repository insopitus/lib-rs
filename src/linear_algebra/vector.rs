use std::{f32::EPSILON, iter::Sum};

use serde::Deserialize;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

pub fn vec2(x: f32, y: f32) -> Vector2 {
    Vector2::new(x, y)
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn distance_to(&self, v: &Vector2) -> f32 {
        self.distance_to_squared(v).sqrt()
    }
    pub fn distance_to_squared(&self, v: &Vector2) -> f32 {
        let dx = self.x - v.x;
        let dy = self.y - v.y;
        dx * dx + dy * dy
    }
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    /// return a normalized version of this vector
    pub fn normalize(&self) -> Self {
        let len = self.length();
        Self {
            x: self.x / len,
            y: self.y / len,
        }
    }
    pub fn normalize_or_zero(&self) -> Self {
        let len = self.length();
        if len.abs() < EPSILON {
            Self { x: 0.0, y: 0.0 }
        } else {
            Self {
                x: self.x / len,
                y: self.y / len,
            }
        }
    }
    pub fn min(&self, rhs: Self) -> Self {
        Self {
            x: self.x.min(rhs.x),
            y: self.y.min(rhs.y),
        }
    }
    pub fn max(&self, rhs: Self) -> Self {
        Self {
            x: self.x.max(rhs.x),
            y: self.y.max(rhs.y),
        }
    }
    /// reflect direction of self, unnormalized
    pub fn reflect(self, normal: Vector2) -> Self {
        self - 2.0 * dot(self, normal) * normal
    }
    pub fn refract(self, normal: Vector2, refract_rate: f32) -> Self {
        let cos_theta = dot(-self, normal).min(1.0);
        let r_out_perp = (self + normal * cos_theta) * refract_rate;
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * normal;
        r_out_parallel + r_out_perp
    }
    pub const ZERO: Vector2 = Self { x: 0.0, y: 0.0 };
    pub const UNIT_X: Vector2 = Self { x: 1.0, y: 0.0 };
    pub const UNIT_Y: Vector2 = Self { x: 0.0, y: 1.0 };
}
impl DotProduct for Vector2 {
    fn dot(&self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }
}
impl std::ops::Add for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Mul<f32> for Vector2 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl std::ops::Mul<Vector2> for f32 {
    type Output = Vector2;
    fn mul(self, rhs: Vector2) -> Self::Output {
        Self::Output {
            x: rhs.x * self,
            y: rhs.y * self,
        }
    }
}
impl std::ops::Div<f32> for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
impl std::ops::Div<Self> for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: Vector2) -> Self::Output {
        Self::Output {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl std::ops::Neg for Vector2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * -1.0
    }
}
impl Sum for Vector2 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut result = vec2(0.0, 0.0);
        for i in iter {
            result = result + i;
        }
        return result;
    }
}
impl From<(f32, f32)> for Vector2 {
    fn from(value: (f32, f32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}
impl From<[f32; 2]> for Vector2 {
    fn from(value: [f32; 2]) -> Self {
        Self {
            x: value[0],
            y: value[1],
        }
    }
}

pub fn vec3(x: f32, y: f32, z: f32) -> Vector3 {
    Vector3::new(x, y, z)
}

pub trait DotProduct {
    fn dot(&self, rhs: Self) -> f32;
}

pub fn dot<T: DotProduct>(a: T, b: T) -> f32 {
    a.dot(b)
}

pub trait CrossProduct {
    fn cross(&self, rhs: Self) -> Self;
}

pub fn cross<T: CrossProduct>(a: T, b: T) -> T {
    a.cross(b)
}

#[derive(Debug, PartialEq, Clone, Copy, Deserialize)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    pub fn distance_to(&self, v: &Vector3) -> f32 {
        self.distance_to_squared(v).sqrt()
    }
    pub fn distance_to_squared(&self, v: &Vector3) -> f32 {
        let dx = self.x - v.x;
        let dy = self.y - v.y;
        let dz = self.z - v.z;
        dx * dx + dy * dy + dz * dz
    }
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// return a normalized version of this vector
    pub fn normalize(&self) -> Self {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Self {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }
    pub fn normalize_or_zero(&self) -> Self {
        let len = self.length();
        if len.abs() < EPSILON {
            Self {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        } else {
            Self {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
            }
        }
    }
    pub fn min(&self, rhs: Self) -> Self {
        Self {
            x: self.x.min(rhs.x),
            y: self.y.min(rhs.y),
            z: self.z.min(rhs.z),
        }
    }
    pub fn max(&self, rhs: Self) -> Self {
        Self {
            x: self.x.max(rhs.x),
            y: self.y.max(rhs.y),
            z: self.z.max(rhs.z),
        }
    }
    /// reflect direction of self, unnormalized
    pub fn reflect(self, normal: Vector3) -> Self {
        self - 2.0 * dot(self, normal) * normal
    }
    pub fn refract(self, normal: Vector3, refract_rate: f32) -> Self {
        let cos_theta = dot(-self, normal).min(1.0);
        let r_out_perp = (self + normal * cos_theta) * refract_rate;
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * normal;
        r_out_parallel + r_out_perp
    }
    pub const ZERO: Vector3 = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    pub const UNIT_X: Vector3 = Self {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };
    pub const UNIT_Y: Vector3 = Self {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    pub const UNIT_Z: Vector3 = Self {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };
}

impl DotProduct for Vector3 {
    fn dot(&self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}
impl CrossProduct for Vector3 {
    fn cross(&self, rhs: Self) -> Vector3 {
        Vector3::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
}

impl std::ops::Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Mul<f32> for Vector3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl std::ops::Mul<Vector3> for f32 {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Self::Output {
        Self::Output {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}
impl std::ops::Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
impl std::ops::Div<Self> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: Vector3) -> Self::Output {
        Self::Output {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl std::ops::Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * -1.0
    }
}
impl Sum for Vector3 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut result = vec3(0.0, 0.0, 0.0);
        for i in iter {
            result = result + i;
        }
        return result;
    }
}
impl From<(f32, f32, f32)> for Vector3 {
    fn from(value: (f32, f32, f32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}
impl From<[f32; 3]> for Vector3 {
    fn from(value: [f32; 3]) -> Self {
        Self {
            x: value[0],
            y: value[1],
            z: value[2],
        }
    }
}

pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn cross_product() {
        let v1 = Vector3::new(1.0, 0.0, 0.0);
        let v2 = Vector3::new(0.0, 1.0, 0.0);
        let v3 = Vector3::new(0.0, 0.0, 1.0);
        assert_eq!(v1.cross(v2), v3);
        assert_eq!(v2.cross(v1), -v3);
        assert_eq!(cross(v1, v2), v3);
    }
    #[test]
    fn dot_product() {
        let v1 = Vector3::new(1.0, 0.0, 0.0);
        let v2 = Vector3::new(0.0, 1.0, 0.0);
        assert_eq!(dot(v1, v2), 0.0);
    }
}
