#![allow(warnings)]

// uncomment for release
// #![windows_subsystem = "windows"]





mod camera;
mod render_context;
mod shaders;

use crate::shaders::shaders::{ProgramHolder, ProgramType};
use crate::camera::Camera::{Camera, Lighting};
use crate::render_context::RenderContext::Render;
use opengl::{GlSettings, WithObject};
//use opengl;
use matrices::Matrix2d;
use window::init_window_and_opengl;




use rust_embed::Embed;



#[derive(Embed)]
#[folder = "src/shaders_glsl/"]
struct Asset;


fn main() {

    

    //let mut window = init_window_and_opengl();
    //let mut camera = Camera::new();
    //let mut lighting = Lighting::new();
    //let program_holder = ProgramHolder::new(
        //&window.opengl,
        //[ProgramType::SimpleOrthographic, ProgramType::BlinnPhongOrthographic]
    //);


    let mut render = Render::Default::default()
    //let mut render = Render::new(window, camera, lighting, program_holder);
    render.setup_render();


    //let triangle = Matrix2d::from([
    //    [5.0,   1.0, 0.0, 0.9, 0.5, 0.1, 1.0],
    //    [1.0,   0.0, 2.0, 0.1, 0.9, 0.5, 1.0],
    //    [0.0, -18.0, 0.0, 0.5, 0.1, 0.9, 1.0],
    //]);
    let triangle_normals = Matrix2d::from([
        [5.0,   1.0, 0.0, 0.9, 0.5, 0.1, 1.0, 0.5, 0.5, 0.5],
        [1.0,   0.0, 2.0, 0.1, 0.9, 0.5, 1.0, 0.5, 0.5, 0.5],
        [0.0, -18.0, 0.0, 0.5, 0.1, 0.9, 1.0, 0.5, 0.5, 0.5],
    ]);



    //let (t_vao, t_vbo) = WithObject::new_vao_vbo(&render.window.opengl, false, &triangle);
    //let (t_vao, t_vbo) = render.create_vao_vbo(&triangle);
    let (t_vao, t_vbo) = render.create_vao_vbo(&triangle_normals);
    //let (tn_vao, tn_vbo) = WithObject::new_vao_vbo(&render.window.opengl, true, &triangle_normals);


    let mut world_transform = Matrix2d::from([
            [1.,0.,0.,0.],
            [0.,0.,1.,0.],
            [0.,1.,0.,0.],
            [0.,0.,0.,1.],
            ]);


    let m = Matrix2d::from([
        [1., 2., 3.],
        [4., 5., 6.],
        [7., 8., 9.]
        ]);
    println!("{:?}", m.transpose());



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