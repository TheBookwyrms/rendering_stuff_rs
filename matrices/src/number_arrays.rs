use crate::matrix::Matrix;
use crate::traits::{IntoDataType, Numerical};
use crate::enums::MatrixError;
use std::fmt::Debug;
use std::iter::Sum;
use std::ops::{Add, Mul, MulAssign, Sub};

impl<T:Clone+IntoDataType+Add<Output = T>> Add for Matrix<T> {
    type Output = Result<Matrix<T>, MatrixError<T>>;

    /// add two matrices together element-wise
    fn add(self, other: Self) -> Result<Matrix<T>, MatrixError<T>> {
        if self.ndims() != other.ndims() {
            Err(MatrixError::InvalidDimensions([self.ndims(), other.ndims()]))
        } else if self.dtype != other.dtype {
            Err(MatrixError::InvalidDataTypes([self.dtype, other.dtype]))
        } else if self.shape != other.shape {
            Err(MatrixError::InvalidShapes([self.shape, other.shape]))
        } else {

            let mut v = self.array.clone();
            v.iter_mut()
             .zip(self.array.into_iter())
             .zip(other.array)
             .for_each(|((a1, a2), a3)| *a1=a2+a3);
            
            Ok(Matrix {shape:self.shape, array:v, dtype:self.dtype})
        }
    }
}

impl<T:Clone+IntoDataType+Sub<Output = T>> Sub for Matrix<T> {
    type Output = Result<Self, MatrixError<T>>;

    /// subtracts two matrices element-wise
    fn sub(self, other: Self) -> Result<Self, MatrixError<T>> {
        if self.ndims() != other.ndims() {
            Err(MatrixError::InvalidDimensions([self.ndims(), other.ndims()]))
        } else if self.dtype != other.dtype {
            Err(MatrixError::InvalidDataTypes([self.dtype, other.dtype]))
        } else if self.shape != other.shape {
            Err(MatrixError::InvalidShapes([self.shape, other.shape]))
        } else {

            let mut v = self.array.clone();
            v.iter_mut()
             .zip(self.array.into_iter())
             .zip(other.array)
             .for_each(|((a1, a2), a3)| *a1=a2-a3);

            Ok(Matrix {shape:self.shape, array:v, dtype:self.dtype})
        }
    }
}

impl<T:IntoDataType + Clone + Numerical + Mul<Output=T> + Sum + MulAssign + Debug> Matrix<T> {
    
    /// performs the dot product of two vectors (1D matrices) 
    pub fn dot(&self, other:&Self) -> Result<T, MatrixError<T>> {
        if (self.ndims() != 1) || (other.ndims() != 1) {
            Err(MatrixError::InvalidDimensions([self.ndims(), other.ndims()]))
        } else if self.array.len() != other.array.len() {
            Err(MatrixError::Invalidlengths([self.array.len(), other.array.len()]))
        } else {
            let mut sums = self.array.clone();
            sums.iter_mut()
             .zip(self.array.clone())
             .zip(other.array.clone())
             .for_each(|((a1, a2), a3)| *a1=a2*a3);

            Ok(sums.into_iter().sum())
        }
    }

    /// performs the matrix multiplication of 2 2D matrices
    pub fn matmul(&self, other:&Self) -> Result<Matrix<T>, MatrixError<T>> {
        if (self.ndims() != 2) || (other.ndims() != 2) {
            Err(MatrixError::InvalidDimensions([self.ndims(), other.ndims()]))
        } else if !(self.shape[0]==other.shape[1]) {
            Err(MatrixError::InvalidShapes([self.shape.clone(), other.shape.clone()]))
        } else if self.dtype != other.dtype {
            Err(MatrixError::InvalidDataTypes([self.dtype, other.dtype]))
        } else {
            let mut rows = vec![];
            for r in 0..self.shape[1] {
                let mut this_row = vec![];
                let row = self.get_row(r)?;

                for c in 0..other.shape[1] {
                    let col = &other.clone().get_col(c)?;
                    this_row.push(row.dot(&col)?);
                }
                rows.extend(this_row);
            }

            Ok(Matrix {shape:vec![other.shape[0], self.shape[1]], array:rows, dtype:self.dtype})
        }
    }

    /// multiplies every element of an n-dimensional matrix by a scalar value
    pub fn multiply_by_constant(&self, scalar:T) -> Matrix<T> {
        let mut narr = self.array.clone();
        (0..self.array.len()).for_each(|i| narr[i] *= scalar.clone());
        Matrix {shape:self.shape.clone(), array:narr, dtype:self.dtype}
    }
}