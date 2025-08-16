pub mod gl {
    include!(concat!(env!("OUT_DIR"), "\\gl_bindings.rs"));

    use std::fmt;
    use crate::gl;

    impl fmt::Debug for gl::Gl {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "opengl fmt")
        }
    }
}

pub mod WindowLoader {

    use crate::gl;
    use glfw::Glfw;
    
    extern crate glfw;
    use glfw::{Action, Context, Key, WindowEvent};
    use glfw::{PWindow, GlfwReceiver};
    use glfw::fail_on_errors;

    pub struct Window {
        pub glfw:Glfw,
        pub window:PWindow,
        pub events:GlfwReceiver<(f64, WindowEvent)>,
        pub opengl:gl::Gl,
        pub width:u32,
        pub height:u32,
    }

    pub fn init_window_and_opengl() -> Window {
        let (width, height) = (450, 450);
        let (mut w, mut h) = (width.clone(), height.clone());
        let window_name = "hello, window!";

        let mut glfw = glfw::init(fail_on_errors!()).unwrap();

        let (mut window, events) = glfw.create_window(
            width,
            height,
            window_name,
            glfw::WindowMode::Windowed
        ).expect("Failed to create GLFW window.");

        let opengl = gl::Gl::load_with(
            |window_name: &'static str| window.get_proc_address(&window_name).unwrap() as *const _
        );


        Window { glfw, window, events, opengl, width:w, height:h }
    }

    impl Window {
        // relabel subaspect functions to Window functions        
        pub fn poll_events(&mut self) { self.glfw.poll_events(); }
        pub fn swap_buffers(&mut self) { self.window.swap_buffers(); }
        pub fn make_current(&mut self) { self.window.make_current(); }

        pub fn set_polling(&mut self) { self.window.set_all_polling(true); }

        pub fn perform_polled_events(&mut self) {
            for (_, event) in glfw::flush_messages(&self.events) {
                match event {
                    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        &self.window.set_should_close(true);
                    },
                    glfw::WindowEvent::Key(_, _, _, _) => {},
                    glfw::WindowEvent::Char(_) => {},
                    glfw::WindowEvent::CharModifiers(_, _) => {},
                    glfw::WindowEvent::Focus(_) => {},
                    glfw::WindowEvent::MouseButton(_, _, _) => {},
                    glfw::WindowEvent::Scroll(_, _) => {},
                    glfw::WindowEvent::Refresh => {},
                    glfw::WindowEvent::CursorPos(_, _) => {},
                    glfw::WindowEvent::CursorEnter(bool) => {},
                    _ => { println!("new even detected! {:?}", event); },
                }
            }
        }

        pub fn gl_enables(&self) {
            unsafe {
                self.opengl.Enable(gl::DEPTH_TEST);
                self.opengl.Enable(gl::MULTISAMPLE);
                //self.opengl.Enable(gl::POINT_SMOOTH); // not in rust ??
                self.opengl.BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
                self.opengl.Enable(gl::BLEND);
            }
        }
    }
}