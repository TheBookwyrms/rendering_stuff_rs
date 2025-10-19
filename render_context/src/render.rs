use std::time::Instant;

use opengl;
use opengl::enums::{BufferBit, DrawMode, GlError, ProgramSelect, UniformType};
use opengl::shader_abstractions;
use opengl::shader_abstractions::{ProgramHolder, WithProgram};
use opengl::high_level_abstractions::WithVertexObject;
//use matrices::_tests::matrix_as_1_array::Matrix;
use matrices::matrix::Matrix;

use glfw;
use glfw::{Action, Key};
//use shaders::{ProgramHolder, ProgramType};
use crate::{camera::Camera};
use crate::lighting::Lighting;
use crate::window::Window;


pub struct Render {
    pub window:Window,
    pub camera:Camera,
    pub lighting:Lighting,
    pub programs:ProgramHolder,
}
impl Render {
    pub fn default() -> Result<Self, GlError> {
        let window = Window::new_opengl_window();
        let camera = Camera::new();
        let lighting = Lighting::new();

        let simple_orthographic_shader = shader_abstractions::create_program(&window.opengl, ProgramSelect::SelectSimpleOrthographic)?;
        let blinn_phone_orthographic_shader = shader_abstractions::create_program(&window.opengl, ProgramSelect::SelectBlinnPhongOrthographic)?;

        let programs = ProgramHolder::new(simple_orthographic_shader, blinn_phone_orthographic_shader)?;

        Ok(Self { window, camera, lighting, programs:programs })
    }
    pub fn render_over(&self) -> bool { self.window.window.should_close() }
    pub fn poll_events(&mut self) { self.window.poll_events(); }

    pub fn new(window:Window, camera:Camera, lighting:Lighting, programs:ProgramHolder) -> Render {
        Render { window, camera, lighting, programs:programs }
    }

    pub fn setup_render(&mut self) {
        self.window.default_gl_settings();
        self.window.make_current();
        self.window.set_polling();
    }

    pub fn begin_render_actions(&self) -> Result<(), GlError> {
        self.window.clear_to_colour(self.camera.background_colour, 1.0)?;
        self.window.clear(vec![BufferBit::ColourBufferBit, BufferBit::DepthBufferBit]);
        Ok(())

    }

    fn clear_bindings(&self) {
        WithVertexObject::vao(    &self.window.opengl, 0);
        WithVertexObject::vbo(    &self.window.opengl, 0);
        //opengl::high_level_abstractions::WithVertexObject::program(&self.window.opengl, 0);
    }
    
    pub fn end_render_actions(&mut self) {
        

        self.clear_bindings();


        let dt = match Instant::now().duration_since(self.camera.current).as_secs_f32() {
            0.0 => 0.0,
            t => t,};
        //println!("dt {}", dt);
        let fps = 1.0/dt;
        println!("fps {}", fps);
        self.camera.current = Instant::now();


        // double buffered window for rendering
        self.window.swap_buffers();

        self.poll_and_perform_polled_events();
    }



    pub fn create_vao_vbo(&self, data:&Matrix<f32>) -> Result<(u32, u32), GlError> {
        let store_normals = match data.shape[0] {
            7 => Ok(false),
            10 => Ok(true),
            _ => Err("data length neither 7 nor 10 items"),
        }.unwrap();

        WithVertexObject::new_vao_vbo(&self.window.opengl, store_normals, data)
    }

    pub fn draw_vao(&self, mode:DrawMode, vao:u32, data:&Matrix<f32>) -> Result<(), GlError> {
        let with_vao = WithVertexObject::vao(&self.window.opengl, vao);
        with_vao.draw_vao(mode, data)
    }


    pub fn use_program(&self, program_type:ProgramSelect) -> Result<(), GlError> {

        let with_program = WithProgram::program(&self.window.opengl, program_type, self.programs);
        with_program.use_program()?;
        match program_type {
            ProgramSelect::SelectSimpleOrthographic => {
                self.set_orthographic_camera_uniforms(&with_program)?;
            },
            ProgramSelect::SelectBlinnPhongOrthographic => {
                self.set_orthographic_camera_uniforms(&with_program)?;
                self.set_blinn_phong_uniforms(&with_program)?;
            },
        }
        Ok(())
    }

    fn set_orthographic_camera_uniforms(&self, with_program:&WithProgram<'_>) -> Result<(), GlError> {
        with_program.set_uniform("world_transform", UniformType::Mat4, Matrix::opengl_to_right_handed())?;
        with_program.set_uniform("orthographic_projection", UniformType::Mat4,
            self.camera.get_orthographic_projection(self.window.width(), self.window.height()))?;
        let camera_transform = match self.camera.get_camera_transform() {
            Ok(mat) => Ok(mat),
            Err(error) => Err(GlError::MatrixError(error)),
        }?;
        with_program.set_uniform("camera_transformation", UniformType::Mat4,
            camera_transform)
    }



    fn set_blinn_phong_uniforms(&self, with_program:&WithProgram<'_>) -> Result<(), GlError> {
        with_program.set_uniform("ambient_strength", UniformType::Float,
            Matrix::from_scalar(self.lighting.ambient_strength))?;
        with_program.set_uniform("ambient_colour", UniformType::Vec3, 
            Matrix::from_1darray(self.lighting.ambient_colour.into()))

        //with_program.set_uniform("diffuse_strength", UniformType::Float,
        //    Matrix::from_float(self.lighting.diffuse_strength));
        //with_program.set_uniform("diffuse_base", UniformType::Float,
        //    Matrix::from_float(self.lighting.diffuse_base));
        //with_program.set_uniform("light_source_pos", UniformType::Vec3,
        //    Matrix::from_1darray(self.lighting.light_source_pos.into()));
        //with_program.set_uniform("light_source_colour", UniformType::Vec3,
        //    Matrix::from_1darray(self.lighting.light_source_colour.into()));
        //with_program.set_uniform("specular_strength", UniformType::Float,
        //    Matrix::from_float(self.lighting.specular_strength));
        //with_program.set_uniform("specular_power", UniformType::Float,
        //    Matrix::from_float(self.lighting.specular_power as f32));
        //let view_vec = self.lighting.view_vec;
        //let view_vec3 = (view_vec.0, view_vec.1, view_vec.2);
        //with_program.set_uniform("camera_viewpos", UniformType::Vec3,
        //    Matrix::from_1darray(view_vec3.into()));
        //with_program.set_uniform("light_y_transform", UniformType::Mat4,
        //    self.lighting.light_y_transform.clone());
    }






    fn poll_and_perform_polled_events(&mut self) {
        self.poll_events();
        for (_, event) in glfw::flush_messages(&self.window.events) {
            //println!("{}", self.camera.zoom);
            //println!("{:?}", self.camera.pan_xyz);
            match event {

                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    let _ = &self.window.window.set_should_close(true);
                },

                glfw::WindowEvent::Close => {
                    let _ = &self.window.window.set_should_close(true);
                },
                
                glfw::WindowEvent::MouseButton(button, action, _mods) => {
                    match action {
                        Action::Press => {
                            match button {
                                glfw::MouseButton::Button1 => {self.camera.panning = true}, // left button
                                glfw::MouseButton::Button2 => {self.camera.angling = true}, // right button
                                _ => {},
                            }
                        },
                        Action::Release => {
                            match button {
                                glfw::MouseButton::Button1 => {self.camera.panning = false}, // left button
                                glfw::MouseButton::Button2 => {self.camera.angling = false}, // right button
                                _ => {},
                            }
                        },
                        Action::Repeat => {},
                    }
                },

                glfw::WindowEvent::Scroll(_xoffset, yoffset) => {
                    self.camera.zoom -= ((0.24*yoffset) as f32) * self.camera.zoom*0.25
                },

                glfw::WindowEvent::CursorPos(xpos, ypos) => {
                    let dx = xpos as f32 - self.window.last_cursor_pos[0];
                    let dy = ypos as f32 - self.window.last_cursor_pos[1];

                    if self.camera.panning {
                        self.camera.pan_xyz.0 += dx * self.camera.pan_sensitivity * self.camera.zoom;
                            // add dx
                        self.camera.pan_xyz.1 -= dy * self.camera.pan_sensitivity * self.camera.zoom;
                            // subtract dy
                    }
                    if self.camera.angling {
                        self.camera.angle_xyz.0 += dy * self.camera.pan_sensitivity * self.camera.zoom;
                            // y and x are swapped
                        self.camera.angle_xyz.1 += dx * self.camera.pan_sensitivity * self.camera.zoom;
                            // y and x are swapped
                    }

                    self.window.last_cursor_pos = [xpos as f32, ypos as f32];
                },

                glfw::WindowEvent::Key(_, _, _, _) => {},
                glfw::WindowEvent::Char(_) => {},
                glfw::WindowEvent::CharModifiers(_, _) => {},
                glfw::WindowEvent::Focus(_) => {},
                glfw::WindowEvent::Pos(_, _) => {},
                glfw::WindowEvent::Size(_, _) => {},
                glfw::WindowEvent::FramebufferSize(_, _) => {},
                glfw::WindowEvent::Iconify(_) => {},
                glfw::WindowEvent::Maximize(_) => {},
                glfw::WindowEvent::Refresh => {},
                glfw::WindowEvent::CursorEnter(_) => {},
                _ => { println!("new even detected! {:?}", event); },
            }
        }
    }
}