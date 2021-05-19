use std::{cell::RefCell, rc::Rc};

use crate::{
    lights::PointLight,
    materials::{lighting, Material},
    matrix::Matrix,
    ray::{hit, Intersection, Ray},
    shape::{Shape, Sphere},
    transformations::Transformation,
    tuple::{Color, Point, Tuple, Vector},
};

const EPSILON: f64 = 0.00003;
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

    pub fn color_at(&self, ray: &Ray) -> Color {
        let xs = self.intersect_world(ray);
        if let Some(i) = crate::ray::hit(xs) {
            let comps = prepare_computations(&i, ray);
            shade_hit(self, &comps)
        } else {
            Color::BLACK
        }
    }
    pub fn intersect_world(&self, r: &Ray) -> Vec<Intersection> {
        intersections(
            self.objects
                .iter()
                .map(|s| r.intersect(s.clone()))
                .flatten()
                .collect(),
        )
    }
}

impl Default for World {
    fn default() -> Self {
        let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::WHITE);
        let s1 = {
            let mut s = Sphere::new(0);
            let mut m = Material::default();
            m.color = Color::new(0.8, 1.0, 0.6);
            m.diffuse = 0.7;
            m.specular = 0.2;
            s.material = m;
            Rc::new(RefCell::new(s))
        };
        let s2 = {
            let mut s = Sphere::new(1);
            s.transform = Matrix::<4>::IDENTITY.scaling(0.5, 0.5, 0.5);
            Rc::new(RefCell::new(s))
        };

        Self {
            objects: vec![s1, s2],
            lights: vec![light],
        }
    }
}
pub fn intersections(mut vec: Vec<Intersection>) -> Vec<Intersection> {
    vec.sort_by(|i1, i2| i1.t.partial_cmp(&i2.t).unwrap_or(std::cmp::Ordering::Equal));
    vec
}

#[derive(Debug)]
pub struct Computations {
    t: f64,
    object: Rc<RefCell<dyn Shape>>,
    point: Point,
    eyev: Vector,
    normalv: Vector,
    inside: bool,
    over_point: Point,
}

impl Computations {
    pub fn new(
        t: f64,
        object: Rc<RefCell<dyn Shape>>,
        point: Point,
        eyev: Vector,
        normalv: Vector,
        inside: bool,
        over_point: Point,
    ) -> Self {
        Self {
            t,
            object,
            point,
            eyev,
            normalv,
            inside,
            over_point,
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

    let over_point = point + normalv * EPSILON;

    Computations::new(t, object, point, eyev, normalv, inside, over_point)
}

fn shade_hit(world: &World, comps: &Computations) -> Color {
    let mut res = Color::BLACK;
    for light in &world.lights {
        let shadowed = is_shadowed(world, light, &comps.over_point);
        res = res
            + lighting(
                &comps.object.clone().borrow().get_material(),
                &*comps.object.clone().borrow(),
                &light,
                &comps.over_point,
                &comps.eyev,
                &comps.normalv,
                shadowed,
            )
    }

    res
}

fn is_shadowed(world: &World, light: &PointLight, point: &Point) -> bool {
    let mut res = true;
    let v = light.position - *point;
    let distance = v.magnitude();
    let direction = v.normalize();
    let r = Ray::new(*point, direction);
    let intersections = world.intersect_world(&r);

    res = res
        && match hit(intersections) {
            Some(h) if h.t < distance => true,
            _ => false,
        };

    res
}
#[cfg(test)]
mod tests {
    use std::cell::RefMut;

    use crate::{
        lights::PointLight,
        materials::Material,
        matrix::Matrix,
        ray::{Intersection, Ray},
        shape::Sphere,
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
        let s1: Rc<RefCell<dyn Shape>> = {
            let mut s = Sphere::new(0);
            let mut m = Material::default();
            m.color = Color::new(0.8, 1.0, 0.6);
            m.diffuse = 0.7;
            m.specular = 0.2;
            s.material = m;
            Rc::new(RefCell::new(s))
        };
        let s2: Rc<RefCell<dyn Shape>> = {
            let mut s = Sphere::new(1);
            s.transform = Matrix::<4>::IDENTITY.scaling(0.5, 0.5, 0.5);
            Rc::new(RefCell::new(s))
        };

        let w = World::default();

        assert!(w.lights.contains(&light));
        assert!(w.objects.contains(&s1));
        assert!(w.objects.contains(&s2));
    }

    #[test]
    fn intersect_world_with_ray() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let xs = w.intersect_world(&r);
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
        let shape = Rc::new(RefCell::new(shape));
        let i = Intersection::new(4.0, shape);
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
        let shape = Rc::new(RefCell::new(shape));
        let i = Intersection::new(4.0, shape);
        let comps = prepare_computations(&i, &r);
        assert_eq!(comps.inside, false);
    }
    #[test]
    fn hit_when_intersection_occurs_on_interior() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Sphere::new(0);
        let shape = Rc::new(RefCell::new(shape));
        let i = Intersection::new(1.0, shape);
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
        let c = w.color_at(&r);
        assert_eq!(c, Color::BLACK);
    }
    #[test]
    fn color_when_ray_hits() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let c = w.color_at(&r);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn color_with_intersection_behind_ray() {
        let w = World::default();
        let outer = w.objects[0].clone();
        RefCell::borrow_mut(&outer).get_mut_material().ambient = 1.0;
        let inner = w.objects[1].clone();
        RefCell::borrow_mut(&inner).get_mut_material().ambient = 1.0;
        let r = Ray::new(Point::new(0.0, 0.0, 0.75), Vector::new(0.0, 0.0, -1.0));
        let c = w.color_at(&r);
        assert_eq!(c, inner.borrow().get_material().color);
    }

    #[test]
    fn no_shadow_when_not_colinear_with_point_and_light() {
        let w = World::default();
        let p = Point::new(0.0, 10.0, 0.0);
        assert!(!is_shadowed(&w, &w.lights[0], &p));
    }
    #[test]
    fn shadow_when_object_between_point_and_light() {
        let w = World::default();
        let p = Point::new(10.0, -10.0, 10.0);
        assert!(is_shadowed(&w, &w.lights[0], &p));
    }
    #[test]
    fn no_shadow_when_object_behind_light() {
        let w = World::default();
        let p = Point::new(-20.0, 20.0, -20.0);
        assert!(!is_shadowed(&w, &w.lights[0], &p));
    }
    #[test]
    fn no_shadow_when_object_behind_point() {
        let w = World::default();
        let p = Point::new(0.0, 10.0, 0.0);
        assert!(!is_shadowed(&w, &w.lights[0], &p));
    }

    #[test]
    fn shade_hit_given_intersection_in_shadow() {
        let mut w = World::default();
        w.lights = vec![PointLight::new(Point::new(0.0, 0.0, -10.0), Color::WHITE)];
        let s1 = Sphere::new(2);
        w.objects.push(Rc::new(RefCell::new(s1)));
        let mut s2 = Sphere::new(3);
        s2.transform = Matrix::<4>::IDENTITY.translation(0.0, 0.0, 10.0);
        w.objects.push(Rc::new(RefCell::new(s2)));
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let i = Intersection::new(4.0, w.objects[3].clone());
        let comps = prepare_computations(&i, &r);
        let c = shade_hit(&w, &comps);
        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn hit_should_offset_point() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut shape = Sphere::new(0);
        shape.transform = Matrix::<4>::IDENTITY.translation(0.0, 0.0, 1.0);
        let shape = Rc::new(RefCell::new(shape));
        let i = Intersection::new(5.0, shape);
        let comps = prepare_computations(&i, &r);
        assert!(comps.over_point.z < -EPSILON / 2.0);
        assert!(comps.point.z > comps.over_point.z);
    }
}
