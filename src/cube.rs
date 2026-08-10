//use numeracy::matrices::Matrix;
use numeracy::{matrices::{Matrix, S2}, vectors::Vector};

fn arr7(pos:[f32; 3], col:[f32; 4]) -> [f32; 7] {
    [pos[0], pos[1], pos[2], col[0], col[1], col[2], col[3]]
}

fn arr5(a3:[f32; 3], a2:[f32; 2]) -> [f32; 5] {
    [a3[0], a3[1], a3[2], a2[0], a2[1]]
}

fn arr10(pos:[f32; 3], col:[f32; 4], norm:[f32;3]) -> [f32; 10] {
    [pos[0], pos[1], pos[2], col[0], col[1], col[2], col[3], norm[0], norm[1], norm[2]]
}

fn arr12(pos:[f32; 3], col:[f32; 4], norm:[f32;3], tex:[f32; 2]) -> [f32; 12] {
    [pos[0], pos[1], pos[2], col[0], col[1], col[2], col[3], norm[0], norm[1], norm[2], tex[0], tex[1]]
}

fn arr3_of_arr7(positions:[[f32;3];3], col:[f32;4]) -> [[f32;7];3] {
    [
        arr7(positions[0], col),
        arr7(positions[1], col),
        arr7(positions[2], col),
    ]
}

fn arr3_of_arr10(positions:[[f32;3];3], col:[f32;4], norm:[f32;3]) -> [[f32;10];3] {
    [
        arr10(positions[0], col, norm),
        arr10(positions[1], col, norm),
        arr10(positions[2], col, norm),
    ]
}

/// r, g, b, a
fn get_face_colours() -> [[f32; 4]; 6] {    
    let c0 = [1.0, 0.0, 0.0, 1.0];
    let c1 = [0.0, 1.0, 0.0, 1.0];
    let c2 = [0.0, 0.0, 1.0, 1.0];
    let c3 = [1.0, 1.0, 0.0, 1.0];
    let c4 = [1.0, 0.0, 1.0, 1.0];
    let c5 = [0.0, 1.0, 1.0, 1.0];

    //let c0 = [1., 1., 1., 1.0];
    //let c1 = [1., 1., 1., 1.0];
    //let c2 = [1., 1., 1., 1.0];
    //let c3 = [1., 1., 1., 1.0];
    //let c4 = [1., 1., 1., 1.0];
    //let c5 = [1., 1., 1., 1.0];

    [c0, c1, c2, c3, c4, c5]
}

fn get_normals() -> [[f32;3];12] {
    let norm_top1 = [ 0.,  1.,  0.];
    let norm_top2 = [ 0.,  1.,  0.];
    let norm_front1 = [ 0.,  0.,  1.];
    let norm_front2 = [ 0.,  0.,  1.];
    let norm_bottom1 = [ 0., -1.,  0.];
    let norm_bottom2 = [ 0., -1.,  0.];
    let norm_back1 = [ 0.,  0., -1.];
    let norm_back2 = [ 0.,  0., -1.];
    let norm_right1 = [ 1.,  0.,  0.];
    let norm_right2 = [ 1.,  0.,  0.];
    let norm_left1 = [-1.,  0.,  0.];
    let norm_left2 = [-1.,  0.,  0.];

    [
        norm_top1, norm_top2, norm_front1, norm_front2,
        norm_bottom1, norm_bottom2, norm_back1, norm_back2,
        norm_right1, norm_right2, norm_left1, norm_left2,
    ]
}

fn get_tex_face_vals(texture_size:f32) -> [[f32; 2]; 4] {
    let tex_tr = [texture_size, texture_size]; // texture top right
    let tex_br = [texture_size,          0.0]; // texture bottom right
    let tex_bl = [         0.0,          0.0]; // texture bottom left
    let tex_tl = [         0.0, texture_size]; // texture top left

    [tex_tr, tex_br, tex_bl, tex_tl]
}

pub fn create_cube_vertices(centre:(f32, f32, f32), side_len:f32) -> [[f32; 3]; 8] {
    let (x, y, z) = centre;

    let top_front_right    = [x+side_len/2.0, y+side_len/2.0, z+side_len/2.0];
    let top_back_right     = [x+side_len/2.0, y+side_len/2.0, z-side_len/2.0];
    let top_front_left     = [x-side_len/2.0, y+side_len/2.0, z+side_len/2.0];
    let top_back_left      = [x-side_len/2.0, y+side_len/2.0, z-side_len/2.0];
    let bottom_front_right = [x+side_len/2.0, y-side_len/2.0, z+side_len/2.0];
    let bottom_back_right  = [x+side_len/2.0, y-side_len/2.0, z-side_len/2.0];
    let bottom_front_left  = [x-side_len/2.0, y-side_len/2.0, z+side_len/2.0];
    let bottom_back_left   = [x-side_len/2.0, y-side_len/2.0, z-side_len/2.0];


    [
        top_back_right,
        top_front_right,
        top_front_left,
        top_back_left,
        bottom_back_right,
        bottom_front_right,
        bottom_front_left,
        bottom_back_left,
    ]
}


//pub fn _might_not_even_work_untested_ebo_cube(centre:(f32, f32, f32), side_len:f32) -> (Matrix<f32, 2>, Matrix<i32, 2>) {
//
//    let vertices_matrix = Matrix {
//        shape:[3, 8],
//        array:create_cube_vertices(centre, side_len).concat(),
//    };
//
//    let indices_arr = [
//        0, 3, 1, // top 1
//        2, 3, 1, // top 2
//        2, 6, 1, // front 1
//        5, 6, 1, // front 2
//        5, 6, 4, // bottom 1
//        7, 6, 4, // bottom 2
//        7, 3, 4, // back 1
//        0, 3, 4, // back 2
//        0, 1, 4, // right 1
//        5, 1, 4, // right 2
//        2, 3, 6, // left 1
//        7, 3, 6, // left 2
//    ];
//
//    let indices_matrix = Matrix {
//        shape:[3, 12],
//        array:indices_arr.to_vec(),
//    };
//
//    (vertices_matrix, indices_matrix)
//
//}

 

 

pub fn colour_cube(centre:(f32, f32, f32), side_len:f32, include_normals:bool) -> Matrix<f32, 2, S2<10, 36>> {
    
    let [
        tbr, tfr, tfl, tbl, bbr, bfr, bfl, bbl
    ] = create_cube_vertices(centre, side_len);

    let [c0, c1, c2, c3, c4, c5] = get_face_colours();

    let cube = if !include_normals {
        let top1     = arr3_of_arr7([tbr, tbl, tfr], c0);
        let top2     = arr3_of_arr7([tfl, tbl, tfr], c0);
        let front1   = arr3_of_arr7([tfl, bfl, tfr], c1);
        let front2   = arr3_of_arr7([bfr, bfl, tfr], c1);
        let bottom1  = arr3_of_arr7([bfr, bfl, bbr], c2);
        let bottom2  = arr3_of_arr7([bbl, bfl, bbr], c2);
        let back1    = arr3_of_arr7([bbl, tbl, bbr], c3);
        let back2    = arr3_of_arr7([tbr, tbl, bbr], c3);
        let right1   = arr3_of_arr7([tbr, tfr, bbr], c4);
        let right2   = arr3_of_arr7([bfr, tfr, bbr], c4);
        let left1    = arr3_of_arr7([tfl, tbl, bfl], c5);
        let left2    = arr3_of_arr7([bbl, tbl, bfl], c5);
        
        vec![
            top1.concat(),
            top2.concat(),
            front1.concat(),
            front2.concat(),
            bottom1.concat(),
            bottom2.concat(),
            back1.concat(),
            back2.concat(),
            right1.concat(),
            right2.concat(),
            left1.concat(),
            left2.concat(),
        ]
    } else {
        
        let top1     = arr3_of_arr10([tbr, tbl, tfr], c0, [ 0.,  1.,  0.]);
        let top2     = arr3_of_arr10([tfl, tbl, tfr], c0, [ 0.,  1.,  0.]);
        let front1   = arr3_of_arr10([tfl, bfl, tfr], c1, [ 0.,  0.,  1.]);
        let front2   = arr3_of_arr10([bfr, bfl, tfr], c1, [ 0.,  0.,  1.]);
        let bottom1  = arr3_of_arr10([bfr, bfl, bbr], c2, [ 0., -1.,  0.]);
        let bottom2  = arr3_of_arr10([bbl, bfl, bbr], c2, [ 0., -1.,  0.]);
        let back1    = arr3_of_arr10([bbl, tbl, bbr], c3, [ 0.,  0., -1.]);
        let back2    = arr3_of_arr10([tbr, tbl, bbr], c3, [ 0.,  0., -1.]);
        let right1   = arr3_of_arr10([tbr, tfr, bbr], c4, [ 1.,  0.,  0.]);
        let right2   = arr3_of_arr10([bfr, tfr, bbr], c4, [ 1.,  0.,  0.]);
        let left1    = arr3_of_arr10([tfl, tbl, bfl], c5, [-1.,  0.,  0.]);
        let left2    = arr3_of_arr10([bbl, tbl, bfl], c5, [-1.,  0.,  0.]);
        
        vec![
            top1.concat(),
            top2.concat(),
            front1.concat(),
            front2.concat(),
            bottom1.concat(),
            bottom2.concat(),
            back1.concat(),
            back2.concat(),
            right1.concat(),
            right2.concat(),
            left1.concat(),
            left2.concat(),
        ]
    }.concat();

    Matrix { shape: S2::<10, 36>, array: cube }
}






pub fn texture_cube(centre:(f32, f32, f32), side_len:f32, texture_size:f32) -> (Matrix<f32, 2, S2<5, 36>>) {
        
    let [
        tbr, tfr, tfl, tbl, bbr, bfr, bfl, bbl
    ] = create_cube_vertices(centre, side_len);


    let [tex_tr, tex_br, tex_bl, tex_tl] = get_tex_face_vals(texture_size);


    // original
    let top1    = [arr5(tbr, tex_tr), arr5(tbl, tex_tl), arr5(tfr, tex_br)];
    let top2    = [arr5(tfl, tex_bl), arr5(tbl, tex_tl), arr5(tfr, tex_br)];
    let front1  = [arr5(tfl, tex_tl), arr5(bfl, tex_bl), arr5(tfr, tex_tr)];
    let front2  = [arr5(bfr, tex_br), arr5(bfl, tex_bl), arr5(tfr, tex_tr)];
    let bottom1 = [arr5(bfr, tex_tr), arr5(bfl, tex_tl), arr5(bbr, tex_br)];
    let bottom2 = [arr5(bbl, tex_bl), arr5(bfl, tex_tl), arr5(bbr, tex_br)];
    let back1   = [arr5(bbl, tex_tl), arr5(tbl, tex_bl), arr5(bbr, tex_tr)];
    let back2   = [arr5(tbr, tex_br), arr5(tbl, tex_bl), arr5(bbr, tex_tr)];
    let right1  = [arr5(tbr, tex_tr), arr5(tfr, tex_tl), arr5(bbr, tex_br)];
    let right2  = [arr5(bfr, tex_bl), arr5(tfr, tex_tl), arr5(bbr, tex_br)];
    let left1   = [arr5(tfl, tex_tr), arr5(tbl, tex_tl), arr5(bfl, tex_br)];
    let left2   = [arr5(bbl, tex_bl), arr5(tbl, tex_tl), arr5(bfl, tex_br)];

    let cube = vec![
        top1.concat(),
        top2.concat(),
        front1.concat(),
        front2.concat(),
        bottom1.concat(),
        bottom2.concat(),
        back1.concat(),
        back2.concat(),
        right1.concat(),
        right2.concat(),
        left1.concat(),
        left2.concat(),
    ].concat();


    let mut cube_matrix =     Matrix { shape: S2::<5, 36>, array: cube };


    cube_matrix
}

 



pub fn texture_colour_cube(centre:(f32, f32, f32), side_len:f32, texture_size:f32) -> Matrix<f32, 2, S2<12, 36>> {

    let [
        tbr, tfr, tfl, tbl, bbr, bfr, bfl, bbl
    ] = create_cube_vertices(centre, side_len);

    let [c0, c1, c2, c3, c4, c5] = get_face_colours();
    let [tex_tr, tex_br, tex_bl, tex_tl] = get_tex_face_vals(texture_size);

    //let [
    //    ntr1, ntr2, nfr1, nfr2,
    //    nbo1, nbo2, nba1, nba2,
    //    nri1, nri2, nle1, nle2
    //] = get_normals();
//
    //
    //let top1    = [[tbr, tbl, tfr].concat(), c0.to_vec(), ntr1.to_vec(), [tex_tr, tex_tl, tex_br].concat()];
    //let top2    = [[tfl, tbl, tfr].concat(), c0.to_vec(), ntr2.to_vec(), [tex_bl, tex_tl, tex_br].concat()];
    //let front1  = [[tfl, bfl, tfr].concat(), c1.to_vec(), nfr1.to_vec(), [tex_tl, tex_bl, tex_tr].concat()];
    //let front2  = [[bfr, bfl, tfr].concat(), c1.to_vec(), nfr2.to_vec(), [tex_br, tex_bl, tex_tr].concat()];
    //let bottom1 = [[bfr, bfl, bbr].concat(), c2.to_vec(), nbo1.to_vec(), [tex_tr, tex_tl, tex_br].concat()];
    //let bottom2 = [[bbl, bfl, bbr].concat(), c2.to_vec(), nbo2.to_vec(), [tex_bl, tex_tl, tex_br].concat()];
    //let back1   = [[bbl, tbl, bbr].concat(), c3.to_vec(), nba1.to_vec(), [tex_tl, tex_bl, tex_tr].concat()];
    //let back2   = [[tbr, tbl, bbr].concat(), c3.to_vec(), nba2.to_vec(), [tex_br, tex_bl, tex_tr].concat()];
    //let right1  = [[tbr, tfr, bbr].concat(), c4.to_vec(), nri1.to_vec(), [tex_tr, tex_tl, tex_br].concat()];
    //let right2  = [[bfr, tfr, bbr].concat(), c4.to_vec(), nri2.to_vec(), [tex_bl, tex_tl, tex_br].concat()];
    //let left1   = [[tfl, tbl, bfl].concat(), c5.to_vec(), nle1.to_vec(), [tex_tr, tex_tl, tex_br].concat()];
    //let left2   = [[bbl, tbl, bfl].concat(), c5.to_vec(), nle2.to_vec(), [tex_bl, tex_tl, tex_br].concat()];


    let norm_top1 = [ 0.,  1.,  0.];
let norm_top2 = [ 0.,  1.,  0.];
let norm_front1 = [ 0.,  0.,  1.];
let norm_front2 = [ 0.,  0.,  1.];
let norm_bottom1 = [ 0., -1.,  0.];
let norm_bottom2 = [ 0., -1.,  0.];
let norm_back1 = [ 0.,  0., -1.];
let norm_back2 = [ 0.,  0., -1.];
let norm_right1 = [ 1.,  0.,  0.];
let norm_right2 = [ 1.,  0.,  0.];
let norm_left1 = [-1.,  0.,  0.];
let norm_left2 = [-1.,  0.,  0.];

    
    let top1    = [
        arr12(tbr, c0, norm_top1, tex_tr),
        arr12(tbl, c0, norm_top1, tex_tl),
        arr12(tfr, c0, norm_top1, tex_br)
        ];
    let top2    = [
        arr12(tfl, c0, norm_top2, tex_bl),
        arr12(tbl, c0, norm_top2, tex_tl),
        arr12(tfr, c0, norm_top2, tex_br)
        ];
    let front1  = [
        arr12(tfl, c1, norm_front1, tex_tl),
        arr12(bfl, c1, norm_front1, tex_bl),
        arr12(tfr, c1, norm_front1, tex_tr)
        ];
    let front2  = [
        arr12(bfr, c1, norm_front2, tex_br),
        arr12(bfl, c1, norm_front2, tex_bl),
        arr12(tfr, c1, norm_front2, tex_tr)
        ];
    let bottom1 = [
        arr12(bfr, c2, norm_bottom1, tex_tr),
        arr12(bfl, c2, norm_bottom1, tex_tl),
        arr12(bbr, c2, norm_bottom1, tex_br)
        ];
    let bottom2 = [
        arr12(bbl, c2, norm_bottom2, tex_bl),
        arr12(bfl, c2, norm_bottom2, tex_tl),
        arr12(bbr, c2, norm_bottom2, tex_br)
        ];
    let back1   = [
        arr12(bbl, c3, norm_back1, tex_tl),
        arr12(tbl, c3, norm_back1, tex_bl),
        arr12(bbr, c3, norm_back1, tex_tr)
        ];
    let back2   = [
        arr12(tbr, c3, norm_back2, tex_br),
        arr12(tbl, c3, norm_back2, tex_bl),
        arr12(bbr, c3, norm_back2, tex_tr)
        ];
    let right1  = [
        arr12(tbr, c4, norm_right1, tex_tr),
        arr12(tfr, c4, norm_right1, tex_tl),
        arr12(bbr, c4, norm_right1, tex_br)
        ];
    let right2  = [
        arr12(bfr, c4, norm_right2, tex_bl),
        arr12(tfr, c4, norm_right2, tex_tl),
        arr12(bbr, c4, norm_right2, tex_br)
        ];
    let left1   = [
        arr12(tfl, c5, norm_left1, tex_tr),
        arr12(tbl, c5, norm_left1, tex_tl),
        arr12(bfl, c5, norm_left1, tex_br)
        ];
    let left2   = [
        arr12(bbl, c5, norm_left2, tex_bl),
        arr12(tbl, c5, norm_left2, tex_tl),
        arr12(bfl, c5, norm_left2, tex_br)
        ];

    let cube = vec![
        top1.concat(),
        top2.concat(),
        front1.concat(),
        front2.concat(),
        bottom1.concat(),
        bottom2.concat(),
        back1.concat(),
        back2.concat(),
        right1.concat(),
        right2.concat(),
        left1.concat(),
        left2.concat(),
    ].concat();

    
    let mut cube_matrix =     Matrix { shape: S2::<12, 36>, array: cube };

    cube_matrix
}






































































pub fn colour_cube_new(centre:(f32, f32, f32), side_len:f32) -> (Matrix<f32, 2, S2<3, 36>>, Matrix<f32, 2, S2<4, 36>>) {
    
    let [
        tbr, tfr, tfl, tbl, bbr, bfr, bfl, bbl
    ] = create_cube_vertices(centre, side_len);

    let [c0, c1, c2, c3, c4, c5] = get_face_colours();

    let top1     = ([tbr, tbl, tfr], [c0;3]);
    let top2     = ([tfl, tbl, tfr], [c0;3]);
    let front1   = ([tfl, bfl, tfr], [c1;3]);
    let front2   = ([bfr, bfl, tfr], [c1;3]);
    let bottom1  = ([bfr, bfl, bbr], [c2;3]);
    let bottom2  = ([bbl, bfl, bbr], [c2;3]);
    let back1    = ([bbl, tbl, bbr], [c3;3]);
    let back2    = ([tbr, tbl, bbr], [c3;3]);
    let right1   = ([tbr, tfr, bbr], [c4;3]);
    let right2   = ([bfr, tfr, bbr], [c4;3]);
    let left1    = ([tfl, tbl, bfl], [c5;3]);
    let left2    = ([bbl, tbl, bfl], [c5;3]);
        
    let position_array = Vector::<f32, {3*36}>::from_vec(vec![
        top1.0, top2.0, front1.0, front2.0,
        bottom1.0, bottom2.0, back1.0, back2.0,
        right1.0, right2.0, left1.0, left2.0,
    ].concat().concat());

    let colour_array = Vector::<f32, {4*36}>::from_vec(vec![
        top1.1, top2.1, front1.1, front2.1,
        bottom1.1, bottom2.1, back1.1, back2.1,
        right1.1, right2.1, left1.1, left2.1,
    ].concat().concat());

    let position_matrix = Matrix::from_vector(position_array).reshape(S2::<3, 36>).unwrap();
    let colour_matrix   = Matrix::from_vector(colour_array).reshape(S2::<4, 36>).unwrap();

    (position_matrix, colour_matrix)
}






pub fn colour_normal_cube_new(centre:(f32, f32, f32), side_len:f32) -> (Matrix<f32, 2, S2<3, 36>>, Matrix<f32, 2, S2<4, 36>>, Matrix<f32, 2, S2<3, 36>>) {
    
    let [
        tbr, tfr, tfl, tbl, bbr, bfr, bfl, bbl
    ] = create_cube_vertices(centre, side_len);

    let [
        ntr1, ntr2, nfr1, nfr2,
        nbo1, nbo2, nba1, nba2,
        nri1, nri2, nle1, nle2
    ] = get_normals();

    let [c0, c1, c2, c3, c4, c5] = get_face_colours();
    let top1     = ([tbr, tbl, tfr], [c0;3], [ntr1;3]);
    let top2     = ([tfl, tbl, tfr], [c0;3], [ntr2;3]);
    let front1   = ([tfl, bfl, tfr], [c1;3], [nfr1;3]);
    let front2   = ([bfr, bfl, tfr], [c1;3], [nfr2;3]);
    let bottom1  = ([bfr, bfl, bbr], [c2;3], [nbo1;3]);
    let bottom2  = ([bbl, bfl, bbr], [c2;3], [nbo2;3]);
    let back1    = ([bbl, tbl, bbr], [c3;3], [nba1;3]);
    let back2    = ([tbr, tbl, bbr], [c3;3], [nba2;3]);
    let right1   = ([tbr, tfr, bbr], [c4;3], [nri1;3]);
    let right2   = ([bfr, tfr, bbr], [c4;3], [nri2;3]);
    let left1    = ([tfl, tbl, bfl], [c5;3], [nle1;3]);
    let left2    = ([bbl, tbl, bfl], [c5;3], [nle2;3]);
        
    let position_array = Vector::<f32, {3*36}>::from_vec(vec![
        top1.0, top2.0, front1.0, front2.0,
        bottom1.0, bottom2.0, back1.0, back2.0,
        right1.0, right2.0, left1.0, left2.0,
    ].concat().concat());

    let colour_array = Vector::<f32, {4*36}>::from_vec(vec![
        top1.1, top2.1, front1.1, front2.1,
        bottom1.1, bottom2.1, back1.1, back2.1,
        right1.1, right2.1, left1.1, left2.1,
    ].concat().concat());

    let normal_array = Vector::<f32, {3*36}>::from_vec(vec![
        top1.2, top2.2, front1.2, front2.2,
        bottom1.2, bottom2.2, back1.2, back2.2,
        right1.2, right2.2, left1.2, left2.2,
    ].concat().concat());



    let position_matrix = Matrix::from_vector(position_array).reshape(S2::<3, 36>).unwrap();
    let colour_matrix   = Matrix::from_vector(colour_array).reshape(S2::<4, 36>).unwrap();
    let normal_matrix   = Matrix::from_vector(normal_array).reshape(S2::<3, 36>).unwrap();

    (position_matrix, colour_matrix, normal_matrix)
}






pub fn texture_cube_new(centre:(f32, f32, f32), side_len:f32, texture_size:f32) -> (Matrix<f32, 2, S2<3, 36>>, Matrix<f32, 2, S2<2, 36>>) {
        
    let [
        tbr, tfr, tfl, tbl, bbr, bfr, bfl, bbl
    ] = create_cube_vertices(centre, side_len);


    let [tex_tr, tex_br, tex_bl, tex_tl] = get_tex_face_vals(texture_size);


    // original
    let top1    = ([tbr, tbl, tfr], [tex_tr, tex_tl, tex_br]);
    let top2    = ([tfl, tbl, tfr], [tex_bl, tex_tl, tex_br]);
    let front1  = ([tfl, bfl, tfr], [tex_tl, tex_bl, tex_tr]);
    let front2  = ([bfr, bfl, tfr], [tex_br, tex_bl, tex_tr]);
    let bottom1 = ([bfr, bfl, bbr], [tex_tr, tex_tl, tex_br]);
    let bottom2 = ([bbl, bfl, bbr], [tex_bl, tex_tl, tex_br]);
    let back1   = ([bbl, tbl, bbr], [tex_tl, tex_bl, tex_tr]);
    let back2   = ([tbr, tbl, bbr], [tex_br, tex_bl, tex_tr]);
    let right1  = ([tbr, tfr, bbr], [tex_tr, tex_tl, tex_br]);
    let right2  = ([bfr, tfr, bbr], [tex_bl, tex_tl, tex_br]);
    let left1   = ([tfl, tbl, bfl], [tex_tr, tex_tl, tex_br]);
    let left2   = ([bbl, tbl, bfl], [tex_bl, tex_tl, tex_br]);

    let position_array = Vector::<f32, {3*36}>::from_vec(vec![
        top1.0, top2.0, front1.0, front2.0,
        bottom1.0, bottom2.0, back1.0, back2.0,
        right1.0, right2.0, left1.0, left2.0,
    ].concat().concat());

    let texture_array = Vector::<f32, {2*36}>::from_vec(vec![
        top1.1, top2.1, front1.1, front2.1,
        bottom1.1, bottom2.1, back1.1, back2.1,
        right1.1, right2.1, left1.1, left2.1,
    ].concat().concat());




    let position_matrix = Matrix::from_vector(position_array).reshape(S2::<3, 36>).unwrap();
    let texture_matrix  = Matrix::from_vector(texture_array).reshape(S2::<2, 36>).unwrap();

    (position_matrix, texture_matrix)
}

 



pub fn texture_colour_cube_new(centre:(f32, f32, f32), side_len:f32, texture_size:f32) -> (Matrix<f32, 2, S2<3, 36>>, Matrix<f32, 2, S2<4, 36>>, Matrix<f32, 2, S2<3, 36>>, Matrix<f32, 2, S2<2, 36>>) {

    let [
        tbr, tfr, tfl, tbl, bbr, bfr, bfl, bbl
    ] = create_cube_vertices(centre, side_len);

    let [c0, c1, c2, c3, c4, c5] = get_face_colours();
    let [tex_tr, tex_br, tex_bl, tex_tl] = get_tex_face_vals(texture_size);

    let [
        ntr1, ntr2, nfr1, nfr2,
        nbo1, nbo2, nba1, nba2,
        nri1, nri2, nle1, nle2
    ] = get_normals();


    
    let top1    = ([tbr, tbl, tfr], [c0;3], [ntr1;3], [tex_tr, tex_tl, tex_br]);
    let top2    = ([tfl, tbl, tfr], [c0;3], [ntr2;3], [tex_bl, tex_tl, tex_br]);
    let front1  = ([tfl, bfl, tfr], [c1;3], [nfr1;3], [tex_tl, tex_bl, tex_tr]);
    let front2  = ([bfr, bfl, tfr], [c1;3], [nfr2;3], [tex_br, tex_bl, tex_tr]);
    let bottom1 = ([bfr, bfl, bbr], [c2;3], [nbo1;3], [tex_tr, tex_tl, tex_br]);
    let bottom2 = ([bbl, bfl, bbr], [c2;3], [nbo2;3], [tex_bl, tex_tl, tex_br]);
    let back1   = ([bbl, tbl, bbr], [c3;3], [nba1;3], [tex_tl, tex_bl, tex_tr]);
    let back2   = ([tbr, tbl, bbr], [c3;3], [nba2;3], [tex_br, tex_bl, tex_tr]);
    let right1  = ([tbr, tfr, bbr], [c4;3], [nri1;3], [tex_tr, tex_tl, tex_br]);
    let right2  = ([bfr, tfr, bbr], [c4;3], [nri2;3], [tex_bl, tex_tl, tex_br]);
    let left1   = ([tfl, tbl, bfl], [c5;3], [nle1;3], [tex_tr, tex_tl, tex_br]);
    let left2   = ([bbl, tbl, bfl], [c5;3], [nle2;3], [tex_bl, tex_tl, tex_br]);

    let position_array = Vector::<f32, {3*36}>::from_vec(vec![
        top1.0, top2.0, front1.0, front2.0,
        bottom1.0, bottom2.0, back1.0, back2.0,
        right1.0, right2.0, left1.0, left2.0,
    ].concat().concat());

    let colour_array = Vector::<f32, {4*36}>::from_vec(vec![
        top1.1, top2.1, front1.1, front2.1,
        bottom1.1, bottom2.1, back1.1, back2.1,
        right1.1, right2.1, left1.1, left2.1,
    ].concat().concat());

    let normal_array = Vector::<f32, {3*36}>::from_vec(vec![
        top1.2, top2.2, front1.2, front2.2,
        bottom1.2, bottom2.2, back1.2, back2.2,
        right1.2, right2.2, left1.2, left2.2,
    ].concat().concat());

    let texture_array = Vector::<f32, {2*36}>::from_vec(vec![
        top1.3, top2.3, front1.3, front2.3,
        bottom1.3, bottom2.3, back1.3, back2.3,
        right1.3, right2.3, left1.3, left2.3,
    ].concat().concat());


    let position_matrix = Matrix::from_vector(position_array).reshape(S2::<3, 36>).unwrap();
    let colour_matrix   = Matrix::from_vector(colour_array).reshape(S2::<4, 36>).unwrap();
    let normal_matrix   = Matrix::from_vector(normal_array).reshape(S2::<3, 36>).unwrap();
    let texture_matrix  = Matrix::from_vector(texture_array).reshape(S2::<2, 36>).unwrap();

    (position_matrix, colour_matrix, normal_matrix, texture_matrix)
}