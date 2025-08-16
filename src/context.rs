pub mod Context {

    use std::time::{Duration, Instant};

    use crate::window_loader::WindowLoader::Window;
    use crate::camera::Camera::Camera;
    use crate::gl;

    pub struct Render {
        pub window:Window,
        pub camera:Camera,
    }
    impl Render {
        pub fn render_over(&self) -> bool { self.window.window.should_close() }
        pub fn new(window:Window, camera:Camera) -> Render {
            Render { window, camera }
        }
        pub fn setup_render(&mut self) {
            self.window.gl_enables();
            self.window.make_current();
            self.window.set_polling();
        }
        pub fn begin_render_actions(&self) {
            let (cr, cg, cb) = self.camera.background_colour;
            unsafe {
                self.window.opengl.ClearColor(cr, cg, cb, 1.0);
                self.window.opengl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT)
            }
        }
        pub fn end_render_actions(&mut self) {
            let end = Instant::now();
            let dt = end.duration_since(self.camera.current).as_secs();
            match dt {
                0 => {},
                _ => self.camera.dt = dt,
            }
            self.camera.current = end;

            // double buffered window for rendering
            self.window.swap_buffers();

            // poll and process events
            self.window.poll_events();
            self.window.perform_polled_events();
        }
    }
}