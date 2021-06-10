pub mod cube;
pub mod plane;
pub mod sphere;
use std::{cell::RefCell, fmt::Debug, rc::Rc};

pub use cube::Cube;
pub use plane::Plane;
pub use sphere::Sphere;

use crate::{
    materials::Material,
    matrix::Matrix,
    pattern::Pattern,
    ray::{Intersection, Ray},
    tuple::{Point, Tuple, Vector},
};

pub trait Shape: Debug {
    fn id(&self) -> usize;
    fn get_transform(&self) -> Matrix<4>;
    fn set_transform(&mut self, transform: Matrix<4>);
    fn get_material(&self) -> &Material;
    fn set_material(&mut self, material: Material);
    fn get_mut_material(&mut self) -> &mut Material;

    fn local_normal_at(&self, p: &Point) -> Vector;
    fn local_intersect(&self, r: &Ray) -> Vec<f64>;

    fn normal_at(&self, p: Point) -> Vector {
        let local_point = self.get_transform().inverse() * p;
        let local_normal = self.local_normal_at(&local_point);
        let world_normal = self.get_transform().inverse().transpose() * local_normal;

        world_normal.normalize()
    }
}

impl PartialEq for dyn Shape {
    fn eq(&self, other: &Self) -> bool {
        self.id().eq(&other.id())
    }
}
impl PartialOrd for dyn Shape {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id().partial_cmp(&other.id())
    }
}
impl Ord for dyn Shape {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id().cmp(&other.id())
    }
}

impl Eq for dyn Shape {}

#[cfg(test)]
mod tests {
    use std::{
        any::type_name,
        cell::RefCell,
        f64::consts::{FRAC_1_SQRT_2, PI},
        rc::Rc,
    };

    use crate::{ray::Ray, transformations::Transformation, tuple::Tuple};

    use super::*;

    #[derive(Debug)]
    struct TestShape {
        pub id: usize,
        pub transform: Matrix<4>,
        pub material: Material,
    }

    impl TestShape {
        pub fn new(id: usize) -> Self {
            Self {
                id,
                transform: Default::default(),
                material: Default::default(),
            }
        }
    }
    impl Shape for TestShape {
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

        fn get_mut_material(&mut self) -> &mut Material {
            &mut self.material
        }

        fn local_normal_at(&self, p: &Point) -> Vector {
            let object_point = p;
            let object_normal = *object_point - Point::new(0.0, 0.0, 0.0);

            object_normal
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
                vec![t1, t2]
            }
        }

        fn normal_at(&self, p: Point) -> Vector {
            let local_point = self.get_transform().inverse() * p;
            let local_normal = self.local_normal_at(&local_point);
            let world_normal = self.get_transform().inverse().transpose() * local_normal;

            world_normal.normalize()
        }
    }

    #[test]
    fn default_transformation() {
        let s = TestShape::new(0);
        assert_eq!(s.get_transform(), Matrix::<4>::IDENTITY);
    }

    #[test]
    fn assign_transformation() {
        let mut s = TestShape::new(0);
        s.set_transform(Matrix::<4>::IDENTITY.translation(2.0, 3.0, 4.0));
        assert_eq!(
            s.transform,
            Matrix::<4>::IDENTITY.translation(2.0, 3.0, 4.0)
        );
    }

    #[test]
    fn default_material() {
        let s = TestShape::new(0);
        let m = s.get_material();
        assert_eq!(m, &Material::default());
    }

    #[test]
    fn assign_material() {
        let mut s = TestShape::new(0);
        s.material.ambient = 1.0;
        let mut m = Material::default();
        m.ambient = 1.0;
        assert_eq!(s.get_material(), &m);
    }

    #[test]
    fn intersect_scaled_shape_with_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut s = TestShape::new(0);
        s.set_transform(Matrix::<4>::IDENTITY.scaling(2.0, 2.0, 2.0));
        let sr = r.transform(s.transform.inverse());
        assert_eq!(sr.origin, Point::new(0.0, 0.0, -2.5));
        assert_eq!(sr.direction, Vector::new(0.0, 0.0, 0.5));
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
        let sr = r.transform(s.transform.inverse());
        assert_eq!(sr.origin, Point::new(-5.0, 0.0, -5.0));
        assert_eq!(sr.direction, Vector::new(0.0, 0.0, 1.0));
        let s = Rc::new(RefCell::new(s));
        let xs = r.intersect(s);
        assert_eq!(xs.len(), 0);
    }
    #[test]
    fn compute_normal_of_translated_shape() {
        let s = Rc::new(RefCell::new(TestShape::new(0)));
        s.borrow_mut()
            .set_transform(Matrix::<4>::IDENTITY.translation(0.0, 1.0, 0.0));
        let n = s.borrow().normal_at(Point::new(0.0, 1.70711, -0.70711));
        assert_eq!(n, Vector::new(0.0, 0.70711, -0.70711));
    }
    #[test]
    fn compute_normal_of_transformed_shape() {
        let s = Rc::new(RefCell::new(TestShape::new(0)));
        s.borrow_mut().set_transform(
            Matrix::<4>::IDENTITY
                .rotation_z(PI / 5.0)
                .scaling(1.0, 0.5, 1.0),
        );
        let n = s
            .borrow()
            .normal_at(Point::new(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
        assert_eq!(n, Vector::new(0.0, 0.97014, -0.24254));
    }
}
