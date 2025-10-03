use std::ops::{Index, Add, Sub};

use crate::vector::Vector;
use crate::matrix_error::MatrixError;


#[derive(Debug, Clone)]
pub struct Matrix2d {
    pub nrows:usize,
    pub ncols:usize,
    pub array:Vec<Vector>,
}
impl Index<[usize;2]> for Matrix2d {
    type Output = f32;

    fn index(&self, idx:[usize;2]) -> &Self::Output {
        &self.array[idx[0]][idx[1]]
    }
}
impl Add for Matrix2d {
    type Output = Result<Self, MatrixError>;

    fn add(self, other: Self) -> Result<Self, MatrixError> {
        if self.shape() == other.shape() {
            let mut arr: Vec<Vec<f32>> = vec![vec![0.0;self.shape()[0]]; self.shape()[1]];
            for i in 0..self.shape()[0] {
                for j in 0..self.shape()[1] {
                    arr[i][j] = self[[i, j]] + other[[i, j]]
                } 
            }
            Matrix2d::from_vec_of_vec(arr)
        } else { Err(MatrixError::InvalidShapes([self.shape(), other.shape()])) }
    }
}
impl Sub for Matrix2d {
    type Output = Result<Self, MatrixError>;

    fn sub(self, other: Self) -> Result<Self, MatrixError> {
        if self.shape() == other.shape() {
            let mut arr: Vec<Vec<f32>> = vec![vec![0.0;self.shape()[0]]; self.shape()[1]];
            for i in 0..self.shape()[0] {
                for j in 0..self.shape()[1] {
                    arr[i][j] = self[[i, j]] - other[[i, j]]
                } 
            }
            Matrix2d::from_vec_of_vec(arr)
        } else { Err(MatrixError::InvalidShapes([self.shape(), other.shape()])) }
    }
}

impl Matrix2d {
    pub fn from_float(f:f32) -> Matrix2d {
        Matrix2d::from_array([[f]])
    }

    pub fn from_1darray<const M:usize>(arr:[f32;M]) -> Matrix2d {
        Matrix2d::from_array([arr])
    }

    pub fn from_vec(vec:Vec<f32>) -> Matrix2d {
        Matrix2d {nrows:1, ncols:vec.len(), array: vec![Vector {vec}]}
    }

    pub fn from_vec_of_vec(vec:Vec<Vec<f32>>) -> Result<Matrix2d, MatrixError> {
        let mut homogenous_rows = true;
        let mut row_lengths = vec![];
        let mut vectors : Vec<Vector> = vec![];
        for row in vec.clone() {
            row_lengths.push(row.len());
            if row.len() != vec[0].len() {
                homogenous_rows = false;
            }
            vectors.push(Vector { vec:row });
        }
        match homogenous_rows {
            true => Ok( Matrix2d { nrows:vec.len(), ncols: vec[0].len(), array: vectors } ),
            false => Err(MatrixError::InhomogenousLength(row_lengths)),
        }
    }

    pub fn from_vector(vector:Vector) -> Matrix2d {
        Matrix2d {nrows:1, ncols:vector.vec.len(), array: vec![vector]}
    }

    pub fn from_array<const M:usize, const N:usize>(arr:[[f32;M];N]) -> Matrix2d {
        let mut rows= vec![];
        let mut nrows = 0;
        let mut ncols = 0;

        for row in arr {
            ncols = row.len().try_into().unwrap();
            nrows += 1;
            //let mut this_col_len = 0;
            let mut this_col = vec![];
            for col in row {
                //this_col_len += 1;
                this_col.push(col);
            }
            let a = Vector::from_vec(this_col);
            rows.push(a);
        }

        Matrix2d {nrows:nrows,
                    ncols:ncols,
                    array:rows,
                }
    }

    pub fn as_ptr(self) -> *const f32 {
        let mut items = vec![];
        for row in self.array {
            for value in row.vec {
                items.push(value);
            }
        }
        items.as_ptr()
    }

    pub fn shape(&self) -> [usize; 2] {[self.nrows, self.ncols]}

    pub fn transpose(self) -> Matrix2d {
        let mut new_rows = vec![];
        let ncols = self.nrows;
        for i in 0..=ncols-1 {
            let mut this_row = vec![];
            for row in self.array.iter().clone() {
            //for row in self.array.vec.iter().into_iter() {
            this_row.push(row.vec.as_slice()[i]);
            }
            new_rows.push(Vector::from_vec(this_row));
        }
        let matrix = new_rows;
        Matrix2d { nrows: self.ncols, ncols: self.nrows, array: matrix }
    }

    pub fn get_row(&self, idx:usize) -> Result<Vector, MatrixError> {
        //match 0<=idx && idx<=self.nrows {
        match idx<self.nrows {
            true => Ok(self.array[idx].clone().to_owned()),
            false => Err(MatrixError::InvalidIndex(idx)),
        }
    }

    pub fn get_col(self, idx:usize)  -> Result<Vector, MatrixError> {
        //match 0<=idx && idx<=self.ncols {
        match idx<self.ncols {
            true => {
                let mut this_col= vec![];
                for row in self.array.iter().clone().to_owned() {
                    //let new_vector = Vector {vec:row.vec.get(idx)};
                    //let idx_value = *row.vec.get(idx as usize).unwrap();
                    //let row_vec : &Vec<f32> = move || row ;
                    //let a: &Vec<f32> = &row.vec;
                    //let b = &c[idx as usize];
                    let idx_value = row.vec[idx];
                    //let idx_value = new_vector.vec[idx as usize];
                    //let idx_value = row_vec[idx as usize];
                    let _ = this_col.push(idx_value).to_owned();
                }
                Ok(Vector::from_vec(this_col))
            },
            false => Err(MatrixError::InvalidIndex(idx)),
        }
    }

    pub fn size(&self) -> usize {
        self.ncols * self.nrows
    }

    pub fn matmul(&self, other:&Matrix2d) -> Result<Matrix2d, MatrixError> {
        if !self.shape()[1]==other.shape()[0] {
            Err("arr1 ncols != arr2 nrows")
        } else {Ok(())}.unwrap();

        let mut rows = vec![];
        for r in 0..self.nrows {
            let mut this_row = vec![];
            for c in 0..other.ncols {
                let row = self.get_row(r)?;
                let col = &other.clone().get_col(c)?;
                this_row.push(row.dot(col).expect("invalid dot product within a matmul"));
            }
            rows.push(Vector::from_vec(this_row));
        }
        Ok(Matrix2d {nrows:self.nrows, ncols:other.ncols, array:rows})
    }

    pub fn scale(sx:f32, sy:f32, sz:f32) -> Matrix2d {
        Matrix2d::from_array([
            [sx, 0., 0., 0.],
            [0., sy, 0., 0.],
            [0., 0., sz, 0.],
            [0., 0., 0., 1.],
        ])
    }

    pub fn translate(t:(f32, f32, f32)) -> Matrix2d {
        Matrix2d::from_array([
            [1., 0., 0., t.0],
            [0., 1., 0., t.1],
            [0., 0., 1., t.2],
            [0., 0., 0., 1.0],
        ])
    }

    pub fn rotate(rx:f32, ry:f32, rz:f32) -> Result<Matrix2d, MatrixError> {
        let (rrx, rry, rrz) = (rx.to_radians(), ry.to_radians(), rz.to_radians());
        
        let rot_x = Matrix2d::from_array([
            [1.,           0.,         0., 0.],
            [0.,    rrx.cos(), rrx.sin(), 0.],
            [0., -1.*rrx.sin(), rrx.cos(), 0.],
            [0.,           0.,         0., 1.]
            ]);

        let rot_y = Matrix2d::from_array([
            [rry.cos(), 0., -1.*rry.sin(), 0.],
            [       0., 1.,            0., 0.],
            [rry.sin(), 0.,     rry.cos(), 0.],
            [       0., 0.,            0., 1.]
            ]);

        let rot_z = Matrix2d::from_array([
            [rrz.cos(), -1.*rrz.sin(), 0., 0.],
            [rrz.sin(),     rrz.cos(), 0., 0.],
            [       0.,            0., 1., 0.],
            [       0.,            0., 0., 1.],
            ]);

        
        Ok(rot_x.matmul(&rot_y.matmul(&rot_z)?)?)
    }

    pub fn rotate_around_p(p:(f32, f32, f32), r:(f32, f32, f32)) -> Result<Matrix2d, MatrixError> {

        // p in form (x_offset, y_offset, z_offset)
        // NOTE : for some reason, y and z switch in calculations
        // thus, p gets deconstructed as :
        let (px, pz, py) = p;
        let (rx, ry, rz) = r;

        let return_to_pos     = Matrix2d::translate((px, py, pz));
        let translate_to_zero = Matrix2d::translate((-px, -py, -pz));

        let rotate = Matrix2d::rotate(rx, ry, rz)?;

        Ok(return_to_pos.matmul(&rotate.matmul(&translate_to_zero)?)?)
    }

    pub fn opengl_to_right_handed() -> Matrix2d {
        Matrix2d::from_array([
            [1.,0.,0.,0.],
            [0.,0.,1.,0.],
            [0.,1.,0.,0.],
            [0.,0.,0.,1.],
        ])
    }

    pub fn without(&self, row_i:usize, col_j:usize) -> Result<Matrix2d, MatrixError> {
        if !(row_i<self.nrows && col_j<self.ncols) {
            Err(MatrixError::InvalidIndices([row_i, col_j]))
        } else {
            let mut v_overall : Vec<Vec<f32>> = vec![];

            for row in 0..self.nrows {
                if row != row_i {
                    let mut v_inner : Vec<f32> = vec![];
                    for col in 0.. self.ncols {
                        if col != col_j {
                            v_inner.push(self.array[row][col]);
                        }
                    }
                    v_overall.push(v_inner);
                }
            }
            
            Matrix2d::from_vec_of_vec(v_overall)
        }
    }

    pub fn multiply_by_constant(&self, scalar:f32) -> Matrix2d {
        let mut narr = self.array.clone();
        for row in 0..self.nrows {
            for col in 0..self.ncols {
                narr[row][col] = self.array[row][col] * scalar;
            }
        }
        
        Matrix2d {nrows:self.nrows, ncols:self.ncols, array:narr}
    }

    pub fn determinant(&self) -> Result<f32, MatrixError> {
        if self.ncols != self.nrows {
            Err(MatrixError::InvalidShape(self.shape()))
        } else if self.nrows == 2 {
            let a = self.array[0][0];
            let b = self.array[0][1];
            let c = self.array[1][0];
            let d = self.array[1][1];
            Ok(a*d - b*c)
        } else {
            let row_i = 0;
            let mut determinant_sum = 0.0;
            for (col_idx, col_val) in self.array[0].vec.clone().into_iter().enumerate() {
                let minor = self.without(row_i, col_idx)?.determinant()?;

                let r = f32::from(u16::try_from(u32::try_from(row_i).unwrap()).unwrap());
                let c = f32::from(u16::try_from(u32::try_from(col_idx).unwrap()).unwrap());

                let cofactor = (-1.0_f32).powf((r+1.0)+(c+1.0)) * minor;
                determinant_sum += col_val * cofactor;
            }
            Ok(determinant_sum)
        }
    }

    pub fn inverse(&self) -> Result<Matrix2d, MatrixError> {
        let determinant = self.determinant()?;
        match determinant {
            0.0 => Err(MatrixError::DeterminantIsZero),
            _ => Ok(self.clone().multiply_by_constant(determinant)),
        }
    }
}