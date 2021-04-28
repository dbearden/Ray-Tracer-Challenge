use std::{cell::RefCell, rc::Rc};

use crate::matrix::Matrix;

use super::Shape;
#[derive(Debug, PartialEq)]
pub struct Sphere {
    id: usize,
    transform: Matrix<4>,
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
}
impl Sphere {
    pub fn new(id: usize) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            id,
            transform: Matrix::<4>::IDENTITY,
        }))
    }
}
