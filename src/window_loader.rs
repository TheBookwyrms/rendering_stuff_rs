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
//use crate::gl::Gl;

pub mod window_loader {

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
    }

    impl Window {
        // relabel subaspect functions to Window functions        
        pub fn poll_events(&mut self) { self.glfw.poll_events(); }
        pub fn swap_buffers(&mut self) { self.window.swap_buffers(); }

        pub fn perform_polled_events(&mut self) {
            for (_, event) in glfw::flush_messages(&self.events) {
                println!("{:?}", event);
                match event {
                    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        &self.window.set_should_close(true);
                    },
                    //glfw::WindowEvent::Key(Key::A, _, Action::Press, _) => {
                    //    println!("hello a")
                    //},
                    //glfw::WindowEvent::MouseButton(MouseButton::Button8, Action::Press, _) => println!("hello"),
                    //_ => {println!("ev")},
                    _ => {},
                }
            }
        }
    }


    pub fn init_window_and_opengl() -> Window {
        let (width, height) = (450, 450);
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

        Window { glfw, window, events, opengl }
    }
}