use core::fmt;
use std::fmt::Debug;
use std::ops::{Add, AddAssign, Mul};

use anyhow::Result;

pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Debug + Copy + Mul<Output = T> + Add<Output = T> + AddAssign + Default,
{
    if a.cols != b.rows {
        return Err(anyhow::anyhow!(
            "Cannot multiply matrices with dimensions {}x{} and {}x{}",
            a.rows,
            a.cols,
            b.rows,
            b.cols
        ));
    }

    let mut result = vec![T::default(); a.rows * b.cols];
    for i in 0..a.rows {
        for j in 0..b.cols {
            for k in 0..a.cols {
                result[i * b.cols + j] += a.data[i * a.cols + k] * b.data[k * b.cols + j];
            }
        }
    }

    Ok(Matrix {
        rows: a.rows,
        cols: b.cols,
        data: result,
    })
}

impl<T: fmt::Debug> Matrix<T> {
    pub fn new(rows: usize, cols: usize, data: impl Into<Vec<T>>) -> Self {
        Self {
            rows,
            cols,
            data: data.into(),
        }
    }
}

impl<T: fmt::Debug> fmt::Display for Matrix<T> {
    // display a 2*3 as {1 2 3, 4 5 6}, 3*2 as {1 2, 3 4, 5 6}
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.rows {
            write!(f, "{{")?;
            for j in 0..self.cols {
                write!(f, "{:?}", self.data[i * self.cols + j])?;
                if j < self.cols - 1 {
                    write!(f, " ")?;
                }
            }
            write!(f, "}}")?;
            if i < self.rows - 1 {
                write!(f, ", ")?;
            }
        }
        Ok(())
    }
}

impl<T> fmt::Debug for Matrix<T>
where
    T: fmt::Display + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Matrix(rows={}, cols={}, {})",
            self.rows, self.cols, self
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_display() {
        let m = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]);
        assert_eq!("{1 2 3}, {4 5 6}", format!("{}", m));
    }

    #[test]
    fn test_matrix_debug() {
        let m = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(
            "Matrix(rows=2, cols=3, {1 2 3}, {4 5 6})",
            format!("{:?}", m)
        );
    }

    #[test]
    fn test_matrix_multiply() -> Result<()> {
        let a = Matrix::new(2, 3, [1, 2, 3, 4, 5, 6]);
        let b = Matrix::new(3, 2, [1, 2, 3, 4, 5, 6]);
        let c = multiply(&a, &b)?;
        assert_eq!("{22 28}, {49 64}", format!("{}", c));
        Ok(())
    }
}
