pub mod checkerboard;
pub mod gradient;
pub mod ring;
pub mod stripe;
use std::{cell::RefCell, fmt::Debug, rc::Rc};

pub use checkerboard::Checkerboard;
pub use gradient::Gradient;
pub use ring::Ring;
pub use stripe::Stripe;

use crate::{
    matrix::Matrix,
    shape::Shape,
    transformations::Transformation,
    tuple::{Color, Point},
};
pub trait Pattern: Debug {
    fn pattern_at(&self, point: &Point) -> Color;
    fn pattern_at_shape(&self, shape: &Shape, point: &Point) -> Color {
        let point = shape.get_transform().inverse().transpose() * *point;
        self.pattern_at(&point)
    }
    fn transform(&self) -> Matrix<4>;
    fn set_transform(&mut self, transform: Matrix<4>);
}

#[cfg(test)]
pub mod tests {
    use crate::{
        matrix::Matrix,
        shape::{Shape, Sphere},
        tuple::{Color, Tuple},
    };

    use super::*;

    #[derive(Debug)]
    pub struct TestPattern {
        transform: Matrix<4>,
    }

    impl TestPattern {
        pub fn new() -> Self {
            Self {
                transform: Default::default(),
            }
        }
    }
    impl Pattern for TestPattern {
        fn transform(&self) -> Matrix<4> {
            self.transform
        }

        fn set_transform(&mut self, transform: Matrix<4>) {
            self.transform = transform
        }

        fn pattern_at(&self, point: &Point) -> Color {
            let point = self.transform.inverse() * *point;
            Color::new(point.x, point.y, point.z)
        }
    }

    #[test]
    fn default_pattern_transformation() {
        let pattern = TestPattern::new();
        assert_eq!(pattern.transform(), Matrix::default());
    }

    #[test]
    fn assign_transformation() {
        let mut pattern = TestPattern::new();
        pattern.set_transform(Matrix::default().translation(1.0, 2.0, 3.0));
        assert_eq!(
            pattern.transform(),
            Matrix::default().translation(1.0, 2.0, 3.0)
        );
    }

    #[test]
    fn pattern_with_object_transformation() {
        let mut shape = Sphere::new(0);
        shape.set_transform(Matrix::default().scaling(2.0, 2.0, 2.0));
        let pattern = TestPattern::new();
        let c = pattern.pattern_at_shape(&shape, &Point::new(2.0, 3.0, 4.0));
        assert_eq!(c, Color::new(1.0, 1.5, 2.0));
    }

    #[test]
    fn pattern_with_pattern_transform() {
        let shape = Sphere::new(0);
        let mut pattern = TestPattern::new();
        pattern.set_transform(Matrix::default().scaling(2.0, 2.0, 2.0));
        let c = pattern.pattern_at_shape(&shape, &Point::new(2.0, 3.0, 4.0));
        assert_eq!(c, Color::new(1.0, 1.5, 2.0));
    }

    #[test]
    fn pattern_with_both_transforms() {
        let mut shape = Sphere::new(0);
        shape.set_transform(Matrix::default().scaling(2.0, 2.0, 2.0));
        let mut pattern = TestPattern::new();
        pattern.set_transform(Matrix::default().translation(0.5, 1.0, 1.5));
        let c = pattern.pattern_at_shape(&shape, &Point::new(2.5, 3.0, 3.5));
        assert_eq!(c, Color::new(0.75, 0.5, 0.25));
    }
}
