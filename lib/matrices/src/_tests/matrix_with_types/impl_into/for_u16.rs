use crate::_tests::matrix_with_types::matrix::Matrix;
use crate::_tests::matrix_with_types::numbers::DataTypes;



impl Into<Matrix<u32>> for Matrix<u16>{
    fn into(self) -> Matrix<u32> {
        let narr = (0..self.array.len()).map(|i| self.array[i] as u32).collect::<Vec<_>>();
        Matrix { shape:self.shape, array:narr, dtype:DataTypes::U32 }
    }
}

impl Into<Matrix<u64>> for Matrix<u16>{
    fn into(self) -> Matrix<u64> {
        let narr = (0..self.array.len()).map(|i| self.array[i] as u64).collect::<Vec<_>>();
        Matrix { shape:self.shape, array:narr, dtype:DataTypes::U64 }
    }
}

impl Into<Matrix<u128>> for Matrix<u16>{
    fn into(self) -> Matrix<u128> {
        let narr = (0..self.array.len()).map(|i| self.array[i] as u128).collect::<Vec<_>>();
        Matrix { shape:self.shape, array:narr, dtype:DataTypes::U128 }
    }
}

impl Into<Matrix<i32>> for Matrix<u16>{
    fn into(self) -> Matrix<i32> {
        let narr = (0..self.array.len()).map(|i| self.array[i] as i32).collect::<Vec<_>>();
        Matrix { shape:self.shape, array:narr, dtype:DataTypes::I32 }
    }
}

impl Into<Matrix<i64>> for Matrix<u16>{
    fn into(self) -> Matrix<i64> {
        let narr = (0..self.array.len()).map(|i| self.array[i] as i64).collect::<Vec<_>>();
        Matrix { shape:self.shape, array:narr, dtype:DataTypes::I64 }
    }
}

impl Into<Matrix<i128>> for Matrix<u16>{
    fn into(self) -> Matrix<i128> {
        let narr = (0..self.array.len()).map(|i| self.array[i] as i128).collect::<Vec<_>>();
        Matrix { shape:self.shape, array:narr, dtype:DataTypes::I128 }
    }
}

impl Into<Matrix<usize>> for Matrix<u16>{
    fn into(self) -> Matrix<usize> {
        let narr = (0..self.array.len()).map(|i| self.array[i] as usize).collect::<Vec<_>>();
        Matrix { shape:self.shape, array:narr, dtype:DataTypes::USIZE }
    }
}

impl Into<Matrix<isize>> for Matrix<u16>{
    fn into(self) -> Matrix<isize> {
        let narr = (0..self.array.len()).map(|i| self.array[i] as isize).collect::<Vec<_>>();
        Matrix { shape:self.shape, array:narr, dtype:DataTypes::ISIZE }
    }
}

impl Into<Matrix<f32>> for Matrix<u16>{
    fn into(self) -> Matrix<f32> {
        let narr = (0..self.array.len()).map(|i| self.array[i] as f32).collect::<Vec<_>>();
        Matrix { shape:self.shape, array:narr, dtype:DataTypes::F32 }
    }
}

impl Into<Matrix<f64>> for Matrix<u16>{
    fn into(self) -> Matrix<f64> {
        let narr = (0..self.array.len()).map(|i| self.array[i] as f64).collect::<Vec<_>>();
        Matrix { shape:self.shape, array:narr, dtype:DataTypes::F64 }
    }
}