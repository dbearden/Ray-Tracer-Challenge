use float_cmp::{self, approx_eq};
#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Self { x, y, z }
    }
}
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        approx_eq!(f64, self.x, other.x)
            && approx_eq!(f64, self.y, other.y)
            && approx_eq!(f64, self.z, other.z)
    }
}

impl std::ops::Add<Vector> for Point {
    type Output = Self;
    fn add(self, other: Vector) -> Self {
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

//impl std::ops::Mul<f64> for Point {
//    type Output = Self;
//
//    fn mul(self, rhs: f64) -> Self::Output {
//        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs, self.3 * rhs)
//    }
//}
//
//impl std::ops::Div<f64> for Point {
//    type Output = Self;
//
//    fn div(self, rhs: f64) -> Self::Output {
//        Self(self.0 / rhs, self.1 / rhs, self.2 / rhs, self.3 / rhs)
//    }
//}

#[derive(Copy, Clone, Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Self { x, y, z }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let m = self.magnitude();
        Self::new(self.x / m, self.y / m, self.z / m)
    }

    pub fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}
impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        approx_eq!(f64, self.x, other.x)
            && approx_eq!(f64, self.y, other.y)
            && approx_eq!(f64, self.z, other.z)
    }
}

impl std::ops::Add for Vector {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl std::ops::Add<Point> for Vector {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl std::ops::Neg for Vector {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}
impl std::ops::Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl std::ops::Div<f64> for Vector {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
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
