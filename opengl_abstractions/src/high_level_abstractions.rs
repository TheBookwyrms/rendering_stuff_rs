use crate::enums::{
    BufferType, DrawMode, DrawType,
    GlError, ProgramVariant, ShaderType,
    UniformType, VertexObject,
};
use crate::gl;
use crate::gl::Gl;
use crate::raw_opengl;
use crate::intermediate_opengl;

use numeracy::matrices::matrix::Matrix;

use std::os::raw::c_void;



pub fn set_uniform<T>(opengl:&Gl, program:ProgramVariant,
                   uniform_name:&str, uniform_type:UniformType,
                   value:*const f32
) -> Result<(), GlError<T>> {
    match program {
        ProgramVariant::SimpleOrthographic(id) => intermediate_opengl::set_uniform(opengl, id, uniform_name, uniform_type, value),
        ProgramVariant::BlinnPhongOrthographic(id) => intermediate_opengl::set_uniform(opengl, id, uniform_name, uniform_type, value),
    }
}


pub fn create_shader_program<T>(opengl:&Gl, vertex_text:&str, fragment_text:&str) -> Result<u32, GlError<T>> {
    let vertex_id = intermediate_opengl::create_shader_variant(opengl, vertex_text, ShaderType::VertexShader)?;
    let fragment_id = intermediate_opengl::create_shader_variant(opengl, fragment_text, ShaderType::FragmentShader)?;

    let program_id = intermediate_opengl::create_shader_program(opengl, vertex_id, fragment_id)?;

    intermediate_opengl::remove_shader_variant(opengl, program_id, vertex_id);
    intermediate_opengl::remove_shader_variant(opengl, program_id, fragment_id);

    Ok(program_id)
}


pub struct WithVertexObject<'l> {
    pub opengl:&'l Gl,
    pub object_type:VertexObject,
    pub vao_id:u32,
    pub vbo_id:u32,
}
impl WithVertexObject<'_> {
    pub fn new_vao(opengl:&Gl) -> (u32, WithVertexObject<'_>) {
        let vao = raw_opengl::gen_vertex_arrays(opengl);
        (vao, WithVertexObject::vao(opengl, vao))
    }
    pub fn new_vbo(opengl:&Gl) -> (u32, WithVertexObject<'_>) {
        let vbo = raw_opengl::gen_buffers(opengl);
        (vbo, WithVertexObject::vbo(opengl, vbo))
    }
    pub fn new_vao_vbo<T>(opengl:&Gl, store_normals:bool, data:&Matrix<f32>) -> Result<(u32, u32), GlError<T>> {
        let (vao, with_vao) = WithVertexObject::new_vao(opengl);
        let (vbo, with_vbo) = WithVertexObject::new_vbo(opengl);

        with_vbo.buffer_data(BufferType::ArrayBuffer, data, DrawType::DynamicDraw);
        
        match data.dtype_memsize().try_into() {
            Ok(dtype_size) => with_vao.set_vertex_attribs(store_normals, dtype_size),
            Err(error) => Err(GlError::TryFromIntError(error)),
        }?;

        Ok((vao, vbo))
    }
    pub fn vao_vbo(opengl:&Gl, vao:u32, vbo:u32) -> WithVertexObject<'_> {
        raw_opengl::bind_vertex_array(opengl, vao);
        intermediate_opengl::bind_buffer(opengl, BufferType::ArrayBuffer, vbo);
        WithVertexObject { opengl, object_type:VertexObject::ArrayAndBuffer,
                        vao_id:vao, vbo_id:vbo }
    }
    pub fn vao(opengl:&Gl, vao:u32) -> WithVertexObject<'_> {
        raw_opengl::bind_vertex_array(opengl, vao);
        WithVertexObject { opengl, object_type:VertexObject::Array,
                     vao_id:vao, vbo_id:0 }
    }
    pub fn vbo(opengl:&Gl, vbo:u32) -> WithVertexObject<'_> {
        intermediate_opengl::bind_buffer(opengl, BufferType::ArrayBuffer, vbo);
        WithVertexObject { opengl, object_type:VertexObject::Buffer,
                     vao_id:0, vbo_id:vbo }
    }
    pub fn update_vbo(&self, data:&Matrix<f32>) {
        self.buffer_sub_data(BufferType::ArrayBuffer, data);
    }
    pub fn buffer_data(&self, target:BufferType, data:&Matrix<f32>, draw_type:DrawType) {
        let data_size = data.memory_size() as gl::types::GLsizeiptr;
        let data_ptr = data.clone().as_ptr() as *const c_void;
        intermediate_opengl::buffer_data(self.opengl, target, data_size, data_ptr, draw_type);
    }
    pub fn buffer_sub_data(&self, target:BufferType, data:&Matrix<f32>) {
        let data_size = data.memory_size() as isize;
        let data_ptr = data.as_ptr() as *const c_void;
        match target {
            BufferType::ArrayBuffer => intermediate_opengl::buffer_sub_data(
                                            self.opengl,
                                            BufferType::ArrayBuffer,
                                            data_size,
                                            data_ptr),
        }
    }
    pub fn set_vertex_attribs<T>(&self, store_normals:bool, dtype_size:i32) -> Result<(), GlError<T>> {
        intermediate_opengl::set_vertex_attrib(self.opengl, 0, store_normals, dtype_size)?;
        intermediate_opengl::set_vertex_attrib(self.opengl, 1, store_normals, dtype_size)?;
        intermediate_opengl::set_vertex_attrib(self.opengl, 2, store_normals, dtype_size)?;
        if store_normals { intermediate_opengl::set_vertex_attrib(self.opengl, 3, store_normals, dtype_size)?; }
        Ok(())
    }
    pub fn draw_vao<T>(&self, mode:DrawMode, data:&Matrix<f32>) -> Result<(), GlError<T>> {
        match data.ndims() {
            2 => {
                let data1 : i32 = match data.shape[1].try_into() {
                    Ok(i) => Ok(i),
                    Err(error) => Err(GlError::TryFromIntError(error)),
                }?;
                Ok(intermediate_opengl::draw_arrays(self.opengl, mode, data1))
            },
            _ => Err(GlError::InvalidDataDims(data.ndims())),
        }
    }
}
impl Drop for WithVertexObject<'_> {
    fn drop(&mut self) {
        match self.object_type {
            VertexObject::Array => raw_opengl::bind_vertex_array(self.opengl, 0),
            VertexObject::Buffer => intermediate_opengl::bind_buffer(self.opengl, BufferType::ArrayBuffer, 0),
            VertexObject::ArrayAndBuffer => {
                raw_opengl::bind_vertex_array(self.opengl, 0);
                intermediate_opengl::bind_buffer(self.opengl, BufferType::ArrayBuffer, 0);
            },
        }
    }
}