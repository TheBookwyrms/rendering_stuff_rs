pub mod Vertices {
    use std::ops::Index;


    #[derive(Debug, Clone)]
    pub struct Vector {
        pub vec:Vec<f32>,
    }
    impl Index<usize> for Vector {
        type Output = f32;

        fn index(&self, idx:usize) -> &Self::Output {
            &self.vec[idx]
        }
    }
    impl Vector {
        fn from_vec(vec:Vec<f32>) -> Vector {
            Vector { vec: vec }
        }
        //fn from_vector(vector:Vec<Vector>) -> Vector {
        //    Vector { vec: vector }
        //}
    }

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
        pub fn from<const M:usize, const N:usize>(arr:[[f32;M];N]) -> Matrix2d {
            let mut rows= vec![];
            let mut nrows = 0;
            let mut ncols = 0;

            for row in arr {
                ncols = row.len().try_into().unwrap();
                nrows += 1;
                let mut this_col_len = 0;
                let mut this_col = vec![];
                for col in row {
                    this_col_len += 1;
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
            match 0<=idx && idx<=self.nrows {
                true => Ok(&self.array[idx as usize]),
                false => Err("invalid row index"),
            }
        }
        pub fn get_col(self, idx:u8)  -> Result<Vector, &'static str> {
            match 0<=idx && idx<=self.ncols {
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
                        this_col.push(idx_value).to_owned();
                    }
                    Ok(Vector::from_vec(this_col))
                },
                false => Err("invalid column index"),
            }
        }
        pub fn size(&self) -> usize {
            (self.ncols * self.nrows).try_into().unwrap()
        }
        //pub fn matmul(&self, other:&Matrix2d<f32>) -> Matrix2d<f32> {
        //    if !self.shape()[1]==other.shape()[0] {
        //        Err("arr1 ncols != arr2 nrows").unwrap()
        //    } else {Ok(()).unwrap()};
        //    
        //}
    }
}