use crate::_tests::matrix_with_types::matrix::Matrix;
use crate::_tests::matrix_with_types::numbers::DataTypes;


impl Into<Matrix<String>> for Matrix<&str>{
    fn into(self) -> Matrix<String> {
        let narr = (0..self.array.len()).map(|i| self.array[i].to_string()).collect::<Vec<_>>();
        Matrix { shape:self.shape, array:narr, dtype:DataTypes::STRING }
    }
}