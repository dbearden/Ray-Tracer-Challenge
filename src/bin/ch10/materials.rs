use super::pattern::Pattern;
use std::{cell::RefCell, rc::Rc};

use crate::{
    lights::PointLight,
    pattern::Stripe,
    shape::{sphere::reflect, Shape},
    tuple::{Color, Point, Tuple, Vector},
};

#[derive(Debug)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub pattern: Option<Box<dyn Pattern>>,
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.color.eq(&other.color)
            && self.ambient.eq(&other.ambient)
            && self.diffuse.eq(&other.diffuse)
            && self.specular.eq(&other.specular)
            && self.shininess.eq(&other.shininess)
    }
}
impl Default for Material {
    fn default() -> Self {
        Self {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            pattern: None,
        }
    }
}

pub fn lighting(
    material: &Material,
    object: &dyn Shape,
    light: &PointLight,
    point: &Point,
    eyev: &Vector,
    normalv: &Vector,
    in_shadow: bool,
) -> Color {
    let color = match &material.pattern {
        Some(p) => p.pattern_at_shape(object, point),
        None => material.color,
    };
    let effective_color = color * light.intensity;
    let lightv = (light.position - *point).normalize();
    let ambient = effective_color * material.ambient;
    let light_dot_normal = lightv.dot(*normalv);
    let (diffuse, specular) = if light_dot_normal < 0.0 || in_shadow {
        (Color::BLACK, Color::BLACK)
    } else {
        let diffuse = effective_color * material.diffuse * light_dot_normal;

        let reflectv = reflect(-lightv, *normalv);
        let reflect_dot_eye = reflectv.dot(*eyev);

        if reflect_dot_eye <= 0.0 {
            (diffuse, Color::BLACK)
        } else {
            let factor = reflect_dot_eye.powf(material.shininess);
            let specular = light.intensity * material.specular * factor;

            (diffuse, specular)
        }
    };

    ambient + diffuse + specular
}

#[cfg(test)]
mod tests {
    use std::f64::consts::FRAC_1_SQRT_2;

    use crate::{
        lights::PointLight,
        shape::Sphere,
        tuple::{Point, Tuple, Vector},
    };

    use super::*;

    #[test]
    fn default_material() {
        let m = Material::default();
        assert_eq!(m.color, Color::new(1.0, 1.0, 1.0));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface() {
        let object = Rc::new(RefCell::new(Sphere::new(0)));
        let m = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let in_shadow = false;
        let result = lighting(
            &m,
            &*object.borrow(),
            &light,
            &position,
            &eyev,
            &normalv,
            in_shadow,
        );
        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_eye_offset_by_45() {
        let object = Rc::new(RefCell::new(Sphere::new(0)));
        let m = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, FRAC_1_SQRT_2, FRAC_1_SQRT_2);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let in_shadow = false;
        let result = lighting(
            &m,
            &*object.borrow(),
            &light,
            &position,
            &eyev,
            &normalv,
            in_shadow,
        );
        assert_eq!(result, Color::new(1.0, 1.0, 1.0));
    }
    #[test]
    fn lighting_with_light_offset_by_45() {
        let object = Rc::new(RefCell::new(Sphere::new(0)));
        let m = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let in_shadow = false;
        let result = lighting(
            &m,
            &*object.borrow(),
            &light,
            &position,
            &eyev,
            &normalv,
            in_shadow,
        );
        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364,));
    }
    #[test]
    fn lighting_with_eye_in_reflection_path() {
        let object = Rc::new(RefCell::new(Sphere::new(0)));
        let m = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, -FRAC_1_SQRT_2, -FRAC_1_SQRT_2);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let in_shadow = false;
        let result = lighting(
            &m,
            &*object.borrow(),
            &light,
            &position,
            &eyev,
            &normalv,
            in_shadow,
        );
        assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364,));
    }
    #[test]
    fn lighting_with_light_behind_surface() {
        let object = Rc::new(RefCell::new(Sphere::new(0)));
        let m = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));
        let in_shadow = false;
        let result = lighting(
            &m,
            &*object.borrow(),
            &light,
            &position,
            &eyev,
            &normalv,
            in_shadow,
        );
        assert_eq!(result, Color::new(0.1, 0.1, 0.1,));
    }

    #[test]
    fn lighting_with_surface_in_shadow() {
        let object = Rc::new(RefCell::new(Sphere::new(0)));
        let m = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::WHITE);
        let in_shadow = true;
        let result = lighting(
            &m,
            &*object.borrow(),
            &light,
            &position,
            &eyev,
            &normalv,
            in_shadow,
        );

        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }
}
