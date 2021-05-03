use std::{cell::RefCell, rc::Rc};

use crate::{
    lights::PointLight,
    materials::{lighting, Material},
    matrix::Matrix,
    ray::{Intersection, Ray},
    shapes::{Shape, Sphere},
    transformations::Transformation,
    tuple::{Color, Point, Tuple, Vector},
};

pub struct World {
    pub objects: Vec<Rc<RefCell<dyn Shape>>>,
    pub lights: Vec<PointLight>,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            lights: Vec::new(),
        }
    }

    pub fn color_at(&self, ray: Ray) -> Color {
        let xs = ray.intersect_world(self);
        if let Some(i) = crate::ray::hit(xs) {
            let comps = prepare_computations(&i, &ray);
            shade_hit(self, &comps)
        } else {
            Color::BLACK
        }
    }
}

impl Default for World {
    fn default() -> Self {
        let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::WHITE);
        let s1 = Rc::new(RefCell::new({
            let mut s = Sphere::new(0);
            let mut m = Material::default();
            m.color = Color::new(0.8, 1.0, 0.6);
            m.diffuse = 0.7;
            m.specular = 0.2;
            s.material = m;
            s
        }));
        let s2 = Rc::new(RefCell::new({
            let mut s = Sphere::new(1);
            s.transform = Matrix::<4>::IDENTITY.scaling(0.5, 0.5, 0.5);
            s
        }));
        Self {
            objects: vec![s1, s2],
            lights: vec![light],
        }
    }
}

#[derive(Debug)]
pub struct Computations {
    t: f64,
    object: Rc<RefCell<dyn Shape>>,
    point: Point,
    eyev: Vector,
    normalv: Vector,
    inside: bool,
}

impl Computations {
    pub fn new(
        t: f64,
        object: Rc<RefCell<dyn Shape>>,
        point: Point,
        eyev: Vector,
        normalv: Vector,
        inside: bool,
    ) -> Self {
        Self {
            t,
            object,
            point,
            eyev,
            normalv,
            inside,
        }
    }
}

fn prepare_computations(i: &Intersection, r: &Ray) -> Computations {
    let t = i.t;
    let object = i.object.clone();

    let point = r.position(t);
    let eyev = -r.direction;
    let normalv = object.borrow().normal_at(point);
    let (inside, normalv) = if normalv.dot(eyev) < 0.0 {
        (true, -normalv)
    } else {
        (false, normalv)
    };

    Computations::new(t, object, point, eyev, normalv, inside)
}

fn shade_hit(world: &World, comps: &Computations) -> Color {
    let mut res = Color::BLACK;
    for light in &world.lights {
        res = res
            + lighting(
                comps.object.borrow().material(),
                *light,
                comps.point,
                comps.eyev,
                comps.normalv,
            )
    }

    res
}
#[cfg(test)]
mod tests {
    use crate::{
        lights::PointLight,
        materials::Material,
        matrix::Matrix,
        ray::{Intersection, Ray},
        shapes::Sphere,
        transformations::Transformation,
        tuple::{Color, Point, Tuple, Vector},
    };

    use super::*;

    #[test]
    fn create_a_world() {
        let w = World::new();
        assert!(w.objects.is_empty());
        assert!(w.lights.is_empty());
    }

    #[test]
    fn default_world() {
        let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::WHITE);
        let s1: Rc<RefCell<dyn Shape>> = Rc::new(RefCell::new({
            let mut s = Sphere::new(0);
            let mut m = Material::default();
            m.color = Color::new(0.8, 1.0, 0.6);
            m.diffuse = 0.7;
            m.specular = 0.2;
            s.material = m;
            s
        }));
        let s2: Rc<RefCell<dyn Shape>> = Rc::new(RefCell::new({
            let mut s = Sphere::new(1);
            s.transform = Matrix::<4>::IDENTITY.scaling(0.5, 0.5, 0.5);
            s
        }));

        let w = World::default();

        assert!(w.lights.contains(&light));
        assert!(w.objects.contains(&s1));
        assert!(w.objects.contains(&s2));
    }

    #[test]
    fn intersect_world_with_ray() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let xs = r.intersect_world(&w);
        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.0);
    }

    #[test]
    fn precomputing_state_of_intersection() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Sphere::new(0);
        let i = Intersection::new(4.0, Rc::new(RefCell::new(shape)));
        let comps = prepare_computations(&i, &r);
        assert_eq!(&comps.t, &i.t);
        assert_eq!(&comps.object, &i.object);
        assert_eq!(comps.point, Point::new(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, Vector::new(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn hit_when_intersection_occurs_on_exterior() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Sphere::new(0);
        let i = Intersection::new(4.0, Rc::new(RefCell::new(shape)));
        let comps = prepare_computations(&i, &r);
        assert_eq!(comps.inside, false);
    }
    #[test]
    fn hit_when_intersection_occurs_on_interior() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Sphere::new(0);
        let i = Intersection::new(1.0, Rc::new(RefCell::new(shape)));
        let comps = prepare_computations(&i, &r);
        assert_eq!(comps.point, Point::new(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, Vector::new(0.0, 0.0, -1.0));
        assert_eq!(comps.inside, true);
        assert_eq!(comps.normalv, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn shading_an_intersection() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = w.objects[0].clone();
        let i = Intersection::new(4.0, shape);
        let comps = prepare_computations(&i, &r);
        let c = shade_hit(&w, &comps);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_an_intersection_from_inside() {
        let mut w = World::default();
        w.lights[0] = PointLight::new(Point::new(0.0, 0.25, 0.0), Color::new(1.0, 1.0, 1.0));

        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let shape = w.objects[1].clone();
        let i = Intersection::new(0.5, shape);
        let comps = prepare_computations(&i, &r);
        let c = shade_hit(&w, &comps);
        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn color_when_ray_misses() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 0.0));
        let c = w.color_at(r);
        assert_eq!(c, Color::BLACK);
    }
    #[test]
    fn color_when_ray_hits() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let c = w.color_at(r);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn color_with_intersection_behind_ray() {
        let w = World::default();
        let outer = w.objects[0].clone();
        outer.borrow_mut().get_mut_material().ambient = 1.0;
        let inner = w.objects[1].clone();
        inner.borrow_mut().get_mut_material().ambient = 1.0;
        let r = Ray::new(Point::new(0.0, 0.0, 0.75), Vector::new(0.0, 0.0, -1.0));
        let c = w.color_at(r);
        assert_eq!(c, inner.borrow().material().color);
    }
}
