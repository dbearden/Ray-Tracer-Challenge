use std::{cell::RefCell, rc::Rc};

use crate::{
    materials::Material,
    matrix::Matrix,
    ray::{Intersection, Ray},
    tuple::{Point, Tuple, Vector},
};

use super::Shape;
#[derive(Debug, PartialEq)]
pub struct Sphere {
    pub id: usize,
    pub transform: Matrix<4>,
    pub material: Material,
}

impl Shape for Sphere {
    fn id(&self) -> usize {
        self.id
    }
    fn get_transform(&self) -> Matrix<4> {
        self.transform
    }

    fn set_transform(&mut self, transform: Matrix<4>) {
        self.transform = transform;
    }

    fn get_material(&self) -> Material {
        self.material
    }

    fn get_mut_material(self: &mut Self) -> &mut Material {
        &mut self.material
    }

    fn local_intersect(&self, r: &Ray) -> Vec<f64> {
        let ray2 = r;
        let sphere_to_ray = ray2.origin - Point::new(0.0, 0.0, 0.0);

        let a = ray2.direction.dot(ray2.direction);
        let b = 2.0 * ray2.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = b.powi(2) - 4.0 * a * c;
        if discriminant < 0.0 {
            Vec::new()
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            vec![t1.min(t2), t2.max(t1)]
        }
    }
    fn local_normal_at(&self, p: &Point) -> Vector {
        let object_point = p;
        let object_normal = *object_point - Point::new(0.0, 0.0, 0.0);

        object_normal
    }
}
impl Sphere {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            transform: Matrix::<4>::IDENTITY,
            material: Material::default(),
        }
    }
}

pub fn reflect(i: Vector, normal: Vector) -> Vector {
    i - normal * 2.0 * i.dot(normal)
}

#[cfg(test)]
mod tests {
    use std::f64::consts::FRAC_1_SQRT_2;

    use crate::tuple::{Point, Tuple, Vector};

    use super::*;

    const ROOT_3_OVER_3: f64 = 0.5773502692;
    #[test]
    fn sphere_normal_on_x() {
        let s = Sphere::new(0);
        let n = s.normal_at(Point::new(1.0, 0.0, 0.0));
        assert_eq!(n, Vector::new(1.0, 0.0, 0.0));
    }
    #[test]
    fn sphere_normal_on_y() {
        let s = Sphere::new(0);
        let n = s.normal_at(Point::new(0.0, 1.0, 0.0));
        assert_eq!(n, Vector::new(0.0, 1.0, 0.0));
    }
    #[test]
    fn sphere_normal_on_z() {
        let s = Sphere::new(0);
        let n = s.normal_at(Point::new(0.0, 0.0, 1.0));
        assert_eq!(n, Vector::new(0.0, 0.0, 1.0));
    }
    #[test]
    fn sphere_normal_at_nonaxial() {
        let s = Sphere::new(0);
        let n = s.normal_at(Point::new(ROOT_3_OVER_3, ROOT_3_OVER_3, ROOT_3_OVER_3));
        assert_eq!(n, Vector::new(ROOT_3_OVER_3, ROOT_3_OVER_3, ROOT_3_OVER_3,));
    }

    #[test]
    fn normal_is_normalized() {
        let s = Sphere::new(0);
        let n = s.normal_at(Point::new(ROOT_3_OVER_3, ROOT_3_OVER_3, ROOT_3_OVER_3));
        assert_eq!(n, n.normalize());
    }

    #[test]
    fn reflect_vector_at_45() {
        let v = Vector::new(1.0, -1.0, 0.0);
        let n = Vector::new(0.0, 1.0, 0.0);
        let r = reflect(v, n);
        assert_eq!(r, Vector::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn reflect_vector_off_slant() {
        let v = Vector::new(0.0, -1.0, 0.0);
        let n = Vector::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2, 0.0);
        let r = reflect(v, n);
        assert_eq!(r, Vector::new(1.0, 0.0, 0.0));
    }
}
