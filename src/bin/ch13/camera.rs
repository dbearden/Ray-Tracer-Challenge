use crate::{
    canvas::Canvas,
    matrix::Matrix,
    ray::Ray,
    tuple::{Point, Tuple},
    world::World,
    DEFAULT_REFLECTION_COUNT,
};

pub struct Camera {
    pub hsize: i32,
    pub vsize: i32,
    pub field_of_view: f64,
    pub transform: Matrix<4>,
    pub half_width: f64,
    pub half_height: f64,
    pub pixel_size: f64,
}

impl Camera {
    pub fn new(hsize: i32, vsize: i32, field_of_view: f64) -> Self {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = hsize as f64 / vsize as f64;
        let (half_width, half_height) = if aspect >= 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };

        let pixel_size = (half_width * 2.0) / hsize as f64;
        Self {
            hsize,
            vsize,
            field_of_view,
            transform: Matrix::<4>::IDENTITY,
            half_height,
            half_width,
            pixel_size,
        }
    }

    pub fn ray_for_pixel(&self, px: i32, py: i32) -> Ray {
        let xoffset = (px as f64 + 0.5) * self.pixel_size;
        let yoffset = (py as f64 + 0.5) * self.pixel_size;

        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let pixel = self.transform.inverse() * Point::new(world_x, world_y, -1.0);
        let origin = self.transform.inverse() * Point::new(0.0, 0.0, 0.0);
        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }
}

pub fn render(camera: Camera, world: World, reflection_count: u32) -> Canvas {
    let mut image = Canvas::new(camera.hsize as usize, camera.vsize as usize);
    for y in 0..camera.vsize - 1 {
        for x in 0..camera.hsize - 1 {
            let ray = camera.ray_for_pixel(x, y);
            let color = world.color_at(&ray, reflection_count);
            image.write(x as usize, y as usize, color);
        }
    }

    image
}
#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_1_SQRT_2, FRAC_PI_2, FRAC_PI_4};

    use float_cmp::approx_eq;

    use crate::{
        matrix::Matrix,
        transformations::{view_transform, Transformation},
        tuple::{Color, Point, Tuple, Vector},
        world::World,
    };

    use super::*;

    #[test]
    fn constructing_a_camera() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = FRAC_PI_2;

        let c = Camera::new(hsize, vsize, field_of_view);

        assert_eq!(c.hsize, 160);
        assert_eq!(c.vsize, 120);
        assert_eq!(c.field_of_view, FRAC_PI_2);
        assert_eq!(c.transform, Matrix::<4>::IDENTITY);
    }

    #[test]
    fn pixel_size_for_horizontal_canvas() {
        let c = Camera::new(200, 125, FRAC_PI_2);
        assert!(approx_eq!(f64, c.pixel_size, 0.01));
    }
    #[test]
    fn pixel_size_for_vertical_canvas() {
        let c = Camera::new(125, 200, FRAC_PI_2);
        assert!(approx_eq!(f64, c.pixel_size, 0.01));
    }

    #[test]
    fn constructing_ray_through_center_of_canvas() {
        let c = Camera::new(201, 101, FRAC_PI_2);
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(r.origin, Point::new(0.0, 0.0, 0.0));
        assert_eq!(r.direction, Vector::new(0.0, 0.0, -1.0));
    }
    #[test]
    fn constructing_ray_through_corner_of_canvas() {
        let c = Camera::new(201, 101, FRAC_PI_2);
        let r = c.ray_for_pixel(0, 0);
        assert_eq!(r.origin, Point::new(0.0, 0.0, 0.0));
        assert_eq!(r.direction, Vector::new(0.66519, 0.33259, -0.66851));
    }
    #[test]
    fn constructing_ray_when_camera_transformed() {
        let mut c = Camera::new(201, 101, FRAC_PI_2);
        c.transform = Matrix::<4>::IDENTITY
            .translation(0.0, -2.0, 5.0)
            .rotation_y(FRAC_PI_4);
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(r.origin, Point::new(0.0, 2.0, -5.0));
        assert_eq!(r.direction, Vector::new(FRAC_1_SQRT_2, 0.0, -FRAC_1_SQRT_2));
    }

    #[test]
    fn render_world_with_camera() {
        let w = World::default();
        let mut c = Camera::new(11, 11, FRAC_PI_2);
        let from = Point::new(0.0, 0.0, -5.0);
        let to = Point::new(0.0, 0.0, 0.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        c.transform = view_transform(from, to, up);

        let image = render(c, w, DEFAULT_REFLECTION_COUNT);
        assert_eq!(image.pixel_at(5, 5), Color::new(0.38066, 0.47583, 0.2855));
    }
}
