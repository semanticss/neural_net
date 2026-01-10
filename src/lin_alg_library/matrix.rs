use rand::prelude::*;
use std::fmt;

pub struct Matrix {
    rows: usize,
    columns: usize,
    data: Vec<f64>
}

// m[(x,y)]
impl std::ops::Index<(usize,usize)> for Matrix {
    type Output = f64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let row = index.0;
        let column = index.1;
        return &self.data[row * self.columns + column];
    }

}

// m[(x,y)] = 
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
            data: vec![0.; rows * columns]
        }
    }


    pub fn new_random(rows: usize, columns: usize,) -> Self {

        let mut rng = rand::rng();
        Self {
            rows: rows,
            columns: columns,
            data: (0..rows*columns).map(|_| rng.random_range::<f64, _>(0.0..1.)).collect()
        }
    }

    pub fn add_matrices(&self, a: Matrix) -> Matrix {
        let mut result = Vec::with_capacity(self.rows * self.columns);

        for i in 0..self.data.len() {
            let f = self.data[i] + a.data[i];
            result.push(f);
        }

        Matrix { rows: self.rows, columns: self.columns, data: result}
    }

    pub fn subtract_matrices(&self, a: Matrix) -> Matrix {
        let mut result = Vec::with_capacity(self.rows * self.columns);

        for i in  0..self.data.len() {
            let f = self.data[i] - a.data[i];
            result.push(f);
        }

        Matrix {rows: self.rows, columns: self.columns, data: result}
    }

    pub fn multiply_matrices(&self, a: Matrix) -> Result<Matrix, String> {
        // Note: # of columns in first matrix must be same number of rows in second matrix
        if self.columns != a.rows {
            return Err("incompatible dimensions".to_string())
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

        return Ok(Matrix {rows: self.rows, columns: a.columns, data: result})

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
    fn test_random_matrix(){
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
        assert!(m.data == vec![1.,2.,3.,4.,5.,6.,7.,8.,9.]);
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

        assert!(m1.add_matrices(m2) == matrix! [2., 4., 6., 8.])
    }


    #[test]
    fn test_matrix_multiplication(){
        let m1: Matrix = matrix![
            1., 2.;
            3., 4.
        ];

        let m2: Matrix = matrix![
            5., 6.;
            7., 8.
        ];

        let m3: Matrix = m1.multiply_matrices(m2).unwrap();
        
        assert!(m3 == matrix! [19., 22., 43., 50.]);
        
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

        let m3: Matrix = m1.multiply_matrices(m2).unwrap();
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