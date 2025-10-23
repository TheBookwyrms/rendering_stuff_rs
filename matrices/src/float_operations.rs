use std::fmt::{Debug, Display};
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::matrix::Matrix;
use crate::numbers::{DataTypes, Float, Numerical};
use crate::errors::MatrixError;
use crate::type_conversions::IntoDataType;

// where <T as Mul>::Output: Sub

impl<T:
    Float + Clone + IntoDataType + Display + Debug + PartialEq
    + SubAssign + MulAssign + AddAssign + Mul + Sub + Numerical + Add<Output = T>
    + Mul<Output = T> + Sub<Output = T> + Sum + Neg<Output = T> + Div<Output = T>
    //+ Deref<Target = T>
    // + Mul + Sub
    //> Matrix<T> where <T as Mul>::Output: Sub {
    > Matrix<T> {

//}
//impl Matrix<f32> {
    
    pub fn minor(&self, row_i:usize, col_j:usize) -> Result<T, MatrixError> {
        if self.ndims() != 2 {
            Err(MatrixError::InvalidDimension(self.ndims()))
        } else if self.shape[0] != self.shape[1] {
            Err(MatrixError::InvalidShape(self.shape.clone()))
        } else {
            let minor = self.without_rc(row_i, col_j)?.determinant();
            minor
        }
    }

    pub fn cofactor(&self, row_i:usize, col_j:usize) -> Result<T, MatrixError> {
        if self.ndims() != 2 {
            Err(MatrixError::InvalidDimension(self.ndims()))
        } else if self.shape[0] != self.shape[1] {
            Err(MatrixError::InvalidShape(self.shape.clone()))
        } else {
            let minor = self.without_rc(row_i, col_j)?.determinant()?;

            let r = T::usize_to_t(row_i);
            let c = T::usize_to_t(col_j);

            //let r = f32::from(u16::try_from(u32::try_from(row_i).unwrap()).unwrap());
            //let c = f32::from(u16::try_from(u32::try_from(col_j).unwrap()).unwrap());

            let n1 = -T::one();
            let cofactor = T::powf(n1, (r+T::one())+(c+T::one())) * minor;
            //let cofactor = n1.powf((r+T::one())+(c+T::one())) * minor;
            Ok(cofactor)
        }
    }

    pub fn determinant(&self) -> Result<T, MatrixError> {
        if self.ndims() != 2 {
            Err(MatrixError::InvalidDimension(self.ndims()))
        } else if self.shape[0] != self.shape[1] {
            Err(MatrixError::InvalidShape(self.shape.clone()))
        } else if self.shape[0] == 2 {
            let a = self[[0, 0]].clone();
            let b = self[[0, 1]].clone();
            let c = self[[1, 0]].clone();
            let d = self[[1, 1]].clone();
            Ok(a*d - b*c)
        } else {
            let row_i = 0;
            let mut determinant_sum = T::zero();

            for (col_j, col_val) in self.array[0..self.shape[0]].iter().enumerate() {
                let cofactor = self.cofactor(row_i, col_j)?;
                determinant_sum += col_val.clone() * cofactor;
            }
            Ok(determinant_sum)
        }
    }

    pub fn cofactor_matrix(&self) -> Result<Matrix<T>, MatrixError> {
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

    pub fn inverse(&self) -> Result<Matrix<T>, MatrixError> {
        let determinant = self.determinant()?;
        if determinant == T::zero() {
            Err(MatrixError::DeterminantIsZero)
        } else {
            Ok(self.cofactor_matrix()?.transpose()?.multiply_by_constant(T::one()/determinant))
        }
        //match determinant {
        //    //0.0 => Err(MatrixError::DeterminantIsZero),
        //    z => Err(MatrixError::DeterminantIsZero),
        //    _ => Ok(self.cofactor_matrix()?.transpose()?.multiply_by_constant(T::one()/determinant)),
        //}
    }

    pub fn col_is_nul(&self, col_j:usize) -> Result<bool, MatrixError> {
        if self.ndims() == 2 {
            let column = self.get_col(col_j)?;
            let zeroes = (0..column.array.len()).map(|i| column.array[i].is_zero()).all(|b| b==true);
            //println!("{:?}", column);
            //println!("{:?}", (0..column.array.len()).map(|i| column.array[i]==0.0).collect::<Vec<bool>>());
            //println!("{:?}", (0..column.array.len()).map(|i| column.array[i]==0.0).all(|b| b==true));
            //println!("");
            Ok(zeroes)
        } else {
            Err(MatrixError::InvalidDimension(self.ndims()))
        }
    }


    pub fn get_echelon_form_of_via_gaussian_elimination(&self) -> Result<Matrix<T>, MatrixError> {
        if self.ndims() != 2 {
            Err(MatrixError::InvalidDimension(self.ndims()))
        } else if self.shape[0] != self.shape[1]+1 {
            Err(MatrixError::AugmentedMatrixShapeError)
        } else {
            let mut mat = self.clone();
            for base_adjustment in 0..mat.shape[1] {
                for row_i in base_adjustment..mat.shape[1] {

                    let leftmost_value = mat[[base_adjustment, row_i]].clone();

            //println!("");
            //println!("{}", leftmost_value);
            //println!("{}", mat);

                    if leftmost_value != T::zero() {
                        Ok(())
                    } else {
                        let is_col_nul = mat.col_is_nul(base_adjustment)?;
                            match is_col_nul {
                                true => {Err(MatrixError::NulColumnInGaussianElimination)},
                                false => {
                                    let mut is_fixed = false;

                                    for row_below in (base_adjustment+1)..mat.shape[1] {
                                        if !is_fixed {
                                            let row_left = mat[[base_adjustment, row_below]].clone();

                                            if row_left != T::zero() {
                                                for col_j in 0..mat.shape[0] {
                                                    mat[[col_j, row_i]] = mat[[col_j, row_i]].clone() + mat[[col_j, row_i+1]].clone();
                                                    //mat[[col_j, row_i]] += mat[[col_j, row_i+1]].clone();
                                                }
                                                is_fixed = true;
                                            }
                                        }
                                    }

                                    match is_fixed {
                                        true => Ok(()),
                                        false => Err(MatrixError::NulColumnInGaussianElimination),
                                    }
                                },
                            }
                    }?;

                    //match leftmost_value {
                    //    T::zero() => {
                    //        let is_col_nul = mat.col_is_nul(base_adjustment)?;
                    //        match is_col_nul {
                    //            true => {Err(MatrixError::NulColumnInGaussianElimination)},
                    //            false => {
                    //                let mut is_fixed = false;
//
                    //                for row_below in (base_adjustment+1)..mat.shape[1] {
                    //                    if !is_fixed {
                    //                        let row_left = mat[[base_adjustment, row_below]];
//
                    //                        if row_left != T::zero() {
                    //                            for col_j in 0..mat.shape[0] {
                    //                                mat[[col_j, row_i]] += mat[[col_j, row_i+1]];
                    //                            }
                    //                            is_fixed = true;
                    //                        }
                    //                    }
                    //                }
//
                    //                match is_fixed {
                    //                    true => Ok(()),
                    //                    false => Err(MatrixError::NulColumnInGaussianElimination),
                    //                }
                    //            },
                    //        }
                    //    },
                    //    _ => {Ok(())},
                    //}?;

                    // set it to new value, in case it changed from the fixing
                    let leftmost_value = mat[[base_adjustment, row_i]].clone();

            //println!("{}", leftmost_value);

                    for col_j in 0..mat.shape[0] {
                        //println!("{:?}, {}, {}, {}, {}", (col_j, row_i), mat[[col_j, row_i]], mat[[col_j, row_i]]/leftmost_value, mat[[col_j, base_adjustment]], mat[[col_j, row_i]]/leftmost_value-mat[[col_j, base_adjustment]]);
                        mat[[col_j, row_i]] *= T::one()/leftmost_value.clone();
                        if row_i != base_adjustment {
                            mat[[col_j, row_i]] = mat[[col_j, row_i]].clone() - mat[[col_j, base_adjustment]].clone();
                            //mat[[col_j, row_i]] -= mat[[col_j, base_adjustment]].clone();
                        }
                    }

            //println!("{}", mat);
            //println!("{}", leftmost_value);
            //println!("");

                }
            }
            //println!("");
            //println!("");
            //println!("");
            //println!("{}", mat);
            //println!("");
            //println!("");
            //println!("");
            Ok(mat)
        }
    }
}