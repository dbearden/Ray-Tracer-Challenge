mod color;
mod point;
mod vector;
use std::ops::{Add, Div, Mul, Neg, Sub};

pub use color::Color;
pub use point::Point;
pub use vector::Vector;

pub trait Tuple: Add + Sub + Neg + Mul<f64> + Div<f64> + Sized {
    fn new(x: f64, y: f64, z: f64) -> Self;
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;
    fn w(&self) -> f64;
    fn dot(&self, other: Self) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    fn cross(&self, other: Self) -> Self {
        Self::new(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        )
    }

    fn magnitude(&self) -> f64 {
        (self.x().powi(2) + self.y().powi(2) + self.z().powi(2)).sqrt()
    }

    fn normalize(&self) -> Self {
        let m = self.magnitude();
        Self::new(self.x() / m, self.y() / m, self.z() / m)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_plus_vector() {
        let p = Point::new(1.0, 2.0, 3.0);
        let v = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(p + v, Point::new(2.0, 4.0, 6.0));
        assert_eq!(v + p, Point::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn point_minus_point() {
        let p1 = Point::new(1.0, 2.0, 3.0);
        let p2 = Point::new(2.0, 3.0, 4.0);
        assert_eq!(p2 - p1, Vector::new(1.0, 1.0, 1.0));
        assert_eq!(p1 - p2, Vector::new(-1.0, -1.0, -1.0));
    }

    #[test]
    fn vector_plus_vector() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(v1 + v2, Vector::new(2.0, 4.0, 6.0));
    }
}
