pub mod RenderContext {

    use std::time::{Duration, Instant};

    use crate::matrices::Matrices;
    use crate::ndarray_abstractions::MyArray::{Arr1D, Arr2D, Arr3D, Arr4D};
    use crate::window_loader::WindowLoader::Window;
    use crate::camera::Camera::{Camera, Lighting};
    use crate::gl_abstractions::OpenGl::{GlSettings, UniformType};

    use crate::shaders::shaders::{ProgramHolder, ProgramType};

    use glfw::{Key, Action};



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
        
        pub fn end_render_actions(&mut self) {

            let dt = match Instant::now().duration_since(self.camera.current).as_secs() {
                0 => 0,
                t => t,};
            self.camera.current = Instant::now();

            // double buffered window for rendering
            self.window.swap_buffers();

            self.poll_and_perform_polled_events();
        }







        //pub fn set_camera_uniforms(&self, program_type:ProgramType) {
        //    &self.programs.set_program_uniform(
        //        &self.window.opengl,
        //        program_type,
        //        "orthographic_projection",
        //        UniformType::Mat4,
        //        &Matrices::get_orthographic_projection(
        //                self.window.width(),
        //                self.window.height(),
        //                self.camera.zoom,
        //                self.camera.render_distance));
        //    &self.programs.set_program_uniform(
        //        &self.window.opengl,
        //        program_type,
        //        "camera_transform",
        //        UniformType::Mat4,
        //        &Matrices::get_camera_transform(
        //                self.camera.angle_xyz,
        //                self.camera.pan_xyz));
        //    &self.programs.set_program_uniform(
        //        &self.window.opengl,
        //        program_type,
        //        "world_transform",
        //        UniformType::Mat4,
        //        &Matrices::get_world_transform());
        //}
//
//
        //pub fn set_lighting_uniforms(&self, program_type:ProgramType) {
        //    &self.programs.set_program_uniform(
        //        &self.window.opengl,
        //        program_type,
        //        "ambient_strength",
        //        UniformType::Float,
        //        &self.lighting.ambient_strength);
        //    &self.programs.set_program_uniform(
        //        &self.window.opengl,
        //        program_type,
        //        "ambient_colour",
        //        UniformType::Vec3,
        //        &Arr1D::from(self.lighting.ambient_colour.into()));
        //    &self.programs.set_program_uniform(
        //        &self.window.opengl,
        //        program_type,
        //        "diffuse_strength",
        //        UniformType::Float,
        //        &self.lighting.diffuse_strength);
        //    &self.programs.set_program_uniform(
        //        &self.window.opengl,
        //        program_type,
        //        "diffuse_base",
        //        UniformType::Float,
        //        &self.lighting.diffuse_base);
        //    &self.programs.set_program_uniform(
        //        &self.window.opengl,
        //        program_type,
        //        "light_source_pos",
        //        UniformType::Vec3,
        //        &Arr1D::from(self.lighting.light_source_pos.into()));
        //    &self.programs.set_program_uniform(
        //        &self.window.opengl,
        //        program_type,
        //        "light_source_colour",
        //        UniformType::Vec3,
        //        &Arr1D::from(self.lighting.light_source_colour.into()));
        //    &self.programs.set_program_uniform(
        //        &self.window.opengl,
        //        program_type,
        //        "specular_strength",
        //        UniformType::Float,
        //        &self.lighting.specular_strength);
        //    let camera_viewpos_vec = Arr1D::from(self.lighting.view_vec.into()).arr.to_vec();
        //    let camera_viewpos = (camera_viewpos_vec[0], camera_viewpos_vec[1], camera_viewpos_vec[2]);
        //    &self.programs.set_program_uniform(
        //        &self.window.opengl,
        //        program_type,
        //        "camera_viewpos",
        //        UniformType::Vec3,
        //        &Arr1D::from(camera_viewpos.try_into().expect("failed conversion")));
        //    &self.programs.set_program_uniform(
        //        &self.window.opengl,
        //        program_type,
        //        "specular_power",
        //        UniformType::Float,
        //        &(self.lighting.specular_power as f32));
        //    &self.programs.set_program_uniform(
        //        &self.window.opengl,
        //        program_type,
        //        "light_y_transform",
        //        UniformType::Mat4,
        //        &self.lighting.light_y_transform);
        //}



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