use crate::enums::{
    BlendFunc, BufferBit, BufferType,
    DrawMode, DrawType, GlEnable,
    GlError, VertexObject, ShaderType,
    UniformType
};
use crate::gl;
use crate::gl::Gl;
use crate::raw_opengl;
use crate::intermediate_opengl;

use matrices::matrix::Matrix;

use std::io::Read;
use std::os::raw::c_void;
use std::fs::File;

use rust_embed::Embed;


#[derive(Embed)]
#[folder = "src/shaders_glsl/"]
struct Asset;


fn get_shader_text_dynamic(filename:&str) -> Result<String, GlError> {
    let mut folder = "src/shaders_glsl/".to_owned();
    folder.push_str(filename);
    folder.push_str(".glsl");
    let file_path = folder.as_str();

    match File::open(file_path) {
        Ok(mut file_handle) => {
            let mut shader_text = String::new();
            match file_handle.read_to_string(&mut shader_text) {
                Ok(_) => Ok(shader_text),
                Err(error) => Err(GlError::FileError(error)),
            }
        },
        Err(error) => Err(GlError::FileError(error)),
    }
}

fn get_shader_text_embed(filename:&str) -> Result<String, GlError> {
    let mut file = filename.to_owned();
    file.push_str(".glsl");
    let file = file.as_str();

    match Asset::get(file) {
        Some(glsl) => {
            match std::str::from_utf8(glsl.data.as_ref()) {
                Ok(shader_text_str) => Ok(shader_text_str.to_owned()),
                Err(error) => Err(GlError::TextError(error)),
            }
        },
        None => Err(GlError::EmbedError),
    }
}

pub fn load_opengl_with<T:FnMut(&'static str) -> *const c_void>(loadfn: T) -> Gl {
    intermediate_opengl::load_opengl_with(loadfn)
}


pub fn viewport(opengl:&Gl, width:i32, height:i32) {
    intermediate_opengl::viewport(opengl, width, height);
}

pub fn use_program(opengl:&Gl, program_id:u32) {
    raw_opengl::use_program(opengl, program_id);
}


pub fn clear_colour(opengl:&Gl, r:f32, g:f32, b:f32, a:f32) -> Result<(), GlError> {
    intermediate_opengl::clear_colour(opengl, r, g, b, a)
}

pub fn clear(opengl:&Gl, masks:Vec<BufferBit>) {
    intermediate_opengl::clear(opengl, masks);
}

pub fn gl_enable(opengl:&Gl, setting:GlEnable) {
    intermediate_opengl::gl_enable(opengl, setting);
}

pub fn gl_blendfunc(opengl:&Gl, setting:BlendFunc) {
    intermediate_opengl::gl_blendfunc(opengl, setting);
}

pub fn set_uniform(opengl:&Gl, program_id:u32,
                   uniform_name:&str, uniform_type:UniformType,
                   value:Matrix<f32>
) -> Result<(), GlError> {
    intermediate_opengl::set_uniform(opengl, program_id, uniform_name, uniform_type, value.as_ptr())
}


pub fn create_shader_program(opengl:&Gl, vertex_text:&str, fragment_text:&str) -> Result<u32, GlError> {
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
    pub fn new_vao_vbo(opengl:&Gl, store_normals:bool, data:&Matrix<f32>) -> Result<(u32, u32), GlError> {
        let (vao, with_vao) = WithVertexObject::new_vao(opengl);
        let (vbo, with_vbo) = WithVertexObject::new_vbo(opengl);

        with_vbo.buffer_data(BufferType::ArrayBuffer, data, DrawType::DynamicDraw);
        with_vao.set_vertex_attribs(store_normals, data.dtype_memsize())?;

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
    pub fn set_vertex_attribs(&self, store_normals:bool, dtype_size:usize) -> Result<(), GlError> {
        intermediate_opengl::set_vertex_attrib(self.opengl, 0, store_normals, dtype_size)?;
        intermediate_opengl::set_vertex_attrib(self.opengl, 1, store_normals, dtype_size)?;
        intermediate_opengl::set_vertex_attrib(self.opengl, 2, store_normals, dtype_size)?;
        if store_normals { intermediate_opengl::set_vertex_attrib(self.opengl, 3, store_normals, dtype_size)?; }
        Ok(())
    }
    pub fn draw_vao(&self, mode:DrawMode, data:&Matrix<f32>) -> Result<(), GlError> {
        match data.ndims() {
            2 => Ok(intermediate_opengl::draw_arrays(self.opengl, mode, data.shape[1].try_into().unwrap())),
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