//! Matrix type
//!
//! TRTC chapter 3.

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
}
