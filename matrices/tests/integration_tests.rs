use matrices::matrix::Matrix;
use matrices::numbers::DataTypes;


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

    let is_zero1 = mat1.col_is_nul(0).unwrap();
    let is_zero2 = mat1.col_is_nul(2).unwrap();
    let is_zero4 = mat2.col_is_nul(0).unwrap();
    let is_zero5 = mat2.col_is_nul(1).unwrap();

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