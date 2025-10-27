use crate::enums::DataTypes;

#[derive(Debug, Clone)]
/// struct to hold matrices of (mostly) arbitrary type T
pub struct Matrix<T> {
    /// shape of the matrix
    /// goes from inner to outer dimensions
    /// ex: ncols before nrows for 2D matrix
    pub shape:Vec<usize>,

    /// 1D container for the n-dimensional matrix
    pub array:Vec<T>,

    /// datatype of the matrix
    pub dtype:DataTypes,
}