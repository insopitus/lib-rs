use std::{
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign},
    str::FromStr,
};

pub trait Mix {
    fn mix(self, c: Self, a: f32) -> Self;
}

pub fn mix<T: Mix>(a: T, b: T, t: f32) -> T {
    a.mix(b, t)
}

/// Color for GPU rendering and game development
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
    Color { r, g, b, a }
}
pub fn rgba8(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color::from_rgba(r, g, b, a)
}

impl Color {
    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }
    pub fn set_a(mut self, a: f32) -> Self {
        self.a = a;
        self
    }
    // fn from_rgba_array(arr: &[u8; 4]) -> Self {
    //     todo!()
    // }
    pub fn as_rgb_bytes(&self) -> [u8; 3] {
        [
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
        ]
    }
    pub fn as_rgba8_bytes(&self) -> [u8; 4] {
        [
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
            (self.a * 255.0) as u8,
        ]
    }
}
impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
impl From<u32> for Color {
    fn from(_: u32) -> Self {
        todo!()
    }
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
            a: self.a + rhs.a,
        }
    }
}
impl AddAssign<Color> for Color {
    fn add_assign(&mut self, rhs: Color) {
        self.r = self.r + rhs.r;
        self.g = self.g + rhs.g;
        self.b = self.b + rhs.b;
        self.a = self.a + rhs.a;
    }
}
impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
            a: self.a * rhs.a,
        }
    }
}
impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
            a: self.a * rhs,
        }
    }
}
impl MulAssign<f32> for Color {
    fn mul_assign(&mut self, rhs: f32) {
        self.r = self.r * rhs;
        self.g = self.g * rhs;
        self.b = self.b * rhs;
        self.a = self.a * rhs;
    }
}
impl Div<f32> for Color {
    type Output = Color;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
            a: self.a / rhs,
        }
    }
}
impl DivAssign<f32> for Color {
    fn div_assign(&mut self, rhs: f32) {
        self.r = self.r / rhs;
        self.g = self.g / rhs;
        self.b = self.b / rhs;
        self.a = self.a / rhs;
    }
}
impl Mix for Color {
    fn mix(self, c: Self, a: f32) -> Self {
        self * (1.0 - a) + c * a
    }
}

/// color names
impl Color {
    const WHITE: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    const BLACK: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    const TRANSPARENT: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 0.0,
    };
}

#[cfg(test)]
mod test {
    use super::Color;

    fn basic() {
        Color::WHITE;
        "#ffff00".parse::<Color>();
        Color::BLACK * Color::WHITE;
    }
}
