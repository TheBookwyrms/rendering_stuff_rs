use crate::enums::{
    BufferObject, ArrayObject, DrawMode,
    DrawType, GlError, ProgramVariant,
    ShaderType, UniformType, Object,
};
use crate::gl::Gl;
use crate::intermediate_opengl;

use numeracy::matrices::matrix::Matrix;

use std::os::raw::c_void;



pub fn set_uniform(opengl:&Gl, program:ProgramVariant,
                   uniform_name:&str, uniform_type:UniformType,
                   value:*const f32
) -> Result<(), GlError> {
    match program {
        ProgramVariant::SimpleOrthographic(id) => intermediate_opengl::set_uniform(opengl, id, uniform_name, uniform_type, value),
        ProgramVariant::BlinnPhongOrthographic(id) => intermediate_opengl::set_uniform(opengl, id, uniform_name, uniform_type, value),
    }
}


pub fn create_shader_program(opengl:&Gl, vertex_text:&str, fragment_text:&str) -> Result<u32, GlError> {
    let vertex_id = intermediate_opengl::create_shader_variant(opengl, vertex_text, ShaderType::VertexShader)?;
    let fragment_id = intermediate_opengl::create_shader_variant(opengl, fragment_text, ShaderType::FragmentShader)?;

    let program_id = intermediate_opengl::create_shader_program(opengl, vertex_id, fragment_id)?;

    intermediate_opengl::remove_shader_variant(opengl, program_id, vertex_id);
    intermediate_opengl::remove_shader_variant(opengl, program_id, fragment_id);

    Ok(program_id)
}


pub struct WithObject<'l> {
    opengl:&'l Gl,
    pub object_type:Object,
    pub vao:u32,
    pub vbo:u32,
    pub ebo:u32,
}
impl WithObject<'_> {
    pub fn new(opengl:&Gl, object:Object) -> WithObject<'_> {
        let object_id = intermediate_opengl::generate(opengl, object);
        WithObject::existing(opengl, object, object_id)
    }

    pub fn existing(opengl:&Gl, object:Object, id:u32) -> WithObject<'_> {
        match object {
            Object::VBO => {
                intermediate_opengl::bind_buffer(opengl, BufferObject::VertexBufferObject, id);
                WithObject { opengl, object_type:Object::VBO,
                             vao:0, vbo:id, ebo:0 }
            },
            Object::VAO => {
                intermediate_opengl::bind_vertex_array(opengl, ArrayObject::VertexArrayObject, id);
                WithObject { opengl, object_type:Object::VAO,
                             vao:id, vbo:0, ebo:0 }
            },
            Object::EBO => {
                intermediate_opengl::bind_buffer(opengl, BufferObject::ElementBufferObject, id);
                WithObject { opengl, object_type:Object::EBO,
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

    pub fn set_vertex_attribs(&self, store_normals:bool, dtype_size:i32) -> Result<(), GlError> {
        if self.object_type != Object::VAO { Err(GlError::InvalidObjectType)? }
        intermediate_opengl::set_vertex_attrib(self.opengl, 0, store_normals, dtype_size)?;
        intermediate_opengl::set_vertex_attrib(self.opengl, 1, store_normals, dtype_size)?;
        intermediate_opengl::set_vertex_attrib(self.opengl, 2, store_normals, dtype_size)?;
        if store_normals { intermediate_opengl::set_vertex_attrib(self.opengl, 3, store_normals, dtype_size)?; }
        Ok(())
    }

    pub fn draw<T:Clone>(&self, mode:DrawMode, data:&Matrix<T>) -> Result<(), GlError> {
        if data.ndims() != 2 { Err(GlError::InvalidDataDims(data.ndims()))? }

        let count : i32 = match data.shape[1].try_into() {
            Ok(i) => Ok(i),
            Err(error) => Err(GlError::TryFromIntError(error)),
        }?;

        let dtype_memsize = match data.dtype_memsize().try_into() {
            Ok(dtype_size) => Ok(dtype_size),
            Err(error) => Err(GlError::TryFromIntError(error)),
        }?;


        match self.object_type {
            Object::VAO => { Ok(intermediate_opengl::draw_arrays(self.opengl, mode, count)) },
            Object::EBO => { Ok(intermediate_opengl::draw_elements(self.opengl, mode, count)) },
            Object::VBO => {

                intermediate_opengl::set_vertex_attrib(self.opengl, 0, false, dtype_memsize)?;
                intermediate_opengl::set_vertex_attrib(self.opengl, 1, false, dtype_memsize)?;
                intermediate_opengl::set_vertex_attrib(self.opengl, 2, false, dtype_memsize)?;
                
                match data.shape[0] {
                    7 => { Ok(()) /* already accounted for above */ },
                    10 => {
                        intermediate_opengl::set_vertex_attrib(self.opengl, 3, true, dtype_memsize)
                    },
                    n => Err(GlError::DataLengthError(n)),
                }
            },
        }
    }

    //pub fn draw_vao(&self, mode:DrawMode, data:&Matrix<f32>) -> Result<(), GlError> {
    //    match data.ndims() {
    //        2 => {
    //            let data1 : i32 = match data.shape[1].try_into() {
    //                Ok(i) => Ok(i),
    //                Err(error) => Err(GlError::TryFromIntError(error)),
    //            }?;
    //            Ok(intermediate_opengl::draw_arrays(self.opengl, mode, data1))
    //        },
    //        _ => Err(GlError::InvalidDataDims(data.ndims())),
    //    }
    //}
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