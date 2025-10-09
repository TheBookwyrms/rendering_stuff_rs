use core::num;
use std::ops::{Index, Add, Sub};
use std::convert::From;
use std::fmt::{Debug, Display};
use std::vec;

use crate::vector::Vector;
use crate::errors::MatrixError;

fn reverse_array(array:Vec<usize>) -> Vec<usize> {
    let mut rev_arr= vec![0;array.len()];
    for (i, j) in array.into_iter().rev().enumerate() {
        rev_arr[i] = j;
    }
    rev_arr.to_vec()
}

fn linear_index_to_indices(linear_index:usize, shape:Vec<usize>) -> Vec<usize> {

    let mut indices = shape.clone();

    let mut curr_max :usize = shape.iter().product();
    let mut curr_lin_idx = linear_index.clone();

    for (i, s_size) in shape.iter().enumerate().rev() {
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


//fn iter_through_shape(array:Vec<usize>, shape:Vec<usize>, outer_dim_idx:usize, held_values:Vec<f32>) {
//    if outer_dim_idx == shape.len()-1 {
//        // reached the interior of the loop
//    } else {
//        for (inner_dim_idx, inner_dim)
//    }
//}








// develop iterating through the matrix
// use closures to do operations on the iteration?
// example:
//     fn test<T: Fn(i32, i32) -> i32>(f:T) {
//         println!("{:?}", f(2, 8));
//     }
//     
//     fn main() {
//         let a = |i, j| (i+2)*j;
//         test(a);
//         println!("Hello, world!");
//     }





pub fn turn_indices_into_linear_index(shape:Vec<usize>, indices:Vec<usize>) -> usize {
    let ndims = shape.clone().len();
    let mut linear_idx = 0;
    let rev_ind = reverse_array(indices);
    for i in (0..ndims).into_iter().rev() {
        let mut idx_max = 1;
        for j in i..(ndims-1) {
            idx_max *= shape[j];
        }
        linear_idx += rev_ind[i]*idx_max;
    }
    //println!("{}", linear_idx);
    linear_idx
}


fn error(msg:String) {
    let a = true;
    let _b = match a {
        true =>Err(msg),
        false =>Ok(msg),
    }.unwrap();
}


#[derive(Debug, Clone)]
pub struct Matrix {
    pub shape:Vec<usize>, // goes from inner to outer dimensions (ex ncols before nrows for 2D)
    pub array:Vec<f32>,
}



impl<const K:usize> Index<[usize;K]> for Matrix {
    type Output = f32;

    fn index(&self, idx:[usize;K]) -> &Self::Output {
        let vidx = idx.to_vec();
        let are_allowed_indices = (0..vidx.len()).map(|i| vidx[i] <= self.shape[i])
                                                       .collect::<Vec<bool>>()
                                                       .iter()
                                                       .all(|b| *b);
        match are_allowed_indices {
            true => &self.array[turn_indices_into_linear_index(self.shape.clone(), idx.to_vec())],
            false => {Err::<usize, MatrixError>(MatrixError::InvalidIndices(vidx)).unwrap(); &0.0},
        }
    }
}

fn write_2d_matrix(f: &mut std::fmt::Formatter<'_>,
                   row_len:usize,
                   ll_lr:(usize, usize),
                   min_idx:usize,
                   max_idx:usize,
                   mat:&Matrix
                ) -> std::fmt::Result {
    //let row_len = mat.shape[0];
    //let (ll, lr) = mat.longest_item_str_len();
    //for i in 0..mat.shape[1] {
    let (ll, lr) = ll_lr;
    for i in min_idx..max_idx {
        write!(f, "  [")?;
        //for j in self.get_row(i) {
        for j in &mat.array[i*row_len..(i+1)*row_len] {
            let js = j.to_string();
            let js_vec = js.trim().split(".").collect::<Vec<_>>();
            let (nl, nr) =
                if j == &j.trunc() {
                    (js_vec[0], "")
                } else {
                    (js_vec[0], js_vec[1])
            };
            write!(f, " {: >ll$}.{: <lr$}", nl, nr)?;
        }
        writeln!(f, "],")?;
    }
    write!(f, "")
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.shape.len() == 1 {
            write!(f, "{:?}", self.array.as_slice())
        } else if self.shape.len() == 2 {
            writeln!(f, "[")?;
            let row_len = self.shape[0];
            let (ll, lr) = self.longest_item_str_len();
            let max = self.shape[1];
            let _ = write_2d_matrix(f, row_len, (ll, lr), 0, max, &self);
            write!(f, "]")
        } else if self.shape.len() == 3 {
            writeln!(f, "[")?;
            let x_len = self.shape[0];
            let y_len = self.shape[0];
            let (ll, lr) = self.longest_item_str_len();
            for i in 0..x_len {
                let min = i*y_len;
                let max = (i+1)*y_len;
                let _ = write_2d_matrix(f, y_len, (ll, lr), min, max, &self);
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

impl Add for Matrix {
    type Output = Result<Self, MatrixError>;

    fn add(self, other: Self) -> Result<Self, MatrixError> {
        if self.ndims() == other.ndims() {
            if self.shape == other.shape {
                let mut v = vec![0.0; self.array.len()];
                for i in 0..self.array.len() {
                    v[i] = self.array[i] + other.array[i];
                }
                Ok(Matrix {shape:self.shape, array:v})
            } else {
                Err(MatrixError::InvalidShapes([self.shape, other.shape]))
            }
        } else {
            Err(MatrixError::InvalidDimensions([self.ndims(), other.ndims()]))
        }
    }
}
impl Sub for Matrix {
    type Output = Result<Self, MatrixError>;

    fn sub(self, other: Self) -> Result<Self, MatrixError> {
        if self.ndims() == other.ndims() {
            if self.shape == other.shape {
                let mut v = vec![0.0; self.array.len()];
                for i in 0..self.array.len() {
                    v[i] = self.array[i] - other.array[i];
                }
                Ok(Matrix {shape:self.shape, array:v})
            } else {
                Err(MatrixError::InvalidShapes([self.shape, other.shape]))
            }
        } else {
            Err(MatrixError::InvalidDimensions([self.ndims(), other.ndims()]))
        }
    }
}

impl Matrix {
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

    pub fn from_float(f:f32) -> Matrix {
        Matrix {shape:vec![1], array:vec![f]}
    }

    pub fn from_1darray<const M:usize>(arr:[f32;M]) -> Matrix {
        Matrix {shape:vec![arr.len()], array:arr.to_vec()}
    }

    pub fn from_vec(vec:Vec<f32>) -> Matrix {
        Matrix {shape:vec![vec.len()], array: vec}
    }

    pub fn from_vec_of_vec(vec:Vec<Vec<f32>>) -> Result<Matrix, MatrixError> {
        let mut homogenous_rows = true;
        let mut row_lengths = vec![];
        let mut data : Vec<f32> = vec![];
        for row in vec.clone() {
            row_lengths.push(row.len());
            if row.len() != vec[0].len() {
                homogenous_rows = false;
            }
            data.extend(row);
        }
        match homogenous_rows {
            true => Ok( Matrix { shape:vec![vec[0].len(), vec.len()], array:data } ),
            false => Err(MatrixError::InhomogenousLength(row_lengths)),
        }
    }

    pub fn from_2darray<const M:usize, const N:usize>(arr:[[f32;M];N]) -> Matrix {
        let mut data = vec![];
        for row in arr {
            data.extend(row);
        }
        Matrix {shape:vec![M, N], array:data}
    }

    pub fn from_3darray<const M:usize, const N:usize, const O:usize>(arr:[[[f32;M];N];O]) -> Matrix {
        let mut data = vec![];
        for ax1 in arr {
            for ax2 in ax1 {
                data.extend(ax2);
            }
        }
        Matrix {shape:vec![M, N, O], array:data}
    }

    pub fn as_ptr(&self) -> *const f32 {
        self.array.as_ptr()
    }

    pub fn swap_axes(&self, axis1:usize, axis2:usize) -> Matrix {

        let mut swapped_arr = vec![0.0;self.array.len()];

        let mut altered_shape = self.shape.clone();
        altered_shape[axis1] = self.shape[axis2];
        altered_shape[axis2] = self.shape[axis1];
        println!("{:?}", altered_shape);

        for index in 0..self.array.len() {

            let indices = linear_index_to_indices(index, self.shape.clone());
            //println!("{}, {:?}, {}", index, indices, self.array[index]);
            let mut swapped_indices = indices.clone();
            swapped_indices[axis1] = indices[axis2];
            swapped_indices[axis2] = indices[axis1];
            
            //println!("{}, {:?}, {}. {:?}", index, indices, self.array[index], swapped_indices);

            let new_linear_index = turn_indices_into_linear_index(altered_shape.clone(), swapped_indices);

            //swapped_arr[index] = self.array[new_linear_index];
            swapped_arr[new_linear_index] = self.array[index];
            //swapped_arr[index] = self.array[new_linear_index];
            //swapped_arr[index] = new_linear_index as f32;


            //let mut sections_rev = vec![0; self.ndims()];
            //let mut curr_max = self.array.len();
            //for i in (0..self.ndims()).rev() {
            //    if i==0 {
            //        println!("{}, {}, {}, {}, {}", index, curr_max, self.shape[i], curr_max/self.shape[i], index%(curr_max/self.shape[i]));
            //    }
            //    let num_per_section = curr_max/self.shape[i];
            //    let section_idx: usize = index%num_per_section;
            //    sections_rev[i] = section_idx;
            //    //println!("{:?}", sections_rev);
            //    curr_max = num_per_section;
            //}
            //swapped_arr[index] = self.array[turn_indices_into_linear_index(
            //        self.ndims(), self.shape.clone(), sections_rev.clone()
            //    )];



            // PROGRESS !!!! (below)

    //let shape = [2, 3, 4];
    //let max: usize = shape.into_iter().product();
    //println!("{:?}, {}", shape, max);
    //
    //let mut v = vec![0.0; max];
    //println!("{:?}", v);
    //
    //let mut linear_index = 13;
    //let mut correct_cut_pos_holder = vec![];
    //
    //let mut curr_max = max;
    ////for dim_idx in (0..shape.len()).rev() {
    //for dim_idx in (0..shape.len()) {
    //    println!("{}", dim_idx);
    //    let cut_length = curr_max / shape[dim_idx];
    //    for cut_pos in 0..shape[dim_idx] {
    //        let (left_bound, right_bound) = (cut_length*cut_pos, cut_length*(cut_pos+1));
    //        println!("{}, {}, {}, {}", cut_pos, left_bound, right_bound, linear_index);
    //        if (left_bound..right_bound).contains(&linear_index) {
    //            correct_cut_pos_holder.push(cut_pos);
    //        }
    //    }
    //    curr_max /= shape[dim_idx];
    //    linear_index = linear_index/cut_length + linear_index%cut_length;
    //    println!("");
    //}
    //println!("{:?}", correct_cut_pos_holder);







            //swapped_arr[turn_indices_into_linear_index(
            //        self.ndims(), altered_shape.clone(), reverse_array(sections_rev.clone())
            //    )] = self.array[turn_indices_into_linear_index(
            //        self.ndims(), self.shape.clone(), sections_rev.clone()
            //    )];
        }
        
        Matrix {shape:altered_shape, array:swapped_arr}
    }


    pub fn transpose(self) -> Result<Matrix, MatrixError> {
        if self.ndims() == 2 {
            Ok(self.swap_axes(0,1))
        } else {
            Err(MatrixError::InvalidDimension(self.ndims()))
        }
    }

    pub fn get_row(&self, idx:usize) -> Result<Matrix, MatrixError> {
        if self.ndims() == 2 {
            match idx<self.shape[1] {
                true => {
                    let row_len = self.shape[0];
                    Ok(Matrix::from_vec(self.array[((idx-1)*row_len)..idx*row_len].to_vec()))},
                false => Err(MatrixError::InvalidIndex(idx)),
            }
        } else {
            Err(MatrixError::InvalidDimension(self.ndims()))
        }
    }

    pub fn get_col(self, idx:usize)  -> Result<Matrix, MatrixError> {
        if self.ndims() == 2 {
            let tarr = self.transpose()?;
            tarr.get_row(idx)
        } else {
            Err(MatrixError::InvalidDimension(self.ndims()))
        }
        ////match 0<=idx && idx<=self.ncols {
        //match idx<self.ncols {
        //    true => {
        //        let mut this_col= vec![];
        //        for row in self.array.iter().clone().to_owned() {
        //            //let new_vector = Vector {vec:row.vec.get(idx)};
        //            //let idx_value = *row.vec.get(idx as usize).unwrap();
        //            //let row_vec : &Vec<f32> = move || row ;
        //            //let a: &Vec<f32> = &row.vec;
        //            //let b = &c[idx as usize];
        //            let idx_value = row.vec[idx];
        //            //let idx_value = new_vector.vec[idx as usize];
        //            //let idx_value = row_vec[idx as usize];
        //            let _ = this_col.push(idx_value).to_owned();
        //        }
        //        Ok(Vector::from_vec(this_col))
        //    },
        //    false => Err(MatrixError::InvalidIndex(idx)),
        //}
    }

    pub fn size(&self) -> usize {
        let mut product = 1;
        for i in &self.shape {
            product *= i;
        }
        product
    }

    
    pub fn dot(&self, other:&Matrix) -> Result<f32, MatrixError> {
        if self.array.len() == other.array.len() {
            let mut sum: f32 = 0.0;
            for i in 0..=self.array.len()-1 {
                sum += self.array[i]*other.array[i]
            }
            Ok(sum)
        } else { Err(MatrixError::Invalidlengths([self.array.len(), other.array.len()])) }
    }

    pub fn matmul(&self, other:&Matrix) -> Result<Matrix, MatrixError> {
        if self.ndims() != 2 {
            Err(MatrixError::InvalidDimension(self.ndims()))
        } else if !(self.shape==other.shape) {
            Err(MatrixError::InvalidShapes([self.shape.clone(), other.shape.clone()]))
        } else {Ok(())}.unwrap();

        let mut rows = vec![];
        for r in 0..self.shape[0] {
            let mut this_row = vec![];
            for c in 0..other.shape[1] {
                let row = self.get_row(r)?;
                let col = &other.clone().get_col(c)?;
                this_row.push(row.dot(col)?);
            }
            rows.push(this_row);
        }
        Matrix::from_vec_of_vec(rows)
    }

    pub fn scale(sx:f32, sy:f32, sz:f32) -> Matrix {
        Matrix::from_2darray([
            [sx, 0., 0., 0.],
            [0., sy, 0., 0.],
            [0., 0., sz, 0.],
            [0., 0., 0., 1.],
        ])
    }

    pub fn translate(t:(f32, f32, f32)) -> Matrix {
        Matrix::from_2darray([
            [1., 0., 0., t.0],
            [0., 1., 0., t.1],
            [0., 0., 1., t.2],
            [0., 0., 0., 1.0],
        ])
    }

    pub fn rotate(rx:f32, ry:f32, rz:f32) -> Result<Matrix, MatrixError> {
        let (rrx, rry, rrz) = (rx.to_radians(), ry.to_radians(), rz.to_radians());
        
        let rot_x = Matrix::from_2darray([
            [1.,           0.,         0., 0.],
            [0.,    rrx.cos(), rrx.sin(), 0.],
            [0., -1.*rrx.sin(), rrx.cos(), 0.],
            [0.,           0.,         0., 1.]
            ]);

        let rot_y = Matrix::from_2darray([
            [rry.cos(), 0., -1.*rry.sin(), 0.],
            [       0., 1.,            0., 0.],
            [rry.sin(), 0.,     rry.cos(), 0.],
            [       0., 0.,            0., 1.]
            ]);

        let rot_z = Matrix::from_2darray([
            [rrz.cos(), -1.*rrz.sin(), 0., 0.],
            [rrz.sin(),     rrz.cos(), 0., 0.],
            [       0.,            0., 1., 0.],
            [       0.,            0., 0., 1.],
            ]);

        
        Ok(rot_x.matmul(&rot_y.matmul(&rot_z)?)?)
    }

    pub fn rotate_around_p(p:(f32, f32, f32), r:(f32, f32, f32)) -> Result<Matrix, MatrixError> {

        // p in form (x_offset, y_offset, z_offset)
        // NOTE : for some reason, y and z switch in calculations
        // thus, p gets deconstructed as :
        let (px, pz, py) = p;
        let (rx, ry, rz) = r;

        let return_to_pos     = Matrix::translate((px, py, pz));
        let translate_to_zero = Matrix::translate((-px, -py, -pz));

        let rotate = Matrix::rotate(rx, ry, rz)?;

        Ok(return_to_pos.matmul(&rotate.matmul(&translate_to_zero)?)?)
    }

    pub fn opengl_to_right_handed() -> Matrix {
        Matrix::from_2darray([
            [1.,0.,0.,0.],
            [0.,0.,1.,0.],
            [0.,1.,0.,0.],
            [0.,0.,0.,1.],
        ])
    }

    pub fn without_rc(&self, row_i:usize, col_j:usize) -> Result<Matrix, MatrixError> {
        if self.ndims() != 2 {
            Err(MatrixError::InvalidDimension(self.ndims()))
        } else if !(row_i<self.shape[0] && col_j<self.shape[1]) {
            Err(MatrixError::InvalidIndices(vec![row_i, col_j]))
        } else {
            let mut v_overall : Vec<Vec<f32>> = vec![];

            for row in 0..self.shape[0] {
                if row != row_i {
                    let mut v_inner : Vec<f32> = vec![];
                    for col in 0.. self.shape[1] {
                        if col != col_j {
                            v_inner.push(self[[row, col]]);
                        }
                    }
                    v_overall.push(v_inner);
                }
            }
            
            Matrix::from_vec_of_vec(v_overall)
        }
    }

    pub fn multiply_by_constant(&self, scalar:f32) -> Matrix {
        let mut narr = self.array.clone();
        for i in 0..self.array.len() {
            narr[i] *= scalar;
        }
        
        Matrix {shape:self.shape.clone(), array:narr}
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

            for (col_idx, col_val) in row_0.clone().into_iter().enumerate() {
                let minor = self.without_rc(row_i, col_idx)?.determinant()?;

                let r = f32::from(u16::try_from(u32::try_from(row_i).unwrap()).unwrap());
                let c = f32::from(u16::try_from(u32::try_from(col_idx).unwrap()).unwrap());

                let cofactor = (-1.0_f32).powf((r+1.0)+(c+1.0)) * minor;
                determinant_sum += col_val * cofactor;
            }
            Ok(determinant_sum)
        }
    }

    pub fn inverse(&self) -> Result<Matrix, MatrixError> {
        let determinant = self.determinant()?;
        match determinant {
            0.0 => Err(MatrixError::DeterminantIsZero),
            _ => Ok(self.clone().multiply_by_constant(determinant)),
        }
    }
}