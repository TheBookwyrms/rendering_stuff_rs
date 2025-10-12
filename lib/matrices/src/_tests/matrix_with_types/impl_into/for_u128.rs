use crate::_tests::matrix_with_types::matrix::Matrix;
use crate::_tests::matrix_with_types::numbers::DataTypes;






impl Into<Matrix<f64>> for Matrix<u128>{
    fn into(self) -> Matrix<f64> {
        let narr = (0..self.array.len()).map(|i| self.array[i] as f64).collect::<Vec<_>>();
        Matrix { shape:self.shape, array:narr, dtype:DataTypes::F64 }
    }
}