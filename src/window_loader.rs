pub mod WindowLoader {

    use crate::gl_abstractions::OpenGl;
    use crate::gl_abstractions::OpenGl::{GlError, GlSettings, Gl};

    use glfw::Glfw;
    
    extern crate glfw;
    use glfw::{Action, Context, Key, WindowEvent};
    use glfw::{PWindow, GlfwReceiver};
    use glfw::fail_on_errors;

    pub struct Window {
        pub glfw:Glfw,
        pub window:PWindow,
        pub events:GlfwReceiver<(f64, WindowEvent)>,
        pub opengl:Gl,
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

        let opengl = OpenGl::load_with(get_glfw_loadfn(window_name, &mut window));

        Window { glfw, window, events, opengl }
    }

    impl Window {
        // relabel subaspect functions to Window functions        
        pub fn poll_events(&mut self) { self.glfw.poll_events(); }

        pub fn set_polling(&mut self) { self.window.set_all_polling(true); }
        pub fn swap_buffers(&mut self) { self.window.swap_buffers(); }
        pub fn make_current(&mut self) { self.window.make_current(); }
        pub fn width(&self)  -> u32 { self.window.get_size().0.try_into().unwrap() }
        pub fn height(&self) -> u32 { self.window.get_size().1.try_into().unwrap() }

        
        pub fn clear_colour(&self, r:f32, g:f32, b:f32, a:f32) {
            OpenGl::clear_colour(&self.opengl, r, g, b, a)}
        pub fn clear(&self, masks:Vec<GlSettings>) {
            OpenGl::clear(&self.opengl, masks)}


        pub fn default_gl_settings(&self) {
            OpenGl::gl_enable(&self.opengl, GlSettings::DepthTest);
            OpenGl::gl_enable(&self.opengl, GlSettings::Multisample);
            OpenGl::gl_enable(&self.opengl, GlSettings::Blend);
            OpenGl::gl_blendfunc(&self.opengl, GlSettings::BlendFunc_SRCAlpha_OneMinusSRCAlpha);
        }
    }
    
    fn get_glfw_loadfn<T>(window_name:&'static str, window:&mut PWindow)
                    -> impl FnMut(&'static str) -> *const T {
        |window_name: &'static str| window.get_proc_address(&window_name).unwrap() as *const _
    }
}