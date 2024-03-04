
use std::ops::Range;

use crate::{
    geometry::Sphere,
    linear_algebra::{vector::dot, Vector3},
};

pub trait Hitable {
    fn hit(&self, ray: Ray, range:Range<f32>) -> Option<HitRecord>;
}

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub point: Vector3,
    pub normal: Vector3,
    pub t: f32,
    pub front_face: bool,
}
impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vector3) {
        self.front_face = dot(ray.direction, outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}
impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        Self { origin, direction }
    }
    pub fn at(&self, distance: f32) -> Vector3 {
        self.origin + distance * self.direction
    }

    pub fn hit(&self, target: impl Hitable, range:Range<f32>) -> Option<HitRecord> {
        target.hit(*self, range)
    }
}
