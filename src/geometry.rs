use std::ops::Range;

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
    fn hit(&self, ray: crate::ray::Ray, range: Range<f32>) -> Option<HitRecord> {
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
            if !range.contains(&root) {
                root = (-half_b - sqrtd) / a;
                if !range.contains(&root) {
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

#[derive(Clone, Copy)]
pub struct AxisAlignedBox {
    pub min: Vector3,
    pub max: Vector3,
}
impl AxisAlignedBox {
    pub fn new(min: Vector3, max: Vector3) -> Self {
        Self { min, max }
    }
}

impl Hitable for AxisAlignedBox {
    fn hit(&self, ray: crate::ray::Ray, range: Range<f32>) -> Option<HitRecord> {
        let t_min = (self.min - ray.origin) / ray.direction;
        let t_max = (self.max - ray.origin) / ray.direction;
        let t1 = t_min.min(t_max);
        let t2 = t_min.max(t_max);
        let t_near = t1.x.max(t1.y).max(t1.z);
        let t_far = t2.x.min(t2.y).min(t2.z);
        if t_near > t_far || !range.contains(&t_near) {
            None
        } else {
            let point = ray.at(t_near);
            // let box_center = (self.min + self.max)*0.5;
            // let dir = (point - box_center).normalize();
            let epsilon = 1e-4;
            let normal = if (point.x - self.min.x).abs() < epsilon {
                -Vector3::UNIT_X
            } else if (point.x - self.max.x).abs() < epsilon {
                Vector3::UNIT_X
            } else if (point.y - self.min.y).abs() < epsilon {
                -Vector3::UNIT_Y
            } else if (point.y - self.max.y).abs() < epsilon {
                Vector3::UNIT_Y
            } else if (point.z - self.min.z).abs() < epsilon {
                -Vector3::UNIT_Z
            } else {
                Vector3::UNIT_Z
            };

            Some(HitRecord {
                point,
                normal,
                t: t_near,
                front_face: true,
            })
        }
    }
    // fn hit(&self, ray: crate::ray::Ray, range: Range<f32>) -> Option<HitRecord> {
    //     let mut t_min = 0.0;
    //     let mut t_max = f32::INFINITY;
    //     let ray_dir = [ray.direction.x, ray.direction.y, ray.direction.z];
    //     let ray_origin = [ray.origin.x, ray.origin.y, ray.origin.z];
    //     let min = [self.min.x, self.min.y, self.min.z];
    //     let max = [self.max.x, self.max.y, self.max.z];
    //     for i in 0..3 {
    //         let inv_d = 1.0 / ray_dir[i];
    //         let mut t0 = (min[i] - ray_origin[i]) * inv_d;
    //         let mut t1 = (max[i] - ray_origin[i]) * inv_d;

    //         if inv_d < 0.0 {
    //             std::mem::swap(&mut t0, &mut t1);
    //         }

    //         t_min = t0.max(t_min);
    //         t_max = t1.min(t_max);

    //         if t_min > t_max {
    //             return None; // 如果没有相交，则返回None
    //         }
    //     }

    //     if t_min >= range.start && t_min <= range.end {
    //         let hit_point = ray.at(t_min);
    //         let outward_normal = if t_min == t_max {
    //             // 边界情况，选择一个合适的法向量
    //             let mut normal = Vector3::ZERO;
    //             if hit_point.x == self.min.x {
    //                 normal.x = -1.0;
    //             } else if hit_point.x == self.max.x {
    //                 normal.x = 1.0;
    //             }
    //             if hit_point.y == self.min.y {
    //                 normal.y = -1.0;
    //             } else if hit_point.y == self.max.y {
    //                 normal.y = 1.0;
    //             }
    //             if hit_point.z == self.min.z {
    //                 normal.z = -1.0;
    //             } else if hit_point.z == self.max.z {
    //                 normal.z = 1.0;
    //             }
    //             dbg!(normal);
    //             normal.normalize()
    //         } else {
    //             // 对于内部点，可以通过hit_point和盒子中心计算法向量
    //             ((hit_point - (self.min + self.max) / 2.0).normalize())
    //         };

    //         Some(HitRecord {
    //             point: hit_point,
    //             normal: outward_normal,
    //             t: t_min,
    //             front_face: true, // 假设光线从外部指向内部（可以根据实际需求调整）
    //         })
    //     } else {
    //         None
    //     }
    // }
}

#[derive(Clone, Copy)]
pub struct Parallelogram {
    pub corner: Vector3,
    pub u: Vector3,
    pub v: Vector3,
}
