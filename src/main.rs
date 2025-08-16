#![allow(warnings)]

// uncomment for release
//#![windows_subsystem = "windows"]

mod gl {
    include!(concat!(env!("OUT_DIR"), "\\gl_bindings.rs"));
}

//use opengl::Gl as gl;

use std::task::Context;

//use crate::camera::camera::Camera;
/// Simple loading example
/// 
use crate::gl::Gl;
//use crate::window_loader::window_loader::init_window_and_opengl;


//extern crate glfw;
//use glfw::ffi::GLFW_MOUSE_BUTTON_LEFT;
////use glfw;
//use glfw::{Action, Context, Key, MouseButton, WindowEvent};
//use glfw::Context;
//use glfw::fail_on_errors;


use ndarray;
//use ndarray::prelude::*;
//use ndarray_linalg;


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
    //window.gl_enables();

    let mut camera = Camera::new();


    let mut render = Render::new(window, camera);
    render.setup_render();

    
    while !render.render_over() {
        render.begin_render_actions();

        render.end_render_actions();
    }
}