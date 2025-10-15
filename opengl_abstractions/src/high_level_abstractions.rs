use crate::enums::BufferType;
use crate::enums::DrawType;
use crate::enums::GlError;
use crate::gl;
use crate::gl::Gl;
use crate::raw_opengl;
use crate::intermediate_opengl;
use crate::enums::{GlSettings, ShaderType, UniformType};

use matrices::matrix::Matrix;

use std::os::raw::c_void;







pub fn clear_colour(opengl:&Gl, r:f32, g:f32, b:f32, a:f32) {
    let validity = vec![r, g, b, a].into_iter().filter(|c| 0.0<=*c && *c<=1.0).count();
    match validity {
        4 => raw_opengl::clear_colour(opengl, r, g, b, a),
        _ => Err("r, g, b, or a not within 0..=1".to_owned()).unwrap(),
    };
    
}


pub fn clear(opengl:&Gl, masks:Vec<GlSettings>) {
    let allowed = vec![GlSettings::ColourBufferBit, GlSettings::DepthBufferBit];
    let validity = masks.clone().into_iter().filter(|m| allowed.contains(m)).count();
    match validity {
        value if value == allowed.len() => {
            //let mut start = u32::MAX;
            let mut start = 0;
            for mask in masks.clone() {
                start = match mask {
                    GlSettings::ColourBufferBit => start | gl::COLOR_BUFFER_BIT,
                    GlSettings::DepthBufferBit  => start | gl::DEPTH_BUFFER_BIT,
                    _ => start,
                };
            }
            raw_opengl::clear(opengl, start);
        },
        _ => Err("invalid clear masks".to_owned()).unwrap(),
    }
}


pub fn gl_enable(opengl:&Gl, setting:GlSettings) {
    match setting {
            GlSettings::DepthTest => raw_opengl::enable(opengl, gl::DEPTH_TEST),
            GlSettings::Multisample => raw_opengl::enable(opengl, gl::MULTISAMPLE),
            GlSettings::Blend => raw_opengl::enable(opengl, gl::BLEND),
            _ => Err("invalid gl_enable value".to_owned()).unwrap(),
        }
    
}


pub fn gl_blendfunc(opengl:&Gl, setting:GlSettings) {
    match setting {
        GlSettings::BlendFunc_SRCAlpha_OneMinusSRCAlpha => {
            raw_opengl::blendfunc(opengl, gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        },
        _ => Err("invalid blendfunc value".to_owned()).unwrap(),
    }
    
}


pub fn set_uniform(opengl:&Gl, program_id:u32,
                    uniform_name:&str, uniform_type:UniformType,
                    value:*const f32) -> Result<(), GlError> {
    let location_name = intermediate_opengl::get_uniform_location(opengl, program_id, uniform_name)?;
    match uniform_type {
        UniformType::Float => raw_opengl::set_uniform_float(opengl, location_name, value),
        UniformType::Vec3  => raw_opengl::set_uniform_vec3(opengl, location_name, value),
        UniformType::Mat4  => raw_opengl::set_uniform_mat4(opengl, location_name, value),
    }
    Ok(())
}


pub fn create_shader_variant(opengl:&Gl, str_text:&str, shader_type:ShaderType) -> Result<u32, GlError> {
    let shader_id = raw_opengl::create_shader(opengl, shader_type);

    intermediate_opengl::shader_source(opengl, shader_id, str_text)?;
    raw_opengl::compile_shader(opengl, shader_id);

    intermediate_opengl::get_compilation_error(opengl, shader_id, ShaderType::AnyShader)?;

    Ok(shader_id)
}


pub fn create_shader_program(opengl:&Gl, vertex_id:u32, fragment_id:u32) -> Result<u32, GlError> {
    let program_id = raw_opengl::create_program(opengl);
    
    raw_opengl::attach_shader(opengl, program_id,   vertex_id);
    raw_opengl::attach_shader(opengl, program_id, fragment_id);
    raw_opengl::link_program(opengl, program_id);
    //raw_opengl::detach_shader(opengl, program_id, vertex_id);
    //raw_opengl::detach_shader(opengl, program_id, fragment_id);
    //raw_opengl::delete_shader(opengl, vertex_id);
    //raw_opengl::delete_shader(opengl, fragment_id);

    intermediate_opengl::get_compilation_error(opengl, program_id, ShaderType::ShaderProgram)?;

    Ok(program_id)
}


pub struct WithObject<'l> {
    pub opengl:&'l Gl,
    pub object_type:GlSettings,
    pub vao_id:u32,
    pub vbo_id:u32,
    pub program_id:u32,
}
impl WithObject<'_> {
    pub fn new_program(opengl:&Gl) -> (u32, WithObject) {
        Err("invalid function for the moment").unwrap()
    }
    pub fn new_vao(opengl:&Gl) -> (u32, WithObject) {
        let vao = raw_opengl::gen_vertex_arrays(opengl);
        (vao, WithObject::vao(opengl, vao))
    }
    pub fn new_vbo(opengl:&Gl) -> (u32, WithObject) {
        let vbo = raw_opengl::gen_buffers(opengl);
        (vbo, WithObject::vbo(opengl, vbo))
    }
    pub fn new_vao_vbo(opengl:&Gl, store_normals:bool, data:&Matrix<f32>) -> (u32, u32) {
        let (vao, with_vao) = WithObject::new_vao(opengl);
        let (vbo, with_vbo) = WithObject::new_vbo(opengl);

        with_vbo.buffer_data(BufferType::ArrayBuffer, data, DrawType::DynamicDraw);
        with_vao.set_vertex_attribs(store_normals, data.dtype_memsize());

        (vao, vbo)
    }
    pub fn vao_vbo(opengl:&Gl, vao:u32, vbo:u32) -> WithObject {
        raw_opengl::bind_vertex_array(opengl, vao);
        raw_opengl::bind_buffer(opengl, BufferType::ArrayBuffer, vbo);
        WithObject { opengl, object_type:GlSettings::Vertex_ArrayObject_BufferObject,
                        vao_id:vao, vbo_id:vbo, program_id:0 }
    }
    pub fn vao(opengl:&Gl, vao:u32) -> WithObject {
        raw_opengl::bind_vertex_array(opengl, vao);
        WithObject { opengl, object_type:GlSettings::VertexArrayObject,
                     vao_id:vao, vbo_id:0, program_id:0 }
    }
    pub fn vbo(opengl:&Gl, vbo:u32) -> WithObject {
        raw_opengl::bind_buffer(opengl, BufferType::ArrayBuffer, vbo);
        WithObject { opengl, object_type:GlSettings::VertexBufferObject,
                     vao_id:0, vbo_id:vbo, program_id:0 }
    }
    pub fn update_vbo(&self, data:&Matrix<f32>) {
        self.buffer_sub_data(BufferType::ArrayBuffer, data);
    }
    pub fn program(opengl:&Gl, program:u32) -> WithObject {
        raw_opengl::use_program(opengl, program);
        WithObject { opengl, object_type:GlSettings::Program,
                     vao_id:0, vbo_id:0, program_id:program }
    }
    pub fn buffer_data(&self, target:GlSettings, data:&Matrix<f32>, draw_type:GlSettings) {
        let data_size = data.memory_size() as gl::types::GLsizeiptr;
        let data_ptr = data.clone().as_ptr() as *const c_void;
        raw_opengl::buffer_data(self.opengl, target, data_size, data_ptr, draw_type);
    }
    pub fn buffer_sub_data(&self, target:GlSettings, data:&Matrix<f32>) {
        let data_size = data.memory_size() as isize;
        let data_ptr = data.as_ptr() as *const c_void;
        match target {
            BufferType::ArrayBuffer => raw_opengl::buffer_sub_data(
                                            self.opengl,
                                            BufferType::ArrayBuffer,
                                            data_size,
                                            data_ptr),
            _ => {},
        }
    }
    pub fn set_vertex_attribs(&self, store_normals:bool, dtype_size:usize) {
        intermediate_opengl::set_vertex_attrib(self.opengl, 0, store_normals, dtype_size);
        intermediate_opengl::set_vertex_attrib(self.opengl, 1, store_normals, dtype_size);
        intermediate_opengl::set_vertex_attrib(self.opengl, 2, store_normals, dtype_size);
        if store_normals { intermediate_opengl::set_vertex_attrib(self.opengl, 3, store_normals, dtype_size); }
    }
    pub fn draw_vao(&self, mode:GlSettings, data:&Matrix<f32>) -> Result<(), GlError> {
        //let num_shapes = 1;
        //let num_shapes = 3;
        match data.ndims() {
            2 =>{ raw_opengl::draw_arrays(self.opengl, mode, data.shape[1]); Ok(())},
            _ => Err(GlError::InvalidDataDims(data.ndims())),
        }
    }
    pub fn set_uniform(&self, uniform_name:&str, uniform_type:UniformType, value:Matrix<f32>) {
        //let mut items = vec![];
        //for row in value.array {
        //    for value in row.vec {
        //        items.push(value);
        //    }
        //}
        //set_uniform(self.opengl, self.program_id, uniform_name, uniform_type, items.as_ptr());
        set_uniform(self.opengl, self.program_id, uniform_name, uniform_type, value.as_ptr());
    }
}
impl Drop for WithObject<'_> {
    fn drop(&mut self) {
        match self.object_type {
            GlSettings::VertexArrayObject => raw_opengl::bind_vertex_array(self.opengl, 0),
            GlSettings::VertexBufferObject => raw_opengl::bind_buffer(self.opengl, BufferType::ArrayBuffer, 0),
            GlSettings::Vertex_ArrayObject_BufferObject => {
                raw_opengl::bind_vertex_array(self.opengl, 0);
                raw_opengl::bind_buffer(self.opengl, BufferType::ArrayBuffer, 0);
            },
            GlSettings::Program => {
                /* this is being done in render clear_bindings by binding to 0 */
                //raw_opengl::use_program(self.opengl, 0)
            },
            _ => Err("invalid object type".to_owned()).unwrap(),
        }
    }
}