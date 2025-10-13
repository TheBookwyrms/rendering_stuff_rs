use crate::matrix::Matrix;
use crate::numbers::DataTypes;


impl From<Matrix<&str>> for Matrix<String> {
    fn from(mat:Matrix<&str>) -> Matrix<String> {
        let narr = (0..mat.array.len()).map(|i| mat.array[i].to_string()).collect::<Vec<_>>();
        Matrix { shape:mat.shape, array:narr, dtype:DataTypes::STRING }
    }
}