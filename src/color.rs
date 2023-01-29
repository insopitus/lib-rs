use std::{
    ops::{Add, Mul},
    str::FromStr,
};

/// Color for GPU rendering and game development
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        todo!()
    }
    fn from_rgba_array(arr: &[u8; 4]) -> Self {
        todo!()
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
