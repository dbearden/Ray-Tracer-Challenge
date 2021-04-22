#![feature(destructuring_assignment)]
#![feature(const_generics)]
#![feature(const_evaluatable_checked)]
mod canvas;
mod matrix;
mod transformations;
mod tuple;
use canvas::Canvas;
use matrix::Matrix;
use transformations::Transformation;
use tuple::{Color, Point, Tuple};

fn main() {
    let mut c = Canvas::new(100, 100);
    let points: Vec<_> = [Point::new(c.width as f64 / 4.0, 0.0, 0.0); 12]
        .iter()
        .scan(0.0, |state, p| {
            let p = p.rotation_z(*state * std::f64::consts::FRAC_PI_6);
            *state += 1.0;
            Some(p)
        })
        .map(|p| p.translation(c.width as f64 / 2.0, c.height as f64 / 2.0, 0.0))
        .collect();

    for point in points {
        c.write(
            point.x() as usize,
            point.y() as usize,
            Color::new(1.0, 1.0, 1.0),
        );
    }

    c.to_ppm("clock.ppm").unwrap();
}
