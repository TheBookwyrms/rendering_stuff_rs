use crate::_tests::matrix_with_types::matrix::Matrix;
use crate::_tests::matrix_with_types::numbers::DataTypes;
use crate::_tests::matrix_with_types::type_conversions::IntoDataType;
use crate::_tests::matrix_with_types::errors::MatrixError;

use std::ops::{Index, Add, Sub};
use std::convert::From;
use std::fmt::{Debug, Display};
use std::{char, vec};
use std::any::Any;


//fn reverse_array(array:Vec<usize>) -> Vec<usize> {
//    let mut rev_arr= vec![0;array.len()];
//    for (i, j) in array.into_iter().rev().enumerate() {
//        rev_arr[i] = j;
//    }
//    rev_arr.to_vec()
//}




fn error(msg:String) {
    let a = true;
    let _b = match a {
        true =>Err(msg),
        false =>Ok(msg),
    }.unwrap();
}



impl<const K:usize, T:Display+IntoDataType+Clone> Index<[usize;K]> for Matrix<T> {
    type Output = T;

    fn index(&self, idx:[usize;K]) -> &Self::Output {           
        &self.array[self.turn_indices_into_linear_index(idx.to_vec())]
    }
}


//pub trait WriteMatrix<T> {
//    fn write2d(&self, f:&mut std::fmt::Formatter<'_>, x_len:usize, ll_lr:(usize, usize),
//                min_idx:usize, max_idx:usize, arr:&Vec<T>) -> std::fmt::Result;
//}
//
//impl<T, U> WriteMatrix<T> for U
//
//fn write_2d_float_matrix<T:Display + Debug + Float + PartialEq>(f: &mut std::fmt::Formatter<'_>,
////fn write_2d_matrix(f: &mut std::fmt::Formatter<'_>,
//                   x_len:usize,
//                   ll_lr:(usize, usize),
//                   min_idx:usize,
//                   max_idx:usize,
//                   arr:&Vec<T>
//                ) -> std::fmt::Result {
//    //let row_len = mat.shape[0];
//    //let (ll, lr) = mat.longest_item_str_len();
//    //for i in 0..mat.shape[1] {
//    let (ll, lr) = ll_lr;
//    for i in min_idx..max_idx {
//        write!(f, "  [")?;
//        //for j in self.get_row(i) {
//        for j in &arr[i*x_len..(i+1)*x_len] {
//            let js = j.to_string();
//            let js_vec = js.trim().split(".").collect::<Vec<_>>();
//            let (nl, nr) =
//                if j == &j.truncate() {
//                    (js_vec[0], "")
//                } else {
//                    (js_vec[0], js_vec[1])
//            };
//            write!(f, " {: >ll$}.{: <lr$}", nl, nr)?;
//        }
//        writeln!(f, "],")?;
//    }
//    write!(f, "")
//}

fn write_2d_matrix<T:Display + Debug + PartialEq>(f: &mut std::fmt::Formatter<'_>,
                   x_len:usize,
                   ll_lr:(usize, usize),
                   min_idx:usize,
                   max_idx:usize,
                   arr:&Vec<T>
                ) -> std::fmt::Result {
    //let row_len = mat.shape[0];
    //let (ll, lr) = mat.longest_item_str_len();
    //for i in 0..mat.shape[1] {
    let (ll, lr) = ll_lr;
    for i in min_idx..max_idx {
        write!(f, "  [")?;
        //for j in self.get_row(i) {
        for j in &arr[i*x_len..(i+1)*x_len] {
            let js = j.to_string();
            let js_vec = js.trim().split(".").collect::<Vec<_>>();

            let mut has_non_zero = false;
            for i in js_vec[1].chars() {
                if i != char::from_u32(0).unwrap() {
                    has_non_zero = true;
                }
            }

            let (nl, nr) = if !has_non_zero {
                    (js_vec[0], "")
                } else {
                    (js_vec[0], js_vec[1])
            };
            //let (nl, nr) = (js_vec[0], "");
            write!(f, " {: >ll$}.{: <lr$}", nl, nr)?;
        }
        writeln!(f, "],")?;
    }
    write!(f, "")
}

//impl<T:Display + Debug + Float> Display for Matrix<T> {
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//        if self.shape.len() == 1 {
//            write!(f, "{:?}", self.array.as_slice())
//        } else if self.shape.len() == 2 {
//            writeln!(f, "[")?;
//            let x_len = self.shape[0];
//            let (ll, lr) = self.longest_item_str_len();
//            let y_len = self.shape[1];
//            let _ = write_2d_float_matrix(f, x_len, (ll, lr), 0, y_len, &self.array);
//            write!(f, "]")
//        } else if self.shape.len() == 3 {
//            writeln!(f, "[")?;
//            let x_len = self.shape[0];
//            let y_len = self.shape[1];
//            let z_len = self.shape[2];
//            let (ll, lr) = self.longest_item_str_len();
//            for i in 0..z_len {
//                let min = i*y_len;
//                let max = (i+1)*y_len;
//                let _ = write_2d_float_matrix(f, x_len, (ll, lr), min, max, &self.array);
//                if i != x_len {
//                    write!(f, "\n")?;
//                }
//            }
//            //let _ = write_2d_matrix(f, &self);
//            write!(f, "]")
//        } else {
//            // needs iterating through dimensions
//            let mut msg = "Display not yet implemented for shape ".to_string();
//            msg.push_str(self.shape.len().to_string().as_str());
//            error(msg);
//            write!(f, "error occurred")
//        }
//    }
//}

impl<T:Display + Debug + PartialEq + IntoDataType + Clone> Display for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.shape.len() == 1 {
            write!(f, "{:?}", self.array.as_slice())
        } else if self.shape.len() == 2 {
            writeln!(f, "[")?;
            let x_len = self.shape[0];
            let (ll, lr) = self.longest_item_str_len();
            let y_len = self.shape[1];
            let _ = write_2d_matrix(f, x_len, (ll, lr), 0, y_len, &self.array);
            write!(f, "]")
        } else if self.shape.len() == 3 {
            writeln!(f, "[")?;
            let x_len = self.shape[0];
            let y_len = self.shape[1];
            let z_len = self.shape[2];
            let (ll, lr) = self.longest_item_str_len();
            for i in 0..z_len {
                let min = i*y_len;
                let max = (i+1)*y_len;
                let _ = write_2d_matrix(f, x_len, (ll, lr), min, max, &self.array);
                if i != x_len {
                    write!(f, "\n")?;
                }
            }
            //let _ = write_2d_matrix(f, &self);
            write!(f, "]")
        } else {
            // needs iterating through dimensions
            let mut msg = "Display not yet implemented for shape ".to_string();
            msg.push_str(self.shape.len().to_string().as_str());
            error(msg);
            write!(f, "error occurred")
        }
    }
}





impl<T:Display + IntoDataType + Clone> Matrix<T> {

    
    pub fn linear_index_to_indices(&self, linear_index:usize) -> Vec<usize> {

        let mut indices = self.shape.clone();

        let mut curr_max :usize = self.shape.iter().product();
        let mut curr_lin_idx = linear_index.clone();

        for (i, s_size) in self.shape.iter().enumerate().rev() {
            // IMPORTANT!!!
            // The divisions here truncate the values
            // not a pure division with exact values
            // ex: 3.75 is truncated to 3
            let section_len = curr_max/s_size;
            let section = curr_lin_idx/section_len;
            curr_lin_idx -= section*section_len;
            curr_max = curr_max/s_size;
            indices[i] = section;
        }
        indices
    }


    pub fn turn_indices_into_linear_index(&self, indices:Vec<usize>) -> usize {        
        let mut linear_idx = 0;
        
        for i in (0..self.ndims()).into_iter().rev() {
            let mut idx_max = 1;
            //for j in i..(ndims-1) {
            for j in 0..i {
                idx_max *= self.shape[j];
            }
            linear_idx += indices[i]*idx_max;
        }
        //println!("{}", linear_idx);
        linear_idx
    }



    pub fn longest_item_str_len(&self) -> (usize, usize) {
        let mut length_left = 0;
        let mut length_right = 0;
        for i in &self.array {
            let l_i = i.to_string();
            let l_i : Vec<usize>= l_i.split(".").map(|u| u.chars().count()).collect();
            let (ll, lr) = match l_i.len() {
                1 => (l_i[0], 0),
                2 => (l_i[0], l_i[1]),
                _ => {error("too many decimals".to_string()); (0, 0)},
            };
            if ll > length_left {
                length_left = ll;
            }
            if lr > length_right {
                length_right = lr;
            }
        }
        (length_left, length_right)
    }

    pub fn ndims(&self) -> usize {
        self.shape.len()
    }

    pub fn new_empty(shape:Vec<usize>) -> Matrix<T> {
        Matrix { shape, array: vec![], dtype:DataTypes::EMPTY }
    }

    pub fn from_scalar(f:T) -> Matrix<T> {
        let dtype = f.as_dtype();
        Matrix {shape:vec![1], array:vec![f], dtype:dtype}
    }

    pub fn from_1darray<const M:usize>(arr:[T;M]) -> Matrix<T> {
        let dtype = arr[0].as_dtype();
        Matrix {shape:vec![arr.len()], array:arr.to_vec(), dtype}
    }

    pub fn from_vec(vec:Vec<T>) -> Matrix<T> {
        let dtype = vec[0].as_dtype();
        Matrix {shape:vec![vec.len()], array: vec, dtype}
    }

    pub fn from_vec_of_vec(vec:Vec<Vec<T>>) -> Result<Matrix<T>, MatrixError> {
        let dtype = vec[0][0].as_dtype();
        let mut homogenous_rows = true;
        let mut row_lengths = vec![];
        let mut data : Vec<T> = vec![];
        for row in vec.clone() {
            row_lengths.push(row.len());
            if row.len() != vec[0].len() {
                homogenous_rows = false;
            }
            data.extend(row);
        }
        match homogenous_rows {
            true => Ok( Matrix { shape:vec![vec[0].len(), vec.len()], array:data, dtype } ),
            false => Err(MatrixError::InhomogenousLength(row_lengths)),
        }
    }

    pub fn from_2darray<const M:usize, const N:usize>(arr:[[T;M];N]) -> Matrix<T> {
        let dtype = arr[0][0].as_dtype();
        let mut data = vec![];
        for row in arr {
            data.extend(row);
        }
        Matrix {shape:vec![M, N], array:data, dtype}
    }

    pub fn from_3darray<const M:usize, const N:usize, const O:usize>(arr:[[[T;M];N];O]) -> Matrix<T> {
        let dtype = arr[0][0][0].as_dtype();
        let mut data = vec![];
        for ax1 in arr {
            for ax2 in ax1 {
                data.extend(ax2);
            }
        }
        Matrix {shape:vec![M, N, O], array:data, dtype}
    }

    pub fn as_ptr(&self) -> *const T {
        self.array.as_ptr()
    }

    pub fn swap_axes(&self, axis1:usize, axis2:usize) -> Matrix<T> {

        let swapped_arr = self.array.clone();


        let mut altered_shape = self.shape.clone();
        altered_shape[axis1] = self.shape[axis2];
        altered_shape[axis2] = self.shape[axis1];
        println!("{:?}", altered_shape);
        
        let mut swapped_mat = Matrix {shape:altered_shape, array:swapped_arr, dtype:self.dtype};

        for index in 0..self.array.len() {

            let indices = self.linear_index_to_indices(index);
            let mut swapped_indices = indices.clone();
            swapped_indices[axis1] = indices[axis2];
            swapped_indices[axis2] = indices[axis1];
            

            let new_linear_index = swapped_mat.turn_indices_into_linear_index(swapped_indices.clone());

            swapped_mat.array[new_linear_index] = self.array[index].clone();
        }
        
        swapped_mat
    }


    pub fn transpose(self) -> Result<Matrix<T>, MatrixError> {
        if self.ndims() == 2 {
            Ok(self.swap_axes(0,1))
        } else {
            Err(MatrixError::InvalidDimension(self.ndims()))
        }
    }

    pub fn get_row(&self, idx:usize) -> Result<Matrix<T>, MatrixError> {
        if self.ndims() == 2 {
            match idx<self.shape[1] {
                true => {
                    let row_len = self.shape[0];
                    let v = self.array[(idx*row_len)..(idx+1)*row_len].to_vec();
                    Ok(Matrix::from_vec(v))},
                false => Err(MatrixError::InvalidIndex(idx)),
            }
        } else {
            Err(MatrixError::InvalidDimension(self.ndims()))
        }
    }

    pub fn get_col(self, idx:usize)  -> Result<Matrix<T>, MatrixError> {
        if self.ndims() == 2 {
            let tarr = self.transpose()?;
            tarr.get_row(idx)
        } else {
            Err(MatrixError::InvalidDimension(self.ndims()))
        }
    }

    pub fn size(&self) -> usize {
        error("not implemented yet, only a number of items, not a size in memory".to_string());
        let num_items = self.shape.iter().product();
        num_items
        //let mut product = 1;
        //for i in &self.shape {
        //    product *= i;
        //}
        //product
    }

    pub fn without_rc(&self, row_i:usize, col_j:usize) -> Result<Matrix<T>, MatrixError> {
        if self.ndims() != 2 {
            Err(MatrixError::InvalidDimension(self.ndims()))
        } else if !(row_i<self.shape[0] && col_j<self.shape[1]) {
            Err(MatrixError::InvalidIndices(vec![row_i, col_j]))
        } else {

            let new_shape = (0..self.ndims()).map(|i| self.shape[i]-1).collect::<Vec<usize>>();
            let mut v = vec![];
            
            for row in 0..self.shape[1] {
                if row != row_i {
                    for (col, val) in self.get_row(row)?.array.into_iter().enumerate() {
                        if col != col_j {
                            v.push(val);
                        }
                    }
                }
            }

            Ok(Matrix {shape:new_shape, array:v, dtype:self.dtype})
        }
    }

    pub fn expand_along_dims(mut self, other:Matrix<T>) -> Result<Matrix<T>, MatrixError> {
        if self.shape[0] != other.shape[0] {
            Err(MatrixError::InvalidShapes([self.shape.to_vec(), other.shape.to_vec()]))
        } else if (self.dtype != DataTypes::EMPTY) && (self.dtype != other.dtype) {
            Err(MatrixError::InvalidDataTypes([self.dtype, other.dtype]))
        } else {
            self.dtype = other.dtype;
            self.shape[1] += other.shape[1];
            self.array.extend(other.array);
            //for v in other.array {
            //    self.array.push(v);
            //}
            Ok(self)
        }
    }
}