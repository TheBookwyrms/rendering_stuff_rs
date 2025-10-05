#[derive(Debug)]
pub enum VectorError {
    Invalidlengths([usize; 2]),
}


#[derive(Debug)]
pub enum MatrixError {
    InvalidShape([usize; 2]),
    InvalidShapes([[usize; 2]; 2]),
    InhomogenousLength(Vec<usize>),
    InvalidIndex(usize),
    InvalidIndices([usize; 2]),
    DeterminantIsZero,
}