#![allow(warnings)]
/// uncomment for release
//#![windows_subsystem = "windows"]




use ndarray;


mod camera;
mod window_loader;
mod context;
mod shaders;

use crate::window_loader::WindowLoader::init_window_and_opengl;
use crate::shaders::shaders::ProgramType;
use crate::camera::Camera::Camera;
use crate::context::Context::Render;



// #[derive(Embed)]
// #[folder = "src/shaders_glsl/"]
// struct Asset;


fn main() {

    //let index_html = Asset::get("object_vertex_shader.glsl").unwrap();
    //println!("{:?}", std::str::from_utf8(index_html.data.as_ref()).unwrap());
    //println!("");

    //let a = nalgebra::matrix

    
    let mut abc = ndarray::array![[1, 2], [3, 4], [5, 6]].into_shared();
    //let mut def = ndarray::array![[7, 8], [9, 10], [11, 12]].into_shared();
    let mut def = ndarray::array![[7, 8, 9], [10, 11, 12]];

    let b = abc.clone();

    println!("{:?}", &abc);
    println!("{:?}", &abc);
    //let b = abc.clone()+1;
    println!("{:?}", &abc.dot(&def));
    //let m1 = abc.dot(&def);
    //println!("{:?}", m1);

    let mut window = init_window_and_opengl();
    let mut camera = Camera::new();

    println!("{:?}", window.window.get_size());

    // implement use_program for Render
    //let object_program = ShaderProgram::new(&window.opengl, ProgramType::Object);
    
    let mut render = Render::new(window, camera, vec![ProgramType::Object, ProgramType::Lighting]);
    render.setup_render();


    
    while !render.render_over() {
        render.begin_render_actions();

        render.end_render_actions();
    }
}