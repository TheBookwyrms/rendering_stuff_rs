pub mod Context {

    use std::time::{Duration, Instant};

    use crate::window_loader::WindowLoader::Window;
    use crate::camera::Camera::Camera;
    use crate::window_loader::gl;

    use crate::shaders::shaders::{ShaderProgram, ProgramType};

    use glfw::{Key, Action};

    struct ProgramHolder {
        programs:Vec<ShaderProgram>
    }
    impl ProgramHolder {
        pub fn new() -> ProgramHolder {
            ProgramHolder { programs: vec![] }
        }
        pub fn add(&mut self, program:ShaderProgram) {
            self.programs.push(program);
        }
    }

    pub struct Render {
        pub window:Window,
        pub camera:Camera,
        pub programs:ProgramHolder,
    }
    impl Render {
        pub fn render_over(&self) -> bool { self.window.window.should_close() }
        pub fn poll_events(&mut self) { self.window.poll_events(); }

        pub fn new(window:Window, camera:Camera, programs:Vec<ProgramType>) -> Render {
            let mut held = ProgramHolder::new();
            for ptype in programs {
                held.add(ShaderProgram::new(&window.opengl, ptype))}
            Render { window, camera, programs:held }
        }

        pub fn setup_render(&mut self) {
            self.window.gl_enables();
            self.window.make_current();
            self.window.set_polling();
        }

        pub fn begin_render_actions(&self) {
            let (cr, cg, cb) = self.camera.background_colour;
            self.window.clear_colour(cr, cg, cb, 1.0);
            self.window.clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        
        pub fn end_render_actions(&mut self) {

            let dt = match Instant::now().duration_since(self.camera.current).as_secs() {
                0 => 0,
                t => t,
            };
            self.camera.current = Instant::now();


            //let end = Instant::now();
            //let dt = end.duration_since(self.camera.current).as_secs();
            //match dt {
            //    0 => {},
            //    _ => self.camera.dt = dt,
            //}
            //self.camera.current = end;

            // double buffered window for rendering
            self.window.swap_buffers();

            self.poll_and_perform_polled_events();
        }

        pub fn poll_and_perform_polled_events(&mut self) {
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