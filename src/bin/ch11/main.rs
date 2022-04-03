#![feature(generic_const_exprs)]
#![feature(assert_matches)]
mod camera;
mod canvas;
mod lights;
mod materials;
mod matrix;
mod pattern;
mod ray;
mod shape;
mod transformations;
mod tuple;
mod world;
use camera::{render, Camera};

use materials::Material;
use matrix::Matrix;

use shape::{Plane, Sphere};
use std::{
    cell::RefCell,
    f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, FRAC_PI_6},
    rc::Rc,
};
use transformations::{view_transform, Transformation};
use world::World;

use tuple::{Color, Point, Tuple, Vector};

use crate::{
    lights::PointLight,
    pattern::{Checkerboard, Gradient, Pattern, Ring, Stripe},
    shape::Shape,
};

pub const DEFAULT_REFLECTION_COUNT: u32 = 4;

fn main() {
    let mut floor = Plane::new(0);
    floor.transform = Matrix::default()
        .rotation_x(FRAC_PI_2)
        .translation(0.0, 0.0, 10.0);
    floor.material = Material::default();
    floor.material.color = Color::WHITE;
    floor.material.specular = 0.0;
    let mut pattern = Checkerboard::new(Color::WHITE, Color::BLACK);

    floor.material.pattern = Some(Box::new(pattern));

    let mut glass = Sphere::new_glass(1);
    glass.material.ambient = 0.0;
    glass.material.diffuse = 0.0;
    glass.material.reflective = 1.0;

    let mut air = Sphere::new_glass(2);
    air.transform = air.transform.scaling(0.5, 0.5, 0.5);
    air.material.ambient = 0.0;
    air.material.diffuse = 0.0;
    air.material.reflective = 1.0;
    air.material.refractive_index = 1.0;

    let objects: Vec<Rc<RefCell<dyn Shape>>> = vec![
        Rc::new(RefCell::new(floor)),
        Rc::new(RefCell::new(glass)),
        Rc::new(RefCell::new(air)),
    ];

    let mut world = World::default();
    world.objects = objects;

    let mut camera = Camera::new(1000, 1000, FRAC_PI_2);
    camera.transform = view_transform(
        Point::new(0.0, 0.0, -1.5),
        Point::new(0.0, 0.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = render(camera, world, DEFAULT_REFLECTION_COUNT);

    canvas.to_ppm("ch11_fresnel.ppm").unwrap();
}
