use crate::enums::DataTypes;

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

/// trait representing floats
pub trait Float {
    fn is_zero(&self) -> bool;
    fn usize_to_t(u:usize) -> Self;
    fn zero() -> Self;
    fn one() -> Self;
    fn powf(base:Self, exponent:Self) -> Self;
    fn as_dtype() -> DataTypes;
}
impl Float for f32 {
    fn is_zero(&self) -> bool { self==&0.0 }
    fn usize_to_t(u:usize) -> Self {f32::from(u16::try_from(u32::try_from(u).unwrap()).unwrap())}
    fn zero() -> Self {0.0}
    fn one() -> Self {1.0}
    fn powf(base:f32, exponent:f32) -> Self {base.powf(exponent)}
    fn as_dtype() -> DataTypes {DataTypes::F32}
}
impl Float for f64 {
    fn is_zero(&self) -> bool { self==&0.0 }
    fn usize_to_t(u:usize) -> Self {f64::from(u16::try_from(u32::try_from(u).unwrap()).unwrap())}
    fn zero() -> Self {0.0}
    fn one() -> Self {1.0}
    fn powf(base:f64, exponent:f64) -> Self {base.powf(exponent)}
    fn as_dtype() -> DataTypes {DataTypes::F64}
}


/// trait converting matrix types into DataTypes for ease of handling
pub trait IntoDataType {
    fn as_dtype(&self) -> DataTypes;
}

impl IntoDataType for usize  {fn as_dtype(&self) -> DataTypes {DataTypes::USIZE}}
impl IntoDataType for u8     {fn as_dtype(&self) -> DataTypes {DataTypes::U8}}
impl IntoDataType for u16    {fn as_dtype(&self) -> DataTypes {DataTypes::U16}}
impl IntoDataType for u32    {fn as_dtype(&self) -> DataTypes {DataTypes::U32}}
impl IntoDataType for u64    {fn as_dtype(&self) -> DataTypes {DataTypes::U64}}
impl IntoDataType for u128   {fn as_dtype(&self) -> DataTypes {DataTypes::U128}}
impl IntoDataType for isize  {fn as_dtype(&self) -> DataTypes {DataTypes::ISIZE}}
impl IntoDataType for i8     {fn as_dtype(&self) -> DataTypes {DataTypes::I8}}
impl IntoDataType for i16    {fn as_dtype(&self) -> DataTypes {DataTypes::I16}}
impl IntoDataType for i32    {fn as_dtype(&self) -> DataTypes {DataTypes::I32}}
impl IntoDataType for i64    {fn as_dtype(&self) -> DataTypes {DataTypes::I64}}
impl IntoDataType for i128   {fn as_dtype(&self) -> DataTypes {DataTypes::I128}}
impl IntoDataType for f32    {fn as_dtype(&self) -> DataTypes {DataTypes::F32}}
impl IntoDataType for f64    {fn as_dtype(&self) -> DataTypes {DataTypes::F64}}
impl IntoDataType for str    {fn as_dtype(&self) -> DataTypes {DataTypes::STR}}
impl IntoDataType for String {fn as_dtype(&self) -> DataTypes {DataTypes::STRING}}
impl IntoDataType for bool   {fn as_dtype(&self) -> DataTypes {DataTypes::BOOL}}