use std::{f32::INFINITY, ops::Range};

use serde::{Deserialize, Serialize};

use crate::{
    aabb::Aabb,
    data_structures,
    linear_algebra::{
        vector::{cross, dot, vec3, Vector2},
        Vector3,
    },
    ray::{HitRecord, Hitable},
};

#[derive(Clone, Copy, Deserialize, Serialize, Debug)]
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
                root = (-half_b + sqrtd) / a;
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
                u: 0.0,
                v: 0.0,
            };
            let outward_normal = (p - self.center) / self.radius;
            record.set_face_normal(&ray, outward_normal);
            Some(record)
        }
    }
    fn bounding_box(&self) -> Aabb {
        let half = vec3(self.radius, self.radius, self.radius);
        Aabb {
            min: self.center - half,
            max: self.center + half,
        }
    }
}

/// an axis-aligned box geometry (renderable, while the `Aabb` struct is a math structure only used for bvh)
#[derive(Clone, Copy, Deserialize, Serialize, Debug)]
pub struct Box {
    pub min: Vector3,
    pub max: Vector3,
}
impl Box {
    pub fn new(min: Vector3, max: Vector3) -> Self {
        Self { min, max }
    }
}

impl Hitable for Box {
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
                u: 0.0,
                v: 0.0,
            })
        }
    }
    fn bounding_box(&self) -> Aabb {
        Aabb {
            min: self.min,
            max: self.max,
        }
    }
}
/// https://raytracing.github.io/books/RayTracingTheNextWeek.html#quadrilaterals/definingthequadrilateral
///
/// acturally a parallelogram
#[derive(Clone, Copy, Deserialize, Serialize, Debug)]
#[serde(from = "QuadParams")]
pub struct Quad {
    /// a corner of the parallelogram
    q: Vector3,
    /// a vector on the edge from the corner q
    u: Vector3,
    /// a vector on the other edge from the corner q
    v: Vector3,
    // normal of the parallelogram
    n: Vector3,
    // D of the plane equation Ax+By+C=D
    d: f32,
    // n / n · n
    w: Vector3,
}
impl Quad {
    pub fn new(q: Vector3, u: Vector3, v: Vector3) -> Self {
        let n = cross(u, v);
        let normal: Vector3 = n.normalize();
        let d = dot(normal, q);
        let w = n / dot(n, n);
        Self {
            q,
            u,
            v,
            n: normal,
            d,
            w,
        }
    }
}
impl Hitable for Quad {
    fn hit(&self, ray: crate::ray::Ray, range: Range<f32>) -> Option<HitRecord> {
        let denom = dot(self.n, ray.direction);
        // no hit if the ray is parallel to the plane
        if denom.abs() < 1e-8 {
            return None;
        }
        let t = (self.d - dot(self.n, ray.origin)) / denom;
        // return None if the hit point t is outside the range
        if !range.contains(&t) {
            return None;
        }
        // determin the hit point lies within the parallogram using
        // its plane coordinates(uv)
        let intersection = ray.at(t);
        let planar_hitpt_vector = intersection - self.q;
        let alpha = dot(self.w, cross(planar_hitpt_vector, self.v));
        if alpha < 0.0 || alpha > 1.0 {
            return None;
        }
        let beta = dot(self.w, cross(self.u, planar_hitpt_vector));
        if beta < 0.0 || beta > 1.0 {
            return None;
        }
        let mut rec = HitRecord {
            point: intersection,
            normal: self.n,
            t,
            front_face: true,
            u: alpha,
            v: beta,
        };
        rec.set_face_normal(&ray, self.n);
        Some(rec)
    }
    fn bounding_box(&self) -> Aabb {
        let a = self.q;
        let b = a + self.u;
        let c = b + self.v;
        let d = a + self.v;
        let min = a.min(b).min(c).min(d);
        let max = a.max(b).max(c).max(d);
        Aabb { min, max }
    }
}
// middleware for serde deserialize
#[derive(Deserialize)]
struct QuadParams {
    q: Vector3,
    u: Vector3,
    v: Vector3,
}
impl From<QuadParams> for Quad {
    fn from(params: QuadParams) -> Self {
        Self::new(params.q, params.u, params.v)
    }
}

#[derive(Clone, Copy, Deserialize, Serialize, Debug)]
pub struct Plane {
    /// a random point on the plane
    pub point: Vector3,
    /// normal of the plane, supposed to be normlized
    pub normal: Vector3,
}
impl Hitable for Plane {
    fn hit(&self, ray: crate::ray::Ray, range: Range<f32>) -> Option<HitRecord> {
        let denom = dot(self.normal, ray.direction);
        // no hit if the ray is parallel to the plane
        if denom.abs() < 1e-8 {
            return None;
        }
        // return None if the hit point t is outside the range
        let t = dot(self.point - ray.origin, self.normal) / denom;
        if !range.contains(&t) {
            return None;
        }
        let intersection = ray.at(t);
        let rec = HitRecord {
            point: intersection,
            normal: self.normal,
            t,
            front_face: denom < 0.0,
            u: 0.,
            v: 0.,
        };
        Some(rec)
    }
    /// it's wrong, but i just don't want to make the function return a Option<Aabb> for now.
    fn bounding_box(&self) -> Aabb {
        Aabb {
            min: vec3(-INFINITY, -INFINITY, -INFINITY),
            max: vec3(INFINITY, INFINITY, INFINITY),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Circle {
    center: Vector3,
    radius: f32,
    normal: Vector3,
}
impl Hitable for Circle {
    fn hit(&self, ray: crate::ray::Ray, range: Range<f32>) -> Option<HitRecord> {
        let plane = Plane {
            point: self.center,
            normal: self.normal,
        };
        if let Some(hit) = plane.hit(ray, range) {
            let intersection = hit.point;
            let distance = intersection - self.center;
            if distance.length_squared() > self.radius * self.radius {
                return None;
            }
            let rec = HitRecord {
                point: intersection,
                normal: self.normal,
                t: hit.t,
                front_face: hit.front_face,
                u: 0.,
                v: 0.,
            };
            Some(rec)
        } else {
            None
        }
    }
    fn bounding_box(&self) -> Aabb {
        let delta = (vec3(1.0, 1.0, 1.0) - self.normal * self.normal) * self.radius;
        let min = self.center - delta;
        let max = self.center + delta;
        Aabb { min, max }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
struct Vertex {
    position: Vector3,
    normal: Vector3,
    uv: Vector2,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Triangle {
    a: Vector3,
    b: Vector3,
    c: Vector3,
}
impl Triangle {
    pub fn new(a: Vector3, b: Vector3, c: Vector3) -> Self {
        Self { a, b, c }
    }
}
impl Hitable for Triangle {
    fn hit(&self, ray: crate::ray::Ray, range: Range<f32>) -> Option<HitRecord> {
        // Möller–Trumbore algorithm
        // https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-rendering-a-triangle/moller-trumbore-ray-triangle-intersection.html
        let e1 = self.b - self.a;
        let e2 = self.c - self.a;
        let direction = ray.direction;
        let origin = ray.origin;
        let ray_cross_e2 = cross(direction, e2);
        let det = dot(e1, ray_cross_e2);

        if det > -f32::EPSILON && det < f32::EPSILON {
            return None; // This ray is parallel to this triangle.
        }

        let inv_det = 1.0 / det;
        let s = origin - self.a;
        let u = inv_det * dot(s, ray_cross_e2);
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let s_cross_e1 = cross(s, e1);
        let v = inv_det * dot(direction, s_cross_e1);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        // At this stage we can compute t to find out where the intersection point is on the line.
        let t = inv_det * dot(e2, s_cross_e1);

        if range.contains(&t) {
            // ray intersection
            let intersection_point = origin + direction * t;
            // TODO: save the normal in struct field?
            let normal = cross(e1, e2).normalize();
            return Some(HitRecord {
                point: intersection_point,
                normal,
                t,
                front_face: dot(normal, direction) < 0.0,
                u: 0.0,
                v: 0.0,
            });
        } else {
            // This means that there is a line intersection but not a ray intersection.
            return None;
        }
    }
    fn bounding_box(&self) -> Aabb {
        Aabb {
            min: self.a.min(self.b).min(self.c),
            max: self.a.max(self.b).max(self.c),
        }
    }
}

pub struct TriMesh {
    vertices: Vec<Vector3>,
    indices: Vec<usize>,
}

impl TriMesh {
    pub fn new(vertices: Vec<Vector3>, indices: Vec<usize>) -> Self {
        Self { vertices, indices }
    }
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.indices.len() % 3 != 0 {
            return Err("invalid indices count");
        }
        let vert_len = self.vertices.len() - 1;
        // indices in bound
        for i in &self.indices {
            if *i > vert_len {
                return Err("indice out of bound");
            }
        }

        return Ok(());
    }
}

pub struct Bvh<T> {
    _tree: data_structures::binary_tree::Node<BvhNode<T>>,
}
struct BvhNode<T> {
    _volume: Aabb,
    object: Vec<T>,
}

impl<T> Bvh<T> {}
