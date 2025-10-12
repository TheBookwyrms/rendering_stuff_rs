#![allow(warnings)]

// uncomment for release
// #![windows_subsystem = "windows"]





use shaders::{ProgramHolder, ProgramType};
use render_context::Render;
use opengl::{GlSettings, WithObject};
use matrices::Matrix2d;
//use matrices::_tests::matrix_as_1_array::Matrix;
use matrices::_tests::matrix_with_types::matrix::Matrix;


use std::fs;


pub fn square_top_left(side_len:f32, xyz:[f32;3], rgb:Vec<&str>, a:f32) -> Matrix<f32> {
    let r : f32 = rgb[0].trim().parse().unwrap();
    let g : f32 = rgb[1].trim().parse().unwrap();
    let b : f32 = rgb[2].trim().parse().unwrap();
    //println!("{}, {}, {}", r, g, b);
    Matrix::from_2darray([
        [xyz[0]         , xyz[1],          xyz[2], r/256.0, g/256.0, b/256.0, a],
        [xyz[0]         , xyz[1]-side_len, xyz[2], r/256.0, g/256.0, b/256.0, a],
        [xyz[0]+side_len, xyz[1]-side_len, xyz[2], r/256.0, g/256.0, b/256.0, a],
        [xyz[0]+side_len, xyz[1],          xyz[2], r/256.0, g/256.0, b/256.0, a],
        [xyz[0]+side_len, xyz[1]-side_len, xyz[2], r/256.0, g/256.0, b/256.0, a],
        [xyz[0]         , xyz[1],          xyz[2], r/256.0, g/256.0, b/256.0, a],
    ])
}





pub fn view_ppm_from_path(ppm_path:&'static str) {
    let ppm = fs::read_to_string(ppm_path).expect("failed to read ppm file");
    //println!("{}", ppm);
    view_ppm_from_text(ppm);
}

pub fn view_ppm_from_text(ppm_text:String) {
    let ppm_text_lines : Vec<&str> = ppm_text.as_str().split("\r\n").collect();

    assert_eq!(ppm_text_lines[0], "P3");
    let line2 : Vec<&str> = ppm_text_lines[1].split(" ").collect();
    let (width, height) : (usize, usize) = (line2[0].trim().parse().unwrap(), line2[1].trim().parse().unwrap());
    let max_colour : f32 = ppm_text_lines[2].trim().parse().unwrap();



    let mut render = Render::default();
    render.setup_render();


    //let mut squares_matrix = Matrix2d::new_empty(0, 7);
    let mut squares_matrices = vec![];
    let a = vec![(4, -1.0)];
    let side_length = 1.0;
    let mut squares_matrix = Matrix::new_empty(vec![0, 7]);
    //squares_matrix.array.pop();
    for row in 0..height {
        for col in 0..width {
            //if squares_matrix.array.len() == 25 {
            //if squares_matrix.array.len() == 1+16*3 {
            if squares_matrix.array.len() == 1+20*3 {

                let (t_vao, t_vbo) = render.create_vao_vbo(&squares_matrix);
                squares_matrices.push((squares_matrix, t_vao, t_vbo));

                squares_matrix = Matrix::new_empty(vec![0, 7]);
                //squares_matrix.array.pop();
            }
            if true {
                let x = ppm_text_lines[3 + row*width + col].split(" ").collect();
                let mat = square_top_left(
                    side_length,
                    [row as f32, col as f32, 0.0],
                    x,
                    1.0);
                    //println!("m {:?}", mat);
                    //println!("");    
                squares_matrix = squares_matrix.expand_along_dims(mat).expect("failed to expand matrix of squares")
            }
        }
    }

    //squares_matrix.array.remove(0);
    //println!("b {:?} {}", squares_matrix.nrows, squares_matrix.ncols);
    //println!("{:?}", squares_matrix);

    //for (i, j) in squares_matrix.array.iter().enumerate() {
    //    println!("{:?}", j);
    //    if i%3==2 {
    //        println!("");
    //        //let a = true;
    //        //let a = match a {
    //        //    true =>Err("a"),
    //        //    false =>Ok("a"),
    //        //}.unwrap();
    //    }
    //}

    // first line is P3, denotes it's RBG in ASCII
    // second line is width by height
    // third line is max colour value
    // beyond that is colours, iterating:
    //      for i in width {
    //          for j in height {
    //              r, g, b
    //          }
    //      }

    println!("a {}", squares_matrices.len());

    println!("t");
    let test = square_top_left(5.0, [0.0, 9.0, 0.0], vec!["128", "250", "14"], 1.0);
    let (test_vao, _) = render.create_vao_vbo(&test);

    while !render.render_over() {
        
        render.begin_render_actions();


        //render.camera.zoom -= 0.001;




        
        render.use_program(ProgramType::SimpleOrthographic);
        //render.use_program(ProgramType::BlinnPhongOrthographic);

        for square in &squares_matrices {
            render.draw_vao(GlSettings::GlTriangles, square.1, &square.0);
        }
        //render.draw_vao(GlSettings::GlTriangles, t_vao, &squares_matrix);
        //render.draw_vao(GlSettings::GlTriangles, test_vao, &test);



        //drop(with_vao);
        //drop(with_program);
        render.end_render_actions();

    }


}