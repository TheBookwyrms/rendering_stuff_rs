use crate::{matrix::Matrix, numbers::DataTypes};

#[derive(Debug)]
pub enum MatrixError<T> {
    InvalidShape(Vec<usize>),
    InvalidShapes([Vec<usize>; 2]),
    InvalidDimension(usize),
    InvalidDimensions([usize; 2]),
    InhomogenousLength(Vec<usize>),
    InvalidIndex(usize),
    InvalidIndices(Vec<usize>),
    DeterminantIsZero,
    Invalidlengths([usize; 2]),
    InvalidDataTypes([DataTypes;2]),
    AugmentedMatrixShapeError,
    InvalidExpansionLength((Vec<usize>, usize)),
    MatrixSolveError((bool, bool)),
    InvalidBounds,
    ExpansionAxisOrDimensionsNotImplemented((usize, usize)),
    MatrixNotInversible(Matrix<T>)
}