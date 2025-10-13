use crate::matrix::Matrix;
use crate::numbers::DataTypes;



impl From<Matrix<f32>> for Matrix<f64>{
    fn from(mat:Matrix<f32>) -> Matrix<f64> {
        let narr = (0..mat.array.len()).map(|i| mat.array[i] as f64).collect::<Vec<_>>();
        Matrix { shape:mat.shape, array:narr, dtype:DataTypes::F64 }
    }
}