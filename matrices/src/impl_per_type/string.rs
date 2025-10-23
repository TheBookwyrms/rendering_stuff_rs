use crate::matrix::Matrix;


impl PartialEq for Matrix<String> {
    fn eq(&self, other: &Self) -> bool {
        self.shape==other.shape && self.array==other.array && self.dtype==other.dtype
    }
}