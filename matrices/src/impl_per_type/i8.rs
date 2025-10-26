use crate::matrix::Matrix;
use crate::enums::DataTypes;


impl PartialEq for Matrix<i8> {
    fn eq(&self, other: &Self) -> bool {
        self.shape==other.shape && self.array==other.array && self.dtype==other.dtype
    }
}

impl From<Matrix<i8>> for Matrix<i16> {
    fn from(mat:Matrix<i8>) -> Matrix<i16> {
        let narr = (0..mat.array.len()).map(|i| mat.array[i] as i16).collect::<Vec<_>>();
        Matrix { shape:mat.shape, array:narr, dtype:DataTypes::I16 }
    }
}

impl From<Matrix<i8>> for Matrix<i32> {
    fn from(mat:Matrix<i8>) -> Matrix<i32> {
        let narr = (0..mat.array.len()).map(|i| mat.array[i] as i32).collect::<Vec<_>>();
        Matrix { shape:mat.shape, array:narr, dtype:DataTypes::I32 }
    }
}

impl From<Matrix<i8>> for Matrix<i64> {
    fn from(mat:Matrix<i8>) -> Matrix<i64> {
        let narr = (0..mat.array.len()).map(|i| mat.array[i] as i64).collect::<Vec<_>>();
        Matrix { shape:mat.shape, array:narr, dtype:DataTypes::I64 }
    }
}

impl From<Matrix<i8>> for Matrix<i128> {
    fn from(mat:Matrix<i8>) -> Matrix<i128> {
        let narr = (0..mat.array.len()).map(|i| mat.array[i] as i128).collect::<Vec<_>>();
        Matrix { shape:mat.shape, array:narr, dtype:DataTypes::I128 }
    }
}


impl From<Matrix<i8>> for Matrix<isize> {
    fn from(mat:Matrix<i8>) -> Matrix<isize> {
        let narr = (0..mat.array.len()).map(|i| mat.array[i] as isize).collect::<Vec<_>>();
        Matrix { shape:mat.shape, array:narr, dtype:DataTypes::ISIZE }
    }
}

impl From<Matrix<i8>> for Matrix<f32> {
    fn from(mat:Matrix<i8>) -> Matrix<f32> {
        let narr = (0..mat.array.len()).map(|i| mat.array[i] as f32).collect::<Vec<_>>();
        Matrix { shape:mat.shape, array:narr, dtype:DataTypes::F32 }
    }
}

impl From<Matrix<i8>> for Matrix<f64> {
    fn from(mat:Matrix<i8>) -> Matrix<f64> {
        let narr = (0..mat.array.len()).map(|i| mat.array[i] as f64).collect::<Vec<_>>();
        Matrix { shape:mat.shape, array:narr, dtype:DataTypes::F64 }
    }
}