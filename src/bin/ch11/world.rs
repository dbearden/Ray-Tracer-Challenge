use std::{cell::RefCell, rc::Rc, u32};

use crate::{
    lights::PointLight,
    materials::{lighting, Material},
    matrix::Matrix,
    ray::{hit, schlick, Intersection, Ray},
    shape::{sphere::reflect, Shape, Sphere},
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

    pub fn color_at(&self, ray: &Ray, remaining: u32) -> Color {
        let xs = self.intersect(ray);
        if let Some(i) = crate::ray::hit(&xs) {
            let comps = prepare_computations(&i, ray, &xs);
            self.shade_hit(&comps, remaining)
        } else {
            Color::BLACK
        }
    }
    pub fn intersect(&self, r: &Ray) -> Vec<Intersection> {
        intersections(
            self.objects
                .iter()
                .map(|s| r.intersect(s.clone()))
                .flatten()
                .collect(),
        )
    }
    pub fn is_shadowed(&self, light: &PointLight, point: &Point) -> bool {
        let mut res = true;
        let v = light.position - *point;
        let distance = v.magnitude();
        let direction = v.normalize();
        let r = Ray::new(*point, direction);
        let intersections = self
            .intersect(&r)
            .into_iter()
            .filter(|i| !(i.object.borrow().get_material().transparency > 0_f64))
            .collect::<Vec<_>>();
        res = res
            && match hit(&intersections) {
                Some(h) if h.t < distance => true,
                _ => false,
            };

        res
    }
    pub fn shade_hit(&self, comps: &Computations, remaining: u32) -> Color {
        let mut res = Color::BLACK;
        for light in &self.lights {
            let shadowed = self.is_shadowed(light, &comps.over_point);
            let surface = lighting(
                &comps.object.clone().borrow().get_material(),
                &*comps.object.clone().borrow(),
                &light,
                &comps.over_point,
                &comps.eyev,
                &comps.normalv,
                shadowed,
            );
            let reflected = self.reflected_color(comps, remaining);
            let refracted = self.refracted_color(comps, remaining);

            let reflective = comps.object.borrow().get_material().reflective;
            let transparency = comps.object.borrow().get_material().transparency;

            if reflective >= EPSILON && transparency >= EPSILON {
                let reflectance = schlick(comps);
                res = res + surface + reflected * reflectance + refracted * (1.0 - reflectance);
            } else {
                res = res + surface + reflected + refracted;
            }
        }

        res
    }

    pub fn reflected_color(&self, comps: &Computations, remaining: u32) -> Color {
        if comps.object.borrow().get_material().reflective == 0.0 || remaining <= 0 {
            Color::BLACK
        } else {
            let reflect_ray = Ray::new(comps.over_point, comps.reflectv);
            let color = self.color_at(&reflect_ray, remaining - 1);

            color * comps.object.clone().borrow().get_material().reflective
        }
    }

    pub fn refracted_color(&self, comps: &Computations, remaining: u32) -> Color {
        if comps.object.borrow().get_material().transparency == 0.0 || remaining <= 0 {
            return Color::BLACK;
        }

        let n_ratio = comps.n1 / comps.n2;
        let cos_i = comps.eyev.dot(comps.normalv);
        let sin2_t = n_ratio.powi(2) * (1.0 - cos_i.powi(2));
        if (sin2_t - 1.0).abs() <= EPSILON {
            return Color::BLACK;
        }

        let cos_t = (1.0 - sin2_t).sqrt();
        let direction = comps.normalv * (n_ratio * cos_i - cos_t) - comps.eyev * n_ratio;
        let refract_ray = Ray::new(comps.under_point, direction);

        let color = self.color_at(&refract_ray, remaining - 1)
            * comps.object.borrow().get_material().transparency;

        color
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
    pub t: f64,
    pub object: Rc<RefCell<dyn Shape>>,
    pub point: Point,
    pub eyev: Vector,
    pub normalv: Vector,
    pub reflectv: Vector,
    pub inside: bool,
    pub over_point: Point,
    pub under_point: Point,
    pub n1: f64,
    pub n2: f64,
}

impl Computations {
    pub fn new(
        t: f64,
        object: Rc<RefCell<dyn Shape>>,
        point: Point,
        eyev: Vector,
        normalv: Vector,
        reflectv: Vector,
        inside: bool,
        over_point: Point,
        under_point: Point,
        n1: f64,
        n2: f64,
    ) -> Self {
        Self {
            t,
            object,
            point,
            eyev,
            normalv,
            reflectv,
            inside,
            over_point,
            under_point,
            n1,
            n2,
        }
    }
}

pub fn prepare_computations(hit: &Intersection, r: &Ray, xs: &Vec<Intersection>) -> Computations {
    let t = hit.t;
    let object = hit.object.clone();

    let point = r.position(t);
    let eyev = -r.direction;
    let normalv = object.borrow().normal_at(point);
    let reflectv = reflect(r.direction, normalv);
    let (inside, normalv) = if normalv.dot(eyev) < 0.0 {
        (true, -normalv)
    } else {
        (false, normalv)
    };

    let over_point = point + normalv * EPSILON;
    let under_point = point - normalv * EPSILON;

    let mut containers = Vec::<Rc<RefCell<dyn Shape>>>::new();
    let mut n1 = 1.0;
    let mut n2 = 1.0;
    for i in xs {
        if &i == &hit {
            n1 = if containers.is_empty() {
                1.0
            } else {
                containers
                    .last()
                    .unwrap()
                    .borrow()
                    .get_material()
                    .refractive_index
            }
        }

        if let Ok(n) = containers.binary_search(&i.object) {
            containers.remove(n);
        } else {
            containers.push(i.object.clone());
        }

        if &i == &hit {
            n2 = if containers.is_empty() {
                1.0
            } else {
                containers
                    .last()
                    .unwrap()
                    .borrow()
                    .get_material()
                    .refractive_index
            };
            break;
        }
    }

    Computations::new(
        t,
        object,
        point,
        eyev,
        normalv,
        reflectv,
        inside,
        over_point,
        under_point,
        n1,
        n2,
    )
}

#[cfg(test)]
mod tests {
    use std::{
        assert_matches::assert_matches,
        cell::RefMut,
        f64::consts::{FRAC_1_SQRT_2, SQRT_2},
    };

    use crate::{
        lights::PointLight,
        materials::Material,
        matrix::Matrix,
        ray::{intersections, Intersection, Ray},
        shape::{Plane, Sphere},
        transformations::Transformation,
        tuple::{Color, Point, Tuple, Vector},
        DEFAULT_REFLECTION_COUNT,
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
        let xs = w.intersect(&r);
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
        let comps = prepare_computations(&i, &r, &vec![]);
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
        let comps = prepare_computations(&i, &r, &vec![]);
        assert_eq!(comps.inside, false);
    }
    #[test]
    fn hit_when_intersection_occurs_on_interior() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Sphere::new(0);
        let shape = Rc::new(RefCell::new(shape));
        let i = Intersection::new(1.0, shape);
        let comps = prepare_computations(&i, &r, &vec![]);
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
        let comps = prepare_computations(&i, &r, &vec![]);
        let c = w.shade_hit(&comps, DEFAULT_REFLECTION_COUNT);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_an_intersection_from_inside() {
        let mut w = World::default();
        w.lights[0] = PointLight::new(Point::new(0.0, 0.25, 0.0), Color::new(1.0, 1.0, 1.0));

        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let shape = w.objects[1].clone();
        let i = Intersection::new(0.5, shape);
        let comps = prepare_computations(&i, &r, &vec![]);
        let c = w.shade_hit(&comps, DEFAULT_REFLECTION_COUNT);
        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn color_when_ray_misses() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 0.0));
        let c = w.color_at(&r, DEFAULT_REFLECTION_COUNT);
        assert_eq!(c, Color::BLACK);
    }
    #[test]
    fn color_when_ray_hits() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let c = w.color_at(&r, DEFAULT_REFLECTION_COUNT);
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
        let c = w.color_at(&r, DEFAULT_REFLECTION_COUNT);
        assert_eq!(c, inner.borrow().get_material().color);
    }

    #[test]
    fn no_shadow_when_not_colinear_with_point_and_light() {
        let w = World::default();
        let p = Point::new(0.0, 10.0, 0.0);
        assert!(!w.is_shadowed(&w.lights[0], &p));
    }
    #[test]
    fn shadow_when_object_between_point_and_light() {
        let w = World::default();
        let p = Point::new(10.0, -10.0, 10.0);
        assert!(w.is_shadowed(&w.lights[0], &p));
    }
    #[test]
    fn no_shadow_when_object_behind_light() {
        let w = World::default();
        let p = Point::new(-20.0, 20.0, -20.0);
        assert!(!w.is_shadowed(&w.lights[0], &p));
    }
    #[test]
    fn no_shadow_when_object_behind_point() {
        let w = World::default();
        let p = Point::new(0.0, 10.0, 0.0);
        assert!(!w.is_shadowed(&w.lights[0], &p));
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
        let comps = prepare_computations(&i, &r, &vec![]);
        let c = w.shade_hit(&comps, DEFAULT_REFLECTION_COUNT);
        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn hit_should_offset_point() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut shape = Sphere::new(0);
        shape.transform = Matrix::<4>::IDENTITY.translation(0.0, 0.0, 1.0);
        let shape = Rc::new(RefCell::new(shape));
        let i = Intersection::new(5.0, shape);
        let comps = prepare_computations(&i, &r, &vec![]);
        assert!(comps.over_point.z < -EPSILON / 2.0);
        assert!(comps.point.z > comps.over_point.z);
    }
    #[test]
    fn precomputing_reflection_vector() {
        let shape = Plane::new(0);
        let r = Ray::new(
            Point::new(0.0, 1.0, -1.0),
            Vector::new(0.0, -FRAC_1_SQRT_2, FRAC_1_SQRT_2),
        );
        let i = Intersection::new(std::f64::consts::SQRT_2, Rc::new(RefCell::new(shape)));
        let comps = prepare_computations(&i, &r, &vec![]);
        assert_eq!(
            comps.reflectv,
            Vector::new(0.0, FRAC_1_SQRT_2, FRAC_1_SQRT_2)
        );
    }

    #[test]
    fn reflection_for_nonreflective_material() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let shape = w.objects[1].clone();
        shape.borrow_mut().get_mut_material().ambient = 1.0;
        let i = Intersection::new(1.0, shape);
        let comps = prepare_computations(&i, &r, &vec![]);
        let color = w.reflected_color(&comps, DEFAULT_REFLECTION_COUNT);
        assert_eq!(color, Color::BLACK);
    }

    #[test]
    fn reflected_color_for_reflective_material() {
        let mut w = World::default();
        let mut shape = Plane::new(0);
        shape.material.reflective = 0.5;
        shape.transform = Matrix::default().translation(0.0, -1.0, 0.0);
        let shape = Rc::new(RefCell::new(shape));
        w.objects.push(shape.clone());
        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -FRAC_1_SQRT_2, FRAC_1_SQRT_2),
        );
        let i = Intersection::new(std::f64::consts::SQRT_2, shape.clone());
        let comps = prepare_computations(&i, &r, &vec![]);
        let color = w.reflected_color(&comps, DEFAULT_REFLECTION_COUNT);
        assert_eq!(color, Color::new(0.19032, 0.2379, 0.14274));
    }

    #[test]
    fn shade_hit_with_reflective_material() {
        let mut w = World::default();
        let mut shape = Plane::new(0);
        shape.material.reflective = 0.5;
        shape.transform = Matrix::default().translation(0.0, -1.0, 0.0);
        let shape = Rc::new(RefCell::new(shape));
        w.objects.push(shape.clone());
        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -FRAC_1_SQRT_2, FRAC_1_SQRT_2),
        );
        let i = Intersection::new(SQRT_2, shape.clone());
        let comps = prepare_computations(&i, &r, &vec![]);
        let color = w.shade_hit(&comps, DEFAULT_REFLECTION_COUNT);
        assert_eq!(color, Color::new(0.87677, 0.92436, 0.82918));
    }

    #[test]
    fn color_at_with_mutually_reflective_surfaces() {
        let mut w = World::default();
        w.lights = vec![PointLight::new(Point::new(0.0, 0.0, 0.0), Color::WHITE)];
        let mut lower = Plane::new(0);
        lower.material.reflective = 1.0;
        lower.transform = Matrix::default().translation(0.0, -1.0, 0.0);
        let lower = Rc::new(RefCell::new(lower));
        let mut upper = Plane::new(1);
        upper.material.reflective = 1.0;
        upper.transform = Matrix::default().translation(0.0, 1.0, 0.0);
        let upper = Rc::new(RefCell::new(upper));
        w.objects = vec![lower.clone(), upper.clone()];

        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        assert_matches!(w.color_at(&r, DEFAULT_REFLECTION_COUNT), Color { .. });
    }

    #[test]
    fn reflected_color_at_max_recursive_depth() {
        let mut w = World::default();
        let mut shape = Plane::new(0);
        shape.material.reflective = 0.5;
        shape.transform = Matrix::default().translation(0.0, -1.0, 0.0);
        let shape = Rc::new(RefCell::new(shape));
        w.objects.push(shape.clone());
        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -FRAC_1_SQRT_2, FRAC_1_SQRT_2),
        );
        let i = Intersection::new(SQRT_2, shape.clone());
        let comps = prepare_computations(&i, &r, &vec![]);
        let color = w.reflected_color(&comps, 0);
        assert_eq!(color, Color::BLACK);
    }
    #[test]
    fn refracted_color_with_opaque_surface() {
        let w = World::default();
        let shape = w.objects[0].clone();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let i1 = Intersection::new(4.0, shape.clone());
        let i2 = Intersection::new(6.0, shape.clone());
        let xs = intersections(vec![i1, i2]);
        let comps = prepare_computations(&xs[0], &r, &xs);
        let c = w.refracted_color(&comps, 5);
        assert_eq!(c, Color::BLACK);
    }
    #[test]
    fn refracted_color_at_maximum_recursive_depth() {
        let w = World::default();
        let shape = w.objects[0].clone();
        {
            let mut sm = shape.borrow_mut();
            sm.get_mut_material().transparency = 1.0;
            sm.get_mut_material().refractive_index = 1.5;
        }
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let i1 = Intersection::new(4.0, shape.clone());
        let i2 = Intersection::new(6.0, shape.clone());
        let xs = intersections(vec![i1, i2]);
        let comps = prepare_computations(&xs[0], &r, &xs);
        let c = w.refracted_color(&comps, 0);
        assert_eq!(c, Color::BLACK);
    }

    #[test]
    fn refracted_color_under_total_internal_reflection() {
        let w = World::default();
        let shape = w.objects[0].clone();
        {
            let mut sm = shape.borrow_mut();
            sm.get_mut_material().transparency = 1.0;
            sm.get_mut_material().refractive_index = 1.5;
        }
        let r = Ray::new(
            Point::new(0.0, 0.0, FRAC_1_SQRT_2),
            Vector::new(0.0, 1.0, 0.0),
        );

        let i1 = Intersection::new(-FRAC_1_SQRT_2, shape.clone());
        let i2 = Intersection::new(FRAC_1_SQRT_2, shape.clone());
        let xs = intersections(vec![i1, i2]);
        let comps = prepare_computations(&xs[1], &r, &xs);
        let c = w.refracted_color(&comps, 5);
        assert_eq!(c, Color::BLACK);
    }

    #[test]
    fn refracted_color_with_refracted_ray() {
        let w = World::default();
        let a = w.objects[0].clone();
        {
            let mut am = a.borrow_mut();
            am.get_mut_material().ambient = 1.0;
            am.get_mut_material().pattern =
                Some(Box::new(crate::pattern::tests::TestPattern::new()));
        }

        let b = w.objects[1].clone();
        {
            let mut bm = b.borrow_mut();
            bm.get_mut_material().transparency = 1.0;
            bm.get_mut_material().refractive_index = 1.5;
        }

        let r = Ray::new(Point::new(0.0, 0.0, 0.1), Vector::new(0.0, 1.0, 0.0));
        let i1 = Intersection::new(-0.9899, a.clone());
        let i2 = Intersection::new(-0.4899, b.clone());
        let i3 = Intersection::new(0.4899, b.clone());
        let i4 = Intersection::new(0.9899, a.clone());
        let xs = intersections(vec![i1, i2, i3, i4]);
        let comps = prepare_computations(&xs[2], &r, &xs);
        let c = w.refracted_color(&comps, 5);
        assert_eq!(c, Color::new(0.0, 0.99888, 0.04725));
    }

    #[test]
    fn shade_hit_with_transparent_material() {
        let mut w = World::default();
        let mut floor = Plane::new(3);
        floor.transform = floor.transform.translation(0.0, -1.0, 0.0);
        floor.material.transparency = 0.5;
        floor.material.refractive_index = 1.5;
        let floor = Rc::new(RefCell::new(floor));
        w.objects.push(floor.clone());

        let mut ball = Sphere::new(4);
        ball.material.color = Color::new(1.0, 0.0, 0.0);
        ball.material.ambient = 0.5;
        ball.transform = ball.transform.translation(0.0, -3.5, -0.5);
        let ball = Rc::new(RefCell::new(ball));
        w.objects.push(ball);

        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -FRAC_1_SQRT_2, FRAC_1_SQRT_2),
        );
        let xs = intersections(vec![Intersection::new(SQRT_2, floor)]);
        let comps = prepare_computations(&xs[0], &r, &xs);
        let color = w.shade_hit(&comps, 5);
        assert_eq!(color, Color::new(1.31450, 0.68642, 0.68642));
    }

    #[test]
    fn shade_hit_with_reflective_transparent_material() {
        let mut w = World::default();
        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -FRAC_1_SQRT_2, FRAC_1_SQRT_2),
        );
        let mut floor = Plane::new(0);
        floor.transform = floor.transform.translation(0.0, -1.0, 0.0);
        floor.material.reflective = 0.5;
        floor.material.transparency = 0.5;
        floor.material.refractive_index = 1.5;
        let floor = Rc::new(RefCell::new(floor));
        w.objects.push(floor.clone());

        let mut ball = Sphere::new(1);
        ball.material.color = Color::new(1.0, 0.0, 0.0);
        ball.material.ambient = 0.5;
        ball.transform = ball.transform.translation(0.0, -3.5, -0.5);
        let ball = Rc::new(RefCell::new(ball));
        w.objects.push(ball);

        let xs = intersections(vec![Intersection::new(SQRT_2, floor)]);
        let comps = prepare_computations(&xs[0], &r, &xs);
        let color = w.shade_hit(&comps, 5);
        assert_eq!(color, Color::new(1.29609, 0.69643, 0.69243));
    }
}
