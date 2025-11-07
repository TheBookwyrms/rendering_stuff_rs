use crate::gl::Gl;
use crate::enums::{
    DataFormat, GlError, ProgramSelect,
    ShaderType, UniformType, DrawCall,
    DrawMode, Object, ArrayObject,
    BufferObject, DrawType,
};
use crate::{gl, intermediate_opengl, raw_opengl};

use numeracy::matrices::Matrix;

use std::os::raw::c_void;

include!(concat!(env!("OUT_DIR"), "\\shaders_glsl.rs"));





pub struct WithObject<'l> {
    opengl:&'l Gl,
    pub object_type:Object,
    pub data_format:DataFormat,
    pub vao:u32,
    pub vbo:u32,
    pub ebo:u32,
}
impl WithObject<'_> {
    pub fn unbind(opengl:&Gl, object:Object) {
        match object {
            Object::VBO => {
                intermediate_opengl::bind_buffer(opengl, BufferObject::VertexBufferObject, 0);
            },
            Object::VAO => {
                intermediate_opengl::bind_vertex_array(opengl, ArrayObject::VertexArrayObject, 0);
            },
            Object::EBO => {
                intermediate_opengl::bind_buffer(opengl, BufferObject::ElementBufferObject, 0);
            },
        }
    }

    pub fn new(opengl:&Gl, object:Object, format:DataFormat) -> WithObject<'_> {
        let object_id = intermediate_opengl::generate(opengl, object);
        WithObject::existing(opengl, object, object_id, format)
    }

    pub fn existing(opengl:&Gl, object:Object, id:u32, format:DataFormat) -> WithObject<'_> {
        match object {
            Object::VBO => {
                intermediate_opengl::bind_buffer(opengl, BufferObject::VertexBufferObject, id);
                WithObject { opengl, object_type:Object::VBO, data_format:format,
                             vao:0, vbo:id, ebo:0 }
            },
            Object::VAO => {
                intermediate_opengl::bind_vertex_array(opengl, ArrayObject::VertexArrayObject, id);
                WithObject { opengl, object_type:Object::VAO, data_format:format,
                             vao:id, vbo:0, ebo:0 }
            },
            Object::EBO => {
                intermediate_opengl::bind_buffer(opengl, BufferObject::ElementBufferObject, id);
                WithObject { opengl, object_type:Object::EBO, data_format:format,
                             vao:0, vbo:0, ebo:id }
            },
        }
    }

    pub fn buffer_data<T:Clone>(&self, data:&Matrix<T>, draw_type:DrawType) -> Result<(), GlError> {
        let data_size = data.memory_size() as isize;
        let data_ptr = data.as_ptr() as *const c_void;
        match self.object_type {
            Object::VBO => Ok(intermediate_opengl::buffer_data(
                                self.opengl,
                                BufferObject::VertexBufferObject,
                                data_size, data_ptr, draw_type)
                            ),
            Object::EBO => Ok(intermediate_opengl::buffer_data(
                                self.opengl,
                                BufferObject::ElementBufferObject,
                                data_size, data_ptr, draw_type)
                            ),
            Object::VAO => Err(GlError::InvalidObjectType),
        }
    }

    pub fn buffer_sub_data(&self, data:&Matrix<f32>) -> Result<(), GlError> {
        let data_size = data.memory_size() as isize;
        let data_ptr = data.as_ptr() as *const c_void;

        match self.object_type {
            Object::VBO => Ok(intermediate_opengl::buffer_sub_data(
                                self.opengl,
                                BufferObject::VertexBufferObject,
                                data_size, data_ptr)
                            ),
            Object::EBO => Ok(intermediate_opengl::buffer_sub_data(
                                self.opengl,
                                BufferObject::ElementBufferObject,
                                data_size, data_ptr)
                            ),
            Object::VAO => Err(GlError::InvalidObjectType),
        }
    }

    pub fn set_vertex_attribs(&self, dtype_size:i32) -> Result<(), GlError> {
        if self.object_type != Object::VAO { Err(GlError::InvalidObjectType)? }
        match self.data_format {
            DataFormat::Position3Colour3Alpha1 => {
                intermediate_opengl::set_vertex_attrib_position_3(self.opengl, 0, 7, 0, dtype_size);
                intermediate_opengl::set_vertex_attrib_colour_3(  self.opengl, 1, 7, 3, dtype_size);
                intermediate_opengl::set_vertex_attrib_alpha_1(   self.opengl, 2, 7, 3+3, dtype_size);
            },
            DataFormat::Position3Colour3Alpha1Normal3 => {
                intermediate_opengl::set_vertex_attrib_position_3(self.opengl, 0, 10, 0, dtype_size);
                intermediate_opengl::set_vertex_attrib_colour_3(  self.opengl, 1, 10, 3, dtype_size);
                intermediate_opengl::set_vertex_attrib_alpha_1(   self.opengl, 2, 10, 3+3, dtype_size);
                intermediate_opengl::set_vertex_attrib_normal_3(  self.opengl, 3, 10, 3+3+1, dtype_size);
            },
            DataFormat::Position3Texture2 => {
                intermediate_opengl::set_vertex_attrib_position_3(self.opengl, 0, 5, 0, dtype_size);
                intermediate_opengl::set_vertex_attrib_colour_3(  self.opengl, 1, 5, 3, dtype_size);
            },
        }
        Ok(())
        //intermediate_opengl::set_vertex_attrib(self.opengl, 0, store_normals, dtype_size)?;
        //intermediate_opengl::set_vertex_attrib(self.opengl, 1, store_normals, dtype_size)?;
        //intermediate_opengl::set_vertex_attrib(self.opengl, 2, store_normals, dtype_size)?;
        //if store_normals { intermediate_opengl::set_vertex_attrib(self.opengl, 3, store_normals, dtype_size)?; }
        //Ok(())
    }

    pub fn draw<T:Clone>(&self, call:DrawCall, mode:DrawMode, data:&Matrix<T>, format:DataFormat) -> Result<(), GlError> {
        if data.ndims() != 2 { Err(GlError::InvalidDataDims(data.ndims()))? }

        match call {
            DrawCall::Vertices => {
                if self.object_type != Object::VBO { Err(GlError::InvalidObjectType)? }
                let is_ok_format = match format {
                    DataFormat::Position3Colour3Alpha1 => true,
                    DataFormat::Position3Colour3Alpha1Normal3 => true,
                    DataFormat::Position3Texture2 => false,
                };
                if !is_ok_format { Err(GlError::InvalidDataFormat)? }

                let dtype_memsize = match data.dtype_memsize().try_into() {
                    Ok(dtype_size) => Ok(dtype_size),
                    Err(error) => Err(GlError::TryFromIntError(error)),
                }?;

                self.set_vertex_attribs(dtype_memsize)
            },
            DrawCall::Arrays => {
                if self.object_type != Object::VAO { Err(GlError::InvalidObjectType)? }

                let count : i32 = match data.shape[1].try_into() {
                    Ok(i) => Ok(i),
                    Err(error) => Err(GlError::TryFromIntError(error)),
                }?;

                intermediate_opengl::draw_arrays(self.opengl, mode, count);
                Ok(())
            },
            DrawCall::Elements => {
                if self.object_type != Object::VAO { Err(GlError::InvalidObjectType)? }

                let count = data.shape.iter().map(|s| *s as i32).product();

                intermediate_opengl::draw_elements(&self.opengl, mode, count);
                Ok(())
            },
        }
    }
    
}
impl Drop for WithObject<'_> {
    fn drop(&mut self) {
        match self.object_type {
            Object::VAO => intermediate_opengl::bind_vertex_array(self.opengl, ArrayObject::VertexArrayObject, 0),
            Object::VBO => intermediate_opengl::bind_buffer(self.opengl, BufferObject::VertexBufferObject, 0),
            Object::EBO => intermediate_opengl::bind_buffer(self.opengl, BufferObject::VertexBufferObject, 0),
        }
    }
}



#[derive(Clone, Copy)]
pub struct Programs {
    pub simple_orthographic_shader:u32,
    pub blinn_phone_orthographic_shader:u32,
    pub simple_texture_shader:u32,
    pub current_program:Option<u32>,
    pub current_program_type:Option<ProgramSelect>,
}
impl Programs {

    pub fn compile_program_from_text(opengl:&Gl, vertex_text:&str, fragment_text:&str) -> Result<u32, GlError> {
        let vertex_id = intermediate_opengl::create_shader_variant(opengl, vertex_text, ShaderType::VertexShader)?;
        let fragment_id = intermediate_opengl::create_shader_variant(opengl, fragment_text, ShaderType::FragmentShader)?;

        let program_id = intermediate_opengl::create_shader_program(opengl, vertex_id, fragment_id)?;

        intermediate_opengl::remove_shader_variant(opengl, program_id, vertex_id);
        intermediate_opengl::remove_shader_variant(opengl, program_id, fragment_id);

        Ok(program_id)
    }

    pub fn compile_program_from_select(opengl:&Gl, program_type:ProgramSelect) -> Result<u32, GlError> {
        match program_type {
            ProgramSelect::SelectBlinnPhongOrthographic => {
                let vertex_text   = BLINN_PHONG_ORTHOGRAPHIC_VERTEX;
                let fragment_text = BLINN_PHONG_ORTHOGRAPHIC_FRAGMENT;
                let shader_id = Programs::compile_program_from_text(
                    opengl, vertex_text, fragment_text
                )?;
                Ok(shader_id)
            },
            ProgramSelect::SelectSimpleOrthographic => {
                let vertex_text   = SIMPLE_ORTHOGRAPHIC_VERTEX;
                let fragment_text = SIMPLE_ORTHOGRAPHIC_FRAGMENT;
                let shader_id = Programs::compile_program_from_text(
                    opengl, vertex_text, fragment_text
                )?;
                Ok(shader_id)
            },
            ProgramSelect::SelectSimpleTexture => {
                let vertex_text   = SIMPLE_TEXTURE_VERTEX;
                let fragment_text = SIMPLE_TEXTURE_FRAGMENT;
                let shader_id = Programs::compile_program_from_text(
                    opengl, vertex_text, fragment_text
                )?;
                Ok(shader_id)
            },
        }
    }

    pub fn compile(opengl:&Gl) -> Result<Programs, GlError> {
        let simple_orthographic_shader = Programs::compile_program_from_select(opengl, ProgramSelect::SelectSimpleOrthographic)?;
        let blinn_phone_orthographic_shader = Programs::compile_program_from_select(opengl, ProgramSelect::SelectBlinnPhongOrthographic)?;
        let simple_texture_shader = Programs::compile_program_from_select(opengl, ProgramSelect::SelectSimpleTexture)?;

        Ok(Programs { simple_orthographic_shader, blinn_phone_orthographic_shader,
                      simple_texture_shader,
                      current_program:None, current_program_type:None })
    }

    pub fn use_program(&mut self, opengl:&Gl, program:ProgramSelect) -> Result<(), GlError> {
        match program {
            ProgramSelect::SelectSimpleOrthographic => {
                intermediate_opengl::use_program(opengl, self.simple_orthographic_shader)?;
                self.current_program = Some(self.simple_orthographic_shader);
                self.current_program_type = Some(ProgramSelect::SelectSimpleOrthographic);
                Ok(())
            },
            ProgramSelect::SelectBlinnPhongOrthographic => {
                intermediate_opengl::use_program(opengl, self.blinn_phone_orthographic_shader)?;
                self.current_program = Some(self.blinn_phone_orthographic_shader);
                self.current_program_type = Some(ProgramSelect::SelectBlinnPhongOrthographic);
                Ok(())
            },
            ProgramSelect::SelectSimpleTexture => {
                intermediate_opengl::use_program(opengl, self.simple_texture_shader)?;
                self.current_program = Some(self.simple_texture_shader);
                self.current_program_type = Some(ProgramSelect::SelectSimpleTexture);
                Ok(())
            },
        }
    }

    pub fn disuse_program(&mut self, opengl:&Gl) {
        intermediate_opengl::disuse_program(opengl);
        self.current_program = None;
        self.current_program_type = None;
    }

    pub fn set_uniform(&self, opengl:&Gl, uniform_name:&str, uniform_type:UniformType, value:Matrix<f32>
    ) -> Result<(), GlError> {
        match self.current_program {
            Some(id) => intermediate_opengl::set_uniform(opengl, id, uniform_name, uniform_type, value.as_ptr()),
            None => Err(GlError::InvalidProgramID),
        }
    }

    pub fn draw<T:Clone>(
        &self, opengl:&Gl, call:DrawCall,
        mode:DrawMode, vao:u32, data:&Matrix<T>,
        format:DataFormat
    ) -> Result<(), GlError> {

        match self.current_program_type {
            None => Err(GlError::InvalidProgramType),
            Some(program) => {
                match program {
                    ProgramSelect::SelectSimpleOrthographic => {
                        if format == DataFormat::Position3Colour3Alpha1 { Ok(()) } else { Err(GlError::InvalidDataFormat) }
                    },
                    ProgramSelect::SelectBlinnPhongOrthographic => {
                        if format == DataFormat::Position3Colour3Alpha1Normal3 { Ok(()) } else { Err(GlError::InvalidDataFormat) }
                    },
                    ProgramSelect::SelectSimpleTexture => {
                        if format == DataFormat::Position3Texture2 { Ok(()) } else { Err(GlError::InvalidDataFormat) }
                    },
                }
            },
        }?;

        //if format == DataFormat::Position3Texture2 {
        //    raw_opengl::bind_texture(opengl, gl::TEXTURE_2D, texture);
        //}

        let with_vao = WithObject::existing(opengl, Object::VAO, vao, format);
        Ok(with_vao.draw(call, mode, data, format)?)

    }
}