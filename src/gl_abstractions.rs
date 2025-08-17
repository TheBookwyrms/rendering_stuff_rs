pub mod gl {
    include!(concat!(env!("OUT_DIR"), "\\gl_bindings.rs"));

    use std::fmt;
    use crate::gl_abstractions::gl;

    impl fmt::Debug for gl::Gl {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "opengl fmt")
        }
    }
}

pub mod OpenGl {

    use std::{fmt, u32, ffi::{CStr, CString}};
    use std::os::raw;
    use ndarray;

    use crate::gl_abstractions::gl;
    use crate::gl_abstractions::gl::Gl;

        #[derive(Debug)]
    pub struct ShaderError {
        msg:String
    }

    impl fmt::Display for ShaderError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.msg)
        }
    }

    pub enum ShaderVariant {
        Shader,
        Program
    }

    #[derive(Clone, Copy, Debug)]
    pub enum ShaderType {
        Vertex,
        Fragment,
    }
    
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub enum GlSettings {
        DepthTest,
        Multisample,
        Blend,
        BlendFunc_SRCAlpha_OneMinusSRCAlpha,
        ColourBufferBit,
        DepthBufferBit,
    }

    pub struct GlError{
        pub msg:String
    }
    impl GlError {
        pub fn ok() -> GlError {
            GlError { msg:"nothing wrong".to_owned() }
        }
    }
    impl fmt::Display for GlError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.msg)
        }
    }

    pub fn load_with<T>(mut loadfn: T) -> Gl
            where T:  FnMut(&'static str) -> *const raw::c_void {
        gl::Gl::load_with(loadfn)
    }
    pub fn clear_colour(opengl:&Gl, r:f32, g:f32, b:f32, a:f32) {
        let validity = vec![r, g, b, a].into_iter().filter(|c| 0.0<=*c && *c<=1.0).count();
        match validity {
            4 => unsafe { opengl.ClearColor(r, g, b, a) },
            _ => Err("r, g, b, or a not within 0..=1".to_owned()).unwrap(),
        };
    }
    pub fn clear(opengl:&Gl, masks:Vec<GlSettings>) {
        let allowed = vec![GlSettings::ColourBufferBit, GlSettings::DepthBufferBit];
        let validity = masks.clone().into_iter().filter(|m| allowed.contains(m)).count();
        match validity {
            value if value == allowed.len() => {
                let mut start = u32::MAX;
                let mut start = 0;
                for mask in masks.clone() {
                    start = match mask {
                        GlSettings::ColourBufferBit => start | gl::COLOR_BUFFER_BIT,
                        GlSettings::DepthBufferBit  => start | gl::DEPTH_BUFFER_BIT,
                        _ => start,
                    };
                }
                unsafe { opengl.Clear(start) };
            },
            _ => Err("invalid clear masks".to_owned()).unwrap(),
        }
    }
    pub fn gl_enable(opengl:&Gl, setting:GlSettings) {
        match setting {
                GlSettings::DepthTest => { unsafe { opengl.Enable(gl::DEPTH_TEST) }},
                GlSettings::Multisample =>  { unsafe { opengl.Enable(gl::MULTISAMPLE) }},
                GlSettings::Blend =>  { unsafe { opengl.Enable(gl::BLEND) }},
                _ => Err("invalid gl_enable value".to_owned()).unwrap(),
            }
    }
    pub fn gl_blendfunc(opengl:&Gl, setting:GlSettings) {
        match setting {
            GlSettings::BlendFunc_SRCAlpha_OneMinusSRCAlpha => {
                unsafe {
                    opengl.BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA)
                };
            },
            _ => Err("invalid blendfunc value".to_owned()).unwrap(),
        }
    }
    fn get_uniform_location(opengl:&Gl, program_id:u32, uniform_name:&str) -> i32 {
        let cstring = CString::new(uniform_name).expect("failed to turn &str into String");
        let cname = cstring.as_bytes_with_nul().as_ptr() as *const i8;
        unsafe { opengl.GetUniformLocation(program_id, cname) }
    }
    pub fn set_uniform_float(opengl:&Gl, program_id:u32, uniform_name:&str, float:f32) {
        let uniform_loc = get_uniform_location(opengl, program_id, uniform_name);
        unsafe { opengl.Uniform1f(uniform_loc, float) }
    }
    pub fn set_uniform_vec3(opengl:&Gl, program_id:u32, uniform_name:&str, vec3_ptr:*const f32) {
        let uniform_loc = get_uniform_location(opengl, program_id, uniform_name);
        unsafe { opengl.Uniform3fv(uniform_loc, 1, vec3_ptr) }
    }
    pub fn set_uniform_mat4(opengl:&Gl, program_id:u32, uniform_name:&str, mat4_ptr:*const f32) {
        let uniform_loc = get_uniform_location(opengl, program_id, uniform_name);
        unsafe {
            opengl.UniformMatrix4fv(uniform_loc, 1, gl::TRUE, mat4_ptr)
        }
    }
    pub fn create_shader(opengl:&Gl, shader_type:ShaderType) -> u32 {
        unsafe {
            match shader_type {
                ShaderType::Vertex => opengl.CreateShader(gl::VERTEX_SHADER),
                ShaderType::Fragment => opengl.CreateShader(gl::FRAGMENT_SHADER),
            }
        }
    }
    pub fn shader_source(opengl:&Gl, shader_id:u32, source:&str) {
        let binding = CString::new(source).expect("failed to &CStr");
        let source_ptr = binding.as_c_str().as_ptr();
        unsafe {
            opengl.ShaderSource(shader_id, 1, &source_ptr, std::ptr::null());
        }
    }
    pub fn compile_shader(opengl:&Gl, shader_id:u32) {
        unsafe { opengl.CompileShader(shader_id) }
    }
    pub fn get_shader_iv(opengl:&Gl, shader:u32, pname:u32, params:*mut i32) {
        unsafe { opengl.GetShaderiv(shader, pname, params) } ;
    }
    pub fn get_program_iv(opengl:&Gl, program:u32, pname:u32, params:*mut i32) {
        unsafe { opengl.GetProgramiv(program, pname, params) } ;
    }
    pub fn get_shader_info_log(opengl:&Gl, shader:u32, bufsize:i32, length:*mut i32, infolog:*mut i8) {
        unsafe { opengl.GetShaderInfoLog(shader, bufsize, length, infolog); } ;
    }
    pub fn get_program_info_log(opengl:&Gl, program:u32, bufsize:i32, length:*mut i32, infolog:*mut i8) {
        unsafe { opengl.GetProgramInfoLog(program, bufsize, length, infolog); } ;
    }
    pub fn read_info_log_error(
        opengl:&Gl,
        iv_func: &dyn Fn(&Gl, u32, u32, *mut i32) -> (),
        log_func: &dyn Fn(&Gl, u32, i32, *mut i32, *mut i8) -> (),
        id:u32) -> String {
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
    pub fn get_compilation_error(opengl:&Gl, id:u32, shader_variant:ShaderVariant) {
        let mut success = 1; // 1 is good, 0 is bad
        let error_msg = match shader_variant {
            ShaderVariant::Shader => {
                get_shader_iv(opengl, id, gl::LINK_STATUS, &mut success);
                read_info_log_error(opengl, &get_shader_iv, &get_shader_info_log, id)
            },
            ShaderVariant::Program => {
                get_shader_iv(opengl, id, gl::COMPILE_STATUS, &mut success);
                read_info_log_error(opengl, &get_program_iv, &get_program_info_log, id)
            },
        };
        match success {
            0 => Err(error_msg).expect("failed to compile"),
            1 => {},
            _ => Err("compilation_success is neither 1 nor 0".to_owned()).expect("failed to compile"),
        }
    }
    pub fn create_program(opengl:&Gl) -> u32 { unsafe { opengl.CreateProgram() } }
    pub fn attach_shader(opengl:&Gl, program_id:u32, shader_id:u32) {
        unsafe { opengl.AttachShader(program_id, shader_id) } }
    pub fn link_program(opengl:&Gl, program_id:u32) { unsafe { opengl.LinkProgram(program_id) } }
    pub fn delete_shader(opengl:&Gl, shader_id:u32) { unsafe { opengl.DeleteShader(shader_id) } }
}