use std::f64::{EPSILON, INFINITY, NEG_INFINITY};

use float_cmp::approx_eq;

use crate::{
    materials::Material,
    matrix::Matrix,
    ray::Ray,
    tuple::{Tuple, Vector},
};

use super::Shape;

#[derive(Debug)]
pub struct Cone {
    id: usize,
    pub transform: Matrix<4>,
    pub material: Material,
    pub minimum: f64,
    pub maximum: f64,
    pub closed: bool,
}

fn check_cap(ray: &Ray, t: f64, y: f64) -> bool {
    let x = ray.origin.x + t * ray.direction.x;
    let z = ray.origin.z + t * ray.direction.z;

    (x.powi(2) + z.powi(2)) <= y.abs()
}
impl Cone {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
    fn intersect_caps(&self, ray: &Ray, xs: &mut Vec<f64>) {
        if !self.closed || approx_eq!(f64, ray.direction.y, 0.0, epsilon = 0.00003) {
            return;
        }

        let t = (self.minimum - ray.origin.y) / ray.direction.y;
        if check_cap(ray, t, self.minimum) {
            xs.push(t);
        }

        let t = (self.maximum - ray.origin.y) / ray.direction.y;
        if check_cap(ray, t, self.maximum) {
            xs.push(t);
        }
    }
}
impl Default for Cone {
    fn default() -> Self {
        Self {
            id: 0,
            minimum: NEG_INFINITY,
            maximum: INFINITY,
            transform: Default::default(),
            material: Default::default(),
            closed: false,
        }
    }
}

impl Shape for Cone {
    fn id(&self) -> usize {
        self.id
    }

    fn get_transform(&self) -> Matrix<4> {
        self.transform
    }

    fn set_transform(&mut self, transform: Matrix<4>) {
        todo!()
    }

    fn get_material(&self) -> &Material {
        &self.material
    }

    fn set_material(&mut self, material: Material) {
        todo!()
    }

    fn get_mut_material(&mut self) -> &mut Material {
        todo!()
    }

    fn local_normal_at(&self, p: &crate::tuple::Point) -> crate::tuple::Vector {
        let dist = p.x.powi(2) + p.z.powi(2);
        if dist < 1.0 && p.y >= self.maximum - 0.00003 {
            Vector::new(0.0, 1.0, 0.0)
        } else if dist < 1.0 && p.y <= self.minimum + 0.00003 {
            Vector::new(0.0, -1.0, 0.0)
        } else {
            let mut y = (p.x.powi(2) + p.z.powi(2)).sqrt();
            if p.y > 0.0 {
                y = -y
            }
            Vector::new(p.x, y, p.z)
        }
    }

    fn local_intersect(&self, r: &crate::ray::Ray) -> Vec<f64> {
        let mut xs = vec![];
        let a = r.direction.x.powi(2) - r.direction.y.powi(2) + r.direction.z.powi(2);
        let b = 2.0 * r.origin.x * r.direction.x - 2.0 * r.origin.y * r.direction.y
            + 2.0 * r.origin.z * r.direction.z;
        let c = r.origin.x.powi(2) - r.origin.y.powi(2) + r.origin.z.powi(2);
        if a.abs() <= EPSILON && b.abs() <= EPSILON {
            return xs;
        } else if a.abs() <= EPSILON {
            xs.push(-c / (2.0 * b));
            self.intersect_caps(r, &mut xs);
            return xs;
        }

        let disc = b.powi(2) - 4.0 * a * c;

        if disc < 0.0 {
            return xs;
        }

        let t0 = (-b - disc.sqrt()) / (2.0 * a);
        let t1 = (-b + disc.sqrt()) / (2.0 * a);

        let (t0, t1) = if t0 > t1 { (t1, t0) } else { (t0, t1) };

        let y0 = r.origin.y + t0 * r.direction.y;
        if self.minimum < y0 && y0 < self.maximum {
            xs.push(t0);
        }
        let y1 = r.origin.y + t1 * r.direction.y;
        if self.minimum < y1 && y1 < self.maximum {
            xs.push(t1);
        }
        self.intersect_caps(r, &mut xs);
        xs
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ray::Ray,
        tuple::{Point, Tuple, Vector},
    };

    use super::*;

    #[test]
    fn intersecting_cone_with_ray() {
        let shape = Cone::new(0);
        let examples = vec![
            (
                Point::new(0.0, 0.0, -5.0),
                Vector::new(0.0, 0.0, 1.0),
                5.0,
                5.0,
            ),
            (
                Point::new(0.0, 0.0, -5.0),
                Vector::new(1.0, 1.0, 1.0),
                8.66025,
                8.66025,
            ),
            (
                Point::new(1.0, 1.0, -5.0),
                Vector::new(-0.5, -1.0, 1.0),
                4.55006,
                49.44994,
            ),
        ];
        for (origin, direction, t0, t1) in examples {
            let direction = direction.normalize();
            let r = Ray::new(origin, direction);
            let xs = shape.local_intersect(&r);
            assert_eq!(xs.len(), 2);
            assert!(approx_eq!(f64, xs[0], t0, epsilon = 0.00003));
            assert!(approx_eq!(f64, xs[1], t1, epsilon = 0.00003));
        }
    }

    #[test]
    fn intersecting_cone_with_ray_parallel_to_one_half() {
        let shape = Cone::new(0);
        let direction = Vector::new(0.0, 1.0, 1.0).normalize();
        let r = Ray::new(Point::new(0.0, 0.0, -1.0), direction);
        let xs = shape.local_intersect(&r);
        assert_eq!(xs.len(), 1);
        assert!(approx_eq!(f64, xs[0], 0.35355, epsilon = 0.00003));
    }

    #[test]
    fn intersecting_cone_end_caps() {
        let mut shape = Cone::new(0);
        shape.minimum = -0.5;
        shape.maximum = 0.5;
        shape.closed = true;
        let examples = vec![
            (Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 0.0), 0),
            (Point::new(0.0, 0.0, -0.25), Vector::new(0.0, 1.0, 1.0), 2),
            (Point::new(0.0, 0.0, -0.25), Vector::new(0.0, 1.0, 0.0), 4),
        ];
        for (origin, direction, count) in examples {
            let direction = direction.normalize();
            let r = Ray::new(origin, direction);
            let xs = shape.local_intersect(&r);
            assert_eq!(xs.len(), count);
        }
    }

    #[test]
    fn computing_normal_vector_on_cone() {
        let shape = Cone::new(0);
        let examples = vec![
            (Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 0.0)),
            (Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 0.0)),
            (Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 0.0)),
        ];
        for (point, normal) in examples {
            let n = shape.local_normal_at(&point);
            assert_eq!(n, normal);
        }
    }
}
