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
    /// row and column should be 1..=4
    /// it's private so it's ok not to check it
    fn get(&self, row: usize, column: usize) -> f32 {
        self.elements[(column - 1) * 4 + row - 1]
    }
}

impl std::ops::Mul for Matrix4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl std::ops::Mul<Vector4> for Matrix4 {
    type Output = Vector4;
    fn mul(self, rhs: Vector4) -> Self::Output {
        todo!()
    }
}

pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}
impl Vector3 {
    /// return a normalized version of this vector
    pub fn normalize(&self) -> Self {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Self {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }
    /// make this vector3 normalized
    pub fn normalize_self(mut self)->Self{
        let len = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        self.x /= len;
        self.y /= len;
        self.z /= len;
        self
    }
}

impl std::ops::Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3 {
            x:self.x + rhs.x,
            y:self.y + rhs.y,
            z:self.z + rhs.z
        }
    }
}

impl std::ops::Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3 {
            x:self.x - rhs.x,
            y:self.y - rhs.y,
            z:self.z - rhs.z
        }
    }
}

pub struct Vector4 {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
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
