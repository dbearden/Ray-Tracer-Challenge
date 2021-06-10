use std::fmt;

use float_cmp::approx_eq;
#[derive(Copy, Clone, Debug, Default)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    pub const WHITE: Self = Self {
        red: 1.0,
        green: 1.0,
        blue: 1.0,
    };
    pub const BLACK: Self = Self {
        red: 0.0,
        green: 0.0,
        blue: 0.0,
    };
    pub fn new(red: f64, green: f64, blue: f64) -> Color {
        Self { red, green, blue }
    }

    pub fn magnitude(&self) -> f64 {
        (self.red.powi(2) + self.green.powi(2) + self.blue.powi(2)).sqrt()
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            (self.red.floor().clamp(0f64, 255f64)),
            (self.green.floor().clamp(0f64, 255f64)),
            (self.blue.floor().clamp(0f64, 255f64))
        )
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        approx_eq!(f64, self.red, other.red, epsilon = 0.00003)
            && approx_eq!(f64, self.green, other.green, epsilon = 0.00003)
            && approx_eq!(f64, self.blue, other.blue, epsilon = 0.00003)
    }
}

impl std::ops::Add for Color {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(
            self.red + other.red,
            self.green + other.green,
            self.blue + other.blue,
        )
    }
}

impl std::ops::Sub for Color {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(
            self.red - other.red,
            self.green - other.green,
            self.blue - other.blue,
        )
    }
}

impl std::ops::Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.red * rhs, self.green * rhs, self.blue * rhs)
    }
}
impl std::ops::Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.red * rhs.red,
            self.green * rhs.green,
            self.blue * rhs.blue,
        )
    }
}

impl std::ops::Div<f64> for Color {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self::new(self.red / rhs, self.green / rhs, self.blue / rhs)
    }
}

impl std::ops::Div for Color {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(
            self.red / rhs.red,
            self.green / rhs.green,
            self.blue / rhs.blue,
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert_eq!(c1 + c2, Color::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn subtraction() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert_eq!(c1 - c2, Color::new(0.2, 0.5, 0.5));
    }

    #[test]
    fn multiplication_by_scalar() {
        let c = Color::new(0.2, 0.3, 0.4);
        assert_eq!(c * 2.0, Color::new(0.4, 0.6, 0.8));
    }

    #[test]
    fn color_multiplication() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        assert_eq!(c1 * c2, Color::new(0.9, 0.2, 0.04));
    }
}
