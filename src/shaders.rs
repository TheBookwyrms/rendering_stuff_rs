pub mod shaders {

    //use crate::{gl, shaders};
    use crate::window_loader::gl;
    use crate::window_loader::gl::Gl;
    use std::ffi::{CStr, CString};

    use std::{error::Error, fmt};

    use rust_embed::Embed;
    

    #[derive(Embed)]
    #[folder = "src/shaders_glsl/"]
    struct Asset;
    

    #[derive(Debug)]
    pub struct ShaderError {
        msg:String
    }

    impl fmt::Display for ShaderError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.msg)
        }
    }

    #[derive(Debug)]
    pub enum ProgramType {
        Object,
        Lighting,
    }
    #[derive(Debug)]
    pub enum ShaderType {
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
        pub shader_type : ShaderType,
        pub shader_id : u32,
    }

    #[derive(Debug)]
    pub struct ShaderProgram {
        pub program_id : u32,
        pub program_type : ProgramType,
    }

    impl ShaderProgram {
        pub fn new<'b>(opengl:&'b Gl, program_type:ProgramType)  -> ShaderProgram {

            let vertex:Shader<'_>;
            let fragment:Shader<'_>;

            match program_type {
                ProgramType::Object=> {
                    vertex   = Shader::new(opengl, get_shader_text("object_vertex_shader"), ShaderType::Vertex).expect("failed to compile");
                    fragment = Shader::new(opengl, get_shader_text("object_fragment_shader"), ShaderType::Fragment).expect("failed to compile");
                },
                ProgramType::Lighting=> {
                    vertex   = Shader::new(opengl, get_shader_text("lighting_vertex_shader"), ShaderType::Vertex).expect("failed to compile");
                    fragment = Shader::new(opengl, get_shader_text("lighting_fragment_shader"), ShaderType::Fragment).expect("failed to compile");
                },
            }

            let program_id = create_program(opengl, vertex.shader_id, fragment.shader_id);

            let error = get_gl_error_msg(
                opengl, &get_program_iv, &get_program_info_log, program_id, ErrorChecks::LinkStatus
            );

            match error {
                Ok(_) => {ShaderProgram { program_id:program_id, program_type:program_type }},
                Err(err) => {Err(err.msg).expect("failed to compile")},
            }
        }

    }

    impl Drop for Shader<'_> {
        fn drop(&mut self) {
            unsafe {
                self.opengl.DeleteShader(self.shader_id);
            }
        }
    }

    impl Shader<'_> {
        pub fn new<'a>(opengl:&'a Gl, shader_text:String, shader_type : ShaderType
                            ) -> Result<Shader<'a>, ShaderError> {

            let str_text = shader_text.as_str();
            let binding = CString::new(str_text).expect("failed to &CStr");
            let source = binding.as_c_str();

            let shader_id : gl::types::GLuint;
            let mut compilation_success : gl::types::GLint = 1;

            unsafe {
                match shader_type {
                    ShaderType::Vertex => shader_id = opengl.CreateShader(gl::VERTEX_SHADER),
                    ShaderType::Fragment => shader_id = opengl.CreateShader(gl::FRAGMENT_SHADER),
                }
                opengl.ShaderSource(shader_id, 1, &source.as_ptr(), std::ptr::null());
                opengl.CompileShader(shader_id);
            }

            let error = get_gl_error_msg(
                opengl,
                &get_shader_iv,
                &get_shader_info_log,
                shader_id,
            ErrorChecks::CompileStatus)?;

            Ok( Shader {opengl:opengl, shader_type:shader_type, shader_id:shader_id} )
        }
    }


    fn get_gl_error_msg(opengl:&Gl,
                        iv_func: &dyn Fn(&Gl, u32, u32, *mut i32) -> (),
                        log_func: &dyn Fn(&Gl, u32, i32, *mut i32, *mut i8) -> (),
                        id:u32, 
                        checking_status:ErrorChecks) -> Result<&'static str, ShaderError> {
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
                //let msg:&str = string_msg.as_str();

                Err(ShaderError { msg: string_msg })},
            1 => Ok("no error"),
            _ => Err(ShaderError { msg: "compilation_success is neither 1 nor 0".to_owned() }),

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

    pub fn get_shader_text(filename:&str) -> String {
        let mut file = filename.to_owned();
        file.push_str(".glsl");
        let file = file.as_str();

        let glsl = Asset::get(file).unwrap();
        let shader_text = std::str::from_utf8(glsl.data.as_ref()).unwrap().to_owned();
        shader_text
    }

    fn create_program<'b>(opengl:&'b Gl, vertex_id:u32, fragment_id:u32) -> u32 {
        unsafe {
            let program_id = opengl.CreateProgram();
            opengl.AttachShader(program_id,   vertex_id);
            opengl.AttachShader(program_id, fragment_id);
            opengl.LinkProgram(program_id);
            opengl.DeleteShader(vertex_id);
            opengl.DeleteShader(fragment_id);
            program_id
        }
    }
}