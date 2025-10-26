use crate::matrix::Matrix;
use crate::enums::DataTypes;



impl PartialEq for Matrix<i64> {
    fn eq(&self, other: &Self) -> bool {
        self.shape==other.shape && self.array==other.array && self.dtype==other.dtype
    }
}



impl From<Matrix<i64>> for Matrix<i128>{
    fn from(mat:Matrix<i64>) -> Matrix<i128> {
        let narr = (0..mat.array.len()).map(|i| mat.array[i] as i128).collect::<Vec<_>>();
        Matrix { shape:mat.shape, array:narr, dtype:DataTypes::I128 }
    }
}

impl From<Matrix<i64>> for Matrix<f32>{
    fn from(mat:Matrix<i64>) -> Matrix<f32> {
        let narr = (0..mat.array.len()).map(|i| mat.array[i] as f32).collect::<Vec<_>>();
        Matrix { shape:mat.shape, array:narr, dtype:DataTypes::F32 }
    }
}

impl From<Matrix<i64>> for Matrix<f64>{
    fn from(mat:Matrix<i64>) -> Matrix<f64> {
        let narr = (0..mat.array.len()).map(|i| mat.array[i] as f64).collect::<Vec<_>>();
        Matrix { shape:mat.shape, array:narr, dtype:DataTypes::F64 }
    }
}