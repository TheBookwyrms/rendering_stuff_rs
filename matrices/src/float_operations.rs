use std::fmt::{Debug, Display};
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::matrix::Matrix;
use crate::traits::{Float, Numerical, IntoDataType};
use crate::enums::{MatrixError, DataTypes};


impl<T:
    Float + Clone + IntoDataType + PartialEq + DivAssign
    + SubAssign + MulAssign + AddAssign + Mul + Sub + Numerical + Add<Output = T>
    + Mul<Output = T> + Sub<Output = T> + Sum + Neg<Output = T> + Div<Output = T>
    + IntoDataType
    + Display + Debug
    > Matrix<T> {

    
    /// gets the minor of a matrix for row i and column j
    pub fn minor(&self, row_i:usize, col_j:usize) -> Result<T, MatrixError<T>> {
        if self.ndims() != 2 {
            Err(MatrixError::InvalidDimension(self.ndims()))
        } else if self.shape[0] != self.shape[1] {
            Err(MatrixError::InvalidShape(self.shape.clone()))
        } else {
            let minor = self.without_rc(row_i, col_j)?.laplace_expansion();
            minor
        }
    }

    /// gets the cofactor of a matrix for row i and column j
    pub fn cofactor(&self, row_i:usize, col_j:usize) -> Result<T, MatrixError<T>> {
        if self.ndims() != 2 {
            Err(MatrixError::InvalidDimension(self.ndims()))
        } else if self.shape[0] != self.shape[1] {
            Err(MatrixError::InvalidShape(self.shape.clone()))
        } else {
            let minor = self.without_rc(row_i, col_j)?.laplace_expansion()?;

            let r = T::usize_to_t(row_i);
            let c = T::usize_to_t(col_j);

            let n1 = -T::one();
            let cofactor = T::powf(n1, (r+T::one())+(c+T::one())) * minor;
            Ok(cofactor)
        }
    }

    /// get the determinant of a matrix via laplace expansion
    pub fn laplace_expansion(&self) -> Result<T, MatrixError<T>> {
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

    /// get the matrix of cofactors of the original matrix
    pub fn cofactor_matrix(&self) -> Result<Matrix<T>, MatrixError<T>> {
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

    /// get the inverse of a matrix
    pub fn inverse(&self) -> Result<Matrix<T>, MatrixError<T>> {

        let inverse = self.gauss_jordan_inverse();
        inverse

        // laplace expansion method of getting inverse

        //let determinant = self.laplace_expansion()?;
        //if determinant == T::zero() {
        //    Err(MatrixError::DeterminantIsZero)
        //} else {
        //    Ok(self.cofactor_matrix()?.transpose()?.multiply_by_constant(T::one()/determinant))
        //}
    }

    /// determines if the column j of a matrix is null (zero)
    pub fn col_is_null(&self, col_j:usize) -> Result<bool, MatrixError<T>> {
        if self.ndims() == 2 {
            let column = self.get_col(col_j)?;
            let zeroes = (0..column.array.len()).map(|i| column.array[i].is_zero()).all(|b| b==true);
            Ok(zeroes)
        } else {
            Err(MatrixError::InvalidDimension(self.ndims()))
        }
    }


    /// get the echelon form of a matrix
    /// via Gaussian elimination algorithm
    pub fn echelon(&self) -> Result<Matrix<T>, MatrixError<T>> {
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

    /// get the reduced echelon form of a matrix
    /// via Gauss-Jordan elimination algorithm
    pub fn reduced_echelon(&self) -> Result<Matrix<T>, MatrixError<T>> {
        if self.ndims() != 2 {
            Err(MatrixError::InvalidDimension(self.ndims()))
        } else {
            let echelon_form = self.echelon()?;
            let mut reduced_echelon_form = echelon_form.clone();
            for base in (0..echelon_form.shape[1]).rev() {
                for row_i in (0..base).rev() {
                    let val_above_pivot = reduced_echelon_form[[base, row_i]].clone();
                    for col_j in 0..self.shape[0] {
                        let pivot_value = reduced_echelon_form[[col_j, base]].clone();
                        reduced_echelon_form[[col_j, row_i]] -= pivot_value.clone()*val_above_pivot.clone();
                    }
                }
            }
            Ok(reduced_echelon_form)
        }
    }

    /// solve a system of linear equations
    /// formatted as an augmented matrix
    /// via Gauss-Jordan elimination
    pub fn solve(&self) -> Result<Matrix<T>, MatrixError<T>> {
        if self.ndims() != 2 {
            Err(MatrixError::InvalidDimension(self.ndims()))
        } else if self.shape[0] != self.shape[1]+1 {
            Err(MatrixError::AugmentedMatrixShapeError)
        } else {
            let reduced_echelon = self.reduced_echelon()?;

            let identity = Matrix::<T>::identity(self.shape[1]);
            let reduced_matrix_left = reduced_echelon.get_submatrix([0..self.shape[0]-1, 0..self.shape[1]])?;
            
            let re_minus_id = (reduced_matrix_left-identity.clone())?;
            let null = Matrix::<T>::null_from_vec(re_minus_id.shape);


            let dtype_eq = re_minus_id.dtype == null.dtype;
            let arr_eq = re_minus_id.array == null.array;

            if dtype_eq && arr_eq {
                let solution = reduced_echelon.get_col(self.shape[0]-1)?;
                Ok(solution)
            } else {
                Err(MatrixError::MatrixSolveError((dtype_eq, arr_eq)))
            }
        }
    }

    /// get the inverse of a matrix using gauss-jordan elimination
    /// on the matrix augmented by the identity
    pub fn gauss_jordan_inverse(&self) -> Result<Matrix<T>, MatrixError<T>> {
        if self.ndims() != 2 {
            Err(MatrixError::InvalidDimension(self.ndims()))
        } else if self.shape[0] != self.shape[1] {
            Err(MatrixError::InvalidShape(self.shape.clone()))
        } else {


            let identity = Matrix::<T>::identity(self.shape[0]);
            let augmented_matrix = self.expand_along_axis(identity.clone(), 0)?;
            let reduced_echelon = augmented_matrix.reduced_echelon()?;
            
            let reduced_matrix_left = reduced_echelon.get_submatrix([0..augmented_matrix.shape[0]/2, 0..augmented_matrix.shape[1]])?;
            let reduced_matrix_right = reduced_echelon.get_submatrix([augmented_matrix.shape[0]/2..augmented_matrix.shape[0], 0..augmented_matrix.shape[1]])?;

            let re_minus_id = (reduced_matrix_left-identity.clone())?;
            let null = Matrix::<T>::null_from_vec(re_minus_id.shape);

            let dtype_eq = re_minus_id.dtype == null.dtype;
            let arr_eq = re_minus_id.array == null.array;
            
            if dtype_eq && arr_eq {
                Ok(reduced_matrix_right)
            } else {
                Err(MatrixError::MatrixNotInversible(augmented_matrix))
            }
        }
    }
}