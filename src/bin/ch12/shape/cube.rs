use std::f64::INFINITY;

use crate::{
    materials::Material,
    matrix::Matrix,
    tuple::{Point, Tuple, Vector},
};

use super::Shape;

#[derive(Debug)]
pub struct Cube {
    pub id: usize,
    pub transform: Matrix<4>,
    pub material: Material,
}

impl Cube {
    pub fn new(id: usize) -> Self {
        Cube {
            id,
            transform: Default::default(),
            material: Default::default(),
        }
    }
}

fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
    let tmin_numerator = -1.0 - origin;
    let tmax_numerator = 1.0 - origin;

    let tmin;
    let tmax;
    if direction.abs() >= 0.00003 {
        tmin = tmin_numerator / direction;
        tmax = tmax_numerator / direction;
    } else {
        tmin = tmin_numerator * INFINITY;
        tmax = tmax_numerator * INFINITY;
    }

    if tmin > tmax {
        (tmax, tmin)
    } else {
        (tmin, tmax)
    }
}

impl Shape for Cube {
    fn id(&self) -> usize {
        self.id
    }

    fn get_transform(&self) -> crate::matrix::Matrix<4> {
        self.transform
    }

    fn set_transform(&mut self, transform: crate::matrix::Matrix<4>) {
        todo!()
    }

    fn get_material(&self) -> &crate::materials::Material {
        &self.material
    }

    fn set_material(&mut self, material: crate::materials::Material) {
        todo!()
    }

    fn get_mut_material(&mut self) -> &mut crate::materials::Material {
        todo!()
    }

    fn local_normal_at(&self, p: &crate::tuple::Point) -> crate::tuple::Vector {
        let maxc = p.x.abs().max(p.y.abs()).max(p.z.abs());

        if maxc == p.x.abs() {
            Vector::new(p.x, 0.0, 0.0)
        } else if maxc == p.y.abs() {
            Vector::new(0.0, p.y, 0.0)
        } else {
            Vector::new(0.0, 0.0, p.z)
        }
    }

    fn local_intersect(&self, r: &crate::ray::Ray) -> Vec<f64> {
        let (xtmin, xtmax) = check_axis(r.origin.x, r.direction.x);
        let (ytmin, ytmax) = check_axis(r.origin.y, r.direction.y);
        let (ztmin, ztmax) = check_axis(r.origin.z, r.direction.z);

        let tmin = xtmin.max(ytmin).max(ztmin);
        let tmax = xtmax.min(ytmax).min(ztmax);

        if tmin > tmax {
            vec![]
        } else {
            vec![tmin, tmax]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ray::Ray, tuple::*};

    use super::*;

    #[test]
    fn ray_intersect_cube() {
        let c = Cube::new(0);
        let examples = vec![
            (
                Point::new(5.0, 0.5, 0.0),
                Vector::new(-1.0, 0.0, 0.0),
                4.0,
                6.0,
            ),
            (
                Point::new(-5.0, 0.5, 0.0),
                Vector::new(1.0, 0.0, 0.0),
                4.0,
                6.0,
            ),
            (
                Point::new(0.5, 5.0, 0.0),
                Vector::new(0.0, -1.0, 0.0),
                4.0,
                6.0,
            ),
            (
                Point::new(0.5, -5.0, 0.0),
                Vector::new(0.0, 1.0, 0.0),
                4.0,
                6.0,
            ),
            (
                Point::new(0.5, 0.0, 5.0),
                Vector::new(0.0, 0.0, -1.0),
                4.0,
                6.0,
            ),
            (
                Point::new(0.5, 0.0, -5.0),
                Vector::new(0.0, 0.0, 1.0),
                4.0,
                6.0,
            ),
            (
                Point::new(0.0, 0.5, 0.0),
                Vector::new(0.0, 0.0, 1.0),
                -1.0,
                1.0,
            ),
        ];

        for (origin, direction, t1, t2) in examples {
            let r = Ray::new(origin, direction);
            let xs = c.local_intersect(&r);
            assert_eq!(xs.len(), 2);
            assert_eq!(xs[0], t1);
            assert_eq!(xs[1], t2);
        }
    }

    #[test]
    fn ray_misses_cube() {
        let c = Cube::new(0);
        let examples = vec![
            (
                Point::new(-2.0, 0.0, 0.0),
                Vector::new(0.2673, 0.5345, 0.8018),
            ),
            (
                Point::new(0.0, -2.0, 0.0),
                Vector::new(0.8018, 0.2673, 0.5345),
            ),
            (
                Point::new(0.0, 0.0, -2.0),
                Vector::new(0.5345, 0.8018, 0.2673),
            ),
            (Point::new(2.0, 0.0, 2.0), Vector::new(0.0, 0.0, -1.0)),
            (Point::new(0.0, 2.0, 2.0), Vector::new(0.0, -1.0, 0.0)),
            (Point::new(2.0, 2.0, 0.0), Vector::new(-1.0, 0.0, 0.0)),
        ];
        for (origin, direction) in examples {
            let r = Ray::new(origin, direction);
            let xs = c.local_intersect(&r);
            assert_eq!(xs.len(), 0);
        }
    }

    #[test]
    fn normal_of_cube() {
        let c = Cube::new(0);
        let examples = vec![
            (Point::new(1.0, 0.5, -0.8), Vector::new(1.0, 0.0, 0.0)),
            (Point::new(-1.0, -0.2, 0.9), Vector::new(-1.0, 0.0, 0.0)),
            (Point::new(-0.4, 1.0, -0.1), Vector::new(0.0, 1.0, 0.0)),
            (Point::new(0.3, -1.0, -0.7), Vector::new(0.0, -1.0, 0.0)),
            (Point::new(-0.6, 0.3, 1.0), Vector::new(0.0, 0.0, 1.0)),
            (Point::new(0.4, 0.4, -1.0), Vector::new(0.0, 0.0, -1.0)),
            (Point::new(1.0, 1.0, 1.0), Vector::new(1.0, 0.0, 0.0)),
            (Point::new(-1.0, -1.0, -1.0), Vector::new(-1.0, 0.0, 0.0)),
        ];
        for (point, normal) in examples {
            let p = point;
            let n = c.local_normal_at(&p);
            assert_eq!(n, normal);
        }
    }
}
