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
impl Aabb{
    pub fn new()->Self{
        Self::default()
    }
    /// if a point is inside the aabb
    pub fn contains_point(&self, p:Vector3)->bool{
        self.min.x<=p.x
        &&self.max.x>=p.x
        &&self.min.y<=p.y
        &&self.max.y>=p.y
        &&self.min.z<=p.z
        &&self.max.z>=p.z
    }
    /// expand the box to contain point p
    pub fn expand_by_point(&mut self, p:Vector3){
        self.max.x = p.x.max(self.max.x);
        self.min.x = p.x.min(self.min.x);
        self.max.y = p.y.max(self.max.y);
        self.min.y = p.y.min(self.min.y);
        self.max.z = p.z.max(self.max.z);
        self.min.z = p.z.min(self.min.z);
    }
}
