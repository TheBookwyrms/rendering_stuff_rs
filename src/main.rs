#![allow(warnings)]

// uncomment for release
// #![windows_subsystem = "windows"]





//pub mod camera;
//pub mod render_context;
//pub mod shaders;
//pub mod _ray_tracer;

use shaders::{ProgramHolder, ProgramType};
//use crate::shaders::shaders::{ProgramHolder, ProgramType};
//use crate::camera::Camera::{Camera, Lighting};
use render_context::Render;
//use crate::render_context::RenderContext::Render;
use opengl::{GlSettings, WithObject};
//use opengl;
use matrices::Matrix2d;
//use matrices::_tests::matrix_as_1_array::Matrix;
use matrices::_tests::matrix_with_types::matrix::Matrix;
//use window::init_window_and_opengl;

use ppm_viewer;




//use rust_embed::Embed;
//#[derive(Embed)]
//#[folder = "src/shaders_glsl/"]
//struct Asset;

fn error(msg:String) {
    let a = true;
    let _b = match a {
        true =>Err(msg),
        false =>Ok(msg),
    }.unwrap();
}


fn main() {


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
    let e = Matrix::from_vec_of_vec(vec![
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


    error("halt".to_string());



    //println!("a");
    //ppm_viewer::view_ppm_from_path("C://Users//black//Documents//code_and_related_stuff//my_code//assorted_rust//rendering_stuff_rs//lib//ppm_viewer//src//test.ppm");
    //println!("b");
    ////_ray_tracer::hello_ppm();
    //let a = true;
    //let a = match a {
    //    true =>Err("a"),
    //    false =>Ok("a"),
    //}.unwrap();
    

    //let mut window = init_window_and_opengl();
    //let mut camera = Camera::new();
    //let mut lighting = Lighting::new();
    //let program_holder = ProgramHolder::new(
        //&window.opengl,
        //[ProgramType::SimpleOrthographic, ProgramType::BlinnPhongOrthographic]
    //);


    //let mut render = Render::Default::default();
    let mut render = Render::default();
    //let mut render = Render::new(window, camera, lighting, program_holder);
    render.setup_render();


    //let triangle = Matrix2d::from([
    //    [5.0,   1.0, 0.0, 0.9, 0.5, 0.1, 1.0],
    //    [1.0,   0.0, 2.0, 0.1, 0.9, 0.5, 1.0],
    //    [0.0, -18.0, 0.0, 0.5, 0.1, 0.9, 1.0],
    //]);
    let triangle_normals = Matrix::from_2darray([
        [5.0,  1.0, 0.0, 0.9, 0.5, 0.1, 1.0, 0.5, 0.5, 0.5],
        [1.0,  0.0, 0.0, 0.1, 0.9, 0.5, 1.0, 0.5, 0.5, 0.5],
        //[0.0, -18.0, 0.0, 0.5, 0.1, 0.9, 1.0, 0.5, 0.5, 0.5],
        [0.0, -5.0, 0.0, 0.5, 0.1, 0.9, 1.0, 0.5, 0.5, 0.5],
    ]);



    //let (t_vao, t_vbo) = WithObject::new_vao_vbo(&render.window.opengl, false, &triangle);
    //let (t_vao, t_vbo) = render.create_vao_vbo(&triangle);
    let (t_vao, t_vbo) = render.create_vao_vbo(&triangle_normals);
    //let (tn_vao, tn_vbo) = WithObject::new_vao_vbo(&render.window.opengl, true, &triangle_normals);

    while !render.render_over() {
        render.begin_render_actions();


        //render.camera.zoom -= 0.001;
        
        //render.use_program(ProgramType::SimpleOrthographic);
        render.use_program(ProgramType::BlinnPhongOrthographic);


        //let with_vao = WithObject::vao(&render.window.opengl, tn_vao);
        //let with_vao = WithObject::vao(&render.window.opengl, t_vao);
        //with_vao.draw_vao(GlSettings::GlTriangles, &triangle);
        //render.draw_vao(GlSettings::GlTriangles, t_vao, &triangle);
        render.draw_vao(GlSettings::GlTriangles, t_vao, &triangle_normals);
        //with_vao.draw_vao(GlSettings::GlTriangles, &triangle_normals);



        //drop(with_vao);
        //drop(with_program);
        render.end_render_actions();
    }


}