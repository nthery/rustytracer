//! Matrix type
//!
//! TRTC chapter 3.

/// A 2D matrix of f64 values.
///
/// TODO: Make it generic over dimensions when generic value parameters supported.
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
}
