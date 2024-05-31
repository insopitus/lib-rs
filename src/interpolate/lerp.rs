use std::ops::{Add, Mul};



pub fn lerp<T>(a: T, b: T, t: f32) -> T
where
    T: Add<T, Output = T> + Mul<f32, Output = T>,
{
    a * (1.0 - t) + b * t
}



#[cfg(test)]
mod test{
    use crate::linear_algebra::Vector3;

    use super::*;
    #[test]
    fn lerp_numbers(){
        assert_eq!(lerp(1.0,0.0,1.0),0.0);
        assert_eq!(lerp(1.0,2.0,0.5),1.5);
        assert_eq!(lerp(2.0,0.0,0.25),1.5);

    }
    #[test]
    fn lerp_vector3(){
        let v1 = Vector3::new(1.0, 1.0, 1.0);
        let v2 = Vector3::new(0.0, 0.0, 0.0);
        assert_eq!(lerp(v1, v2, 0.5),Vector3::new(0.5, 0.5, 0.5));
    }
}
