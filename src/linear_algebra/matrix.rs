pub struct Matrix4 {
    /// the matrix elements are in column major order
    elements: [f32; 16],
}

impl Matrix4 {
    pub fn identity() -> Self {
        Matrix4 {
            elements: [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        }
    }
}

impl std::ops::Mul for Matrix4 {
    type Output = Self;

    fn mul(self, _rhs: Self) -> Self::Output {
        todo!()
    }
}

use crate::linear_algebra::vector::Vector4;

impl std::ops::Mul<Vector4> for Matrix4 {
    type Output = Vector4;
    fn mul(self, _rhs: Vector4) -> Self::Output {
        self.elements;
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::Matrix4;

    #[test]
    fn basic() {
        let idm = Matrix4::identity();
        let idm2 = Matrix4::identity();

        let m3 = idm * idm2;
    }
}
