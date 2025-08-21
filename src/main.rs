#![allow(warnings)]
/// uncomment for release
//#![windows_subsystem = "windows"]






mod camera;
mod window_loader;
mod render_context;
mod shaders;
mod gl_abstractions;
mod ndarray_abstractions;
mod matrices;

use crate::window_loader::WindowLoader::init_window_and_opengl;
use crate::shaders::shaders::{ProgramHolder, ProgramType};
use crate::camera::Camera::{Camera, Lighting};
use crate::render_context::RenderContext::Render;
use crate::ndarray_abstractions::{MyArray};



fn main() {

    

    let mut window = init_window_and_opengl();
    let mut camera = Camera::new();
    let mut lighting = Lighting::new();



    // implement use_program for Render
    // function to set shader uniforms
    // object class
    // stores draw mode, which is then fed into draw_vao and update_vbo !!
    



    let program_types = vec![ProgramType::ForObject, ProgramType::ForLighting];
    let program_holder = ProgramHolder::new(&window.opengl, program_types);
    
    let mut render = Render::new(window, camera, lighting, program_holder);
    render.setup_render();


    
    while !render.render_over() {
        render.begin_render_actions();

        render.end_render_actions();
    }
}