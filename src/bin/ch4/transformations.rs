

pub trait Transformation {
    fn translation(&self, x: f64, y: f64, z: f64) -> Self;
    fn scaling(&self, x: f64, y: f64, z: f64) -> Self;
    fn shearing(&self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self;
    fn rotation_x(&self, r: f64) -> Self;
    fn rotation_y(&self, r: f64) -> Self;
    fn rotation_z(&self, r: f64) -> Self;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::FRAC_1_SQRT_2;
    use std::f64::consts::FRAC_PI_2;
    use std::f64::consts::FRAC_PI_4;

    #[test]
    fn multiply_by_transformation() {
        let t = Matrix::<4>::IDENTITY.translation(5.0, -3.0, 2.0);
        let p = Point::new(-3.0, 4.0, 5.0);
        assert_eq!(t * p, Point::new(2.0, 1.0, 7.0));
    }

    #[test]
    fn multiply_by_translation_inverse() {
        let inv = Matrix::<4>::IDENTITY.translation(5.0, -3.0, 2.0).inverse();
        let p = Point::new(-3.0, 4.0, 5.0);
        assert_eq!(inv * p, Point::new(-8.0, 7.0, 3.0));
    }

    #[test]
    fn translation_does_not_affect_vector() {
        let t = Matrix::<4>::IDENTITY.translation(5.0, -3.0, 2.0);
        let v = Vector::new(-3.0, 4.0, 5.0);
        assert_eq!(t * v, v.clone());
    }

    #[test]
    fn scaling_a_point() {
        let t = Matrix::<4>::IDENTITY.scaling(2.0, 3.0, 4.0);
        let p = Point::new(-4.0, 6.0, 8.0);
        assert_eq!(t * p, Point::new(-8.0, 18.0, 32.0));
    }
    #[test]
    fn scaling_a_vector() {
        let t = Matrix::<4>::IDENTITY.scaling(2.0, 3.0, 4.0);
        let v = Vector::new(-4.0, 6.0, 8.0);
        assert_eq!(t * v, Vector::new(-8.0, 18.0, 32.0));
    }

    #[test]
    fn inverse_scaling_a_vector() {
        let t = Matrix::<4>::IDENTITY.scaling(2.0, 3.0, 4.0);
        let inv = t.inverse();
        let v = Vector::new(-4.0, 6.0, 8.0);
        assert_eq!(inv * v, Vector::new(-2.0, 2.0, 2.0));
    }

    #[test]
    fn reflection() {
        let t = Matrix::<4>::IDENTITY.scaling(-1.0, 1.0, 1.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(t * p, Point::new(-2.0, 3.0, 4.0));
    }

    #[test]
    fn rotate_around_x() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = Matrix::<4>::IDENTITY.rotation_x(FRAC_PI_4);
        let full_quarter = Matrix::<4>::IDENTITY.rotation_x(FRAC_PI_2);
        assert_eq!(
            half_quarter * p,
            Point::new(0.0, FRAC_1_SQRT_2, FRAC_1_SQRT_2)
        );
        assert_eq!(full_quarter * p, Point::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn inverse_rotation_around_x() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = Matrix::<4>::IDENTITY.rotation_x(FRAC_PI_4);
        let inv = half_quarter.inverse();
        assert_eq!(inv * p, Point::new(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
    }

    #[test]
    fn rotate_around_y() {
        let p = Point::new(0.0, 0.0, 1.0);
        let half_quarter = Matrix::<4>::IDENTITY.rotation_y(FRAC_PI_4);
        let full_quarter = Matrix::<4>::IDENTITY.rotation_y(FRAC_PI_2);
        assert_eq!(
            half_quarter * p,
            Point::new(FRAC_1_SQRT_2, 0.0, FRAC_1_SQRT_2)
        );
        assert_eq!(full_quarter * p, Point::new(1.0, 0.0, 0.0));
    }
    #[test]
    fn rotate_around_z() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = Matrix::<4>::IDENTITY.rotation_z(FRAC_PI_4);
        let full_quarter = Matrix::<4>::IDENTITY.rotation_z(FRAC_PI_2);
        assert_eq!(
            half_quarter * p,
            Point::new(-FRAC_1_SQRT_2, FRAC_1_SQRT_2, 0.0)
        );
        assert_eq!(full_quarter * p, Point::new(-1.0, 0.0, 0.0));
    }

    #[test]
    fn shear_x_by_y() {
        let t = Matrix::<4>::IDENTITY.shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(t * p, Point::new(5.0, 3.0, 4.0));
    }
    #[test]
    fn shear_x_by_z() {
        let t = Matrix::<4>::IDENTITY.shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(t * p, Point::new(6.0, 3.0, 4.0));
    }
    #[test]
    fn shear_y_by_x() {
        let t = Matrix::<4>::IDENTITY.shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(t * p, Point::new(2.0, 5.0, 4.0));
    }
    #[test]
    fn shear_y_by_z() {
        let t = Matrix::<4>::IDENTITY.shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(t * p, Point::new(2.0, 7.0, 4.0));
    }
    #[test]
    fn shear_z_by_x() {
        let t = Matrix::<4>::IDENTITY.shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(t * p, Point::new(2.0, 3.0, 6.0));
    }
    #[test]
    fn shear_z_by_y() {
        let t = Matrix::<4>::IDENTITY.shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(t * p, Point::new(2.0, 3.0, 7.0));
    }

    #[test]
    fn individual_transformations_applied_in_sequence() {
        let p = Point::new(1.0, 0.0, 1.0);
        let A = Matrix::<4>::IDENTITY.rotation_x(FRAC_PI_2);
        let B = Matrix::<4>::IDENTITY.scaling(5.0, 5.0, 5.0);
        let C = Matrix::<4>::IDENTITY.translation(10.0, 5.0, 7.0);

        let p2 = A * p;
        assert_eq!(p2, Point::new(1.0, -1.0, 0.0));

        let p3 = B * p2;
        assert_eq!(p3, Point::new(5.0, -5.0, 0.0));

        let p4 = C * p3;
        assert_eq!(p4, Point::new(15.0, 0.0, 7.0));
    }

    #[test]
    fn chained_transformations_applied_in_reverse() {
        let p = Point::new(1.0, 0.0, 1.0);
        let T = Matrix::<4>::IDENTITY
            .rotation_x(FRAC_PI_2)
            .scaling(5.0, 5.0, 5.0)
            .translation(10.0, 5.0, 7.0);

        assert_eq!(T * p, Point::new(15.0, 0.0, 7.0));
    }
}
