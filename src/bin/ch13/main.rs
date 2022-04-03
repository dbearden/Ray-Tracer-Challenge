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

use float_cmp::approx_eq;
use materials::Material;
use matrix::Matrix;

use shape::{Cube, Plane, Shape, Sphere};
use std::{
    cell::RefCell,
    cmp::Ordering,
    f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, FRAC_PI_6},
    rc::Rc,
};
use transformations::{view_transform, Transformation};
use world::World;

use tuple::{Color, Point, Tuple, Vector};

use crate::{
    lights::PointLight,
    pattern::{Checkerboard, Gradient, Pattern, Ring, Stripe},
    shape::{Cone, Cylinder},
};

pub const DEFAULT_REFLECTION_COUNT: u32 = 4;

fn main() {
    let mut floor = Plane::new(0);
    floor.material.reflective = 0.07;
    floor.material.pattern = Some(Box::new(Checkerboard::new(
        Color::new(0.9, 0.9, 0.9),
        Color::BLACK,
    )));

    let mut room = Cube::new(1);
    room.transform = Matrix::default()
        .scaling(10.0, 10.0, 10.0)
        .translation(0.0, 9.0, 0.0);
    room.material.color = Color::new(0.0, 0.3, 0.3);

    let mut tabletop = Cylinder::new(2);
    tabletop.maximum = 1.0;
    tabletop.minimum = -1.0;
    tabletop.closed = true;
    tabletop.transform = Matrix::default()
        .scaling(3.0, 0.2, 3.0)
        .translation(0.0, 3.0, 0.0);
    tabletop.material.reflective = 0.01;
    tabletop.material.color = Color::new(0.4, 0.2, 0.0);
    let mut pattern = Stripe::new(Color::new(0.0, 1.0, 0.0), Color::new(0.4, 0.2, 0.0));
    pattern.set_transform(Matrix::default().scaling(0.2, 0.2, 0.2));
    tabletop.material.pattern = Some(Box::new(pattern));

    let mut leg1 = Cylinder::new(3);
    leg1.maximum = 1.0;
    leg1.minimum = -1.0;
    leg1.closed = true;
    leg1.transform = Matrix::default()
        .scaling(0.15, 1.5, 0.15)
        .translation(-2.85, 1.3, -2.85);
    leg1.material.color = Color::new(0.4, 0.2, 0.0);

    let mut leg2 = Cylinder::new(4);
    leg2.maximum = 1.0;
    leg2.minimum = -1.0;
    leg2.closed = true;
    leg2.transform = Matrix::default()
        .scaling(0.15, 1.5, 0.15)
        .translation(2.85, 1.3, -2.85);
    leg2.material.color = Color::new(0.4, 0.2, 0.0);

    let mut leg3 = Cylinder::new(5);
    leg3.maximum = 1.0;
    leg3.minimum = -1.0;
    leg3.closed = true;
    leg3.transform = Matrix::default()
        .scaling(0.15, 1.5, 0.15)
        .translation(-2.85, 1.3, 2.85);
    leg3.material.color = Color::new(0.4, 0.2, 0.0);

    let mut leg4 = Cylinder::new(6);
    leg4.maximum = 1.0;
    leg4.minimum = -1.0;
    leg4.closed = true;
    leg4.transform = Matrix::default()
        .scaling(0.15, 1.5, 0.15)
        .translation(2.85, 1.3, 2.85);
    leg4.material.color = Color::new(0.4, 0.2, 0.0);

    let mut ball = Sphere::new_glass(7);
    ball.transform = Matrix::default()
        .scaling(0.5, 0.5, 0.5)
        .translation(-3.0, 4.0, 0.6);
    ball.material.diffuse = 0.001;
    ball.material.reflective = 1.0;

    let mut cube = Cube::new(8);
    cube.material.transparency = 1.0;
    cube.material.reflective = 1.0;
    cube.material.refractive_index = 1.9;
    cube.material.diffuse = 0.01;
    cube.transform = Matrix::default()
        .scaling(0.5, 0.5, 0.5)
        .translation(-2.0, 4.0, 1.9);

    let mut cube2 = Cube::new(9);
    cube2.material.color = Color::new(1.0, 0.0, 0.0);
    cube2.transform = Matrix::default()
        .scaling(0.12, 1.0, 0.25)
        .translation(0.0, 4.0, 0.8);
    let mut cube3 = Cube::new(10);
    cube3.material.color = Color::new(0.0, 0.0, 1.0);
    cube3.transform = Matrix::default()
        .scaling(0.2, 0.2, 2.0)
        .translation(-0.3, 3.4, -0.3);

    let objects: Vec<Rc<RefCell<dyn Shape>>> = vec![
        Rc::new(RefCell::new(floor)),
        Rc::new(RefCell::new(room)),
        Rc::new(RefCell::new(tabletop)),
        Rc::new(RefCell::new(leg1)),
        Rc::new(RefCell::new(leg2)),
        Rc::new(RefCell::new(leg3)),
        Rc::new(RefCell::new(leg4)),
        Rc::new(RefCell::new(ball)),
        Rc::new(RefCell::new(cube)),
        Rc::new(RefCell::new(cube2)),
        Rc::new(RefCell::new(cube3)),
    ];

    let mut world = World::default();
    world.objects = objects;
    world.lights[0].position = Point::new(-4.0, 9.0, 3.0);

    let mut camera = Camera::new(1000, 750, FRAC_PI_2);
    camera.transform = view_transform(
        Point::new(-6.0, 5.0, 3.0),
        Point::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = render(camera, world, DEFAULT_REFLECTION_COUNT);

    canvas.to_ppm("ch13_table_scene.ppm").unwrap();
}

fn float_cmp(first: &f64, second: &f64) -> std::cmp::Ordering {
    if approx_eq!(f64, *first, *second, epsilon = 0.00003) {
        Ordering::Equal
    } else if *first < *second {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}
