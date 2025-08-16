#![allow(warnings)]
//#![windows_subsystem = "windows"]

mod gl {
    include!(concat!(env!("OUT_DIR"), "\\gl_bindings.rs"));
}

//use opengl::Gl as gl;

/// Simple loading example
/// 
use crate::gl::Gl;
use crate::window_loader::window_loader::init_window_and_opengl;

mod window_loader;

extern crate glfw;
//use glfw::ffi::GLFW_MOUSE_BUTTON_LEFT;
////use glfw;
//use glfw::{Action, Context, Key, MouseButton, WindowEvent};
use glfw::Context;
//use glfw::fail_on_errors;


use ndarray;
//use ndarray::prelude::*;
//use ndarray_linalg;

mod shaders;


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
    // let (width, height) = (450, 450);
    // let window_name = "hello, window!";
    // let mut glfw = glfw::init(fail_on_errors!()).unwrap();
    //let (mut window.window, events) = glfw.create_window(
    //    width,
    //    height,
    //    window_name,
    //    glfw::WindowMode::Windowed
    //).expect("Failed to create GLFW window.");;
    //let opengl = gl::Gl::load_with(|window_name: &'static str| window.window.get_proc_address(
    //&window_name).unwrap() as *const _
    //);




    window.window.make_current();
    window.window.set_key_polling(true);
    window.window.set_mouse_button_polling(true);
    // NOTE : set other polling methods



    
    while !window.window.should_close() {
        window.swap_buffers();
    
        // Poll for and process events
        window.poll_events();
        window.perform_polled_events();
    }
}