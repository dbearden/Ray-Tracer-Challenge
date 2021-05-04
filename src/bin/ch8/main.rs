#![feature(const_generics)]
#![feature(const_evaluatable_checked)]
mod camera;
mod canvas;
mod lights;
mod materials;
mod matrix;
mod ray;
mod shapes;
mod transformations;
mod tuple;
mod world;
use camera::{render, Camera};

use materials::Material;
use matrix::Matrix;

use shapes::Sphere;
use std::{
    cell::RefCell,
    f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4},
    rc::Rc,
};
use transformations::{view_transform, Transformation};
use world::World;

use tuple::{Color, Point, Tuple, Vector};

use crate::{lights::PointLight, shapes::Shape};

fn main() {
    let mut floor = Sphere::new(0);
    floor.transform = Matrix::<4>::IDENTITY.scaling(10.0, 0.01, 10.0);
    floor.material = Material::default();
    floor.material.color = Color::new(1.0, 0.9, 0.9);
    floor.material.specular = 0.0;

    let mut left_wall = Sphere::new(1);
    left_wall.transform = Matrix::<4>::IDENTITY
        .scaling(10.0, 0.01, 10.0)
        .rotation_x(FRAC_PI_2)
        .rotation_y(-FRAC_PI_4)
        .translation(0.0, 0.0, 5.0);
    left_wall.material = floor.material;

    let mut right_wall = Sphere::new(1);
    right_wall.transform = Matrix::<4>::IDENTITY
        .scaling(10.0, 0.01, 10.0)
        .rotation_x(FRAC_PI_2)
        .rotation_y(FRAC_PI_4)
        .translation(0.0, 0.0, 5.0);
    right_wall.material = floor.material;

    let mut middle = Sphere::new(2);
    middle.transform = Matrix::<4>::IDENTITY.translation(-0.3, 1.0, -1.0);
    middle.material = Material::default();
    middle.material.color = Color::new(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut right = Sphere::new(3);
    right.transform = Matrix::<4>::IDENTITY
        .scaling(0.5, 0.5, 0.5)
        .translation(1.5, 0.5, -0.5);
    right.material = Material::default();
    right.material.color = Color::new(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut left = Sphere::new(4);
    left.transform = Matrix::<4>::IDENTITY
        .scaling(0.33, 0.33, 0.33)
        .translation(-0.9, 1.8, -2.3);
    left.material = Material::default();
    left.material.color = Color::new(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    let objects: Vec<Rc<RefCell<dyn Shape>>> = vec![
        Rc::new(RefCell::new(floor)),
        Rc::new(RefCell::new(left_wall)),
        Rc::new(RefCell::new(right_wall)),
        Rc::new(RefCell::new(middle)),
        Rc::new(RefCell::new(left)),
        Rc::new(RefCell::new(right)),
    ];
    let mut world = World::default();
    world.objects = objects;

    let light = PointLight::new(Point::new(-10.0, 10.0, -5.0), Color::new(0.7, 0.0, 0.0));
    world.lights.push(light);

    let mut camera = Camera::new(500, 400, FRAC_PI_2);
    camera.transform = view_transform(
        Point::new(0.0, 1.5, -5.0),
        Point::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = render(camera, world);
    canvas.to_ppm("shadows.ppm").unwrap();
}
