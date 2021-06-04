use std::{f64::EPSILON, rc::Rc};

use float_cmp::approx_eq;

use crate::{
    materials::Material,
    matrix::Matrix,
    pattern::Pattern,
    tuple::{Tuple, Vector},
};

use super::Shape;

#[derive(Debug)]
pub struct Plane {
    pub id: usize,
    pub transform: Matrix<4>,
    pub material: Material,
}
impl Plane {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            transform: Default::default(),
            material: Default::default(),
        }
    }
}
impl Shape for Plane {
    fn id(&self) -> usize {
        self.id
    }
    fn get_transform(&self) -> Matrix<4> {
        self.transform
    }

    fn set_transform(&mut self, transform: Matrix<4>) {
        self.transform = transform;
    }

    fn get_material(&self) -> &Material {
        &self.material
    }

    fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    fn local_normal_at(&self, p: &crate::tuple::Point) -> crate::tuple::Vector {
        Vector::new(0.0, 1.0, 0.0)
    }

    fn local_intersect(&self, r: &crate::ray::Ray) -> Vec<f64> {
        if r.direction.y.abs() < EPSILON {
            vec![]
        } else {
            let t = -r.origin.y / r.direction.y;
            vec![t]
        }
    }

    fn get_mut_material(&mut self) -> &mut Material {
        &mut self.material
    }
}
#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use crate::{
        ray::Ray,
        tuple::{Point, Tuple, Vector},
    };

    use super::*;

    #[test]
    fn normal_of_plane_is_constant() {
        let p = Plane::new(0);
        let n1 = p.normal_at(Point::new(0.0, 0.0, 0.0));
        let n2 = p.normal_at(Point::new(10.0, 0.0, -10.0));
        let n3 = p.normal_at(Point::new(-5.0, 0.0, 150.0));
        assert_eq!(n1, Vector::new(0.0, 1.0, 0.0));
        assert_eq!(n2, Vector::new(0.0, 1.0, 0.0));
        assert_eq!(n3, Vector::new(0.0, 1.0, 0.0));
    }
    #[test]
    fn intersect_ray_parallel() {
        let p = Plane::new(0);
        let r = Ray::new(Point::new(0.0, 10.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let xs = p.local_intersect(&r);
        assert_eq!(xs.len(), 0);
    }
    #[test]
    fn intersect_coplanar_ray() {
        let p = Plane::new(0);
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let xs = p.local_intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_intersect_from_above() {
        let p = Rc::new(RefCell::new(Plane::new(0)));
        let r = Ray::new(Point::new(0.0, 1.0, 0.0), Vector::new(0.0, -1.0, 0.0));
        let xs = r.intersect(p.clone());
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[0].object.borrow().id(), p.borrow().id());
    }

    #[test]
    fn ray_intersect_from_below() {
        let p = Rc::new(RefCell::new(Plane::new(0)));
        let r = Ray::new(Point::new(0.0, -1.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        let xs = r.intersect(p.clone());
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[0].object.borrow().id(), p.borrow().id());
    }
}
