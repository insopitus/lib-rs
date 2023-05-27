use std::ops::{Add, Mul, Sub};

pub fn cubic_bezier<T>(p0: T, p1: T, p2: T, p3: T, t: f64) -> T
where
    T: Add<T, Output = T> + Sub<T, Output = T> + Mul<f64, Output = T> + Copy,
{
    p0+
    (p0 * -3.0 + p1 * 3.0) * t
        + (p0 * 3.0 - p1 * 6.0 + p2 * 3.0) * t * t
        + (p0 * -1.0 + p1 * 3.0 - p2 * 3.0 + p3) * t * t * t
}
