use crate::cartesian_product;
use crate::matrix::Matrix;
use crate::numbers::DataTypes;
use crate::type_conversions::IntoDataType;
use crate::errors::MatrixError;

use std::ops::{Index, IndexMut, Range};
use std::fmt::{Debug, Display};
use std::{char, vec};


fn error(msg:String) {
    let a = true;
    let _b = match a {
        true =>Err(msg),
        false =>Ok(msg),
    }.unwrap();
}
impl<T:Clone> Matrix<T> {

    /// converts linear index into corresponding matrix indices
    pub fn indices_of(&self, linear_index:usize) -> Vec<usize> {

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

    /// turns matrix indices into corresponding linear index
    pub fn linear_index_of(&self, indices:Vec<usize>) -> usize {        
        let mut linear_idx = 0;
        
        for i in (0..self.ndims()).into_iter().rev() {
            let mut idx_max = 1;
            for j in 0..i {
                idx_max *= self.shape[j];
            }
            linear_idx += indices[i]*idx_max;
        }
        linear_idx
    }

    pub fn ndims(&self) -> usize {
        self.shape.len()
    }

    pub fn as_ptr(&self) -> *const T {
        self.array.as_ptr()
    }
}


impl<T:Display + Clone> Matrix<T> {
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
}

//impl<const K:usize, T:Display+Clone> Index<[Range<usize>;K]> for Matrix<T> {
//    type Output = Matrix<T>;
//
//    /// indexes a matrix by Ranges of indices
//    fn index(&self, idx:[Range<usize>;K]) -> &Self::Output {
//
//        let new_shape = idx.clone().map(|range| range.len()).to_vec();
//        let full_len = new_shape.iter().product();
//        let mut idx_vec = vec![vec![]; full_len];
//
//        for (i, range) in idx.into_iter().enumerate() {
//            let num_repeated = full_len/range.len();
//            for reps in 0..num_repeated {
//                for val in range.clone() {
//                    idx_vec[reps*range.len() + i].push(val);
//                }
//            }
//        }
//
//        let mut new_arr = vec![];
//        for indices in idx_vec {
//            let linear_index = self.linear_index_of(indices);
//            let idx_val = self.array[linear_index].clone();
//            new_arr.push(idx_val);
//        }
//
//        &Self{
//            shape:new_shape,//.clone(),
//            array:new_arr,//.clone(),
//            dtype:self.dtype,//.clone(),
//        }//.clone()
//    }
//}


impl<T:Clone> Matrix<T> {
    pub fn get_submatrix<const K:usize>(&self, bounds:[Range<usize>;K]) -> Result<Matrix<T>, MatrixError> {
        if bounds.len() != self.ndims() {
            Err(MatrixError::InvalidDimensions([bounds.len(), self.ndims()]))
        } else {
            let not_bounded = bounds.iter().enumerate().map(|(i, range)| match range.clone().max() {
                Some(max) => max < self.shape[i],
                None => false,
            }).collect::<Vec<bool>>().contains(&false);

            if not_bounded {
                Err(MatrixError::InvalidBounds)
            } else {
                let mut new_shape = bounds.clone().map(|range| range.len()).to_vec();
                new_shape.reverse();

                //let full_len = new_shape.iter().product();
                //let mut idx_vec = vec![vec![]; full_len];

                //let mut b = vec![];
                //for (i, range) in bounds.into_iter().enumerate() {
                //    for j in range {
                //        for bl in b {
                //            bl.push(j);
                //        }
                //    }
                //}


                //for (i, range) in bounds.into_iter().enumerate() {
                //    let num_repeated = full_len/range.len();
                //    println!("num_repeated {}", num_repeated);
                //    for reps in 0..num_repeated {
                //        for (j, val) in range.clone().enumerate() {
                //            //println!("{}, {}, {}, {}, {}, {}", reps, range.len(), j, reps*range.len(), reps*range.len()+j, val);
                //            let offset = 1*reps;
                //            let j_offset = (j+offset)%num_repeated;
                //            println!("j {}, {}, {}, f {}, v {}", j, offset, j_offset, reps*range.len() + j_offset, val);
                //            idx_vec[reps*range.len() + j].push(val);
                //        }
                //    }
                //}


                //println!("{:?}", idx_vec.clone());
                //idx_vec.sort();
                //println!("{:?}", idx_vec.clone());
                            
                let mut new_arr = vec![];
                let iters = bounds.map(|range| range.collect::<Vec<usize>>());
                for indices in cartesian_product::cartesian_product(iters) {
                //for indices in idx_vec {
                    let linear_index = self.linear_index_of(indices.clone());
                    println!("indices {:?}, index {}", indices.clone(), linear_index);
                    let idx_val = self.array[linear_index].clone();
                    new_arr.push(idx_val);
                }
            
                //panic!();
                Ok(Matrix { shape:new_shape, array:new_arr, dtype:self.dtype }.swap_axes(0, 1))
            }
        }
    }
}



//impl<const K:usize, T:Display+Clone> IndexMut<[Range<usize>;K]> for Matrix<T> {
//    /// indexes a matrix by Ranges of indices
//    fn index_mut(&mut self, idx:[Range<usize>;K]) -> &mut Matrix<T> {
//
//        let new_shape = idx.clone().map(|range| range.len()).to_vec();
//
//        let full_len = new_shape.iter().product();
//
//        let mut idx_vec = vec![vec![]; full_len];
//
//        for (i, range) in idx.into_iter().enumerate() {
//            let num_repeated = full_len/range.len();
//            for reps in 0..num_repeated {
//                for val in range.clone() {
//                    idx_vec[reps*range.len() + i].push(val);
//                }
//            }
//        }
//
//        let mut new_arr = vec![];
//        for indices in idx_vec {
//            let linear_index = self.linear_index_of(indices);
//            let idx_val = self.array[linear_index].clone();
//            new_arr.push(idx_val);
//        }
//
//        self.shape = new_shape;
//        self.array = new_arr;
//
//        self
//    }
//}

impl<const K:usize, T:Clone> Index<[usize;K]> for Matrix<T> {
    type Output = T;

    /// indexes a matrix by its indices
    fn index(&self, idx:[usize;K]) -> &Self::Output {
        &self.array[self.linear_index_of(idx.to_vec())]
    }
}

impl<const K:usize, T:Clone> IndexMut<[usize;K]> for Matrix<T> {
    fn index_mut(&mut self, index: [usize;K]) -> &mut Self::Output {
        let linear_idx = self.linear_index_of(index.to_vec());
        &mut self.array[linear_idx]
    }
}

/// generic function for writing out a 2D matrix
fn write_2d_matrix<T:Display + Debug + PartialEq>(f: &mut std::fmt::Formatter<'_>,
                   x_len:usize,
                   ll_lr:(usize, usize),
                   min_idx:usize,
                   max_idx:usize,
                   arr:&Vec<T>
                ) -> std::fmt::Result {
    let (ll, lr) = ll_lr;
    for i in min_idx..max_idx {
        write!(f, "  [")?;
        for j in &arr[i*x_len..(i+1)*x_len] {
            let js = j.to_string();
            let js_vec = js.trim().split(".").collect::<Vec<_>>();

            let mut has_non_zero = false;
            if js_vec.len() == 2 {
                for i in js_vec[1].chars() {
                    if i != char::from_u32(0).unwrap() {
                        has_non_zero = true;
                    }
                }
            }

            let (nl, nr) = match js_vec.len() {
                0 => {error("invalid matrix?".to_string()); ("", "")},
                1 => {(js_vec[0], "")},
                2 => {(js_vec[0], if !has_non_zero {""} else {js_vec[1]})},
                _ => {error("invalid matrix?".to_string()); ("", "")},
            };

            write!(f, " {: >ll$}.{: <lr$}", nl, nr)?;
        }
        writeln!(f, "],")?;
    }
    write!(f, "")
}



impl<T:Display + Debug + PartialEq + IntoDataType + Clone> Display for Matrix<T> {
    /// format implementation for matrix
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





impl<T:Clone> Matrix<T> {

    pub fn swap_axes(&self, axis1:usize, axis2:usize) -> Matrix<T> {

        let swapped_arr = self.array.clone();


        let mut altered_shape = self.shape.clone();
        altered_shape[axis1] = self.shape[axis2];
        altered_shape[axis2] = self.shape[axis1];
        
        let mut swapped_mat = Matrix {shape:altered_shape, array:swapped_arr, dtype:self.dtype};

        for index in 0..self.array.len() {

            let indices = self.indices_of(index);
            let mut swapped_indices = indices.clone();
            swapped_indices[axis1] = indices[axis2];
            swapped_indices[axis2] = indices[axis1];
            

            let new_linear_index = swapped_mat.linear_index_of(swapped_indices.clone());

            swapped_mat.array[new_linear_index] = self.array[index].clone();
        }
        
        swapped_mat
    }


    pub fn transpose(&self) -> Result<Matrix<T>, MatrixError> {
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
                    Ok(Matrix {shape:vec![v.len()], array:v, dtype:self.dtype})},
                false => Err(MatrixError::InvalidIndex(idx)),
            }
        } else {
            Err(MatrixError::InvalidDimension(self.ndims()))
        }
    }

    pub fn get_col(&self, idx:usize)  -> Result<Matrix<T>, MatrixError> {
        if self.ndims() == 2 {
            let tarr = self.transpose()?;
            tarr.get_row(idx)
        } else {
            Err(MatrixError::InvalidDimension(self.ndims()))
        }
    }

    pub fn dtype_memsize(&self) -> usize {
        let type_size = std::mem::size_of::<T>();
        type_size
    }

    pub fn memory_size(&self) -> usize {
        let type_size = std::mem::size_of::<T>();
        let num_items = self.array.len();
        num_items*type_size
    }

    pub fn without_rc(&self, row_i:usize, col_j:usize) -> Result<Matrix<T>, MatrixError> {
        if self.ndims() != 2 {
            Err(MatrixError::InvalidDimension(self.ndims()))
        } else if !(row_i<self.shape[1] && col_j<self.shape[0]) {
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

    pub fn without_col(&self, col_j:usize) -> Result<Matrix<T>, MatrixError> {
        if self.ndims() != 2 {
            Err(MatrixError::InvalidDimension(self.ndims()))
        } else if !(col_j<self.shape[0]) {
            Err(MatrixError::InvalidIndex(col_j))
        } else {

            let mut new_shape = self.shape.clone();
            new_shape[0] -= 1;
            let mut v = vec![];
            
            for row in 0..self.shape[1] {
                for (col, val) in self.get_row(row)?.array.into_iter().enumerate() {
                    if col != col_j {
                        v.push(val);
                    }
                }
            }

            Ok(Matrix {shape:new_shape, array:v, dtype:self.dtype})
        }
    }

    pub fn expand_along_outer_dim(mut self, other:Matrix<T>) -> Result<Matrix<T>, MatrixError> {
        if self.shape[0] != other.shape[0] {
            Err(MatrixError::InvalidShapes([self.shape.to_vec(), other.shape.to_vec()]))
        } else if (self.dtype != DataTypes::EMPTY) && (self.dtype != other.dtype) {
            Err(MatrixError::InvalidDataTypes([self.dtype, other.dtype]))
        } else {
            self.dtype = other.dtype;
            self.shape[1] += other.shape[1];
            self.array.extend(other.array);
            Ok(self)
        }
    }
}