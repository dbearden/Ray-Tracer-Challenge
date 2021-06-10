use std::{cell::RefCell, rc::Rc};

use float_cmp::approx_eq;

use crate::{
    matrix::Matrix,
    shape::Shape,
    tuple::{Color, Point},
};

use super::Pattern;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Stripe {
    a: Color,
    b: Color,
    pub transform: Matrix<4>,
}

impl Stripe {
    pub fn new(a: Color, b: Color) -> Self {
        Self {
            a,
            b,
            transform: Default::default(),
        }
    }
}

impl Pattern for Stripe {
    fn transform(&self) -> Matrix<4> {
        self.transform
    }

    fn set_transform(&mut self, transform: Matrix<4>) {
        self.transform = transform;
    }

    fn pattern_at(&self, point: &Point) -> Color {
        let point = self.transform.inverse() * *point;
        if approx_eq!(f64, point.x.floor() % 2.0, 0.0) {
            self.a
        } else {
            self.b
        }
    }
}
#[cfg(test)]
mod tests {

    use crate::{
        lights::PointLight,
        materials::{lighting, Material},
        matrix::Matrix,
        shape::{Shape, Sphere},
        transformations::Transformation,
        tuple::{Color, Tuple, Vector},
    };

    use super::*;

    #[test]
    fn create_stripe_pattern() {
        let pattern = Stripe::new(Color::WHITE, Color::BLACK);
        assert_eq!(pattern.a, Color::WHITE);
        assert_eq!(pattern.b, Color::BLACK);
    }

    #[test]
    fn stripe_constant_in_y() {
        let pattern = Stripe::new(Color::WHITE, Color::BLACK);
        assert_eq!(pattern.pattern_at(&Point::new(0.0, 0.0, 0.0)), Color::WHITE);
        assert_eq!(pattern.pattern_at(&Point::new(0.0, 1.0, 0.0)), Color::WHITE);
        assert_eq!(pattern.pattern_at(&Point::new(0.0, 2.0, 0.0)), Color::WHITE);
    }

    #[test]
    fn stripe_constant_in_z() {
        let pattern = Stripe::new(Color::WHITE, Color::BLACK);
        assert_eq!(pattern.pattern_at(&Point::new(0.0, 0.0, 0.0)), Color::WHITE);
        assert_eq!(pattern.pattern_at(&Point::new(0.0, 0.0, 1.0)), Color::WHITE);
        assert_eq!(pattern.pattern_at(&Point::new(0.0, 0.0, 2.0)), Color::WHITE);
    }

    #[test]
    fn stripe_alternates_in_x() {
        let pattern = Stripe::new(Color::WHITE, Color::BLACK);
        assert_eq!(pattern.pattern_at(&Point::new(0.0, 0.0, 0.0)), Color::WHITE);
        assert_eq!(pattern.pattern_at(&Point::new(0.9, 0.0, 0.0)), Color::WHITE);
        assert_eq!(pattern.pattern_at(&Point::new(1.0, 0.0, 0.0)), Color::BLACK);
        assert_eq!(
            pattern.pattern_at(&Point::new(-0.1, 0.0, 0.0)),
            Color::BLACK
        );
        assert_eq!(
            pattern.pattern_at(&Point::new(-1.0, 0.0, 0.0)),
            Color::BLACK
        );
        assert_eq!(
            pattern.pattern_at(&Point::new(-1.1, 0.0, 0.0)),
            Color::WHITE
        );
    }

    #[test]
    fn lighting_with_pattern_applied() {
        let object = Rc::new(RefCell::new(Sphere::new(0)));
        let mut m = Material::default();
        m.pattern = Some(Box::new(Stripe::new(Color::WHITE, Color::BLACK)));
        m.ambient = 1.0;
        m.diffuse = 0.0;
        m.specular = 0.0;
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::WHITE);
        let c1 = lighting(
            &m,
            &*object.borrow(),
            &light,
            &Point::new(0.9, 0.0, 0.0),
            &eyev,
            &normalv,
            false,
        );
        let c2 = lighting(
            &m,
            &*object.borrow(),
            &light,
            &Point::new(1.1, 0.0, 0.0),
            &eyev,
            &normalv,
            false,
        );
        assert_eq!(c1, Color::WHITE);
        assert_eq!(c2, Color::BLACK);
    }

    #[test]
    fn stripes_with_object_transformation() {
        let mut object = Sphere::new(0);
        object.set_transform(Matrix::<4>::IDENTITY.scaling(2.0, 2.0, 2.0));
        let pattern = Stripe::new(Color::WHITE, Color::BLACK);
        let c = pattern.pattern_at_shape(&object, &Point::new(1.5, 0.0, 0.0));

        assert_eq!(c, Color::WHITE);
    }
    #[test]
    fn stripes_with_pattern_transformation() {
        let object = Sphere::new(0);
        let mut pattern = Stripe::new(Color::WHITE, Color::BLACK);
        pattern.set_transform(Matrix::<4>::IDENTITY.scaling(2.0, 2.0, 2.0));
        let c = pattern.pattern_at_shape(&object, &Point::new(1.5, 0.0, 0.0));

        assert_eq!(c, Color::WHITE);
    }
    #[test]
    fn stripes_with_both_transformation() {
        let mut object = Sphere::new(0);
        object.set_transform(Matrix::<4>::IDENTITY.scaling(2.0, 2.0, 2.0));
        let object = object;
        let mut pattern = Stripe::new(Color::WHITE, Color::BLACK);
        pattern.set_transform(Matrix::<4>::IDENTITY.translation(0.5, 0.0, 0.0));
        let c = pattern.pattern_at_shape(&object, &Point::new(2.5, 0.0, 0.0));

        assert_eq!(c, Color::WHITE);
    }
}
