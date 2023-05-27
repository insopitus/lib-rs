use std::ops::{Add, Mul, Sub};

pub fn cubic_bezier<T>(p0: T, p1: T, p2: T, p3: T, t: f32) -> T
where
    T: Add<T, Output = T> + Sub<T, Output = T> + Mul<f32, Output = T> + Copy,
{
    p0+
    (p0 * -3.0 + p1 * 3.0) * t
        + (p0 * 3.0 - p1 * 6.0 + p2 * 3.0) * t * t
        + (p0 * -1.0 + p1 * 3.0 - p2 * 3.0 + p3) * t * t * t
}

#[cfg(test)]
mod test{
    use super::*;
    use crate::linear_algebra::Vector3;
    #[test]
    fn lerp_numbers(){
        let p0 = Vector3::new(0.0,0.0,0.0);
        let p1 = Vector3::new(0.0,0.0,0.0);
        let p2 = Vector3::new(0.0,0.0,0.0);
        let p3 = Vector3::new(0.0,0.0,0.0);

        assert_eq!(cubic_bezier(p0, p1, p2, p3, 1.0),p0)

    }
}
