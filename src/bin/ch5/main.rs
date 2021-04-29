#![feature(const_generics)]
#![feature(const_evaluatable_checked)]
mod canvas;
mod matrix;
mod ray;
mod shapes;
mod transformations;
mod tuple;
use canvas::Canvas;
use matrix::Matrix;
use ray::{hit, set_transform, Ray};
use shapes::Sphere;

use transformations::Transformation;
use tuple::{Color, Point, Tuple};

fn main() {
    let s = Sphere::new(0);
    let ray_origin = Point::new(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 100;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;
    let mut c = Canvas::new(canvas_pixels, canvas_pixels);
    let color = Color::new(1.0, 0.0, 0.0);
    set_transform(
        s.clone(),
        Matrix::<4>::IDENTITY
            .scaling(0.5, 1.0, 1.0)
            .translation(1.0, 0.0, 1.0),
    );
    for y in 0..canvas_pixels - 1 {
        let world_y = half - pixel_size * y as f64;
        for x in 0..canvas_pixels - 1 {
            let world_x = -half + pixel_size * x as f64;
            let position = Point::new(world_x, world_y, wall_z);
            let r = Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = r.intersect(s.clone());
            if let Some(_i) = hit(xs) {
                c.write(x, y, color)
            }
        }
    }
    c.to_ppm("sphere_silhouette.ppm").unwrap();
}
