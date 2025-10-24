use crate::numbers::DataTypes;

#[derive(Debug)]
pub enum MatrixError {
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
    MatrixSolveError((bool, bool, bool)),
}