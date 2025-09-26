use std::ops::{Index, Add, Sub};

use crate::vector::Vector;


#[derive(Debug, Clone)]
pub struct Matrix2d {
    pub nrows:u8,
    pub ncols:u8,
    pub array:Vec<Vector>,
}
impl Index<[usize;2]> for Matrix2d {
    type Output = f32;

    fn index(&self, idx:[usize;2]) -> &Self::Output {
        &self.array[idx[0]][idx[1]]
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
        Matrix2d {nrows:1, ncols:vec.len() as u8, array: vec![Vector {vec}]}
    }
    pub fn from_vec_of_vec(vec:Vec<Vec<f32>>) -> Result<Matrix2d, &'static str> {
        let mut homogenous_rows = true;
        let mut vectors : Vec<Vector> = vec![];
        for row in vec.clone() {
            if row.len() != vec[0].len() {
                homogenous_rows = false;
            }
            vectors.push(Vector { vec:row });
        }
        match homogenous_rows {
            true => Ok( Matrix2d { nrows:vec.len() as u8, ncols: vec[0].len() as u8, array: vectors } ),
            false => Err("not homogenous array lengths"),
        }
    }
    pub fn from_vector(vector:Vector) -> Matrix2d {
        Matrix2d {nrows:1, ncols:vector.vec.len() as u8, array: vec![vector]}
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
    pub fn shape(&self) -> [u8; 2] {[self.nrows, self.ncols]}
    pub fn transpose(self) -> Matrix2d {
        let mut new_rows = vec![];
        let ncols = self.nrows;
        for i in 0..=ncols-1 {
            let mut this_row = vec![];
            for row in self.array.iter().clone() {
            //for row in self.array.vec.iter().into_iter() {
            this_row.push(row.vec.as_slice()[i as usize]);
            }
            new_rows.push(Vector::from_vec(this_row));
        }
        let matrix = new_rows;
        Matrix2d { nrows: self.ncols, ncols: self.nrows, array: matrix }
    }
    pub fn get_row(&self, idx:u8) -> Result<&Vector, &str> {
        //match 0<=idx && idx<=self.nrows {
        match idx<self.nrows {
            true => Ok(&self.array[idx as usize]),
            false => Err("invalid row index"),
        }
    }
    pub fn get_col(self, idx:u8)  -> Result<Vector, &'static str> {
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
                    let idx_value = row.vec[idx as usize];
                    //let idx_value = new_vector.vec[idx as usize];
                    //let idx_value = row_vec[idx as usize];
                    let _ = this_col.push(idx_value).to_owned();
                }
                Ok(Vector::from_vec(this_col))
            },
            false => Err("invalid column index"),
        }
    }
    pub fn size(&self) -> usize {
        (self.ncols * self.nrows).try_into().unwrap()
    }
    pub fn matmul(&self, other:&Matrix2d) -> Matrix2d {
        if !self.shape()[1]==other.shape()[0] {
            Err("arr1 ncols != arr2 nrows")
        } else {Ok(())}.unwrap();

        let mut rows = vec![];
        for r in 0..self.nrows {
            let mut this_row = vec![];
            for c in 0..other.ncols {
                let row = self.get_row(r).unwrap();
                let col = &other.clone().get_col(c).unwrap();
                this_row.push(row.dot(col).expect("invalid dot product within a matmul"));
            }
            rows.push(Vector::from_vec(this_row));
        }
        Matrix2d {nrows:self.nrows, ncols:other.ncols, array:rows}
    }
}
impl Add for Matrix2d {
    type Output = Result<Self, &'static str>;

    fn add(self, other: Self) -> Result<Self, &'static str> {
        if self.shape() == other.shape() {
            let mut arr: Vec<Vec<f32>> = vec![vec![0.0;self.shape()[0] as usize]; self.shape()[1] as usize];
            for i in 0..self.shape()[0] as usize {
                for j in 0..self.shape()[1] as usize {
                    arr[i][j] = self[[i, j]] + other[[i, j]]
                } 
            }
            Matrix2d::from_vec_of_vec(arr)
        } else { Err("can't add two vectors of different lengths") }
    }
}
impl Sub for Matrix2d {
    type Output = Result<Self, &'static str>;

    fn sub(self, other: Self) -> Result<Self, &'static str> {
        if self.shape() == other.shape() {
            let mut arr: Vec<Vec<f32>> = vec![vec![0.0;self.shape()[0] as usize]; self.shape()[1] as usize];
            for i in 0..self.shape()[0] as usize {
                for j in 0..self.shape()[1] as usize {
                    arr[i][j] = self[[i, j]] - other[[i, j]]
                } 
            }
            Matrix2d::from_vec_of_vec(arr)
        } else { Err("can't add two vectors of different lengths") }
    }
}