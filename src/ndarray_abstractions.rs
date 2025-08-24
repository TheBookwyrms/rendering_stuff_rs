use std::any::type_name;

use crate::ndarray_abstractions::MyArray::Arr1D;

pub mod MyArray {

    use ndarray;
    use std::any::type_name;
    use std::os::raw::c_void;
    use std::f32;
    use std::u32;

    type N1 = ndarray::ArrayBase<ndarray::OwnedRepr<f32>, ndarray::Dim<[usize; 1]>> ;
    type N2 = ndarray::ArrayBase<ndarray::OwnedRepr<f32>, ndarray::Dim<[usize; 2]>> ;
    type N3 = ndarray::ArrayBase<ndarray::OwnedRepr<f32>, ndarray::Dim<[usize; 3]>> ;
    type N4 = ndarray::ArrayBase<ndarray::OwnedRepr<f32>, ndarray::Dim<[usize; 4]>> ;


    pub fn test_arrs() {
        let k = Arr1D::from([1.0, 2.0, 3.0]);
        let k = Arr2D::from([
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
        ]);
        let k = Arr3D::from(
            [
                [
                    [1.0, 2.0, 3.0],
                    [4.0, 5.0, 6.0],
                ],
                [
                    [7.0, 8.0, 9.0],
                    [10.0, 11., 12.],
                ],
                [
                    [13., 14., 15.],
                    [16., 17., 18.],
                ],
            ]
        );
        let k = Arr4D::from([
            //[
                [
                    [
                        [1.0, 2.0, 3.0],
                        [4., 5., 6.],
                        [1.0, 2.0, 3.0],
                        [4., 5., 6.],
                    ],
                    [
                        [7., 8., 9.,],
                        [10., 11., 12.,],
                        [1.0, 2.0, 3.0],
                        [4., 5., 6.],
                    ]
                ],
                [
                    [
                        [13., 14., 15.],
                        [16., 17., 18.],
                        [1.0, 2.0, 3.0],
                        [4., 5., 6.],
                    ],
                    [
                        [19., 20., 21.],
                        [22., 23., 24.],
                        [1.0, 2.0, 3.0],
                        [4., 5., 6.],
                    ],
                ]
            //],
        ]);
        println!("{:?}", k.arr);
    }

    pub struct Arr1D {
        pub arr:N1
    }
    impl Arr1D {
        pub fn from<const K:usize>(arr_to_be:[f32; K]) -> Arr1D {
            Arr1D { arr : ndarray::ArrayBase::from_vec(arr_to_be.to_vec()) }
        }
        pub fn dot(&self, other:Arr1D) -> f32 {
            self.arr.dot(&other.arr)
        }
    }
    impl N for Arr1D {
        fn shape(&self) -> usize {self.arr.len()}
        fn as_ptr(&self) -> *const f32 {self.arr.as_ptr() as *const f32}
        fn as_ptr_void(&self) -> *const c_void {self.arr.as_ptr() as *const c_void}
        fn dimension0(&self) -> (i32) {i32::try_from(self.arr.dim()).unwrap()}
    }





    pub struct Arr2D {
        pub arr:N2
    }
    impl Arr2D {
        pub fn from<const K:usize, const L:usize>(arr_to_be:[[f32; K]; L]) -> Arr2D {
            let ncols = arr_to_be[0].len();
            let nrows = arr_to_be.len();
            let for_shape = arr_to_be.as_flattened().to_vec();
            Arr2D { arr : ndarray::Array2::from_shape_vec((nrows, ncols), for_shape).unwrap() }
        }
        pub fn dot1d(&self, other:Arr1D) -> Arr1D {
            Arr1D { arr : self.arr.dot(&other.arr) } }
        pub fn dot2d(&self, other:Arr2D) -> Arr2D { Arr2D { arr : self.arr.dot(&other.arr) } }
    }
    impl N for Arr2D {
        fn shape(&self) -> usize {self.arr.len()}
        fn as_ptr(&self) -> *const f32 {self.arr.as_ptr() as *const f32}
        fn as_ptr_void(&self) -> *const c_void {self.arr.as_ptr() as *const c_void}
        fn dimension0(&self) -> i32 {i32::try_from(self.arr.dim().0).unwrap()}
    }


    pub struct Arr3D {
        pub arr:N3
    }
    impl Arr3D {
        pub fn from<const K:usize, const L:usize, const M:usize>(arr_to_be:[[[f32; K]; L]; M]) -> Arr3D {
            let nz = arr_to_be[0][0].len();
            let ny = arr_to_be[0].len();
            let nx = arr_to_be.len();
            let for_shape = arr_to_be.as_flattened().as_flattened().to_vec();
            Arr3D { arr : ndarray::Array3::from_shape_vec((nx, ny, nz), for_shape).unwrap() }
        }
    }
    impl N for Arr3D {
        fn shape(&self) -> usize {self.arr.len()}
        fn as_ptr(&self) -> *const f32 {self.arr.as_ptr() as *const f32}
        fn as_ptr_void(&self) -> *const c_void {self.arr.as_ptr() as *const c_void}
        fn dimension0(&self) -> (i32) {i32::try_from(self.arr.dim().0).unwrap()}
    }



    pub struct Arr4D {
        pub arr:N4
    }
    impl Arr4D {
        pub fn from<const J:usize,
                const K:usize,
                const L:usize,
                const M:usize>(arr_to_be:[[[[f32; J]; K]; L]; M]) -> Arr4D {
            let n4 = arr_to_be[0][0][0].len();
            let nz = arr_to_be[0][0].len();
            let ny = arr_to_be[0].len();
            let nx = arr_to_be.len();
            let for_shape = arr_to_be.as_flattened().as_flattened().as_flattened().to_vec();
            Arr4D { arr : ndarray::Array4::from_shape_vec((nx, ny, nz, n4), for_shape).unwrap() }
        }
    }
    impl N for Arr4D {
        fn shape(&self) -> usize {self.arr.len()}
        fn as_ptr(&self) -> *const f32 {self.arr.as_ptr() as *const f32}
        fn as_ptr_void(&self) -> *const c_void {self.arr.as_ptr() as *const c_void}
        fn dimension0(&self) -> (i32) {i32::try_from(self.arr.dim().0).unwrap()}
    }



    impl N for f32 {
        fn shape(&self) -> usize { 1 }
        fn as_ptr(&self) -> *const f32 { self as *const f32 }
        fn as_ptr_void(&self) -> *const c_void { self as *const f32 as *const c_void }
        fn dimension0(&self) -> (i32) { 1 }
    }



    pub trait N {
        fn shape(&self) -> usize;
        fn as_ptr(&self) -> *const f32;
        fn as_ptr_void(&self) -> *const c_void;
        fn dimension0(&self) -> (i32);
        //fn matmul<T:N>(&self, other:Box<T>) -> T;
    }


    
}