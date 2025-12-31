use std::{f32::INFINITY, ops::Range};

use crate::{
    linear_algebra::{vector::vec3, Vector3},
    ray::Ray,
};

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
    pub fn new(min: Vector3, max: Vector3) -> Self {
        Self { min, max }
    }
    pub fn empty() -> Self {
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
    /// ray intersects, only checks intersection rather than a full HitRecord
    pub fn intersect_ray(&self, ray: Ray, range: Range<f32>) -> bool {
        let p = ray.origin;
        let d = ray.direction;
        let dir_inv = vec3(1.0 / d.x, 1.0 / d.y, 1.0 / d.z);
        let t_min = (self.min - p) * dir_inv;
        let t_max = (self.max - p) * dir_inv;
        let t1 = t_min.min(t_max);
        let t2 = t_min.max(t_max);
        let t_near = t1.x.max(t1.y).max(t1.z);
        let t_far = t2.x.min(t2.y).min(t2.z);
        range.contains(&t_near) || range.contains(&t_far)
    }
}
