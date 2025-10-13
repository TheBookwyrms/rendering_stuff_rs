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
mod vertices;

use crate::window_loader::WindowLoader::init_window_and_opengl;
use crate::shaders::shaders::{ProgramHolder, ProgramType};
use crate::shaders::shaders::{create_vao_vbo, update_vbo, draw_vao};
use crate::camera::Camera::{Camera, Lighting};
use crate::render_context::RenderContext::Render;
use crate::ndarray_abstractions::MyArray::{Arr1D, Arr2D, Arr3D, Arr4D};
use crate::gl_abstractions::OpenGl::GlSettings;
use crate::vertices::Vertices::{V7, V10};



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






    let triangle = Arr2D::from([
        [ 5.0,  1.0, 0.0, 1.0, 0.0, 0.0, 1.0],
        [ 1.0,  0.0, 2.0, 0.0, 1.0, 0.0, 1.0],
        [ 0.0, -18.0, 0.0, 0.0, 0.0, 1.0, 1.0],
    ]);
    
    //let triangle = vec![
    //    [-1.0,  1.0, 0.0, 0.5, 0.5, 0.5, 1.0],
    //    [1.0,  0.0, 0.0, 0.5, 0.5, 0.5, 1.0],
    //    [0.0, -1.0, 0.0, 0.5, 0.5, 0.5, 1.0],
    //];
    //let triangle = vec![
    //    V7::from([-1.0,  1.0, 0.0, 0.9, 0.5, 0.1, 1.0]),
    //    V7::from([ 1.0,  0.0, 2.0, 0.9, 0.5, 0.1, 1.0]),
    //    V7::from([ 0.0, -1.0, 0.0, 0.9, 0.5, 0.1, 1.0]),
    //];
    //let triangle = vec![
    //    V7::from([ 0.0,  1.0, 0.0, 0.9, 0.5, 0.1, 1.0]),
    //    V7::from([ 1.0,  0.0, 0.0, 0.9, 0.5, 0.1, 1.0]),
    //    V7::from([ 0.0, -1.0, 0.0, 0.9, 0.5, 0.1, 1.0]),
    //];

    //let triangle = vec![
    //     0.0,  1.0, 0.0, 0.9, 0.5, 0.1, 1.0,
    //     1.0,  0.0, 0.0, 0.9, 0.5, 0.1, 1.0,
    //     0.0, -1.0, 0.0, 0.9, 0.5, 0.1, 1.0,
    //];


    let (t_vao, t_vbo) = create_vao_vbo(&render.window.opengl, false, &triangle);









    
    while !render.render_over() {
        render.begin_render_actions();

        //render.camera.pan_xyz.2 += 0.001;


        render.programs.use_program(&render.window.opengl, ProgramType::ForLighting);
        render.set_camera_uniforms(ProgramType::ForLighting);
        render.set_lighting_uniforms(ProgramType::ForLighting);

        update_vbo(&render.window.opengl, t_vbo, &triangle);
        draw_vao(&render.window.opengl, GlSettings::GlTriangles, t_vao, &triangle);
        //draw_vao(&render.window.opengl, GlSettings::GlPoints, t_vao, &triangle);




        render.end_render_actions();
    }
}