use opengl::enums::{BlendFunc, BufferBit, GlEnable, GlError};
use opengl::gl::Gl;

use glfw::Glfw;

//extern crate glfw;
pub use glfw;
use glfw::{Context, WindowEvent};
use glfw::{PWindow, GlfwReceiver};
use glfw::fail_on_errors;

pub struct Window {
    pub glfw:Glfw,
    pub window:PWindow,
    pub events:GlfwReceiver<(f64, WindowEvent)>,
    pub opengl:Gl,
    pub last_cursor_pos : [f32; 2],
}

impl Window {
    pub fn new_opengl_window() -> Window {
        let (width, height) = (450, 450);
        let window_name = "hello, window!";

        let mut glfw = glfw::init(fail_on_errors!()).unwrap();

        let (mut window, events) = glfw.create_window(
            width,
            height,
            &window_name,
            glfw::WindowMode::Windowed
        ).expect("Failed to create GLFW window.");

        //let opengl = opengl::load_with(get_glfw_loadfn(window_name, &mut window));
        let opengl = opengl::intermediate_opengl::load_opengl_with(get_glfw_loadfn(&mut window));

        Window { glfw, window, events, opengl, last_cursor_pos:[0.0, 0.0] }
    }

    // relabel subaspect functions to Window functions        
    pub fn poll_events(&mut self)  { self.glfw.poll_events(); }

    pub fn set_polling(&mut self)  { self.window.set_all_polling(true); }
    pub fn swap_buffers(&mut self) { self.window.swap_buffers(); }
    pub fn make_current(&mut self) { self.window.make_current(); }
    pub fn width(&self)  -> u32    { self.window.get_size().0.try_into().unwrap() }
    pub fn height(&self) -> u32    { self.window.get_size().1.try_into().unwrap() }

    pub fn clear(&self, masks:Vec<BufferBit>) { opengl::intermediate_opengl::clear(&self.opengl, masks) }
    pub fn clear_to_colour(&self, rgb:(f32, f32, f32), a:f32) -> Result<(), GlError> {
        opengl::intermediate_opengl::clear_colour(&self.opengl, rgb.0, rgb.1, rgb.2, a)
    }    

    pub fn default_gl_settings(&self) {
        opengl::intermediate_opengl::gl_enable(&self.opengl, GlEnable::DepthTest);
        opengl::intermediate_opengl::gl_enable(&self.opengl, GlEnable::Multisample);
        opengl::intermediate_opengl::gl_enable(&self.opengl, GlEnable::Blend);
        opengl::intermediate_opengl::gl_blendfunc(&self.opengl, BlendFunc::SRCAlphaOneMinusSRCAlpha);
    }
}

//fn get_glfw_loadfn<T>(window_name:&'static str, window:&mut PWindow)
fn get_glfw_loadfn<T>(window:&mut PWindow)
                -> impl FnMut(&'static str) -> *const T {
    |window_name: &'static str| window.get_proc_address(&window_name).unwrap() as *const _
}