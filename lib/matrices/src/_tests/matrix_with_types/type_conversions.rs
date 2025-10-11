use crate::_tests::matrix_with_types::numbers::DataTypes;

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

//pub fn turn_T_into_datatype<T>(t:T) {
//    match T {
//        usize => {},
//        u8 => {},
//        u16 => {},
//        u32 => {},
//        u64 => {},
//        u128 => {},
//        isize => {},
//        i8 => {},
//        i16 => {},
//        i32 => {},
//        i64 => {},
//        i128 => {},
//        f32 => {},
//        f64 => {},
//    }
//}