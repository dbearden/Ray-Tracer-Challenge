use std::{cell::RefCell, rc::Rc};

use float_cmp::approx_eq;

use crate::shapes::Shape;
use crate::world::World;
use crate::{
    matrix::Matrix,
    tuple::{Point, Tuple, Vector},
};

#[derive(Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Ray {
        Ray { origin, direction }
    }
    pub fn position(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }
    pub fn transform(&self, t: Matrix<4>) -> Self {
        Self {
            origin: t * self.origin,
            direction: t * self.direction,
        }
    }
    pub fn intersect(&self, shape: Rc<RefCell<dyn Shape>>) -> Vec<Intersection> {
        let local_ray = self.transform(shape.borrow().get_transform().inverse());
        let mut res = Vec::new();
        for t in shape.borrow().local_intersect(&local_ray) {
            res.push(Intersection::new(t, shape.clone()));
        }

        intersections(res)
    }
}

#[derive(Debug, Clone)]
pub struct Intersection {
    pub t: f64,
    pub object: Rc<RefCell<dyn Shape>>,
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        approx_eq!(f64, self.t, other.t)
    }
}

impl Intersection {
    pub fn new(t: f64, object: Rc<RefCell<dyn Shape>>) -> Self {
        Self { t, object }
    }
}

pub fn intersections(mut vec: Vec<Intersection>) -> Vec<Intersection> {
    vec.sort_by(|i1, i2| i1.t.partial_cmp(&i2.t).unwrap_or(std::cmp::Ordering::Equal));
    vec
}

pub fn hit(xs: Vec<Intersection>) -> Option<Intersection> {
    xs.into_iter()
        .filter(|i| approx_eq!(f64, i.t, 0.0) || i.t > 0.0)
        .min_by(|i1, i2| i1.t.partial_cmp(&i2.t).unwrap_or(std::cmp::Ordering::Equal))
}
#[cfg(test)]
mod tests {
    use crate::{matrix::Matrix, shapes::Sphere, transformations::Transformation};

    use super::*;

    #[test]
    fn create_and_query_ray() {
        let origin = Point::new(1.0, 2.0, 3.0);
        let direction = Vector::new(4.0, 5.0, 6.0);
        let r = Ray::new(origin, direction);
        assert_eq!(r.origin, origin);
        assert_eq!(r.direction, direction);
    }

    #[test]
    fn point_from_distance() {
        let r = Ray::new(Point::new(2.0, 3.0, 4.0), Vector::new(1.0, 0.0, 0.0));
        assert_eq!(r.position(0.0), Point::new(2.0, 3.0, 4.0));
        assert_eq!(r.position(1.0), Point::new(3.0, 3.0, 4.0));
        assert_eq!(r.position(-1.0), Point::new(1.0, 3.0, 4.0));
        assert_eq!(r.position(2.5), Point::new(4.5, 3.0, 4.0));
    }

    #[test]
    fn ray_intersect_sphere_at_two_points() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Rc::new(RefCell::new(Sphere::new(0)));
        let xs = r.intersect(s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }
    #[test]
    fn ray_intersect_sphere_at_tangent() {
        let r = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Rc::new(RefCell::new(Sphere::new(0)));
        let xs = r.intersect(s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }
    #[test]
    fn ray_misses_sphere() {
        let r = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Rc::new(RefCell::new(Sphere::new(0)));
        let xs = r.intersect(s);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_originates_in_sphere() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let s = Rc::new(RefCell::new(Sphere::new(0)));
        let xs = r.intersect(s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }
    #[test]
    fn sphere_behind_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Rc::new(RefCell::new(Sphere::new(0)));
        let xs = r.intersect(s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let s = Sphere::new(0);
        let s = Rc::new(RefCell::new(s));
        let i = Intersection::new(3.5, s.clone());
        assert_eq!(i.t, 3.5);
        assert_eq!(i.object.borrow().id(), s.borrow().id());
    }

    #[test]
    fn aggregating_intersections() {
        let s = Sphere::new(0);
        let s = Rc::new(RefCell::new(s));
        let i1 = Intersection::new(1.0, s.clone());
        let i2 = Intersection::new(2.0, s);
        let xs: Vec<Intersection> = intersections(vec![i1, i2]);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[1].t, 2.0);
    }

    #[test]
    fn intersect_sets_object_on_intersection() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new(0);
        let s = Rc::new(RefCell::new(s));
        let xs = r.intersect(s.clone());
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object.borrow().id(), s.clone().borrow().id());
        assert_eq!(xs[1].object.borrow().id(), s.borrow().id());
    }

    #[test]
    fn hit_when_all_positive_t() {
        let s = Sphere::new(0);
        let s = Rc::new(RefCell::new(s));
        let i1 = Intersection::new(1.0, s.clone());
        let i2 = Intersection::new(2.0, s);
        let xs = intersections(vec![i2, i1.clone()]);
        let i = hit(xs);
        assert_eq!(i, Some(i1));
    }
    #[test]
    fn hit_when_some_negative_t() {
        let s = Sphere::new(0);
        let s = Rc::new(RefCell::new(s));
        let i1 = Intersection::new(-1.0, s.clone());
        let i2 = Intersection::new(1.0, s);
        let xs = intersections(vec![i2.clone(), i1]);
        let i = hit(xs);
        assert_eq!(i, Some(i2));
    }
    #[test]
    fn hit_when_all_negative_t() {
        let s = Sphere::new(0);
        let s = Rc::new(RefCell::new(s));
        let i1 = Intersection::new(-2.0, s.clone());
        let i2 = Intersection::new(-1.0, s);
        let xs = intersections(vec![i2, i1]);
        let i = hit(xs);
        assert_eq!(i, None);
    }
    #[test]
    fn hit_is_always_lowest_nonnegative_intersection() {
        let s = Sphere::new(0);
        let s = Rc::new(RefCell::new(s));
        let i1 = Intersection::new(5.0, s.clone());
        let i2 = Intersection::new(7.0, s.clone());
        let i3 = Intersection::new(-3.0, s.clone());
        let i4 = Intersection::new(2.0, s);
        let xs = intersections(vec![i1, i2, i3, i4.clone()]);
        let i = hit(xs);
        assert_eq!(i, Some(i4));
    }

    #[test]
    fn translate_ray() {
        let r = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
        let m = Matrix::<4>::IDENTITY.translation(3.0, 4.0, 5.0);
        let r2 = r.transform(m);
        assert_eq!(r2.origin, Point::new(4.0, 6.0, 8.0));
        assert_eq!(r2.direction, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn scale_ray() {
        let r = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
        let m = Matrix::<4>::IDENTITY.scaling(2.0, 3.0, 4.0);
        let r2 = r.transform(m);
        assert_eq!(r2.origin, Point::new(2.0, 6.0, 12.0));
        assert_eq!(r2.direction, Vector::new(0.0, 3.0, 0.0));
    }

    #[test]
    fn default_sphere_transformation() {
        let s = Rc::new(RefCell::new(Sphere::new(0)));
        assert_eq!(s.borrow().get_transform(), Matrix::<4>::IDENTITY);
    }

    #[test]
    fn change_sphere_transformation() {
        let mut s = Sphere::new(0);
        let t = Matrix::<4>::IDENTITY.translation(2.0, 3.0, 4.0);
        s.set_transform(t);
        assert_eq!(s.get_transform(), t);
    }

    #[test]
    fn intersect_scaled_sphere_with_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut s = Sphere::new(0);
        s.set_transform(Matrix::<4>::IDENTITY.scaling(2.0, 2.0, 2.0));
        let s = Rc::new(RefCell::new(s));
        let xs = r.intersect(s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }
    #[test]
    fn intersect_translated_sphere_with_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut s = Sphere::new(0);
        s.set_transform(Matrix::<4>::IDENTITY.translation(5.0, 0.0, 0.0));
        let s = Rc::new(RefCell::new(s));
        let xs = r.intersect(s);
        assert_eq!(xs.len(), 0);
    }
}
