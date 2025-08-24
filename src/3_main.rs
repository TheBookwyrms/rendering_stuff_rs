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

//use crate::window_loader::WindowLoader::init_window_and_opengl;
use crate::shaders::shaders::{ProgramHolder, ProgramType};
use crate::shaders::shaders::{create_vao_vbo, update_vbo, draw_vao};
use crate::camera::Camera::{Camera, Lighting};
use crate::render_context::RenderContext::Render;
use crate::ndarray_abstractions::MyArray::{Arr1D, Arr2D, Arr3D, Arr4D};
use crate::gl_abstractions::OpenGl::GlSettings;
use crate::gl_abstractions::OpenGl;
use crate::vertices::Vertices::{V7, V10};

use std::os::raw::c_void;

use rust_embed::Embed;

pub mod render_gl;


pub mod gl {
    include!(concat!(env!("OUT_DIR"), "\\gl_bindings.rs"));

    use std::fmt;

    pub(super) mod Magic {
        pub use super::Gl;
    }

    impl fmt::Debug for Gl {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "opengl fmt")
        }
    }
}

    use std::ffi::{CStr, CString};


use gl::Gl;


extern crate glfw;
use glfw::Glfw;
use glfw::{Action, Context, Key, WindowEvent};
use glfw::{PWindow, GlfwReceiver};
use glfw::fail_on_errors;


#[derive(Embed)]
#[folder = "src/shaders_glsl/"]
struct Asset;

fn get_glfw_loadfn<T>(window_name:&'static str, window:&mut PWindow)
                -> impl FnMut(&'static str) -> *const T {
    |window_name: &'static str| window.get_proc_address(&window_name).unwrap() as *const _
}

pub struct Window {
    pub glfw:Glfw,
    pub window:PWindow,
    pub events:GlfwReceiver<(f64, WindowEvent)>,
    pub opengl:gl::Gl,
}
pub fn init_window_and_opengl() -> Window {
    let (width, height) = (450, 450);
    let window_name = "hello, window!";

    let mut glfw = glfw::init(fail_on_errors!()).unwrap();

    let (mut window, events) = glfw.create_window(
        width,
        height,
        &window_name,
        glfw::WindowMode::Windowed
    ).expect("Failed to create GLFW window.");

    let opengl = gl::Gl::load_with(get_glfw_loadfn(window_name, &mut window));

    Window { glfw, window, events, opengl }
}

fn get_shader_text(filename:&str) -> String {
    let mut file = filename.to_owned();
    file.push_str(".glsl");
    let file = file.as_str();

    let glsl = Asset::get(file).unwrap();
    let shader_text = std::str::from_utf8(glsl.data.as_ref()).unwrap().to_owned();
    shader_text
}

fn get_shader_id(opengl:&Gl, filename:&str, stype:gl::types::GLenum) -> u32 {
    unsafe {
        let text   = get_shader_text(filename);
        let id   = opengl.CreateShader(stype);
        let binding = CString::new(text).expect("failed to CString");
        let source_ptr = binding.as_c_str().as_ptr();
        opengl.ShaderSource(id, 1, &source_ptr, std::ptr::null());
        opengl.CompileShader(id);
        id
    }
}
fn get_program_id(opengl:&Gl, vid:u32, fid:u32) -> u32 {
    unsafe {
        let id = opengl.CreateProgram();
        opengl.AttachShader(id, vid);
        opengl.AttachShader(id, fid);
        opengl.LinkProgram(id);
        id
    }
}

fn main() {

    
    

    let mut window = init_window_and_opengl();
    let mut camera = Camera::new();
    

    //window.set_polling();
    window.window.make_current();


    use std::ffi::CString;
    let vert_shader_id = get_shader_id(&window.opengl, "lighting_vertex_shader", gl::VERTEX_SHADER);
    let frag_shader_id = get_shader_id(&window.opengl, "lighting_fragment_shader", gl::FRAGMENT_SHADER);

    let program = get_program_id(&window.opengl, vert_shader_id, frag_shader_id);


        // set up vertex buffer object

    let vertices: Vec<f32> = vec![-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::Gl::GenBuffers(&window.opengl, 1, &mut vbo);
    }

    unsafe {
        gl::Gl::BindBuffer(&window.opengl, gl::ARRAY_BUFFER, vbo);
        gl::Gl::BufferData(&window.opengl, 
            gl::ARRAY_BUFFER,                                                       // target
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
            vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW,                               // usage
        );
        gl::Gl::BindBuffer(&window.opengl, gl::ARRAY_BUFFER, 0);
    }

    // set up vertex array object

    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::Gl::GenVertexArrays(&window.opengl, 1, &mut vao);
    }

    unsafe {
        gl::Gl::BindVertexArray(&window.opengl, vao);
        gl::Gl::BindBuffer(&window.opengl, gl::ARRAY_BUFFER, vbo);
        gl::Gl::EnableVertexAttribArray(&window.opengl, 0); // this is "layout (location = 0)" in vertex shader
        gl::Gl::VertexAttribPointer(&window.opengl, 
            0,         // index of the generic vertex attribute ("layout (location = 0)")
            3,         // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (3 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null(),                                     // offset of the first component
        );
        gl::Gl::BindBuffer(&window.opengl, gl::ARRAY_BUFFER, 0);
        gl::Gl::BindVertexArray(&window.opengl, 0);
    }

    // set up shared state for window

    unsafe {
        gl::Gl::Viewport(&window.opengl, 0, 0, 900, 700);
        gl::Gl::ClearColor(&window.opengl, 0.3, 0.3, 0.5, 1.0);
    }


    loop {
        //for event in event_pump.poll_iter() {
        //    match event {
        //        sdl2::event::Event::Quit { .. } => break 'main,
        //        _ => {}
        //    }
        //}

        unsafe {
            gl::Gl::Clear(&window.opengl, gl::COLOR_BUFFER_BIT);
        }

        // draw triangle

        //shader_program.set_used(&window.opengl);
        unsafe {
            gl::Gl::UseProgram(&window.opengl, program);
        }
        unsafe {
            gl::Gl::BindVertexArray(&window.opengl, vao);
            gl::Gl::DrawArrays(&window.opengl, 
                gl::TRIANGLES, // mode
                0,             // starting index in the enabled arrays
                3,             // number of indices to be rendered
            );
        }

        window.window.swap_buffers();
    }


}