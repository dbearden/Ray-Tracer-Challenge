use crate::{matrix::Matrix, transformations::Transformation};

use super::vector::Vector;
use super::Tuple;
use float_cmp::{self, approx_eq};

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Tuple for Point {
    fn new(x: f64, y: f64, z: f64) -> Point {
        Self { x, y, z }
    }

    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }

    fn z(&self) -> f64 {
        self.z
    }
    fn w(&self) -> f64 {
        1.0
    }
}
impl Transformation for Point {
    fn translation(&self, x: f64, y: f64, z: f64) -> Point {
        Matrix::new([
            [1.0, 0.0, 0.0, x],
            [0.0, 1.0, 0.0, y],
            [0.0, 0.0, 1.0, z],
            [0.0, 0.0, 0.0, 1.0],
        ]) * *self
    }
    fn scaling(&self, x: f64, y: f64, z: f64) -> Point {
        Matrix::new([
            [x, 0.0, 0.0, 0.0],
            [0.0, y, 0.0, 0.0],
            [0.0, 0.0, z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]) * *self
    }
    fn shearing(&self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Point {
        Matrix::new([
            [1.0, xy, xz, 0.0],
            [yx, 1.0, yz, 0.0],
            [zx, zy, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]) * *self
    }
    fn rotation_x(&self, r: f64) -> Point {
        Matrix::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, r.cos(), -(r.sin()), 0.0],
            [0.0, r.sin(), r.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]) * *self
    }
    fn rotation_y(&self, r: f64) -> Point {
        Matrix::new([
            [r.cos(), 0.0, r.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-(r.sin()), 0.0, r.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]) * *self
    }
    fn rotation_z(&self, r: f64) -> Point {
        Matrix::new([
            [r.cos(), -(r.sin()), 0.0, 0.0],
            [r.sin(), r.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]) * *self
    }
}
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        approx_eq!(f64, self.x, other.x, epsilon = 0.00003)
            && approx_eq!(f64, self.y, other.y, epsilon = 0.00003)
            && approx_eq!(f64, self.z, other.z, epsilon = 0.00003)
    }
}

impl std::ops::Add<Vector> for Point {
    type Output = Self;
    fn add(self, other: Vector) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}
impl std::ops::Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl std::ops::Sub for Point {
    type Output = Vector;
    fn sub(self, other: Self) -> Vector {
        Vector::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl std::ops::Neg for Point {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl std::ops::Mul<f64> for Point {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Point::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl std::ops::Div<f64> for Point {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Point::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}
