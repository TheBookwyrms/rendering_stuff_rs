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
}