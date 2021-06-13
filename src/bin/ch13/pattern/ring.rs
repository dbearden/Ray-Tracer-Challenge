use crate::{
    matrix::Matrix,
    tuple::{Color, Point},
};

use super::Pattern;

#[derive(Debug)]
pub struct Ring {
    a: Color,
    b: Color,
    pub transform: Matrix<4>,
}
impl Ring {
    pub fn new(a: Color, b: Color) -> Self {
        Self {
            a,
            b,
            transform: Default::default(),
        }
    }
}

impl Pattern for Ring {
    fn transform(&self) -> Matrix<4> {
        todo!()
    }

    fn set_transform(&mut self, transform: Matrix<4>) {
        todo!()
    }

    fn pattern_at(&self, point: &Point) -> Color {
        let point = self.transform.inverse() * *point;
        if (point.x.powi(2) + point.z.powi(2)).sqrt().floor() % 2.0 == 0.0 {
            self.a
        } else {
            self.b
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tuple::{Point, Tuple};

    use super::*;

    #[test]
    fn ring_should_extend_in_both_x_and_z() {
        let pattern = Ring::new(Color::WHITE, Color::BLACK);
        assert_eq!(pattern.pattern_at(&Point::new(0.0, 0.0, 0.0)), Color::WHITE);
        assert_eq!(pattern.pattern_at(&Point::new(1.0, 0.0, 0.0)), Color::BLACK);
        assert_eq!(pattern.pattern_at(&Point::new(0.0, 0.0, 1.0)), Color::BLACK);
        assert_eq!(
            pattern.pattern_at(&Point::new(0.708, 0.0, 0.708)),
            Color::BLACK
        );
    }
}
