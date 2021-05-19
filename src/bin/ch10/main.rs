#![feature(const_generics)]
#![feature(const_evaluatable_checked)]
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
    pattern::{Checkerboard, Gradient, Ring, Stripe},
    shape::Shape,
};

fn main() {
    let mut floor = Plane::new(0);
    floor.material = Material::default();
    floor.material.color = Color::new(1.0, 0.0, 0.9);
    floor.material.specular = 0.0;
    let mut pattern = Stripe::new(Color::new(0.8, 0.5, 0.5), Color::BLACK);
    pattern.transform = pattern.transform.scaling(0.1, 0.1, 0.1);
    floor.material.pattern = Some(Box::new(pattern));

    let mut backing = Plane::new(1);
    backing.transform = Matrix::<4>::IDENTITY
        .rotation_x(FRAC_PI_2)
        .rotation_y(FRAC_PI_4)
        .translation(0.0, 0.0, 4.0);
    backing.material.color = Color::new(1.0, 0.9, 0.9);
    backing.material.specular = 0.2;
    let mut pattern = Gradient::new(Color::WHITE, Color::BLACK);
    pattern.transform = pattern
        .transform
        .scaling(16.0, 1.0, 1.0)
        .rotation_x(-FRAC_PI_2)
        .rotation_y(FRAC_PI_4)
        .translation(0.0, 0.0, -5.0);
    backing.material.pattern = Some(Box::new(pattern));

    let mut middle = Sphere::new(1);
    middle.transform = Matrix::<4>::IDENTITY.translation(-0.5, 1.0, 0.5);
    middle.material = Material::default();
    middle.material.color = Color::new(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut right = Sphere::new(2);
    right.transform = Matrix::<4>::IDENTITY
        .rotation_y(FRAC_PI_3)
        .scaling(0.5, 0.5, 0.5)
        .translation(1.5, 0.5, -0.5);
    right.material = Material::default();
    right.material.color = Color::new(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;
    let mut pattern = Ring::new(Color::WHITE, Color::BLACK);
    pattern.transform = pattern.transform.scaling(0.1, 0.1, 0.1);
    right.material.pattern = Some(Box::new(pattern));

    let mut left = Sphere::new(3);
    left.transform = Matrix::<4>::IDENTITY
        .scaling(0.33, 0.33, 0.33)
        .translation(-1.5, 0.33, -0.75);
    left.material = Material::default();
    left.material.color = Color::new(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;
    let mut pattern = Checkerboard::new(Color::WHITE, Color::BLACK);
    left.material.pattern = Some(Box::new(pattern));

    let objects: Vec<Rc<RefCell<dyn Shape>>> = vec![
        Rc::new(RefCell::new(floor)),
        Rc::new(RefCell::new(backing)),
        Rc::new(RefCell::new(middle)),
        Rc::new(RefCell::new(left)),
        Rc::new(RefCell::new(right)),
    ];
    let mut world = World::default();
    world.objects = objects;

    let mut camera = Camera::new(500, 250, FRAC_PI_3);
    camera.transform = view_transform(
        Point::new(0.0, 1.5, -5.0),
        Point::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = render(camera, world);
    canvas.to_ppm("ch10.ppm").unwrap();
}
