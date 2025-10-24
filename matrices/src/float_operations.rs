use std::fmt::{Debug, Display};
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::matrix::Matrix;
use crate::numbers::{DataTypes, Float, Numerical};
use crate::errors::MatrixError;
use crate::type_conversions::IntoDataType;


impl<T:
    Float + Clone + IntoDataType + Display + Debug + PartialEq + DivAssign
    + SubAssign + MulAssign + AddAssign + Mul + Sub + Numerical + Add<Output = T>
    + Mul<Output = T> + Sub<Output = T> + Sum + Neg<Output = T> + Div<Output = T>
    + IntoDataType
    > Matrix<T> {

    
    pub fn minor(&self, row_i:usize, col_j:usize) -> Result<T, MatrixError> {
        if self.ndims() != 2 {
            Err(MatrixError::InvalidDimension(self.ndims()))
        } else if self.shape[0] != self.shape[1] {
            Err(MatrixError::InvalidShape(self.shape.clone()))
        } else {
            let minor = self.without_rc(row_i, col_j)?.laplace_expansion();
            minor
        }
    }

    pub fn cofactor(&self, row_i:usize, col_j:usize) -> Result<T, MatrixError> {
        if self.ndims() != 2 {
            Err(MatrixError::InvalidDimension(self.ndims()))
        } else if self.shape[0] != self.shape[1] {
            Err(MatrixError::InvalidShape(self.shape.clone()))
        } else {
            let minor = self.without_rc(row_i, col_j)?.laplace_expansion()?;

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

    pub fn laplace_expansion(&self) -> Result<T, MatrixError> {
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
        let determinant = self.laplace_expansion()?;
        if determinant == T::zero() {
            Err(MatrixError::DeterminantIsZero)
        } else {
            Ok(self.cofactor_matrix()?.transpose()?.multiply_by_constant(T::one()/determinant))
        }
    }

    pub fn col_is_nul(&self, col_j:usize) -> Result<bool, MatrixError> {
        if self.ndims() == 2 {
            let column = self.get_col(col_j)?;
            let zeroes = (0..column.array.len()).map(|i| column.array[i].is_zero()).all(|b| b==true);
            Ok(zeroes)
        } else {
            Err(MatrixError::InvalidDimension(self.ndims()))
        }
    }


    /// Gaussian elimination algorithm to get the row echelon form of a matrix
    pub fn echelon(&self) -> Result<Matrix<T>, MatrixError> {
        if self.ndims() != 2 {
            Err(MatrixError::InvalidDimension(self.ndims()))
        } else {
            let mut mat = self.clone();
            for base_adjustment in 0..mat.shape[1] {


                let this_col = mat.get_col(base_adjustment)?;
                let col_below_is_nul = this_col.array[base_adjustment..this_col.array.len()]
                                                     .iter()
                                                     .map(|i| i==&T::zero())
                                                     .all(|b| b==true);
                match col_below_is_nul {
                    true => {
                        /* this column is null for un-gaussed rows, so pass (variable not linked) */
                    },
                    false => {

                        for row_i in base_adjustment..mat.shape[1] {

                            let mut row_below = row_i+1;
                            while &mat[[base_adjustment, row_i]]==&T::zero() {
                                for col_j in 0..mat.shape[0] {
                                    let val_below = mat[[col_j, row_below]].clone();
                                    mat[[col_j, row_i]] += val_below;
                                }
                                row_below += 1;
                            }

                            let row_leftmost_val = mat[[base_adjustment, row_i]].clone();
                            for col_j in 0..mat.shape[0] {
                                mat[[col_j, row_i]] /= row_leftmost_val.clone();
                                if row_i != base_adjustment {
                                    let pivot_row_val = mat[[col_j, base_adjustment]].clone();
                                    mat[[col_j, row_i]] -= pivot_row_val;
                                }
                            }
                        }
                    },
                }
            }
            Ok(mat)
        }
    }

    /// Gauss-Jordan elimination algorithm
    pub fn reduced_echelon(&self) -> Result<Matrix<T>, MatrixError> {
        if self.ndims() != 2 {
            Err(MatrixError::InvalidDimension(self.ndims()))
        } else {
            let echelon_form = self.echelon()?;
            let mut reduced_echelon_form = echelon_form.clone();
            for base in (0..echelon_form.shape[1]).rev() {
                for row_i in (0..base).rev() {
                    //println!("");
                    let val_above_pivot = reduced_echelon_form[[base, row_i]].clone();
                    for col_j in 0..self.shape[0] {
                        let pivot_value = reduced_echelon_form[[col_j, base]].clone();
                        //let position_val = reduced_echelon_form[[col_j, row_i]].clone();
                        //println!("{}, {}, {}", pivot_value.clone(), val_above_pivot.clone(), position_val.clone());
                        reduced_echelon_form[[col_j, row_i]] -= pivot_value.clone()*val_above_pivot.clone();
                    }
                    //println!("{}", reduced_echelon_form);
                }
            }
            Ok(reduced_echelon_form)
        }
    }

    /// via Gauss-Jordan elimination
    pub fn solve(&self) -> Result<Matrix<T>, MatrixError> {
        if self.ndims() != 2 {
            Err(MatrixError::InvalidDimension(self.ndims()))
        } else if self.shape[0] != self.shape[1]+1 {
            Err(MatrixError::AugmentedMatrixShapeError)
        } else {
            let reduced_echelon = self.reduced_echelon()?;
            //error("a".to_string());
            let identity = Matrix::<T>::identity_from_vec(reduced_echelon.shape.clone());
            let re_minus_id = (reduced_echelon.clone()-identity.clone())?;
            let without_results = re_minus_id.without_col(self.shape[0]-1)?;
            let null = Matrix::<T>::null_from_vec(without_results.shape.clone());

            //println!("a, {}", reduced_echelon);
            //println!("b, {}", identity);
            //println!("c, {}", re_minus_id);
            //println!("d, {}", without_results);
            //println!("e, {}", null);


            //println!("{}, {}", without_results.array.len(), null.array.len());
            let shape_eq = without_results.shape == null.shape;
            let dtype_eq = without_results.dtype == null.dtype;
            let arr_eq = without_results.array == null.array;
            //for i in 0..without_results.array.len() {
            //    println!("{}, {}, {}, {}", i, without_results.array[i], null.array[i], without_results.array[i]==null.array[i])
            //}

            if shape_eq && dtype_eq && arr_eq {
                let solution = reduced_echelon.get_col(self.shape[0]-1)?;
                Ok(solution)
            } else {
                Err(MatrixError::MatrixSolveError((shape_eq, dtype_eq, arr_eq)))
            }

        }
    }
}