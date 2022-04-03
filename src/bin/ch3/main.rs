#![feature(generic_const_exprs)]
mod canvas;
mod matrix;
mod tuple;
use matrix::Matrix;
use tuple::{Point, Tuple};

fn main() {
    //1.
    println!("{:#?}\n\n", Matrix::<4>::IDENTITY.inverse());

    //2.
    let a = Matrix::new([
        [6.0, 4.0, 4.0, 4.0],
        [5.0, 5.0, 7.0, 6.0],
        [4.0, -9.0, 3.0, -7.0],
        [9.0, 1.0, 7.0, -6.0],
    ]);

    assert_eq!(a * a.inverse(), Matrix::<4>::IDENTITY);
    println!("{:#?}\n\n", a * a.inverse());

    //3.
    assert_eq!(a.transpose().inverse(), a.inverse().transpose());
    println!(
        "{:#?}\n{:#?}\n\n",
        a.transpose().inverse(),
        a.inverse().transpose()
    );

    //4.
    let mut i = Matrix::<4>::IDENTITY;
    i[1][2] = 2.0;
    let t = Point::new(1.0, 1.0, 1.0);
    println!("{:#?}", i * t);
}
