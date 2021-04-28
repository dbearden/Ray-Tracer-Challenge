mod tuple;
use std::{io::Write};

use tuple::{Color, Point, Tuple, Vector};
mod canvas;
use canvas::Canvas;

struct Projectile {
    position: Point,
    velocity: Vector,
}
struct Environment {
    gravity: Vector,
    wind: Vector,
}
fn main() {
    let mut p = Projectile {
        position: Point::new(0.0, 1.0, 0.0),
        velocity: Vector::new(1.0, 2.0, 0.0).normalize() * 10.0,
    };

    let e = Environment {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    let mut path = Vec::new();

    let mut max_x = 0f64;
    let mut max_y = 0f64;
    while p.position.y >= 0f64 {
        println!("x: {}, y: {}", p.position.x, p.position.y);
        path.push((p.position.x, p.position.y));
        p = tick(&e, p);
        max_x = max_x.max(p.position.x);

        max_y = max_y.max(p.position.y);
    }
    path.push((p.position.x, p.position.y));

    let mut c = Canvas::new(max_x as usize + 1, max_y as usize + 1);

    for (x, y) in path {
        c.write(
            x as usize,
            c.height - y as usize - 1,
            Color::new(x / c.width as f64, y / c.height as f64, 1.0),
        );
    }

    c.to_ppm("foo.ppm").unwrap();
}

fn tick(env: &Environment, proj: Projectile) -> Projectile {
    let position = proj.position + proj.velocity;
    let velocity = proj.velocity + env.gravity + env.wind;
    Projectile { position, velocity }
}
