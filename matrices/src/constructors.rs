use crate::matrix::Matrix;
use crate::traits::{IntoDataType, Float};
use crate::enums::DataTypes;
use crate::enums::MatrixError;

impl<T:Clone + Float> Matrix<T> {
    /// returns an identity matrix of order N
    pub fn identity(order:usize) -> Matrix<T> {
        let arr = vec![T::zero(); order*order];
        let mut identity_mat = Matrix {shape:vec![order, order], array:arr, dtype:T::as_dtype()};
        for i in 0..order {
            identity_mat[[i, i]] = T::one();
        }
        identity_mat
    }

    /// returns a null matrix of shape [usize; k]
    pub fn null<const K:usize>(shape:[usize; K]) -> Matrix<T> {
        let arr = vec![T::zero(); shape.iter().product()];
        Matrix { shape: shape.to_vec(), array: arr, dtype: T::as_dtype() }
    }
    /// returns a null matrix of shape given by by Vec<usize>
    pub fn null_from_vec(shape:Vec<usize>) -> Matrix<T> {
        let arr = vec![T::zero(); shape.iter().product()];
        Matrix { shape: shape.to_vec(), array: arr, dtype: T::as_dtype() }
    }
}

impl<T:IntoDataType + Clone> Matrix<T> {

    /// creates an empty matrix of given shape
    pub fn new_empty(shape:Vec<usize>) -> Matrix<T> {
        Matrix { shape, array: vec![], dtype:DataTypes::EMPTY }
    }

    /// creates matrix from a scalar value
    pub fn from_scalar(f:T) -> Matrix<T> {
        let dtype = f.as_dtype();
        Matrix {shape:vec![1], array:vec![f], dtype:dtype}
    }

    /// creates a 1-dimensional matrix from an array
    pub fn from_1darray<const M:usize>(arr:[T;M]) -> Matrix<T> {
        let dtype = arr[0].as_dtype();
        Matrix {shape:vec![arr.len()], array:arr.to_vec(), dtype}
    }

    /// creates a 1-dimensional matrix from a vec
    pub fn from_vec(vec:Vec<T>) -> Matrix<T> {
        let dtype = vec[0].as_dtype();
        Matrix {shape:vec![vec.len()], array: vec, dtype}
    }

    /// creates a 2-dimensional matrix from a vec of vecs
    pub fn from_vec_of_vec(vec:Vec<Vec<T>>) -> Result<Matrix<T>, MatrixError<T>> {
        let dtype = vec[0][0].as_dtype();
        let mut homogenous_rows = true;
        let mut row_lengths = vec![];
        let mut data : Vec<T> = vec![];
        for row in vec.clone() {
            row_lengths.push(row.len());
            if row.len() != vec[0].len() {
                homogenous_rows = false;
            }
            data.extend(row);
        }
        match homogenous_rows {
            true => Ok( Matrix { shape:vec![vec[0].len(), vec.len()], array:data, dtype } ),
            false => Err(MatrixError::InhomogenousLength(row_lengths)),
        }
    }

    /// creates a 2-dimensional matrix from an array of arrays
    pub fn from_2darray<const M:usize, const N:usize>(arr:[[T;M];N]) -> Matrix<T> {
        let dtype = arr[0][0].as_dtype();
        let mut data = vec![];
        for row in arr {
            data.extend(row);
        }
        Matrix {shape:vec![M, N], array:data, dtype}
    }

    /// creates a 3-dimensional matrix from an array of arrays of arrays
    pub fn from_3darray<const M:usize, const N:usize, const O:usize>(arr:[[[T;M];N];O]) -> Matrix<T> {
        let dtype = arr[0][0][0].as_dtype();
        let mut data = vec![];
        for ax1 in arr {
            for ax2 in ax1 {
                data.extend(ax2);
            }
        }
        Matrix {shape:vec![M, N, O], array:data, dtype}
    }
}