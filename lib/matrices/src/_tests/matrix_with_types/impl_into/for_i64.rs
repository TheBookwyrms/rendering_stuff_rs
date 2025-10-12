use crate::_tests::matrix_with_types::matrix::Matrix;
use crate::_tests::matrix_with_types::numbers::DataTypes;






impl Into<Matrix<i128>> for Matrix<i64>{
    fn into(self) -> Matrix<i128> {
        let narr = (0..self.array.len()).map(|i| self.array[i] as i128).collect::<Vec<_>>();
        Matrix { shape:self.shape, array:narr, dtype:DataTypes::I128 }
    }
}



impl Into<Matrix<f32>> for Matrix<i64>{
    fn into(self) -> Matrix<f32> {
        let narr = (0..self.array.len()).map(|i| self.array[i] as f32).collect::<Vec<_>>();
        Matrix { shape:self.shape, array:narr, dtype:DataTypes::F32 }
    }
}

impl Into<Matrix<f64>> for Matrix<i64>{
    fn into(self) -> Matrix<f64> {
        let narr = (0..self.array.len()).map(|i| self.array[i] as f64).collect::<Vec<_>>();
        Matrix { shape:self.shape, array:narr, dtype:DataTypes::F64 }
    }
}