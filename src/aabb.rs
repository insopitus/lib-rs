use std::f32::INFINITY;

use crate::linear_algebra::{vector::vec3, Vector3};

pub struct Aabb {
    pub min: Vector3,
    pub max: Vector3,
}
impl Default for Aabb {
    fn default() -> Self {
        Self {
            min: vec3(INFINITY, INFINITY, INFINITY),
            max: vec3(-INFINITY, -INFINITY, -INFINITY),
        }
    }
}
impl Aabb {
    pub fn new() -> Self {
        Self::default()
    }
    /// create a new aabb from min and max points
    pub fn from_min_max(min: Vector3, max: Vector3) -> Self {
        Self { min, max }
    }
    /// create a new aabb from center and size
    pub fn from_center_size(center: Vector3, size: Vector3) -> Self {
        let half_size = size / 2.0;
        Self {
            min: center - half_size,
            max: center + half_size,
        }
    }
    /// create a new aabb from a list of points
    pub fn from_points(points: &[Vector3]) -> Self {
        let mut aabb = Self::default();
        for p in points {
            aabb.expand_by_point(*p);
        }
        aabb
    }
    /// if a point is inside the aabb
    pub fn contains_point(&self, p: Vector3) -> bool {
        self.min.x <= p.x
            && self.max.x >= p.x
            && self.min.y <= p.y
            && self.max.y >= p.y
            && self.min.z <= p.z
            && self.max.z >= p.z
    }
    /// expand the box to contain point p
    pub fn expand_by_point(&mut self, p: Vector3) {
        self.max.x = p.x.max(self.max.x);
        self.min.x = p.x.min(self.min.x);
        self.max.y = p.y.max(self.max.y);
        self.min.y = p.y.min(self.min.y);
        self.max.z = p.z.max(self.max.z);
        self.min.z = p.z.min(self.min.z);
    }
}
