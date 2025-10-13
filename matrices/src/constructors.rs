use crate::matrix::Matrix;
use crate::numbers::DataTypes;
use crate::type_conversions::IntoDataType;
use crate::errors::MatrixError;

impl<T:IntoDataType + Clone> Matrix<T> {

    pub fn new_empty(shape:Vec<usize>) -> Matrix<T> {
        Matrix { shape, array: vec![], dtype:DataTypes::EMPTY }
    }

    pub fn from_scalar(f:T) -> Matrix<T> {
        let dtype = f.as_dtype();
        Matrix {shape:vec![1], array:vec![f], dtype:dtype}
    }

    pub fn from_1darray<const M:usize>(arr:[T;M]) -> Matrix<T> {
        let dtype = arr[0].as_dtype();
        Matrix {shape:vec![arr.len()], array:arr.to_vec(), dtype}
    }

    pub fn from_vec(vec:Vec<T>) -> Matrix<T> {
        let dtype = vec[0].as_dtype();
        Matrix {shape:vec![vec.len()], array: vec, dtype}
    }

    pub fn from_vec_of_vec(vec:Vec<Vec<T>>) -> Result<Matrix<T>, MatrixError> {
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

    pub fn from_2darray<const M:usize, const N:usize>(arr:[[T;M];N]) -> Matrix<T> {
        let dtype = arr[0][0].as_dtype();
        let mut data = vec![];
        for row in arr {
            data.extend(row);
        }
        Matrix {shape:vec![M, N], array:data, dtype}
    }

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