use float_cmp::{approx_eq};

use crate::{transformations::Transformation, tuple::Vector};

use super::tuple::{Point, Tuple};

#[derive(Debug, Copy, Clone)]
pub struct Matrix<const N: usize> {
    pub data: [[f64; N]; N],
}

impl<const N: usize> Matrix<N> {
    pub fn new(data: [[f64; N]; N]) -> Self {
        assert!((2..=4).contains(&N));
        Matrix { data }
    }

    pub fn transpose(&self) -> Self {
        let mut res = Matrix::new([[0.0; N]; N]);
        for row in 0..N {
            for col in 0..N {
                res[row][col] = self[col][row];
            }
        }
        res
    }
}

impl Matrix<4> {
    pub const IDENTITY: Matrix<4> = Matrix {
        data: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    };
    pub fn submatrix(&self, row: usize, col: usize) -> Matrix<3> {
        let mut m = Matrix::new([[0.0; 3]; 3]);

        let (mut x, mut y) = (0, 0);
        for i in 0..3 {
            if x == row {
                x += 1;
            }
            for j in 0..3 {
                if y == col {
                    y += 1;
                }

                m[i][j] = self[x][y];

                y += 1;
            }
            x += 1;
            y = 0;
        }
        m
    }
    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }
    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        self.minor(row, col) * (-1.0_f64).powi((row + col) as i32)
    }
    pub fn determinant(&self) -> f64 {
        let mut det = 0.0;
        for column in 0..4 {
            det += self[0][column] * self.cofactor(0, column)
        }

        det
    }
    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }
    pub fn inverse(&self) -> Self {
        if !self.is_invertible() {
            panic!();
        }
        let mut m = Matrix::new([[0.0; 4]; 4]);
        for row in 0..4 {
            for col in 0..4 {
                let c = self.cofactor(row, col);
                m[col][row] = c / self.determinant();
            }
        }
        m
    }
}
impl Matrix<3> {
    pub const IDENTITY: Matrix<3> = Matrix {
        data: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
    };
    pub fn submatrix(&self, row: usize, col: usize) -> Matrix<2> {
        let mut m = Matrix::new([[0.0; 2]; 2]);

        let (mut x, mut y) = (0, 0);
        for i in 0..2 {
            if x == row {
                x += 1;
            }
            for j in 0..2 {
                if y == col {
                    y += 1;
                }

                m[i][j] = self[x][y];

                y += 1;
            }
            x += 1;
            y = 0;
        }
        m
    }
    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }
    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        self.minor(row, col) * (-1.0_f64).powi((row + col) as i32)
    }
    pub fn determinant(&self) -> f64 {
        let mut det = 0.0;
        for column in 0..3 {
            det += self[0][column] * self.cofactor(0, column)
        }

        det
    }
    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }
    pub fn inverse(&self) -> Self {
        if !self.is_invertible() {
            panic!();
        }
        let mut m = Matrix::new([[0.0; 3]; 3]);
        for row in 0..3 {
            for col in 0..3 {
                let c = self.cofactor(row, col);
                m[col][row] = c / self.determinant();
            }
        }
        m
    }
}
impl Matrix<2> {
    pub const IDENTITY: Matrix<2> = Matrix {
        data: [[1.0, 0.0], [0.0, 1.0]],
    };
    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self[2 - row][2 - col]
    }
    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        self.minor(row, col) * (-1.0_f64).powi((row + col) as i32)
    }
    pub fn determinant(&self) -> f64 {
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }
    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }
    pub fn inverse(&self) -> Self {
        if !self.is_invertible() {
            panic!();
        }
        let mut m = Matrix::new([[0.0; 2]; 2]);
        for row in 0..2 {
            for col in 0..2 {
                let c = self.cofactor(row, col);
                m[col][row] = c / self.determinant();
            }
        }
        m
    }
}
impl Transformation for Matrix<4> {
    fn translation(&self, x: f64, y: f64, z: f64) -> Matrix<4> {
        Matrix::new([
            [1.0, 0.0, 0.0, x],
            [0.0, 1.0, 0.0, y],
            [0.0, 0.0, 1.0, z],
            [0.0, 0.0, 0.0, 1.0],
        ]) * *self
    }
    fn scaling(&self, x: f64, y: f64, z: f64) -> Matrix<4> {
        Matrix::new([
            [x, 0.0, 0.0, 0.0],
            [0.0, y, 0.0, 0.0],
            [0.0, 0.0, z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]) * *self
    }
    fn shearing(&self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix<4> {
        Matrix::new([
            [1.0, xy, xz, 0.0],
            [yx, 1.0, yz, 0.0],
            [zx, zy, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]) * *self
    }
    fn rotation_x(&self, r: f64) -> Matrix<4> {
        Matrix::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, r.cos(), -(r.sin()), 0.0],
            [0.0, r.sin(), r.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]) * *self
    }
    fn rotation_y(&self, r: f64) -> Matrix<4> {
        Matrix::new([
            [r.cos(), 0.0, r.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-(r.sin()), 0.0, r.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]) * *self
    }
    fn rotation_z(&self, r: f64) -> Matrix<4> {
        Matrix::new([
            [r.cos(), -(r.sin()), 0.0, 0.0],
            [r.sin(), r.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]) * *self
    }
}
impl<const N: usize> PartialEq for Matrix<N> {
    fn eq(&self, other: &Self) -> bool {
        self.data
            .iter()
            .flatten()
            .zip(other.data.iter().flatten())
            .all(|(a, b)| approx_eq!(f64, *a, *b, epsilon = 0.00003))
    }
}

impl<const N: usize> std::ops::Index<usize> for Matrix<N> {
    type Output = [f64; N];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
impl<const N: usize> std::ops::IndexMut<usize> for Matrix<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<const N: usize> std::ops::Mul for Matrix<N> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut m = Matrix::<N> {
            data: [[0.0; N]; N],
        };
        for row in 0..N {
            for col in 0..N {
                for i in 0..N {
                    m[row][col] += self[row][i] * rhs[i][col]
                }
            }
        }

        m
    }
}
impl std::ops::Mul<Vector> for Matrix<4> {
    type Output = Vector;
    fn mul(self, rhs: Vector) -> Self::Output {
        let mut v = Vector::new(0.0, 0.0, 0.0);

        v.x = self[0][0] * rhs.x()
            + self[0][1] * rhs.y()
            + self[0][2] * rhs.z()
            + self[0][3] * rhs.w();
        v.y = self[1][0] * rhs.x()
            + self[1][1] * rhs.y()
            + self[1][2] * rhs.z()
            + self[1][3] * rhs.w();
        v.z = self[2][0] * rhs.x()
            + self[2][1] * rhs.y()
            + self[2][2] * rhs.z()
            + self[2][3] * rhs.w();

        v
    }
}
impl std::ops::Mul<Point> for Matrix<4> {
    type Output = Point;
    fn mul(self, rhs: Point) -> Self::Output {
        let mut p = Point::new(0.0, 0.0, 0.0);

        p.x = self[0][0] * rhs.x()
            + self[0][1] * rhs.y()
            + self[0][2] * rhs.z()
            + self[0][3] * rhs.w();
        p.y = self[1][0] * rhs.x()
            + self[1][1] * rhs.y()
            + self[1][2] * rhs.z()
            + self[1][3] * rhs.w();
        p.z = self[2][0] * rhs.x()
            + self[2][1] * rhs.y()
            + self[2][2] * rhs.z()
            + self[2][3] * rhs.w();

        p
    }
}

#[cfg(test)]
mod tests {
    use crate::tuple::{Point, Tuple};

    use super::*;

    #[test]
    fn create_4x4() {
        let m = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);
        assert_eq!(1.0, m[0][0]);
        assert_eq!(4.0, m[0][3]);
        assert_eq!(5.5, m[1][0]);
        assert_eq!(7.5, m[1][2]);
        assert_eq!(11.0, m[2][2]);
        assert_eq!(13.5, m[3][0]);
        assert_eq!(15.5, m[3][2]);
    }

    #[test]
    fn create_2x2() {
        let m = Matrix::new([[-3.0, 5.0], [1.0, -2.0]]);

        assert_eq!(-3.0, m[0][0]);
        assert_eq!(5.0, m[0][1]);
        assert_eq!(1.0, m[1][0]);
        assert_eq!(-2.0, m[1][1]);
    }

    #[test]
    fn create_3x3() {
        let m = Matrix::new([[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]]);

        assert_eq!(m[0][0], -3.0);
        assert_eq!(m[1][1], -2.0);
        assert_eq!(m[2][2], 1.0);
    }

    #[test]
    fn equality() {
        let a = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);
        let b = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);

        assert_eq!(a, b);
    }
    #[test]
    fn inequality() {
        let a = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);
        let b = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
        ]);

        assert_ne!(a, b);
    }

    #[test]
    fn multiply_by_matrix() {
        let a = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);
        let b = Matrix::new([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);

        let result = Matrix::new([
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0],
        ]);

        assert_ne!(a * b, result);
    }

    #[test]
    fn multiply_by_point() {
        let a = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let b = Point::new(1.0, 2.0, 3.0);
        let result = Point::new(18.0, 24.0, 33.0);
        assert_eq!(a * b, result);
    }

    #[test]
    fn multiply_by_identity() {
        let a = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let i = Matrix::<4>::IDENTITY;

        assert_eq!(a * i, a);
    }

    #[test]
    fn transpose_matrix() {
        let a = Matrix::new([
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        ]);
        let b = Matrix::new([
            [0.0, 9.0, 1.0, 0.0],
            [9.0, 8.0, 8.0, 0.0],
            [3.0, 0.0, 5.0, 5.0],
            [0.0, 8.0, 3.0, 8.0],
        ]);

        assert_eq!(a.transpose(), b);
    }
    #[test]
    fn transpose_identity() {
        let i = Matrix::<4>::IDENTITY;
        assert_eq!(i, i.transpose());
    }

    #[test]
    fn determinant_2x2() {
        let m = Matrix::new([[1.0, 5.0], [-3.0, 2.0]]);
        assert_eq!(m.determinant(), 17.0);
    }

    #[test]
    fn submatrix_3x3() {
        let m = Matrix::new([[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]]);
        let s = Matrix::new([[-3.0, 2.0], [0.0, 6.0]]);

        assert_eq!(m.submatrix(0, 2), s);
    }

    #[test]
    fn submatrix_4x4() {
        let m = Matrix::new([
            [-6.0, 1.0, 1.0, 6.0],
            [-8.0, 5.0, 8.0, 6.0],
            [-1.0, 0.0, 8.0, 2.0],
            [-7.0, 1.0, -1.0, 1.0],
        ]);
        let s = Matrix::new([[-6.0, 1.0, 6.0], [-8.0, 8.0, 6.0], [-7.0, -1.0, 1.0]]);

        assert_eq!(m.submatrix(2, 1), s);
    }

    #[test]
    fn minor_3x3() {
        let a = Matrix::new([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
        let b = a.submatrix(1, 0);
        assert_eq!(b.determinant(), 25.0);
        assert_eq!(a.minor(1, 0), 25.0);
    }

    #[test]
    fn cofactor_3x3() {
        let m = Matrix::new([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
        assert_eq!(m.minor(0, 0), -12.0);
        assert_eq!(m.cofactor(0, 0), -12.0);
        assert_eq!(m.minor(1, 0), 25.0);
        assert_eq!(m.cofactor(1, 0), -25.0);
    }

    #[test]
    fn determinant_3x3() {
        let m = Matrix::new([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);
        assert_eq!(m.cofactor(0, 0), 56.0);
        assert_eq!(m.cofactor(0, 1), 12.0);
        assert_eq!(m.cofactor(0, 2), -46.0);
        assert_eq!(m.determinant(), -196.0);
    }
    #[test]
    fn determinant_4x4() {
        let m = Matrix::new([
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ]);
        assert_eq!(m.cofactor(0, 0), 690.0);
        assert_eq!(m.cofactor(0, 1), 447.0);
        assert_eq!(m.cofactor(0, 2), 210.0);
        assert_eq!(m.cofactor(0, 3), 51.0);
        assert_eq!(m.determinant(), -4071.0);
    }

    #[test]
    fn invertible() {
        let m = Matrix::new([
            [6.0, 4.0, 4.0, 4.0],
            [5.0, 5.0, 7.0, 6.0],
            [4.0, -9.0, 3.0, -7.0],
            [9.0, 1.0, 7.0, -6.0],
        ]);
        assert_eq!(m.determinant(), -2120.0);
        assert!(m.is_invertible());
    }
    #[test]
    fn not_invertible() {
        let m = Matrix::new([
            [-4.0, 2.0, -2.0, -3.0],
            [9.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0; 4],
        ]);
        assert_eq!(m.determinant(), 0.0);
        assert!(!m.is_invertible());
    }

    #[test]
    fn inverse() {
        let a = Matrix::new([
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0],
        ]);
        let b = a.inverse();
        assert_eq!(a.determinant(), 532.0);
        assert_eq!(a.cofactor(2, 3), -160.0);
        assert_eq!(b[3][2], -160.0 / 532.0);
        assert_eq!(a.cofactor(3, 2), 105.0);
        assert_eq!(b[2][3], 105.0 / 532.0);
        assert_eq!(
            b,
            Matrix::new([
                [0.21805, 0.45113, 0.24060, -0.04511],
                [-0.80827, -1.45677, -0.44361, 0.52068],
                [-0.07895, -0.22368, -0.05263, 0.19737],
                [-0.52256, -0.81391, -0.30075, 0.30639],
            ])
        );
    }

    #[test]
    fn another_inverse() {
        let a = Matrix::new([
            [8.0, -5.0, 9.0, 2.0],
            [7.0, 5.0, 6.0, 1.0],
            [-6.0, 0.0, 9.0, 6.0],
            [-3.0, 0.0, -9.0, -4.0],
        ]);
        assert_eq!(
            a.inverse(),
            Matrix::new([
                [-0.15385, -0.15385, -0.28205, -0.53846],
                [-0.07692, 0.12308, 0.02564, 0.03077],
                [0.35897, 0.35897, 0.43590, 0.92308],
                [-0.69231, -0.69231, -0.76923, -1.92308],
            ])
        );
    }

    #[test]
    fn another_inverse_again() {
        let a = Matrix::new([
            [9.0, 3.0, 0.0, 9.0],
            [-5.0, -2.0, -6.0, -3.0],
            [-4.0, 9.0, 6.0, 4.0],
            [-7.0, 6.0, 6.0, 2.0],
        ]);

        assert_eq!(
            a.inverse(),
            Matrix::new([
                [-0.04074, -0.07778, 0.14444, -0.22222],
                [-0.07778, 0.03333, 0.36667, -0.33333],
                [-0.02901, -0.14630, -0.10926, 0.12963],
                [0.17778, 0.06667, -0.26667, 0.33333],
            ])
        );
    }
    #[test]
    fn inverse_multiplication() {
        let a = Matrix::new([
            [3.0, -9.0, 7.0, 3.0],
            [3.0, -8.0, 2.0, -9.0],
            [-4.0, 4.0, 4.0, 1.0],
            [-6.0, -2.0, 0.0, 5.0],
        ]);
        let b = Matrix::new([
            [8.0, 2.0, 2.0, 2.0],
            [3.0, -1.0, 7.0, 0.0],
            [7.0, 0.0, 5.0, 4.0],
            [6.0, -3.0, 0.0, 5.0],
        ]);

        let c = a * b;
        assert_eq!(c * b.inverse(), a);
    }
}
