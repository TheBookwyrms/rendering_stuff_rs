pub mod RenderContext {

    use std::time::{Duration, Instant};

    //use crate::window_loader::WindowLoader::Window;
    use window::Window;
    use crate::camera::Camera::{Camera, Lighting};

    use opengl::{GlSettings, UniformType, WithObject};
    use matrices::{Matrix2d, right_handed};

    use crate::shaders::shaders::{ProgramHolder, ProgramType};

    use window::glfw;
    use window::glfw::{Key, Action};



    pub struct Render {
        pub window:Window,
        pub camera:Camera,
        pub lighting:Lighting,
        pub programs:ProgramHolder,
    }
    impl Render {
        pub fn render_over(&self) -> bool { self.window.window.should_close() }
        pub fn poll_events(&mut self) { self.window.poll_events(); }

        pub fn new(window:Window, camera:Camera, lighting:Lighting, programs:ProgramHolder) -> Render {
            //let mut held = ProgramHolder::new();
            //for ptype in programs {
            //    held.add(ShaderProgram::new(&window.opengl, ptype))}
            Render { window, camera, lighting, programs:programs }
        }

        pub fn setup_render(&mut self) {
            self.window.default_gl_settings();
            self.window.make_current();
            self.window.set_polling();
        }

        pub fn begin_render_actions(&self) {
            self.window.clear_to_colour(self.camera.background_colour, 1.0);
            self.window.clear(vec![GlSettings::ColourBufferBit, GlSettings::DepthBufferBit]);

        }

        fn clear_bindings(&self) {
            WithObject::vao(    &self.window.opengl, 0);
            WithObject::vbo(    &self.window.opengl, 0);
            WithObject::program(&self.window.opengl, 0);
        }
        
        pub fn end_render_actions(&mut self) {
            

            self.clear_bindings();

            //let dt = match Instant::now().duration_since(self.camera.current).as_secs() {
            //    0 => 0,
            //    t => t,};
            //self.camera.current = Instant::now();
            

            let dt = match Instant::now().duration_since(self.camera.current).as_micros() {
                0 => 0,
                t => t,};
            self.camera.current = Instant::now();

            // double buffered window for rendering
            self.window.swap_buffers();

            self.poll_and_perform_polled_events();
        }



        pub fn use_program(&self, program_type:ProgramType) {
            let with_program = self.programs.use_program(&self.window.opengl, program_type);
            match program_type {
                ProgramType::SimpleOrthographic => {
                    self.set_orthographic_camera_uniforms(&with_program);
                },
                ProgramType::BlinnPhongOrthographic => {
                    self.set_orthographic_camera_uniforms(&with_program);
                    self.set_blinn_phong_uniforms(&with_program);
                    Err("uniforms for Blinn-Phong are not fully yet implemented").unwrap()
                },
            }
        }

        fn set_orthographic_camera_uniforms(&self, with_program:&WithObject<'_>) {
            with_program.set_uniform("world_transform", UniformType::Mat4, right_handed());
            with_program.set_uniform("orthographic_projection", UniformType::Mat4,
                self.camera.get_orthographic_projection(self.window.width(), self.window.height()));
            with_program.set_uniform("camera_transformation", UniformType::Mat4,
                self.camera.get_camera_transform());
        }



        fn set_blinn_phong_uniforms(&self, with_program:&WithObject<'_>) {
            with_program.set_uniform("ambient_strength", UniformType::Float,
                Matrix2d::from_float(self.lighting.ambient_strength));
            with_program.set_uniform("ambient_colour", UniformType::Vec3, 
                Matrix2d::from_1darray(self.lighting.ambient_colour.into()));
            with_program.set_uniform("diffuse_strength", UniformType::Float,
                Matrix2d::from_float(self.lighting.diffuse_strength));
            with_program.set_uniform("diffuse_base", UniformType::Float,
                Matrix2d::from_float(self.lighting.diffuse_base));
            with_program.set_uniform("light_source_pos", UniformType::Vec3,
                Matrix2d::from_1darray(self.lighting.light_source_pos.into()));
            with_program.set_uniform("light_source_colour", UniformType::Vec3,
                Matrix2d::from_1darray(self.lighting.light_source_colour.into()));
            with_program.set_uniform("specular_strength", UniformType::Float,
                Matrix2d::from_float(self.lighting.specular_strength));
            with_program.set_uniform("specular_power", UniformType::Float,
                Matrix2d::from_float(self.lighting.specular_power as f32));
            let view_vec = self.lighting.view_vec;
            let view_vec3 = (view_vec.0, view_vec.1, view_vec.2);
            with_program.set_uniform("camera_viewpos", UniformType::Vec3,
                Matrix2d::from_1darray(view_vec3.into()));
            with_program.set_uniform("light_y_transform", UniformType::Mat4,
                self.lighting.light_y_transform.clone());
        }






        fn poll_and_perform_polled_events(&mut self) {
            self.poll_events();
            for (_, event) in glfw::flush_messages(&self.window.events) {
                match event {
                    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        &self.window.window.set_should_close(true); },
                    glfw::WindowEvent::Close => {
                        &self.window.window.set_should_close(true); },
                    glfw::WindowEvent::Key(_, _, _, _) => {},
                    glfw::WindowEvent::Char(_) => {},
                    glfw::WindowEvent::CharModifiers(_, _) => {},
                    glfw::WindowEvent::Focus(_) => {},
                    glfw::WindowEvent::MouseButton(_, _, _) => {},
                    glfw::WindowEvent::Scroll(xoffset, yoffset) => {
                        self.camera.zoom -= (0.24*yoffset) as f32},
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