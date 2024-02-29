use crate::{
    linear_algebra::{vector::dot, Vector3},
    ray::{HitRecord, Hitable},
};

#[derive(Clone, Copy)]
pub struct Sphere {
    pub center: Vector3,
    pub radius: f32,
}
impl Sphere {
    pub fn new(center: Vector3, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: crate::ray::Ray, min_t: f32, max_t: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = dot(oc, ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant <= 0.0 {
            None
        } else {
            let sqrtd = discriminant.sqrt();
            // find the nearest root that lies in the acceptable range
            let mut root = (-half_b - sqrtd) / a;
            if root <= min_t || root >= max_t {
                root = (-half_b - sqrtd) / a;
                if root <= min_t || root >= max_t {
                    return None;
                }
            }
            let t = root;
            let p = ray.at(t);
            let mut record = HitRecord {
                point: p,
                normal: Vector3::ZERO,
                t,
                front_face: false,
            };
            let outward_normal = (p - self.center) / self.radius;
            record.set_face_normal(&ray, outward_normal);
            Some(record)
        }
    }
}
