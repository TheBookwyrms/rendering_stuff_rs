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
use crate::gl_abstractions::OpenGl::{clear_colour, GlSettings};
use crate::gl_abstractions::OpenGl;
use crate::vertices::Vertices::{V7, V10};

use std::os::raw::c_void;

use rust_embed::Embed;



#[derive(Embed)]
#[folder = "src/shaders_glsl/"]
struct Asset;


fn main() {

    

    let mut window = init_window_and_opengl();
    let mut camera = Camera::new();
    

    window.set_polling();
    window.make_current();


    let program_types = vec![ProgramType::ForObject, ProgramType::ForLighting];
    let program_holder = ProgramHolder::new(&window.opengl, program_types);




    window.default_gl_settings();



    let triangle: Vec<f32> = vec![
         5.0,  1.0, 0.0, 0.9, 0.5, 0.1, 1.0,
         1.0,  0.0, 2.0, 0.1, 0.9, 0.5, 1.0,
         0.0, -18.0, 0.0, 0.5, 0.1, 0.9, 1.0,
    ];



    let (t_vao, t_vbo) = create_vao_vbo(&window.opengl, false, &triangle);



    let l = -1.0 * (window.width() / window.height()) as f32 * camera.zoom;
    let r = (window.width() / window.height()) as f32 * camera.zoom;
    let b = -1.0 * camera.zoom as f32;
    let t = camera.zoom as f32;
    let n = -1.0 * camera.render_distance as f32;
    let f = camera.render_distance as f32;
    let orthographic_projection = vec![
            2.0/(r-l), 0.0, 0.0, 0.0,
            0.0, 2.0/(t-b), 0.0, 0.0,
            0.0, 0.0, 2.0/(f-n), 0.0,
            -1.0*(r+l)/(r-l), -1.0*(t+b)/(t-b), -1.0*(f+n)/(f-n), 1.0,
        ];
    let mut camera_transform: Vec<f32> = vec![
            1.,0.,0.,0.,
            0.,1.,0.,0.,
            0.,0.,1.,0.,
            0.,0.,0.,1.,
            ];
    let world_transform: Vec<f32> = vec![
            1.,0.,0.,0.,
            0.,0.,1.,0.,
            0.,1.,0.,0.,
            0.,0.,0.,1.,
            ];



    while !window.window.should_close() {

        window.clear_to_colour(camera.background_colour, 1.0);
        window.clear(vec![GlSettings::ColourBufferBit, GlSettings::DepthBufferBit]);





        program_holder.use_program(&window.opengl, ProgramType::ForLighting);
        program_holder.set_program_uniform(
            &window.opengl,
            ProgramType::ForLighting,
            "world_transform",
            OpenGl::UniformType::Mat4,
            &world_transform);
        program_holder.set_program_uniform(
            &window.opengl,
            ProgramType::ForLighting,
            "orthographic_projection",
            OpenGl::UniformType::Mat4,
            &orthographic_projection);
        program_holder.set_program_uniform(
            &window.opengl,
            ProgramType::ForLighting,
            "camera_transformation",
            OpenGl::UniformType::Mat4,
            &camera_transform);



        draw_vao(&window.opengl, GlSettings::GlTriangles, t_vao, &triangle);


        window.swap_buffers();
        window.poll_events();
        window.perform_polled_events();

    }


}


fn get_shader_text(filename:&str) -> String {
    let mut file = filename.to_owned();
    file.push_str(".glsl");
    let file = file.as_str();

    let glsl = Asset::get(file).unwrap();
    let shader_text = std::str::from_utf8(glsl.data.as_ref()).unwrap().to_owned();
    shader_text
}