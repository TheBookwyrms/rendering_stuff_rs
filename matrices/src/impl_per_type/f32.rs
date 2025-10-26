use crate::matrix::Matrix;
use crate::enums::DataTypes;

impl PartialEq for Matrix<f32> {
    fn eq(&self, other: &Self) -> bool {
        let shape_eq = self.shape==other.shape;
        let dtype_eq = self.dtype==other.dtype;
        let epsilon = 0.00001;
        let mut val_eq = vec![false;self.array.len()];
        for i in 0..self.array.len() {
            let above_lower_bound = self.array[i]-epsilon < other.array[i];
            let under_upper_bound = self.array[i]+epsilon > other.array[i];
            val_eq[i] = above_lower_bound && under_upper_bound;
        }
        shape_eq && dtype_eq && !val_eq.contains(&false)
    }
}

impl From<Matrix<f32>> for Matrix<f64>{
    fn from(mat:Matrix<f32>) -> Matrix<f64> {
        let narr = (0..mat.array.len()).map(|i| mat.array[i] as f64).collect::<Vec<_>>();
        Matrix { shape:mat.shape, array:narr, dtype:DataTypes::F64 }
    }
}