use std::ops::{Add, Index, IndexMut, Sub};



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
impl IndexMut<usize> for Vector {
    fn index_mut(&mut self, idx:usize) -> &mut f32 {
        &mut self.vec[idx]
    }
}


impl Add for Vector {
    type Output = Result<Self, &'static str>;

    fn add(self, other: Self) -> Result<Self, &'static str> {
        if self.len() == other.len() {
            let mut arr = vec![0.0; self.len()];
            for i in 0..self.vec.len() {
                arr[i] = self[i] + other[i]
            }
            Ok(Vector::from_vec(arr))
        } else { Err("can't add two vectors of different lengths") }
    }
}
impl Sub for Vector {
    type Output = Result<Self, &'static str>;

    fn sub(self, other: Self) -> Result<Self, &'static str> {
        if self.len() == other.len() {
            let mut arr = vec![0.0; self.len()];
            for i in 0..self.vec.len() {
                arr[i] = self[i] - other[i]
            }
            Ok(Vector::from_vec(arr))
        } else { Err("can't add two vectors of different lengths") }
    }
}


impl Vector {
    pub fn len(&self) -> usize {
        self.vec.len()
    }
    pub fn from_1darray<const M:usize>(arr:[f32;M]) -> Vector {
        Vector { vec: arr.to_vec() }
    }
    pub fn from_vec(vec:Vec<f32>) -> Vector {
        Vector { vec: vec }
    }
    pub fn dot(&self, other:&Vector) -> Result<f32, &'static str> {
        if self.len() == other.len() {
            let mut sum: f32 = 0.0;
            for i in 0..=self.vec.len()-1 {
                sum += self.vec[i]*other.vec[i]
            }
            Ok(sum)
        } else { Err("can't dot product two vectors of different lengths") }
    }
}