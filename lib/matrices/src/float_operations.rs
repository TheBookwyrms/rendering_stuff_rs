use crate::matrix::Matrix;
use crate::numbers::DataTypes;
use crate::errors::MatrixError;


impl Matrix<f32> {
    
    pub fn minor(&self, row_i:usize, col_j:usize) -> Result<f32, MatrixError> {
        if self.ndims() != 2 {
            Err(MatrixError::InvalidDimension(self.ndims()))
        } else if self.shape[0] != self.shape[1] {
            Err(MatrixError::InvalidShape(self.shape.clone()))
        } else {
            let minor = self.without_rc(row_i, col_j)?.determinant();
            minor
        }
    }

    pub fn cofactor(&self, row_i:usize, col_j:usize) -> Result<f32, MatrixError> {
        if self.ndims() != 2 {
            Err(MatrixError::InvalidDimension(self.ndims()))
        } else if self.shape[0] != self.shape[1] {
            Err(MatrixError::InvalidShape(self.shape.clone()))
        } else {
            let minor = self.without_rc(row_i, col_j)?.determinant()?;

            let r = f32::from(u16::try_from(u32::try_from(row_i).unwrap()).unwrap());
            let c = f32::from(u16::try_from(u32::try_from(col_j).unwrap()).unwrap());

            let cofactor = (-1.0_f32).powf((r+1.0)+(c+1.0)) * minor;
            Ok(cofactor)
        }
    }

    pub fn determinant(&self) -> Result<f32, MatrixError> {
        if self.ndims() != 2 {
            Err(MatrixError::InvalidDimension(self.ndims()))
        } else if self.shape[0] != self.shape[1] {
            Err(MatrixError::InvalidShape(self.shape.clone()))
        } else if self.shape[0] == 2 {
            let a = self[[0, 0]];
            let b = self[[0, 1]];
            let c = self[[1, 0]];
            let d = self[[1, 1]];
            Ok(a*d - b*c)
        } else {
            let row_i = 0;
            let mut determinant_sum = 0.0;


            let row0_len = self.shape[0];
            let mut row_0 = vec![];
            for i in 0..row0_len {
                row_0.push(self.array[i]);
            }



            //for (col_j, col_val) in row_0.clone().into_iter().enumerate() {
            for (col_j, col_val) in row_0.iter().enumerate() {
                //let minor = self.without_rc(row_i, col_j)?.determinant()?;
                //let r = f32::from(u16::try_from(u32::try_from(row_i).unwrap()).unwrap());
                //let c = f32::from(u16::try_from(u32::try_from(col_j).unwrap()).unwrap());
                //let cofactor = (-1.0_f32).powf((r+1.0)+(c+1.0)) * minor;
                let cofactor = self.cofactor(row_i, col_j)?;
                determinant_sum += col_val * cofactor;
            }
            Ok(determinant_sum)
        }
    }

    pub fn cofactor_matrix(&self) -> Result<Matrix<f32>, MatrixError> {
        if self.ndims() != 2 {
            Err(MatrixError::InvalidDimension(self.ndims()))
        }  else if self.shape[0] != self.shape[1] {
            Err(MatrixError::InvalidShape(self.shape.clone()))
        } else {
            let mut cofactor_matrix = self.array.clone();
            for i in 0..cofactor_matrix.len() {
                let indices = self.indices_of(i);
                cofactor_matrix[i] = self.cofactor(indices[0], indices[1])?;
            }

            // transposed because of swapped linear algebra indexing conventions
            Matrix {shape:self.shape.clone(), array:cofactor_matrix, dtype:DataTypes::F32}.transpose()
        }
    }

    pub fn inverse(&self) -> Result<Matrix<f32>, MatrixError> {
        let determinant = self.determinant()?;
        match determinant {
            0.0 => Err(MatrixError::DeterminantIsZero),
            _ => Ok(self.cofactor_matrix()?.transpose()?.multiply_by_constant(1.0/determinant)),
        }
    }

}