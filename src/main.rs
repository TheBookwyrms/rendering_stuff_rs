#![allow(warnings)]
/// uncomment for release
//#![windows_subsystem = "windows"]






use ndarray;


mod camera;
mod window_loader;
mod context;

use crate::window_loader::WindowLoader::init_window_and_opengl;
//mod shaders;
use crate::camera::Camera::Camera;
use crate::context::Context::Render;


fn main() {

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
    let mut render = Render::new(window, camera);
    render.setup_render();

    
    while !render.render_over() {
        render.begin_render_actions();

        render.end_render_actions();
    }
}