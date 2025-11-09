use std::ops::Range;

use crate::linear_algebra::{vector::dot, Transform, Vector3};

pub trait Hitable {
    fn hit(&self, ray: Ray, range: Range<f32>) -> Option<HitRecord>;
}

#[derive(Clone, Copy, Debug)]
pub struct HitRecord {
    pub point: Vector3,
    pub normal: Vector3,
    pub t: f32,
    pub front_face: bool,
    pub u: f32,
    pub v: f32,
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

    pub fn hit<T>(
        &self,
        target: &T,
        range: Range<f32>,
        transform: Option<Transform>,
    ) -> Option<HitRecord>
    where
        T: Hitable + Sync,
    {
        if let Some(trans) = transform {
            let rotation = trans.rotation;
            let translation = trans.translation;
            let cos_theta = rotation.cos();
            let sin_theta = rotation.sin();
            let direction = Vector3::new(
                self.direction.x * cos_theta - self.direction.z * sin_theta,
                self.direction.y,
                self.direction.x * sin_theta + self.direction.z * cos_theta,
            );
            let o = self.origin - translation;
            let origin = Vector3::new(
                o.x * cos_theta - o.z * sin_theta,
                o.y,
                o.x * sin_theta + o.z * cos_theta,
            );
            let rotated_ray = Ray::new(origin, direction);
            let mut record = target.hit(rotated_ray, range);
            if let Some(ref mut rec) = record {
                let p = rec.point;
                rec.point = Vector3::new(
                    p.x * cos_theta + p.z * sin_theta,
                    p.y,
                    -p.x * sin_theta + p.z * cos_theta,
                ) + translation;
                rec.normal = Vector3::new(
                    rec.normal.x * cos_theta + rec.normal.z * sin_theta,
                    rec.normal.y,
                    -rec.normal.x * sin_theta + rec.normal.z * cos_theta,
                );
            }
            record
        } else {
            target.hit(*self, range)
        }
    }
}
