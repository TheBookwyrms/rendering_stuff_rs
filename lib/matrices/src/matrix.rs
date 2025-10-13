use crate::numbers::DataTypes;

#[derive(Debug, Clone)]
pub struct Matrix<T> {
    pub shape:Vec<usize>, // goes from inner to outer dimensions (ex ncols before nrows for 2D)
    pub array:Vec<T>,
    pub dtype:DataTypes,
}