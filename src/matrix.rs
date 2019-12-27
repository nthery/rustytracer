//! Matrix type
//!
//! TRTC chapter 3.

use crate::tuple::Tuple;
use crate::util;

/// A 2D matrix of f64 values.
///
/// TODO: Make it generic over dimensions when generic value parameters supported.
#[derive(Debug)]
pub struct Matrix {
    nrows: usize,
    ncols: usize,
    cells: Vec<f64>,
}

impl Matrix {
    /// Create a new zero matrix.
    pub fn new(nrows: usize, ncols: usize) -> Matrix {
        Matrix {
            nrows,
            ncols,
            cells: vec![0.0; nrows * ncols],
        }
    }

    /// Create a new 4x4 matrix containing the given values.
    pub fn new_4x4(values: &[[f64; 4]; 4]) -> Matrix {
        let cells = values.iter().flatten().cloned().collect::<Vec<f64>>();
        Matrix {
            nrows: 4,
            ncols: 4,
            cells,
        }
    }

    pub fn new_4x4_identity() -> Matrix {
        let mut m = Matrix::new(4, 4);
        for i in 0..4 {
            m.set(i, i, 1.0)
        }
        m
    }

    /// Create a new 3x3 matrix containing the given values.
    pub fn new_3x3(values: &[[f64; 3]; 3]) -> Matrix {
        let cells = values.iter().flatten().cloned().collect::<Vec<f64>>();
        Matrix {
            nrows: 3,
            ncols: 3,
            cells,
        }
    }

    /// Create a new 2x2 matrix containing the given values.
    pub fn new_2x2(values: &[[f64; 2]; 2]) -> Matrix {
        let cells = values.iter().flatten().cloned().collect::<Vec<f64>>();
        Matrix {
            nrows: 2,
            ncols: 2,
            cells,
        }
    }

    /// Return the value at the specified row and column.
    pub fn get(&self, r: usize, c: usize) -> f64 {
        assert!(r < self.nrows);
        assert!(c < self.ncols);
        self.cells[r * self.nrows + c]
    }

    /// Change the cell at the specified row and column to the specified value.
    pub fn set(&mut self, r: usize, c: usize, v: f64) {
        assert!(r < self.nrows);
        assert!(c < self.ncols);
        self.cells[r * self.nrows + c] = v;
    }

    /// Return transposition of this matrix.
    /// TODO: transpose in place needed?
    pub fn transposed(&self) -> Matrix {
        let mut m = Matrix::new(self.ncols, self.nrows); // TODO: useless init
        for r in 0..self.nrows {
            for c in 0..self.ncols {
                m.set(c, r, self.get(r, c));
            }
        }
        m
    }

    pub fn determinant(&self) -> f64 {
        // TODO: implement for non 2x2
        assert!(self.nrows == 2 && self.ncols == 2);
        self.get(0, 0) * self.get(1, 1) - self.get(0, 1) * self.get(1, 0)
    }

    /// Return copy of this matrix without specified row and column.
    /// TODO: Return view (slice) on existing matrix?
    pub fn submatrix(&self, row: usize, col: usize) -> Matrix {
        assert!(row < self.nrows && col < self.ncols);
        assert!(self.nrows > 1 && self.ncols > 1);

        // TODO: useless init
        let mut m = Matrix::new(self.nrows - 1, self.ncols - 1);

        let mut dst_row = 0;
        for src_row in 0..self.nrows {
            if src_row == row {
                continue;
            }
            let mut dst_col = 0;
            for src_col in 0..self.ncols {
                if src_col == col {
                    continue;
                }
                m.set(dst_row, dst_col, self.get(src_row, src_col));
                dst_col += 1;
            }
            dst_row += 1;
        }

        m
    }

    /// Returns the minor of element `(row, col)`.
    pub fn minor(&self, row: usize, col: usize) -> f64 {
        // TODO: implement for non 3x3
        assert!(self.nrows == 3 && self.ncols == 3);

        // TODO: avoid creation of temporary submatrix
        self.submatrix(row, col).determinant()
    }

    /// Returns cofactor of element `(row, col)`.
    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let f = if (row + col) & 1 == 1 { -1.0 } else { 1.0 };
        f * self.minor(row, col)
    }
}

impl PartialEq for Matrix {
    /// Return true if arguments are approximately equal.
    fn eq(&self, o: &Matrix) -> bool {
        if self.nrows != o.nrows || self.ncols != o.ncols {
            return false;
        }
        // TODO: Can use Iterator::cmp_by() when available.
        for i in 0..self.cells.len() {
            if !util::nearly_equal(self.cells[i], o.cells[i]) {
                return false;
            }
        }
        return true;
    }
}

impl std::ops::Mul for &Matrix {
    type Output = Matrix;

    /// Multiply given matrices.
    /// TODO: naive algorithm
    fn mul(self, o: Self) -> Self::Output {
        assert_eq!(self.nrows, o.ncols);
        let mut res = Matrix::new(o.ncols, self.nrows);
        for r in 0..self.nrows {
            for c in 0..self.ncols {
                let mut dot = 0.0;
                for i in 0..self.nrows {
                    dot += self.get(r, i) * o.get(i, c);
                }
                res.set(r, c, dot);
            }
        }
        res
    }
}

impl std::ops::Mul<&Tuple> for &Matrix {
    type Output = Tuple;

    /// Multiply given matrix and tuple.
    /// TODO: naive algorithm
    fn mul(self, o: &Tuple) -> Self::Output {
        assert_eq!(self.ncols, 4);
        let mut res = Tuple::new_zero(); // TODO: useless init
        for r in 0..self.nrows {
            let mut dot = 0.0;
            for c in 0..self.ncols {
                dot += self.get(r, c) * o.get(c);
            }
            res.set(r, dot);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructing_and_inspecting_4x4_matrix() {
        let m = Matrix::new_4x4(&[
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);
        assert_eq!(m.get(0, 0), 1.0);
        assert_eq!(m.get(3, 2), 15.5);
    }

    #[test]
    fn constructing_and_inspecting_3x3_matrix() {
        let m = Matrix::new_3x3(&[[1.0, 2.0, 3.0], [5.5, 6.5, 7.5], [9.0, 10.0, 11.0]]);
        assert_eq!(m.get(0, 0), 1.0);
        assert_eq!(m.get(2, 1), 10.0);
    }

    #[test]
    fn constructing_and_inspecting_2x2_matrix() {
        let m = Matrix::new_2x2(&[[1.0, 2.0], [5.5, 6.5]]);
        assert_eq!(m.get(0, 0), 1.0);
        assert_eq!(m.get(1, 1), 6.5);
    }

    #[test]
    fn setting_and_getting_cells() {
        let mut m = Matrix::new(2, 2);
        m.set(0, 0, 42.0);
        m.set(1, 0, 24.0);
        assert_eq!(m.get(0, 0), 42.0);
        assert_eq!(m.get(1, 0), 24.0);
    }

    #[test]
    fn comparing_equal_matrices() {
        let l = Matrix::new_4x4(&[
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let r = Matrix::new_4x4(&[
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        assert_eq!(l, r);
    }

    #[test]
    fn comparing_different_matrices() {
        let l = Matrix::new_4x4(&[
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let r = Matrix::new_4x4(&[
            [5.0, 6.0, 7.0, 8.0],
            [1.0, 2.0, 3.0, 4.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        assert_ne!(l, r);
    }

    #[test]
    fn multiplying_matrices() {
        let l = Matrix::new_4x4(&[
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let r = Matrix::new_4x4(&[
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);
        assert_eq!(
            &l * &r,
            Matrix::new_4x4(&[
                [20.0, 22.0, 50.0, 48.0],
                [44.0, 54.0, 114.0, 108.0],
                [40.0, 58.0, 110.0, 102.0],
                [16.0, 26.0, 46.0, 42.0],
            ])
        );
    }

    #[test]
    fn multiplying_matrix_and_tuple() {
        let m = Matrix::new_4x4(&[
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let t = Tuple::new(1.0, 2.0, 3.0, 1.0);
        assert_eq!(&m * &t, Tuple::new(18.0, 24.0, 33.0, 1.0));
    }

    #[test]
    fn multiplying_matrix_by_identity() {
        let m = Matrix::new_4x4(&[
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        assert_eq!(&m * &Matrix::new_4x4_identity(), m);
    }

    #[test]
    fn transposing_matrix() {
        let m = Matrix::new_4x4(&[
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        ]);
        assert_eq!(
            m.transposed(),
            Matrix::new_4x4(&[
                [0.0, 9.0, 1.0, 0.0],
                [9.0, 8.0, 8.0, 0.0],
                [3.0, 0.0, 5.0, 5.0],
                [0.0, 8.0, 3.0, 8.0]
            ])
        );
    }

    #[test]
    fn calculating_determinant_of_2x2_matrix() {
        let m = Matrix::new_2x2(&[[1.0, 5.0], [-3.0, 2.0]]);
        assert_eq!(m.determinant(), 17.0);
    }

    #[test]
    fn submatrix_of_3x3_is_2x2() {
        let m = Matrix::new_3x3(&[[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]]);
        assert_eq!(
            m.submatrix(0, 2),
            Matrix::new_2x2(&[[-3.0, 2.0], [0.0, 6.0]])
        );
    }

    #[test]
    fn submatrix_of_4x4_is_3x3() {
        let m = Matrix::new_4x4(&[
            [-6.0, 1.0, 1.0, 6.0],
            [-8.0, 5.0, 8.0, 6.0],
            [-1.0, 0.0, 8.0, 2.0],
            [-7.0, 1.0, -1.0, 1.0],
        ]);
        assert_eq!(
            m.submatrix(2, 1),
            Matrix::new_3x3(&[[-6.0, 1.0, 6.0], [-8.0, 8.0, 6.0], [-7.0, -1.0, 1.0],])
        );
    }

    #[test]
    fn calculating_minor_of_3x3_matrix() {
        let m = Matrix::new_3x3(&[[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, 1.0, 5.0]]);
        let s = m.submatrix(1, 0);
        assert_eq!(s.determinant(), 25.0);
        assert_eq!(m.minor(1, 0), 25.0);
    }

    #[test]
    fn calculating_cofactor_of_3x3_matrix() {
        let m = Matrix::new_3x3(&[[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
        assert_eq!(m.minor(0, 0), -12.0);
        assert_eq!(m.cofactor(0, 0), -12.0);
        assert_eq!(m.minor(1, 0), 25.0);
        assert_eq!(m.cofactor(1, 0), -25.0);
    }
}
