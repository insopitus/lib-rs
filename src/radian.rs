use std::{
    f64::consts::PI,
    ops::{Add, Div, Mul, Sub},
};

const RATIO: f64 = PI / 180.0;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Radian(f64);
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Degree(f64);

// convert

impl Into<Degree> for Radian {
    fn into(self) -> Degree {
        Degree(self.0 / RATIO)
    }
}

impl Into<Radian> for Degree {
    fn into(self) -> Radian {
        Radian(self.0 * RATIO)
    }
}

// add

impl Add<Radian> for Radian {
    type Output = Radian;
    fn add(self, rhs: Radian) -> Self::Output {
        Radian(self.0 + rhs.0)
    }
}
impl Add<Degree> for Degree {
    type Output = Degree;
    fn add(self, rhs: Degree) -> Self::Output {
        Degree(self.0 + rhs.0)
    }
}
impl Add<Degree> for Radian {
    type Output = Radian;
    fn add(self, rhs: Degree) -> Self::Output {
        self + Into::<Radian>::into(rhs)
    }
}
impl Add<Radian> for Degree {
    type Output = Degree;
    fn add(self, rhs: Radian) -> Self::Output {
        self + Into::<Degree>::into(rhs)
    }
}

// minus

impl Sub<Radian> for Radian {
    type Output = Radian;
    fn sub(self, rhs: Radian) -> Self::Output {
        Radian(self.0 - rhs.0)
    }
}
impl Sub<Degree> for Degree {
    type Output = Degree;
    fn sub(self, rhs: Degree) -> Self::Output {
        Degree(self.0 - rhs.0)
    }
}
impl Sub<Degree> for Radian {
    type Output = Radian;
    fn sub(self, rhs: Degree) -> Self::Output {
        self - Into::<Radian>::into(rhs)
    }
}
impl Sub<Radian> for Degree {
    type Output = Degree;
    fn sub(self, rhs: Radian) -> Self::Output {
        self - Into::<Degree>::into(rhs)
    }
}

// multiply
impl Mul<Radian> for Radian {
    type Output = Radian;
    fn mul(self, rhs: Radian) -> Self::Output {
        Radian(self.0 * rhs.0)
    }
}
impl Mul<Degree> for Degree {
    type Output = Degree;
    fn mul(self, rhs: Degree) -> Self::Output {
        Degree(self.0 * rhs.0)
    }
}
impl Mul<Degree> for Radian {
    type Output = Radian;
    fn mul(self, rhs: Degree) -> Self::Output {
        self * Into::<Radian>::into(rhs)
    }
}
impl Mul<Radian> for Degree {
    type Output = Degree;
    fn mul(self, rhs: Radian) -> Self::Output {
        self * Into::<Degree>::into(rhs)
    }
}

// devide
impl Div<Radian> for Radian {
    type Output = Radian;
    fn div(self, rhs: Radian) -> Self::Output {
        Radian(self.0 / rhs.0)
    }
}
impl Div<Degree> for Degree {
    type Output = Degree;
    fn div(self, rhs: Degree) -> Self::Output {
        Degree(self.0 / rhs.0)
    }
}
impl Div<Degree> for Radian {
    type Output = Radian;
    fn div(self, rhs: Degree) -> Self::Output {
        self / Into::<Radian>::into(rhs)
    }
}
impl Div<Radian> for Degree {
    type Output = Degree;
    fn div(self, rhs: Radian) -> Self::Output {
        self / Into::<Degree>::into(rhs)
    }
}

// TODO compare

#[cfg(test)]
mod tests {
    use super::{Degree, Radian, PI};
    #[test]
    fn basic() {
        assert!(((Degree(180.0) + Radian(PI)).0).abs() - 360.0 < 0.0000001);
    }
}
