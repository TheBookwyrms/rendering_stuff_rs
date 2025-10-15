use crate::gl;
use crate::gl::Gl;
use crate::raw_opengl;
use crate::enums::{BufferType, DrawMode, DrawType, GlError, ShaderType, UniformType};

use std::{ffi::CString, os::raw::c_void};




pub fn load_with<T>(loadfn: T) -> Gl
//pub pub fn load_with<T>(mut loadpub fn: T) -> Gl
        where T:  FnMut(&'static str) -> *const c_void {
    gl::Gl::load_with(loadfn)
}






pub fn create_shader(opengl:&Gl, shader_type:ShaderType) -> Result<u32, GlError> {
    match shader_type {
        ShaderType::VertexShader   => Ok(raw_opengl::create_shader(opengl, gl::VERTEX_SHADER)),
        ShaderType::FragmentShader => Ok(raw_opengl::create_shader(opengl, gl::FRAGMENT_SHADER)),
        other => Err(GlError::InvalidShaderType(other))
    }
}


pub fn shader_source(opengl:&Gl, shader_id:u32, source:&str) -> Result<(), GlError> {
    match CString::new(source) {
        Ok(binding) => {
            let source_ptr = binding.as_c_str().as_ptr();
            raw_opengl::shader_source(opengl, shader_id, &source_ptr);
            Ok(())
        },
        Err(error) => Err(GlError::CStringError(error)),
    }
}


pub fn get_uniform_location(opengl:&Gl, program_id:u32, uniform_name:&str) -> Result<i32, GlError> {
    match CString::new(uniform_name) {
        Ok(cstring) => {
            let cname = cstring.as_bytes_with_nul().as_ptr() as *const i8;
            Ok(raw_opengl::get_uniform_location(opengl, program_id, cname))
        },
        Err(error) => Err(GlError::CStringError(error)),
    }
}


pub fn read_info_log_error(
    opengl:&Gl,
    iv_func: &dyn Fn(&Gl, u32, u32, *mut i32) -> (),
    log_func: &dyn Fn(&Gl, u32, i32, *mut i32, *mut i8) -> (),
    id:u32
) -> String {
        let mut log_len : gl::types::GLint = 0;
        iv_func(opengl, id, gl::INFO_LOG_LENGTH, &mut log_len);

        let mut buffer: Vec<u8> = Vec::with_capacity(log_len as usize + 1);
        buffer.extend([b' '].iter().cycle().take(log_len as usize));
        let error = unsafe { CString::from_vec_unchecked(buffer.clone()) };

        log_func(opengl,
                id, log_len, std::ptr::null_mut(), 
                error.as_ptr() as *mut gl::types::GLchar);
        let error_msg = error.to_string_lossy().into_owned();
        error_msg
}


pub fn get_compilation_error(opengl:&Gl, id:u32, shader_type:ShaderType) -> Result<(), GlError> {

    let mut success = 0; // this defaults error unless it worked // 1 is good, 0 is bad
    let error_msg = match shader_type {
        ShaderType::AnyShader => {
            raw_opengl::get_shader_iv(opengl, id, gl::COMPILE_STATUS, &mut success);
            read_info_log_error(opengl, &raw_opengl::get_shader_iv,
                                &raw_opengl::get_shader_info_log, id)
        },
        ShaderType::ShaderProgram => {
            raw_opengl::get_program_iv(opengl, id, gl::LINK_STATUS, &mut success);
            read_info_log_error(opengl, &raw_opengl::get_program_iv,
                                &raw_opengl::get_program_info_log, id)
        },
        _ => {success = 0; "error".to_owned()}
    };
    match success {
        0 => Err(GlError::CompilationSuccessFailed(error_msg)),
        1 => Ok(()),
        _ => Err(GlError::CompilationSuccessFailed("compilation_success is neither 1 nor 0".to_owned())),
    }
}


pub fn bind_buffer(opengl:&Gl, target:BufferType, buffer:u32) -> Result<(), GlError>{
    match target {
        BufferType::ArrayBuffer => {
            raw_opengl::bind_buffer(opengl, gl::ARRAY_BUFFER, buffer);
            Ok(())
        },
        other => Err(GlError::InvalidBufferType(other)),
    }
}

pub fn buffer_data(
    opengl:&Gl,
    target:BufferType,
    size:gl::types::GLsizeiptr,
    data_ptr:*const gl::types::GLvoid,
    draw_type:DrawType,
) -> Result<(), GlError>{
    match target {
        BufferType::ArrayBuffer => {
            match draw_type {
                DrawType::StaticDraw  => { raw_opengl::buffer_data(opengl, gl::ARRAY_BUFFER, size, data_ptr, gl::STATIC_DRAW); Ok(()) },
                DrawType::StreamDraw  => { raw_opengl::buffer_data(opengl, gl::ARRAY_BUFFER, size, data_ptr, gl::STREAM_DRAW); Ok(()) },
                DrawType::DynamicDraw => { raw_opengl::buffer_data(opengl, gl::ARRAY_BUFFER, size, data_ptr, gl::DYNAMIC_DRAW); Ok(()) },
                other => Err(GlError::InvalidDrawType(other)),
            }
        },
        other => Err(GlError::InvalidBufferType(other)),
    }
    
}

pub fn set_vertex_attrib(opengl:&Gl, layout_location:u32, store_normals:bool, dtype_size:usize
) -> Result<(), GlError>{
    let n_per_vertice : usize = 3;
    let n_per_colour  : usize = 3;
    let n_per_opacity : usize = 1;
    let n_per_normal  : usize = 3;
    let len_ptr = n_per_vertice + n_per_colour +
                            n_per_opacity + if store_normals
                            {n_per_normal} else {0};
    let stride = (len_ptr * dtype_size) as i32;
    let (num_items, offset) = match layout_location {
        0 => Ok((n_per_vertice as i32, 0 as *const c_void)),
        1 => Ok(( n_per_colour as i32 , ((n_per_vertice) * dtype_size) as *const c_void)),
        2 => Ok((n_per_opacity as i32, ((n_per_vertice + n_per_colour) * dtype_size) as *const c_void)),
        3 => if store_normals {
                Ok((n_per_normal as i32, ((n_per_vertice + n_per_colour + n_per_opacity) * dtype_size) as *const c_void))
            } else {Err(GlError::InvalidLayoutLocation(3))},
        n => Err(GlError::InvalidLayoutLocation(n)),
    }?;
    raw_opengl::enable_vertex_attrib_array(opengl, layout_location);
    raw_opengl::vertex_attrib_pointer(opengl, layout_location, num_items, stride, offset);
    Ok(())
}


pub fn buffer_sub_data(opengl:&Gl, target:BufferType, size:isize, data:*const c_void
) -> Result<(), GlError>{
    match target {
        BufferType::ArrayBuffer => {
            raw_opengl::buffer_sub_data(opengl, gl::ARRAY_BUFFER, size, data);
            Ok(())
        },
        other => Err(GlError::InvalidBufferType(other)),
    }
}    


pub fn draw_arrays(opengl:&Gl, mode:DrawMode, num_shapes:i32) -> Result<(), GlError> {
    raw_opengl::point_size(opengl, 10.0);
    match mode {
        DrawMode::GlPoints =>    { raw_opengl::draw_arrays(opengl, gl::POINTS, num_shapes); Ok(())},
        DrawMode::GlLines =>     { raw_opengl::draw_arrays(opengl, gl::LINES, num_shapes); Ok(())},
        DrawMode::GlTriangles => { raw_opengl::draw_arrays(opengl, gl::TRIANGLES, num_shapes); Ok(())},
        other => Err(GlError::InvalidDrawMode(other)),
    }
}