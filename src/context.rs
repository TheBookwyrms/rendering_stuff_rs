pub mod Context {

    use std::time::{Duration, Instant};

    use crate::window_loader::WindowLoader::Window;
    use crate::camera::Camera::{Camera, Lighting};
    use crate::gl_abstractions::OpenGl::GlSettings;

    use crate::shaders::shaders::ProgramHolder;

    use glfw::{Key, Action};



    pub struct Render {
        pub window:Window,
        pub camera:Camera,
        pub programs:ProgramHolder,
    }
    impl Render {
        pub fn render_over(&self) -> bool { self.window.window.should_close() }
        pub fn poll_events(&mut self) { self.window.poll_events(); }

        pub fn new(window:Window, camera:Camera, programs:ProgramHolder) -> Render {
            //let mut held = ProgramHolder::new();
            //for ptype in programs {
            //    held.add(ShaderProgram::new(&window.opengl, ptype))}
            Render { window, camera, programs:programs }
        }

        pub fn setup_render(&mut self) {
            self.window.default_gl_settings();
            self.window.make_current();
            self.window.set_polling();
        }

        pub fn begin_render_actions(&self) {
            let (cr, cg, cb) = self.camera.background_colour;
            self.window.clear_colour(cr, cg, cb, 1.0);
            self.window.clear(
                vec![GlSettings::ColourBufferBit, GlSettings::DepthBufferBit]
            );
        }
        
        pub fn end_render_actions(&mut self) {

            let dt = match Instant::now().duration_since(self.camera.current).as_secs() {
                0 => 0,
                t => t,};
            self.camera.current = Instant::now();

            // double buffered window for rendering
            self.window.swap_buffers();

            self.poll_and_perform_polled_events();
        }



        fn poll_and_perform_polled_events(&mut self) {
            self.poll_events();
            for (_, event) in glfw::flush_messages(&self.window.events) {
                match event {
                    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        &self.window.window.set_should_close(true); },
                    glfw::WindowEvent::Close => { &self.window.window.set_should_close(true); },
                    glfw::WindowEvent::Key(_, _, _, _) => {},
                    glfw::WindowEvent::Char(_) => {},
                    glfw::WindowEvent::CharModifiers(_, _) => {},
                    glfw::WindowEvent::Focus(_) => {},
                    glfw::WindowEvent::MouseButton(_, _, _) => {},
                    glfw::WindowEvent::Scroll(_, _) => {},
                    glfw::WindowEvent::Pos(_, _) => {},
                    glfw::WindowEvent::Size(_, _) => {},
                    glfw::WindowEvent::FramebufferSize(_, _) => {},
                    glfw::WindowEvent::Iconify(_) => {},
                    glfw::WindowEvent::Maximize(_) => {},
                    glfw::WindowEvent::Refresh => {},
                    glfw::WindowEvent::CursorPos(_, _) => {},
                    glfw::WindowEvent::CursorEnter(bool) => {},
                    _ => { println!("new even detected! {:?}", event); },
                }
            }
        }
    }
}