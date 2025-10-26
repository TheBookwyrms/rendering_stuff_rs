use matrices::matrix::Matrix;
use matrices::enums::DataTypes;

use matrices::cartesian_product;


#[test]
fn gauss_jordan_inverse() {
    let mat = Matrix::from_2darray([
        [1., 2., 3.],
        [2., 4., 8.],
        [3., 9., 27.],
    ]);

    let inv_mat = mat.gauss_jordan_inverse().unwrap();

    assert_eq!(inv_mat, Matrix::from_2darray([
        [-6.0, 4.5, -2./3.],
        [5., -3., 1./3.],
        [-1., 0.5, 0.],
    ]))
}

#[test]
fn expand_along_axis_test2() {
    let m1 = Matrix::from_2darray([
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
    ]);

    let m2 = Matrix::from_2darray([
        [10, 11, 12],
        [13, 14, 15],
        [16, 17, 18],
    ]);

    let extend_ax0 = m1.expand_along_axis(m2, 0).unwrap();
    println!("{}", extend_ax0);

    assert_eq!(extend_ax0, Matrix::from_2darray([
        [1, 2, 3, 10, 11, 12],
        [4, 5, 6, 13, 14, 15],
        [7, 8, 9, 16, 17, 18],])
    );
}

#[test]
fn expand_along_axis_test1() {
    let m1 = Matrix::from_2darray([
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
    ]);

    let m2 = Matrix::from_2darray([
        [10, 11, 12],
        [13, 14, 15],
    ]);

    let extend_ax1 = m1.expand_along_axis(m2, 1).unwrap();

    println!("{}", extend_ax1);

    assert_eq!(extend_ax1, Matrix::from_2darray([
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
        [10, 11, 12],
        [13, 14, 15],])
    );
}


#[test]
fn cartesian_product_two_vec() {
    let a1 = vec![1, 2, 3];
    let a2 = vec![4, 5, 6];

    let cp = cartesian_product::cartesian_product([a1, a2]);

    assert_eq!(cp, vec![
        vec![1, 4],
        vec![1, 5],
        vec![1, 6],
        vec![2, 4],
        vec![2, 5],
        vec![2, 6],
        vec![3, 4],
        vec![3, 5],
        vec![3, 6],
    ]);
}
#[test]
fn cartesian_product_three_vec() {
    let a1 = vec![1, 2];
    let a2 = vec![3, 4];
    let a3 = vec![5, 6];

    let cp = cartesian_product::cartesian_product([a1, a2, a3]);

    assert_eq!(cp, vec![
        vec![1, 3, 5],
        vec![1, 3, 6],
        vec![1, 4, 5],
        vec![1, 4, 6],
        vec![2, 3, 5],
        vec![2, 3, 6],
        vec![2, 4, 5],
        vec![2, 4, 6],
    ]);
}

#[test]
fn range_indexing() {
    let mat = Matrix::from_2darray([
        [00, 01, 02, 03, 04, 05, 06, 07, 08, 09],
        [10, 11, 12, 13, 14, 15, 16, 17, 18, 19],
        [20, 21, 22, 23, 24, 25, 26, 27, 28, 29],
        [30, 31, 32, 33, 34, 35, 36, 37, 38, 39],
        [40, 41, 42, 43, 44, 45, 46, 47, 48, 49],
        [50, 51, 52, 53, 54, 55, 56, 57, 58, 59],
        [60, 61, 62, 63, 64, 65, 66, 67, 68, 69],
        [70, 71, 72, 73, 74, 75, 76, 77, 78, 79],
        [80, 81, 82, 83, 84, 85, 86, 87, 88, 89],
        [90, 91, 92, 93, 94, 95, 96, 97, 98, 99],
    ]);

    let indexed_mat = mat.get_submatrix([2..6, 3..8]).unwrap();

    let real = Matrix::from_2darray([
        [32, 33, 34, 35],
        [42, 43, 44, 45],
        [52, 53, 54, 55],
        [62, 63, 64, 65],
        [72, 73, 74, 75],
    ]);

    println!("{}", indexed_mat);

    assert_eq!(indexed_mat, real);
}

#[test]
fn solve() {
    let augmented_mat = Matrix::from_2darray([
        [2.0, 3.0, 4.0, 1.5],
        [0.0, 0.0, 9.0, 0.3],
        [1.0, 1.0, 2.0, 9.0],
    ]);

    let reduced_echelon = augmented_mat.solve().unwrap();

    println!("abc");

    let solution = Matrix::from_1darray([763.0/30.0, -16.5, 1.0/30.0,]);

    assert_eq!(reduced_echelon, solution);

}

#[test]
fn reduced_echelon_form() {
    let mat = Matrix::from_2darray([
        [2.0, 3.0, 4.0, 1.5],
        [0.0, 0.0, 9.0, 0.3],
        [1.0, 1.0, 2.0, 9.0],
    ]);

    let reduced_echelon_form = mat.reduced_echelon().unwrap();

    let reduced_echelon_algorithm_by_hand = Matrix::from_2darray([
        [1.0, 0.0, 0.0, 763.0/30.0],
        [0.0, 1.0, 0.0, -16.5],
        [0.0, 0.0, 1.0, 1.0/30.0],
    ]);


    assert_eq!(reduced_echelon_form, reduced_echelon_algorithm_by_hand);
}

#[test]
fn echelon_form() {
    let mat = Matrix::from_2darray([
        [2.0, 3.0, 4.0, 1.5],
        [0.0, 0.0, 9.0, 0.3],
        [1.0, 1.0, 2.0, 9.0],
    ]);

    let echelon_form = mat.echelon().unwrap();

    let echelon_algorithm_by_hand = Matrix::from_2darray([
        [1.0, 3.0/2.0, 2.0, 3.0/4.0],
        [0.0, 1.0, -18.0, -171.0/10.0],
        [0.0, 0.0, 1.0, 1.0/30.0],
    ]);

    assert_eq!(echelon_form, echelon_algorithm_by_hand);
}



#[test]
fn column_zeroes() {
    let mat1 = Matrix::from_2darray([
        [1.0, 2.0, 3.0],
        [4.0, 0.0, 3.0],
        [0.0, 0.0, 0.0],
    ]);
    let mat2 = Matrix::from_2darray([
        [1.0, 0.0, 3.0],
        [4.0, 0.0, 3.0],
        [0.0, 0.0, 0.0],
    ]);

    let is_zero1 = mat1.col_is_null(0).unwrap();
    let is_zero2 = mat1.col_is_null(2).unwrap();
    let is_zero4 = mat2.col_is_null(0).unwrap();
    let is_zero5 = mat2.col_is_null(1).unwrap();

    assert_eq!(is_zero1, false);
    assert_eq!(is_zero2, false);
    assert_eq!(is_zero4, false);
    assert_eq!(is_zero5, true);
}


#[test]
fn dtype() {
    let mat = Matrix::<f32>::from_scalar(23.3);
    assert_eq!(mat.dtype, DataTypes::F32);
    assert_ne!(mat.dtype, DataTypes::F64);
}

#[test]
fn indices_conversion() {
    let mat = Matrix::<u8>::new_empty(vec![2, 3, 4]);
    assert_eq!(mat.linear_index_of(vec![0, 1, 2]), 14);
    assert_eq!(19, mat.linear_index_of(vec![1, 0, 3]));
}



#[test]
fn test() {

    let a = Matrix::from_scalar(23.3);
    let b = Matrix::from_1darray([1.0, 2.0, 3.0, 4.0]);
    let c = Matrix::from_vec(vec![9.9, 8.3, 7.2]);
    let d = Matrix::from_2darray([
        [13.3],
        [9.9],
        [7.7],
        [5.5],
    ]);
    println!("{}\n|| {:?} \n\n", a, a);
    println!("{}\n|| {:?} \n\n", b, b);
    println!("{}\n|| {:?} \n\n", c, c);
    println!("{}\n|| {:?} \n\n", d, d);
    let _e = Matrix::from_vec_of_vec(vec![
        vec![13.3, 18.3, 18.3, 108.3],
        vec![9.9, 29.9, 06.0, 0.9235],
        vec![7.7, 19.9, 05.0, 0.18235],
        vec![5.5, 39.9, 40.0, 0.235],
    ]).unwrap();
    let e = Matrix::from_vec_of_vec(vec![
        vec![13.3, 18.3, 18.4],
        vec![9.9, 29.9,  6.0],
        vec![7.7, 19.9,  5.0],
        vec![5.5, 39.9, 40.0],
    ]).unwrap();
    let f = e.swap_axes(0, 1);
    println!("{}\n|| {:?} \n\n", e, e);
    println!("{}\n|| {:?} \n\n", f, f);
    let g = Matrix::from_3darray([
        [
            [1., 3.],
            [4., 6.],
            [7., 9.],
        ],
        [
            [10., 12.,],
            [13., 15.],
            [16., 18.],
        ],
        [
            [19., 21.],
            [22., 24.,],
            [25., 27.],
        ]
    ]);

    println!("{:?}, {:?}", g.shape, g.array);
    //println!("{}\n|| {:?} \n\n", g, g);
    //let h = g.swap_axes(0, 1);
    //println!("{}\n|| {:?} \n\n", h, h);

    let m1 = Matrix::from_2darray([
        [1.1, 2.2, 9.3],
        [9.9, 2.3, 8.3],
        [7.7, 2.4, 7.3],
        [5.5, 2.5, 6.3],
    ]);
    let m2 = Matrix::from_2darray([
        [7.2, 3.3, 1.8],
        [7.3, 8.6, 0.9],
        [7.4, 7.1, 2.7],
    ]);
    println!("e {}\n|| {:?} \n\n", e, e);
    let f = m1.matmul(&m2).unwrap();
    println!("f {}\n|| {:?} \n\n", f, f);

    let rc = m1.without_rc(0, 2).unwrap();
    println!("f {}\n|| {:?} \n\n", rc, rc);

    let inv = m2.inverse().unwrap();
    //let inv = m2.cofactor_matrix().unwrap();
    println!("f {}\n|| {:?} \n\n", inv, inv);

    let b = Matrix::from_1darray([1.0, 2.0, 3.0, 4.0]);
    let b = b.dot(&b).unwrap();
    println!("f {}\n|| {:?} \n\n", b, b);

    let k = Matrix::<i8>::from_1darray([1, 2, 3]);
    let c = Matrix::<i16>::from(k.clone());
    let _b = Matrix::<f32>::from(c);
    //let d = Matrix::<i8>::from(b);

    println!("{}", (k.multiply_by_constant(3)));

    assert_eq!(2+3, 5);
}