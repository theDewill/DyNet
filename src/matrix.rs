// src/matrix.rs

pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize, data: Vec<f64>) -> Self {
        assert_eq!(rows * cols, data.len());
        Matrix { rows, cols, data }
    }

    pub fn zero(rows: usize, cols: usize) -> Self {
        Matrix {
            rows,
            cols,
            data: vec![0.0; rows * cols],
        }
    }

    pub fn multiply(&self, other: &Matrix) -> Matrix {
        assert_eq!(self.cols, other.rows);
        let mut data = vec![0.0; self.rows * other.cols];
        for i in 0..self.rows {
            for j in 0..other.cols {
                for k in 0..self.cols {
                    data[i * other.cols + j] += self.data[i * self.cols + k] * other.data[k * other.cols + j];
                }
            }
        }
        Matrix::new(self.rows, other.cols, data)
    }

    // Add an element-wise application of a function
    pub fn apply<F>(&mut self, f: F)
    where
        F: Fn(f64) -> f64,
    {
        for i in 0..self.data.len() {
            self.data[i] = f(self.data[i]);
        }
    }
}