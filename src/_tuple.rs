use float_cmp::{self, approx_eq};
#[derive(Debug, Copy, Clone)]
pub struct Tuple(f64, f64, f64, f64);

impl Tuple {
    pub fn point(x: f64, y: f64, z: f64) -> Self {
        Tuple(x, y, z, 1.0)
    }
    pub fn vector(x: f64, y: f64, z: f64) -> Self {
        Tuple(x, y, z, 0.0)
    }
    pub fn x(&self) -> f64 {
        self.0
    }
    pub fn y(&self) -> f64 {
        self.1
    }
    pub fn z(&self) -> f64 {
        self.2
    }
    pub fn w(&self) -> f64 {
        self.3
    }

    pub fn is_point(&self) -> bool {
        approx_eq!(f64, self.w(), 1.0)
    }

    pub fn is_vector(&self) -> bool {
        approx_eq!(f64, self.w(), 0.0)
    }

    pub fn magnitude(&self) -> f64 {
        (self.0.powi(2) + self.1.powi(2) + self.2.powi(2) + self.3.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let m = self.magnitude();
        Self(self.0 / m, self.1 / m, self.2 / m, self.3 / m)
    }

    pub fn dot(&self, other: Tuple) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z() + self.w() * other.w()
    }

    pub fn cross(&self, other: Tuple) -> Tuple {
        Tuple::vector(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        )
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        approx_eq!(f64, self.0, other.0)
            && approx_eq!(f64, self.1, other.1)
            && approx_eq!(f64, self.2, other.2)
            && approx_eq!(f64, self.3, other.3)
    }
}

impl std::ops::Add for Tuple {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2,
            self.3 + other.3,
        )
    }
}

impl std::ops::Sub for Tuple {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self(
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2,
            self.3 - other.3,
        )
    }
}

impl std::ops::Neg for Tuple {
    type Output = Self;
    fn neg(self) -> Self {
        Self(-self.0, -self.1, -self.2, -self.3)
    }
}

impl std::ops::Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs, self.3 * rhs)
    }
}

impl std::ops::Div<f64> for Tuple {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs, self.2 / rhs, self.3 / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_point() {
        let a = Tuple(4.3, -4.2, 3.1, 1.0);
        assert_eq!(a.x(), 4.3);
        assert_eq!(a.y(), -4.2);
        assert_eq!(a.z(), 3.1);
        assert_eq!(a.w(), 1.0);
        assert!(a.is_point());
        assert!(!a.is_vector());
    }
    #[test]
    fn test_vector() {
        let a = Tuple(4.3, -4.2, 3.1, 0.0);
        assert_eq!(a.x(), 4.3);
        assert_eq!(a.y(), -4.2);
        assert_eq!(a.z(), 3.1);
        assert_eq!(a.w(), 0.0);
        assert!(!a.is_point());
        assert!(a.is_vector());
    }

    #[test]
    fn test_point_function() {
        let p = Tuple::point(4.0, -4.0, 3.0);
        assert_eq!(p, Tuple(4.0, -4.0, 3.0, 1.0));
    }

    #[test]
    fn test_vector_function() {
        let v = Tuple::vector(4.0, -4.0, 3.0);
        assert_eq!(v, Tuple(4.0, -4.0, 3.0, 0.0));
    }

    #[test]
    fn test_tuple_addition() {
        let a1 = Tuple(3.0, -2.0, 5.0, 1.0);
        let a2 = Tuple(-2.0, 3.0, 1.0, 0.0);
        assert_eq!(a1 + a2, Tuple(1.0, 1.0, 6.0, 1.0));
    }

    #[test]
    fn test_point_subtract_point() {
        let p1 = Tuple::point(3.0, 2.0, 1.0);
        let p2 = Tuple::point(5.0, 6.0, 7.0);
        assert_eq!(p1 - p2, Tuple::vector(-2.0, -4.0, -6.0));
    }
    #[test]
    fn test_point_subtract_vector() {
        let p = Tuple::point(3.0, 2.0, 1.0);
        let v = Tuple::vector(5.0, 6.0, 7.0);
        assert_eq!(p - v, Tuple::point(-2.0, -4.0, -6.0));
    }
    #[test]
    fn test_vector_subtract_vector() {
        let v1 = Tuple::vector(3.0, 2.0, 1.0);
        let v2 = Tuple::vector(5.0, 6.0, 7.0);
        assert_eq!(v1 - v2, Tuple::vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn test_0vector_subtract_vector() {
        let zero = Tuple::vector(0.0, 0.0, 0.0);
        let v = Tuple::vector(1.0, -2.0, 3.0);
        assert_eq!(zero - v, Tuple::vector(-1.0, 2.0, -3.0))
    }

    #[test]
    fn test_tuple_negation() {
        let a = Tuple(1.0, -2.0, 3.0, -4.0);
        assert_eq!(-a, Tuple(-1.0, 2.0, -3.0, 4.0))
    }

    #[test]
    fn test_tuple_scalar_multiplication() {
        let a = Tuple(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a * 3.5, Tuple(3.5, -7.0, 10.5, -14.0));
        assert_eq!(a * 0.5, Tuple(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn test_tuple_scalar_division() {
        let a = Tuple(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a / 2.0, Tuple(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn test_vector_magnitude() {
        let v = Tuple::vector(1.0, 0.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
        let v = Tuple::vector(0.0, 1.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
        let v = Tuple::vector(0.0, 0.0, 1.0);
        assert_eq!(v.magnitude(), 1.0);
        let v = Tuple::vector(1.0, 2.0, 3.0);
        assert_eq!(v.magnitude(), 14.0f64.sqrt());
        let v = Tuple::vector(-1.0, -2.0, -3.0);
        assert_eq!(v.magnitude(), 14.0f64.sqrt());
    }

    #[test]
    fn test_vector_normalization() {
        let v = Tuple::vector(4.0, 0.0, 0.0);
        assert_eq!(v.normalize(), Tuple::vector(1.0, 0.0, 0.0));
        let v = Tuple::vector(1.0, 2.0, 3.0);
        assert_eq!(
            v.normalize(),
            Tuple::vector(1.0 / 14.0f64.sqrt(), 2.0 / 14f64.sqrt(), 3.0 / 14f64.sqrt())
        );
        let v = Tuple::vector(1.0, 2.0, 3.0);
        assert_eq!(v.normalize().magnitude(), 1.0);
    }

    #[test]
    fn test_vector_dot_product() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);
        assert_eq!(a.dot(b), 20.0);
    }

    #[test]
    fn test_vector_cross_product() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);
        assert_eq!(a.cross(b), Tuple::vector(-1.0, 2.0, -1.0));
        assert_eq!(b.cross(a), Tuple::vector(1.0, -2.0, 1.0));
    }
}
