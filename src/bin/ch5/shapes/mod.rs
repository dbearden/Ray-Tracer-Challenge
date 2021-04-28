pub mod sphere;
use std::fmt::Debug;

pub use sphere::Sphere;

use crate::matrix::Matrix;
pub trait Shape: Debug {
    fn id(&self) -> usize;
    fn transform(&self) -> Matrix<4>;
    fn set_transform(&mut self, transform: Matrix<4>);
}
