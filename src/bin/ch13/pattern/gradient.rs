use crate::{
    matrix::Matrix,
    tuple::{Color, Point},
};

use super::Pattern;

#[derive(Debug)]
pub struct Gradient {
    a: Color,
    b: Color,
    pub transform: Matrix<4>,
}
impl Gradient {
    pub fn new(a: Color, b: Color) -> Self {
        Self {
            a,
            b,
            transform: Default::default(),
        }
    }
}
impl Pattern for Gradient {
    fn pattern_at(&self, point: &Point) -> Color {
        let point = self.transform.inverse() * *point;
        let distance = self.b - self.a;
        let fraction = point.x - point.x.floor();
        self.a + distance * fraction
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
    fn gradient_linearly_interpolates_between_colors() {
        let pattern = Gradient::new(Color::WHITE, Color::BLACK);
        assert_eq!(pattern.pattern_at(&Point::new(0.0, 0.0, 0.0)), Color::WHITE);
        assert_eq!(
            pattern.pattern_at(&Point::new(0.25, 0.0, 0.0)),
            Color::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            pattern.pattern_at(&Point::new(0.5, 0.0, 0.0)),
            Color::new(0.5, 0.5, 0.5)
        );
        assert_eq!(
            pattern.pattern_at(&Point::new(0.75, 0.0, 0.0)),
            Color::new(0.25, 0.25, 0.25)
        );
    }
}
