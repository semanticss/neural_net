use rand::prelude::*;
use std::fmt;

pub struct Matrix {
    pub rows: usize,
    pub columns: usize,
    pub data: Vec<f64>,
}

// m[(x,y)]
impl std::ops::Index<(usize, usize)> for Matrix {
    type Output = f64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let row = index.0;
        let column = index.1;
        return &self.data[row * self.columns + column];
    }
}

// m[(x,y)] = ...
impl std::ops::IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (row, column) = index;
        &mut self.data[row * self.columns + column]
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        return self.data == other.data;
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, val) in self.data.iter().enumerate() {
            if i > 0 && i % self.columns == 0 {
                writeln!(f)?;
            }

            write!(f, "{:>8.4} ", val)?;
        }
        Ok(())
    }
}

impl Matrix {
    pub fn new(rows: usize, columns: usize) -> Self {
        Self {
            rows: rows,
            columns: columns,
            data: vec![0.; rows * columns],
        }
    }

    pub fn new_random(rows: usize, columns: usize) -> Self {
        let mut rng = rand::rng();
        Self {
            rows: rows,
            columns: columns,
            data: (0..rows * columns)
                .map(|_| rng.random_range::<f64, _>(0.0..1.))
                .collect(),
        }
    }

    // pub fn elementwise multiplication

    pub fn add_matrices(&self, a: &Matrix) -> Result<Matrix, String> {
        if self.rows != a.rows {
            return Err("incompatible matrices".to_string());
        }
        let mut result = Vec::with_capacity(self.rows * self.columns);

        for i in 0..self.data.len() {
            let f = self.data[i] + a.data[i];
            result.push(f);
        }

        return Ok(Matrix {
            rows: self.rows,
            columns: self.columns,
            data: result,
        });
    }

    pub fn determinant_by_guassian_elimination(&mut self) -> f64 {
        self.forward_guassian_elimination();
        return self.data.iter().step_by(self.rows + 1).product();
    }
    // qr decomposition
    // code taken from psuedocode from wikipedia
    pub fn forward_guassian_elimination(&mut self) {
        let m = self.rows;
        let n = self.columns;

        let mut h = 0;
        let mut k = 0;
        let mut sign = 1.0;

        while h < m && k < n {
            let i_max = (h..m)
                .max_by(|&i, &j| {
                    self.data[i * n + k]
                        .abs()
                        .partial_cmp(&self.data[j * n + k].abs())
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
                .unwrap();

            if self.data[i_max * n + k].abs() < 1e-12 {
                k += 1;
                continue;
            }

            if h != i_max {
                self.swap_rows(h, i_max);
                sign *= -1.;
            }

            let pivot_value = self.data[h * n + k];
            for i in (h + 1)..m {
                let factor = self.data[i * n + k] / pivot_value;
                self.data[i * n + k] = 0.0;

                for j in (k + 1)..n {
                    self.data[i * n + j] -= self.data[h * n + j] * factor;
                }
            }
            h += 1;
            k += 1;
        }
    }

    pub fn swap_columns(&mut self, col_a: usize, col_b: usize) {
        if col_a == col_b {
            return;
        }

        let rows = self.rows;
    }
    pub fn swap_rows(&mut self, row_a: usize, row_b: usize) {
        if row_a == row_b {
            return;
        }

        let cols = self.columns;
        let (first, second) = if row_a < row_b {
            (row_a, row_b)
        } else {
            (row_b, row_a)
        };

        let (left, right) = self.data.split_at_mut(second * cols);

        let slice_a = &mut left[first * cols..(first * cols) + cols];
        let slice_b = &mut right[0..cols];

        slice_a.swap_with_slice(slice_b);
    }

    pub fn get_row(&self, row: usize) -> &[f64] {
        return &self.data[row * self.columns..(row * self.columns) + (self.columns)];
    }

    pub fn get_column(&self, column: usize) -> Vec<f64> {
        return self.data.iter().step_by(self.rows).map(|x| *x).collect();
    }

    pub fn subtract_matrices(&self, a: &Matrix) -> Result<Matrix, String> {
        if self.rows != a.rows {
            return Err("incompatible dimensions".to_string());
        }
        let mut result = Vec::with_capacity(self.rows * self.columns);

        for i in 0..self.data.len() {
            let f = self.data[i] - a.data[i];
            result.push(f);
        }

        return Ok(Matrix {
            rows: self.rows,
            columns: self.columns,
            data: result,
        });
    }

    pub fn multiply_matrices(&self, a: &Matrix) -> Result<Matrix, String> {
        if self.columns != a.rows {
            return Err("incompatible dimensions".to_string());
        }

        let mut result = vec![0.0; self.rows * a.columns];

        for i in 0..self.rows {
            for j in 0..a.columns {
                let mut sum = 0.;
                for k in 0..self.columns {
                    sum += self.data[i * self.columns + k] * a.data[k * a.columns + j];
                }
                result[i * a.columns + j] = sum;
            }
        }

        return Ok(Matrix {
            rows: self.rows,
            columns: a.columns,
            data: result,
        });
    }
}

#[macro_export]
macro_rules! matrix {
    ( $( $( $v:expr ),+ );* ) => {
        {
        let mut data = Vec::<f64>::new();
        let mut rows = 0;
        let mut columns = 0;
        $(
            let data_row = vec![ $( $v as f64), *];
            let current_row_len = data_row.len();

            if columns == 0 {
                columns = current_row_len;
            } else if columns != current_row_len {
                panic!("imcompatible matrix dimensions, expected: {}, got {}", columns, current_row_len);
            }
            data.extend(data_row);
            rows += 1;
        )*

        Matrix {rows, columns, data}
    }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determinant_by_guassian() {
        let mut m: Matrix = matrix! [
            1., 2.;
            3., 4.
        ];

        println!("{}", m.determinant_by_guassian_elimination());
    }

    #[test]
    fn test_feed_forward() {
        let inputs = matrix![1., 1., 1.];

        let weights = matrix! [
            0.5, -0.25;
            0.7, 0.1;
            -0.2, 0.1
        ];

        let bias = matrix![0.1, -0.2];

        let weighted_sum = inputs
            .multiply_matrices(&weights)
            .unwrap()
            .add_matrices(&bias)
            .unwrap();
    }

    #[test]
    fn test_guassian_elimination() {
        let mut m: Matrix = matrix! [
            1., 2.;
            3., 4.
        ];

        m.forward_guassian_elimination();
        println!("{}", m);
    }

    #[test]
    fn test_get_row() {
        let m: Matrix = matrix! [
            1., 2.;
            3., 4.
        ];

        assert!(m.get_row(0) == [1., 2.]);
    }

    #[test]
    fn test_get_column() {
        let m: Matrix = matrix! [
            1., 2.;
            3., 4.
        ];

        assert!(m.get_column(0) == vec![1., 3.]);
    }
    #[test]
    fn test_random_matrix() {
        let m: Matrix = Matrix::new_random(5, 8);
        println!("{}", m);
    }

    #[test]
    fn test_matrix_macro() {
        let m: Matrix = matrix![
            1., 2., 3.;
            4., 5., 6.;
            7., 8., 9.
        ];

        assert!(m.rows == 3);
        assert!(m.columns == 3);
        assert!(m.data == vec![1., 2., 3., 4., 5., 6., 7., 8., 9.]);
    }

    #[test]
    fn test_matrix_addition() {
        let m1: Matrix = matrix! [
            1., 2.;
            3., 4.
        ];

        let m2: Matrix = matrix! [
            1., 2.;
            3., 4.
        ];

        let m3 = m1.add_matrices(&m2).unwrap();

        assert!(m3 == matrix![2., 4., 6., 8.])
    }

    #[test]
    fn test_matrix_subtraction() {
        let m1: Matrix = matrix![
            1., 2.;
            3., 4.
        ];

        let m2: Matrix = matrix![
            5., 6.;
            7., 8.
        ];

        let m3 = m1.subtract_matrices(&m2).unwrap();

        assert!(m3 == matrix![-4., -4., -4., -4.]);
    }

    #[test]
    fn test_matrix_multiplication() {
        let m1: Matrix = matrix![
            1., 2.;
            3., 4.
        ];

        let m2: Matrix = matrix![
            5., 6.;
            7., 8.
        ];

        let m3: Matrix = m1.multiply_matrices(&m2).unwrap();

        assert!(m3 == matrix![19., 22., 43., 50.]);
    }

    #[test]
    fn test_dimension_assertion() {
        let m1: Matrix = matrix![
            1., 2.;
            3., 4.
        ];

        let m2: Matrix = matrix![
            5., 6., 15.;
            7., 8., 53.
        ];

        let m3: Matrix = m1.multiply_matrices(&m2).unwrap();
    }

    #[test]
    fn test_display() {
        let m1: Matrix = matrix! [
            1., 2.;
            3., 4.
        ];

        println!("{}", m1);
    }
}
