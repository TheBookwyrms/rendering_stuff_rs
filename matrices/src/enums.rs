use crate::{matrix::Matrix};

#[derive(Debug)]
/// enumerates possible errors originating due to Matrix implementations
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


#[derive(Debug, Clone, Copy, PartialEq)]
/// data type enum to hold Matrix data types
pub enum DataTypes {
    USIZE,
    ISIZE,

    U8,
    U16,
    U32,
    U64,
    U128,

    I8,
    I16,
    I32,
    I64,
    I128,

    F32,
    F64,

    STR,
    STRING,
    BOOL,

    EMPTY,
}
