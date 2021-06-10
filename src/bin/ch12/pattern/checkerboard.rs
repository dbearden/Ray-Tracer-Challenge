use crate::{
    matrix::Matrix,
    tuple::{Color, Point},
};

use super::Pattern;

#[derive(Debug)]
pub struct Checkerboard {
    a: Color,
    b: Color,
    pub transform: Matrix<4>,
}

impl Checkerboard {
    pub fn new(a: Color, b: Color) -> Self {
        Self {
            a,
            b,
            transform: Default::default(),
        }
    }
}

impl Pattern for Checkerboard {
    fn pattern_at(&self, point: &Point) -> Color {
        let point = self.transform.inverse() * *point;
        if (point.x.floor() + point.y.floor() + point.z.floor()) % 2.0 == 0.0 {
            self.a
        } else {
            self.b
        }
    }

    fn transform(&self) -> Matrix<4> {
        todo!()
    }

    fn set_transform(&mut self, transform: Matrix<4>) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::tuple::Tuple;

    use super::*;

    #[test]
    fn checker_should_repeat_in_x() {
        let pattern = Checkerboard::new(Color::WHITE, Color::BLACK);
        assert_eq!(pattern.pattern_at(&Point::new(0.0, 0.0, 0.0)), Color::WHITE);
        assert_eq!(
            pattern.pattern_at(&Point::new(0.99, 0.0, 0.0)),
            Color::WHITE
        );
        assert_eq!(
            pattern.pattern_at(&Point::new(1.01, 0.0, 0.0)),
            Color::BLACK
        );
    }
    #[test]
    fn checker_should_repeat_in_y() {
        let pattern = Checkerboard::new(Color::WHITE, Color::BLACK);
        assert_eq!(pattern.pattern_at(&Point::new(0.0, 0.0, 0.0)), Color::WHITE);
        assert_eq!(
            pattern.pattern_at(&Point::new(0.0, 0.99, 0.0)),
            Color::WHITE
        );
        assert_eq!(
            pattern.pattern_at(&Point::new(0.0, 1.01, 0.0)),
            Color::BLACK
        );
    }
    #[test]
    fn checker_should_repeat_in_z() {
        let pattern = Checkerboard::new(Color::WHITE, Color::BLACK);
        assert_eq!(pattern.pattern_at(&Point::new(0.0, 0.0, 0.0)), Color::WHITE);
        assert_eq!(
            pattern.pattern_at(&Point::new(0.0, 0.0, 0.99)),
            Color::WHITE
        );
        assert_eq!(
            pattern.pattern_at(&Point::new(0.0, 0.0, 1.01)),
            Color::BLACK
        );
    }
}
