use crate::matrix::Matrix;


impl PartialEq for Matrix<f64> {
    fn eq(&self, other: &Self) -> bool {
        let shape_eq = self.shape==other.shape;
        if !shape_eq {
            return false
        }
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