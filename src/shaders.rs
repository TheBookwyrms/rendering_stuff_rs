pub mod shaders {

    use crate::{gl, shaders};
    use crate::gl::Gl;
    use std::ffi::{CString, CStr};

    #[derive(Debug)]
    pub enum Shaders {
        Vertex,
        Fragment,
    }
    pub enum ErrorChecks {
        LinkStatus,
        CompileStatus,
    }

    #[derive(Debug)]
    pub struct Shader<'a> {
        pub opengl : &'a Gl,
        pub shader_type : Shaders,
        pub shader_id : u32,
    }

    #[derive(Debug)]
    pub struct ShaderProgram<'b> {
        pub opengl : &'b Gl,
        pub program_id : u32,
    }

    impl ShaderProgram<'_> {
        pub fn new<'b>(opengl:&'b Gl)  -> Result<ShaderProgram, String> {
            let program_id : gl::types::GLuint;
            let vertex = Shader::new(opengl, get_vertex_shader_text(), Shaders::Vertex).expect("failed to compile");
            let fragment = Shader::new(opengl, get_fragment_shader_text(), Shaders::Fragment).expect("failed to compile");
            unsafe {
                program_id = opengl.CreateProgram();
                opengl.AttachShader(program_id,   vertex.shader_id);
                opengl.AttachShader(program_id, fragment.shader_id);
                opengl.LinkProgram(program_id);
                //opengl.DetachShader(program_id, vertex.shader_id);
                //opengl.DetachShader(program_id, fragment.shader_id);
                opengl.DeleteShader(vertex.shader_id);
                opengl.DeleteShader(fragment.shader_id);
            }

            let shader_program = ShaderProgram { opengl:opengl, program_id:program_id };


            //let mut success: gl::types::GLint = 1;
            //unsafe { opengl.GetProgramiv(program_id, gl::LINK_STATUS, &mut success) }
            //println!("d {:?} {:?}", success, success);

            let error = get_gl_error_msg(
                opengl,
                 &get_program_iv,
                &get_program_info_log,
                program_id,
            ErrorChecks::LinkStatus);

            match error {
                Err(msg) => Err(msg),
                Ok(_) => Ok(shader_program),
            }

            //let mut link_success: gl::types::GLint = 1;
            //unsafe {
            //    gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut link_success);
            //}
    //
            //match link_success {
            //    0 => {let error_msg = shader.get_error_message();
            //        Err(error_msg).expect("failed to compile shader")},
            //    1 => shader,
            //    _ => Err("shader compilation_success is neither 1 nor 0".to_string()).expect("failed to compile shader"),
            //}

        }

        pub fn use_program(&self) {
            unsafe { self.opengl.UseProgram(self.program_id) };
        }
    }

    impl Drop for Shader<'_> {
        fn drop(&mut self) {
            unsafe {
                self.opengl.DeleteShader(self.shader_id);
            }
        }
    }
    impl Drop for ShaderProgram<'_> {
        fn drop(&mut self) {
            unsafe {
                self.opengl.DeleteProgram(self.program_id);
            }
        }
    }

    impl Shader<'_> {
        //fn get_error_message(&self) -> String {
        //    let mut log_len : gl::types::GLint = 0;
        //
        //    let mut buffer: Vec<u8> = Vec::with_capacity(log_len as usize + 1);
        //    buffer.extend([b' '].iter().cycle().take(log_len as usize));
        //    unsafe {
        //        let error: CString = CString::from_vec_unchecked(buffer);
        //        self.opengl.GetShaderiv(self.shader_id, gl::INFO_LOG_LENGTH, &mut log_len);
        //        self.opengl.GetShaderInfoLog(
        //                self.shader_id, log_len, std::ptr::null_mut(), 
        //                error.as_ptr() as *mut gl::types::GLchar);
        //        error.to_string_lossy().into_owned()
        //    }
        //}

        pub fn new<'a>(opengl:&'a Gl, shader_text:&'a str, shader_type : Shaders) -> Result<Shader<'a>, String> {

            let binding = CString::new(shader_text)
                                                .expect("failed to turn &str into CString");
            let source = binding.as_c_str();

            let shader_id : gl::types::GLuint;
            let mut compilation_success : gl::types::GLint = 1;

            unsafe {
                match shader_type {
                    Shaders::Vertex => shader_id = opengl.CreateShader(gl::VERTEX_SHADER),
                    Shaders::Fragment => shader_id = opengl.CreateShader(gl::FRAGMENT_SHADER),
                }
                //opengl.ShaderSource(shader_id, 1, &source.as_ptr() as *const *const i8, std::ptr::null());
                opengl.ShaderSource(shader_id, 1, &source.as_ptr(), std::ptr::null());
                opengl.CompileShader(shader_id);
                //opengl.GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut compilation_success);
            }

            let shader = Shader {opengl:opengl, shader_type:shader_type, shader_id:shader_id};


            //let mut success: gl::types::GLint = 1;
            //unsafe { opengl.GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success) }

            //let mut success: gl::types::GLint = 1;
            //unsafe { opengl.GetShaderiv(shader_id, checking_status, &mut success) }

            let error = get_gl_error_msg(
                opengl,
                &get_shader_iv,
                &get_shader_info_log,
                shader_id,
            ErrorChecks::CompileStatus);

            match error {
                Err(msg) => Err(msg),
                Ok(_) => Ok(shader),
            }

            //match compilation_success {
            //    0 => {let error_msg = shader.get_error_message();
            //        Err(error_msg).expect("failed to compile shader")},
            //    1 => shader,
            //    _ => Err("shader compilation_success is neither 1 nor 0".to_string()).expect("failed to compile shader"),
            //}
        }
    }


    fn get_gl_error_msg(opengl:&Gl,
                        iv_func: &dyn Fn(&Gl, u32, u32, *mut i32) -> (),
                        log_func: &dyn Fn(&Gl, u32, i32, *mut i32, *mut i8) -> (),
                        id:u32, 
                        checking_status:ErrorChecks) -> Result<&'static str, String> {
        let mut success: gl::types::GLint = 1;
        unsafe {
            match checking_status {
                ErrorChecks::LinkStatus => iv_func(opengl, id, gl::LINK_STATUS, &mut success),
                ErrorChecks::CompileStatus => iv_func(opengl, id, gl::COMPILE_STATUS, &mut success),   
            }
        }

        match success {
            0 => {
                let error_msg = unsafe {
                    let mut log_len : gl::types::GLint = 0;
                    iv_func(opengl, id, gl::INFO_LOG_LENGTH, &mut log_len);

                    let mut buffer: Vec<u8> = Vec::with_capacity(log_len as usize + 1);
                    buffer.extend([b' '].iter().cycle().take(log_len as usize));
                    let error: CString = CString::from_vec_unchecked(buffer.clone());

                    log_func(opengl,
                            id, log_len, std::ptr::null_mut(), 
                            error.as_ptr() as *mut gl::types::GLchar);
                    error
                };
                let string_msg = error_msg.to_string_lossy().into_owned();
                let msg:&str = string_msg.as_str();

                Err(msg).expect("failed to compile")},
            1 => Ok("no error"),
            _ => Err("compilation_success is neither 1 nor 0".to_owned()).expect("failed to compile"),

        }
    }


    fn get_shader_iv(opengl:&Gl, shader:u32, pname:u32, params:*mut i32) {
        unsafe { opengl.GetShaderiv(shader, pname, params) } ;
    }
    fn get_program_iv(opengl:&Gl, program:u32, pname:u32, params:*mut i32) {
        unsafe { opengl.GetProgramiv(program, pname, params) } ;
    }
    fn get_shader_info_log(opengl:&Gl, shader:u32, bufsize:i32, length:*mut i32, infolog:*mut i8) {
        unsafe { opengl.GetShaderInfoLog(shader, bufsize, length, infolog); } ;
    }
    fn get_program_info_log(opengl:&Gl, program:u32, bufsize:i32, length:*mut i32, infolog:*mut i8) {
        unsafe { opengl.GetProgramInfoLog(program, bufsize, length, infolog); } ;
    }

    pub fn get_vertex_shader_text() -> &'static str {
        let vert_shader_text = include_str!("shaders_glsl/vertex_shader.glsl");
        vert_shader_text
    }

    pub fn get_fragment_shader_text() -> &'static str {
        let frag_shader_text = include_str!("shaders_glsl/fragment_shader.glsl");
        frag_shader_text
    }
}