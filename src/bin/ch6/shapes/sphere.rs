use crate::{
    materials::Material,
    matrix::Matrix,
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
    fn transform(&self) -> Matrix<4> {
        self.transform
    }

    fn set_transform(&mut self, transform: Matrix<4>) {
        self.transform = transform;
    }

    fn normal_at(&self, world_point: crate::tuple::Point) -> crate::tuple::Vector {
        let object_point = self.transform.inverse() * world_point;
        let object_normal = object_point - Point::new(0.0, 0.0, 0.0);
        let world_normal = self.transform.inverse().transpose() * object_normal;
        world_normal.normalize()
    }

    fn material(&self) -> Material {
        self.material
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
    use std::{cell::RefCell, f64::consts::FRAC_1_SQRT_2, f64::consts::PI, rc::Rc};

    use crate::{
        materials::Material,
        ray::set_transform,
        transformations::Transformation,
        tuple::{Point, Tuple, Vector},
    };

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
    fn compute_normal_of_translated_sphere() {
        let s = Rc::new(RefCell::new(Sphere::new(0)));
        set_transform(s.clone(), Matrix::<4>::IDENTITY.translation(0.0, 1.0, 0.0));
        let n = s.borrow().normal_at(Point::new(0.0, 1.70711, -0.70711));
        assert_eq!(n, Vector::new(0.0, 0.70711, -0.70711));
    }
    #[test]
    fn compute_normal_of_transformed_sphere() {
        let s = Rc::new(RefCell::new(Sphere::new(0)));
        set_transform(
            s.clone(),
            Matrix::<4>::IDENTITY
                .rotation_z(PI / 5.0)
                .scaling(1.0, 0.5, 1.0),
        );
        let n = s
            .borrow()
            .normal_at(Point::new(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
        assert_eq!(n, Vector::new(0.0, 0.97014, -0.24254));
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

    #[test]
    fn sphere_has_default_material() {
        let s = Sphere::new(0);
        let m = s.material;
        assert_eq!(m, Material::default());
    }

    #[test]
    fn sphere_may_be_assigned_material() {
        let mut s = Sphere::new(0);
        let mut m = Material::default();
        m.ambient = 1.0;
        s.material = m;
        assert_eq!(s.material, m);
    }
}
