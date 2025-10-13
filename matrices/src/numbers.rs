
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


/// numerical trait for numerical data types that are valid in matrix
pub trait Numerical {}
impl Numerical for u8 {}
impl Numerical for u16 {}
impl Numerical for u32 {}
impl Numerical for usize {}
impl Numerical for u64 {}
impl Numerical for u128 {}

impl Numerical for i8 {}
impl Numerical for i16 {}
impl Numerical for i32 {}
impl Numerical for isize {}
impl Numerical for i64 {}
impl Numerical for i128 {}

impl Numerical for f32 {}
impl Numerical for f64 {}