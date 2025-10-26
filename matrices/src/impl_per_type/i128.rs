use crate::matrix::Matrix;
use crate::enums::DataTypes;




impl PartialEq for Matrix<i128> {
    fn eq(&self, other: &Self) -> bool {
        self.shape==other.shape && self.array==other.array && self.dtype==other.dtype
    }
}



impl From<Matrix<i128>> for Matrix<f64>{
    fn from(mat:Matrix<i128>) -> Matrix<f64> {
        let narr = (0..mat.array.len()).map(|i| mat.array[i] as f64).collect::<Vec<_>>();
        Matrix { shape:mat.shape, array:narr, dtype:DataTypes::F64 }
    }
}