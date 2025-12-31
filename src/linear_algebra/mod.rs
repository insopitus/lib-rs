pub mod matrix;
pub mod vector;
pub use matrix::Matrix4;
pub use vector::{Vector3, Vector4};

#[derive(Debug, Clone, Copy, serde::Deserialize)]
pub struct Transform {
    pub rotation: f32,
    pub translation: Vector3,
}
