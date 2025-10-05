use std::ops::{Add, Index, IndexMut, Sub};



#[derive(Debug, Clone)]
pub struct Vector<const K:usize> {
    pub vec:[f32; K],
}


impl<const K:usize> Index<usize> for Vector<K> {
    type Output = f32;

    fn index(&self, idx:usize) -> &Self::Output {
        &self.vec[idx]
    }
}
impl<const K:usize> IndexMut<usize> for Vector<K> {
    fn index_mut(&mut self, idx:usize) -> &mut f32 {
        &mut self.vec[idx]
    }
}


impl<const K:usize> Add for Vector<K> {
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
impl<const K:usize> Sub for Vector<K> {
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


impl<const K:usize> Vector<K> {
    pub fn len(&self) -> usize {
        self.vec.len()
    }
    pub fn from_1darray(arr:[f32;K]) -> Vector<K> {
        Vector { vec: arr }
    }
    pub fn from_vec(vec:Vec<f32>) -> Vector<K> {
        Vector { vec: vec.try_into().unwrap() }
    }
    pub fn dot(&self, other:&Vector<K>) -> Result<f32, &'static str> {
        if self.len() == other.len() {
            let mut sum: f32 = 0.0;
            for i in 0..=self.vec.len()-1 {
                sum += self.vec[i]*other.vec[i]
            }
            Ok(sum)
        } else { Err("can't dot product two vectors of different lengths") }
    }
}