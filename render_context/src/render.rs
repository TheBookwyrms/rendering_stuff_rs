use std::time::{Duration, Instant};

use opengl::{self, intermediate_opengl};
use opengl::enums::{
    BufferBit, DrawMode, GlError,
    ProgramSelect, UniformType, Object,
    BufferObject, DrawType, ArrayObject
};
use opengl::shader_abstractions;
use opengl::shader_abstractions::{ProgramHolder, WithProgram};
use opengl::high_level_abstractions::WithObject;
//use matrices::_tests::matrix_as_1_array::Matrix;
use numeracy::matrices::matrix::Matrix;

use glfw;
use glfw::{Action, Key};
use crate::errors::RenderError;
//use shaders::{ProgramHolder, ProgramType};
use crate::{camera::Camera};
use crate::lighting::Lighting;
use crate::window::Window;


pub struct Render {
    pub window:Window,
    pub camera:Camera,
    pub lighting:Lighting,
    pub programs:ProgramHolder,
    pub paused:bool,
    pub pause_time:Instant,
    pub current_time:Instant
}
impl Render {
    pub fn default() -> Result<Self, RenderError> {
        let window = Window::new_opengl()?;
        let camera = Camera::new();
        let lighting = Lighting::new();

        let simple_orthographic_shader = shader_abstractions::create_program(&window.opengl, ProgramSelect::SelectSimpleOrthographic)?;
        let blinn_phone_orthographic_shader = shader_abstractions::create_program(&window.opengl, ProgramSelect::SelectBlinnPhongOrthographic)?;

        let programs = ProgramHolder::new(simple_orthographic_shader, blinn_phone_orthographic_shader)?;

        Ok(Self { window, camera, lighting, programs:programs,
            paused:false, pause_time:Instant::now(), current_time:Instant::now() })
    }
    pub fn render_over(&self) -> bool { self.window.window.should_close() }
    pub fn poll_events(&mut self) { self.window.poll_events(); }

    pub fn new(window:Window, camera:Camera, lighting:Lighting, programs:ProgramHolder) -> Render {
        Render { window, camera, lighting, programs:programs,
            paused:false, pause_time:Instant::now(), current_time:Instant::now() }
    }

    pub fn setup_render(&mut self) {
        self.window.default_gl_settings();
        self.window.make_current();
        self.window.set_polling();
    }

    pub fn begin_render_actions(&self) -> Result<(), RenderError> {
        self.window.clear_to_colour(self.camera.background_colour, 1.0)?;
        self.window.clear(vec![BufferBit::ColourBufferBit, BufferBit::DepthBufferBit]);
        Ok(())

    }

    fn clear_bindings(&self) {
        WithObject::existing(&self.window.opengl, Object::VBO, 0);
        WithObject::existing(&self.window.opengl, Object::VAO, 0);
        //opengl::high_level_abstractions::WithObject::program(&self.window.opengl, 0);
    }
    
    pub fn end_render_actions(&mut self) -> Result<(), RenderError> {
        

        self.clear_bindings();


        let dt = match Instant::now().duration_since(self.current_time).as_secs_f32() {
            0.0 => 0.0,
            t => t,};
        //println!("dt {}", dt);
        let fps = 1.0/dt;
        //println!("fps {}", fps);
        self.current_time = Instant::now();


        // double buffered window for rendering
        self.window.swap_buffers();

        self.poll_and_perform_polled_events()
    }


    pub fn create_vao_vbo_ebo(&self, vertices:&Matrix<f32>, indices:&Matrix<i32>
    ) -> Result<(u32, u32, u32), RenderError> {
        let gl = &self.window.opengl;


        let with_vao = WithObject::new(&self.window.opengl, Object::VAO);
        
        let with_vbo = WithObject::new(&self.window.opengl, Object::VBO);
        with_vbo.buffer_data(vertices, DrawType::DynamicDraw)?;

        let with_ebo = WithObject::new(&self.window.opengl, Object::EBO);
        with_ebo.buffer_data(indices, DrawType::DynamicDraw)?;


        with_vao.set_vertex_attribs(false, vertices.dtype_memsize() as i32)?;

        let vao = with_vao.vao;
        let vbo = with_vbo.vbo;
        let ebo = with_vbo.ebo;
        drop(with_vbo);
        drop(with_vao);
        //drop(with_ebo);
        intermediate_opengl::bind_buffer(gl, BufferObject::ElementBufferObject, 0);

        Ok((vao, vbo, ebo))

        //let with_vao = WithObject::new(&self.window.opengl, opengl::enums::Object::VAO);
        //let with_vbo = WithObject::new(&self.window.opengl, opengl::enums::Object::VBO);
//
        ////println!("{}", with_ebo.ebo);
//
        //with_vbo.buffer_data(&vertices, DrawType::DynamicDraw)?;
//
        //let with_ebo = WithObject::new(&self.window.opengl, opengl::enums::Object::EBO);
        //with_ebo.buffer_data(&indices, DrawType::DynamicDraw)?;
//
        //println!("vi {}, {}", vertices.dtype_memsize(), indices.dtype_memsize());
//
        //match vertices.dtype_memsize().try_into() {
        //    Ok(dtype_size) => with_vao.set_vertex_attribs(false, dtype_size),
        //    Err(error) => Err(GlError::TryFromIntError(error)),
        //}?;
//
        //Ok((with_vao.vao, with_vbo.vbo, with_ebo.ebo))
    }


    pub fn create_vao_vbo(&self, data:&Matrix<f32>) -> Result<(u32, u32), RenderError> {
        let store_normals = match data.shape[0] {
            7 => Ok(false),
            10 => Ok(true),
            n => Err(RenderError::DataLengthError(n)),
        }?;

        let with_vao = WithObject::new(&self.window.opengl, Object::VAO);
        let with_vbo = WithObject::new(&self.window.opengl, Object::VBO);

        with_vbo.buffer_data(data, DrawType::DynamicDraw)?;

        match data.dtype_memsize().try_into() {
            Ok(dtype_size) => with_vao.set_vertex_attribs(store_normals, dtype_size),
            Err(error) => Err(GlError::TryFromIntError(error)),
        }?;

        Ok((with_vao.vao, with_vbo.vbo))
    }

    pub fn draw_vao_ebo(&self, mode:DrawMode, vao:u32, count:i32) {
        intermediate_opengl::bind_vertex_array(&self.window.opengl, opengl::enums::ArrayObject::VertexArrayObject, vao);
        //intermediate_opengl::draw_elements(&self.window.opengl, mode, data.shape[1].try_into().unwrap());
        intermediate_opengl::draw_elements(&self.window.opengl, mode, count);
        intermediate_opengl::bind_vertex_array(&self.window.opengl, opengl::enums::ArrayObject::VertexArrayObject, 0);

    }

    pub fn draw_vao(&self, mode:DrawMode, vao:u32, data:&Matrix<f32>) -> Result<(), RenderError> {
        let with_vao = WithObject::existing(&self.window.opengl, Object::VAO, vao);
        Ok(with_vao.draw(mode, data)?)
    }


    pub fn use_program(&self, program_type:ProgramSelect) -> Result<(), RenderError> {

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

    fn set_orthographic_camera_uniforms(&self, with_program:&WithProgram<'_>) -> Result<(), RenderError> {
        with_program.set_uniform("world_transform", UniformType::Mat4, Matrix::opengl_to_right_handed())?;
        with_program.set_uniform("orthographic_projection", UniformType::Mat4,
            self.camera.get_orthographic_projection(self.window.aspect_ratio))?;
        let camera_transform = match self.camera.get_camera_transform() {
            Ok(mat) => Ok(mat),
            Err(error) => Err(GlError::MatrixError(error)),
        }?;
        with_program.set_uniform("camera_transformation", UniformType::Mat4,
            camera_transform)?;
        Ok(())
    }



    fn set_blinn_phong_uniforms(&self, with_program:&WithProgram<'_>) -> Result<(), RenderError> {
        with_program.set_uniform("ambient_strength", UniformType::Float,
            Matrix::from_scalar(self.lighting.ambient_strength))?;
        with_program.set_uniform("ambient_colour", UniformType::Vec3, 
            Matrix::from_1darray(self.lighting.ambient_colour.into()))?;
        Ok(())

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






    fn poll_and_perform_polled_events(&mut self) -> Result<(), RenderError> {
        self.poll_events();
        for (_, event) in glfw::flush_messages(&self.window.events) {
            match event {

                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    {let _ = &self.window.window.set_should_close(true); Ok(())}
                },
                glfw::WindowEvent::Key(Key::Space, _, Action::Press, _) => {
                    match self.paused {
                        false => {self.paused=true; self.pause_time=Instant::now()},
                        true => if Instant::now().duration_since(self.pause_time) > Duration::from_millis(10) {self.paused=false},
                    };
                    Ok(())
                },

                glfw::WindowEvent::Close => {
                    {let _ = &self.window.window.set_should_close(true); Ok(())}
                },
                
                glfw::WindowEvent::MouseButton(button, action, _mods) => {
                    {
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
                    };
                    Ok(())
                    }
                },

                glfw::WindowEvent::Scroll(_xoffset, yoffset) => {
                    {self.camera.zoom -= ((0.24*yoffset) as f32) * self.camera.zoom*0.25; Ok(())}
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
                        self.camera.angle_xyz.0 += dy * self.camera.angle_sensitivity * self.camera.zoom;
                            // y and x are swapped
                        self.camera.angle_xyz.1 += dx * self.camera.angle_sensitivity * self.camera.zoom;
                            // y and x are swapped
                    }

                    self.window.last_cursor_pos = [xpos as f32, ypos as f32];

                    Ok(())
                },

                glfw::WindowEvent::Size(width, height) => {
                    match (width==0) || (height==0) {
                        true => Err(RenderError::GLFWResizeBoundsError((width, height))),
                        false => {
                            self.window.aspect_ratio = width as f32/height as f32;
                            Ok(opengl::intermediate_opengl::viewport(&self.window.opengl, width, height))
                        },
                    }
                },

                glfw::WindowEvent::Key(_, _, _, _) => {Ok(())},
                glfw::WindowEvent::Char(_) => {Ok(())},
                glfw::WindowEvent::CharModifiers(_, _) => {Ok(())},
                glfw::WindowEvent::Focus(_) => {Ok(())},
                glfw::WindowEvent::Pos(_, _) => {Ok(())},
                glfw::WindowEvent::FramebufferSize(_, _) => {Ok(())},
                glfw::WindowEvent::Iconify(_) => {Ok(())},
                glfw::WindowEvent::Maximize(_) => {Ok(())},
                glfw::WindowEvent::Refresh => {Ok(())},
                glfw::WindowEvent::CursorEnter(_) => {Ok(())},
                _ => Err(RenderError::NewGLFWEventDetected(event)),
            }?;
        }
        Ok(())
    }
}