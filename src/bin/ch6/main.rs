#![feature(generic_const_exprs)]
mod canvas;
mod lights;
mod materials;
mod matrix;
mod ray;
mod shapes;
mod transformations;
mod tuple;
use canvas::Canvas;
use lights::PointLight;
use materials::{lighting, Material};
use ray::{hit, Ray};
use shapes::Sphere;
use std::{cell::RefCell, rc::Rc};

use tuple::{Color, Point, Tuple};

fn main() {
    let s = Rc::new(RefCell::new(Sphere::new(0)));
    s.borrow_mut().material = Material::default();
    s.borrow_mut().material.color = Color::new(1.0, 0.2, 1.0);

    let light_position = Point::new(-10.0, 10.0, -10.0);
    let light_color = Color::new(1.0, 1.0, 1.0);
    let light = PointLight::new(light_position, light_color);

    let ray_origin = Point::new(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 500;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;
    let mut c = Canvas::new(canvas_pixels, canvas_pixels);
    for y in 0..canvas_pixels - 1 {
        let world_y = half - pixel_size * y as f64;
        for x in 0..canvas_pixels - 1 {
            let world_x = -half + pixel_size * x as f64;
            let position = Point::new(world_x, world_y, wall_z);
            let r = Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = r.intersect(s.clone());
            if let Some(hit) = hit(xs) {
                let point = r.position(hit.t);
                let normal = hit.object.borrow().normal_at(point);
                let eye = -r.direction;
                let color = lighting(hit.object.borrow().material(), light, point, eye, normal);
                c.write(x, y, color)
            }
        }
    }
    c.to_ppm("sphere_lit_and_shaded.ppm").unwrap();
}
