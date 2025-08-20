pub mod OpenGl {
    mod gl {
        include!(concat!(env!("OUT_DIR"), "\\gl_bindings.rs"));

        use std::fmt;

        pub(super) mod Magic {
            pub use super::Gl;
        }

        impl fmt::Debug for Gl {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "opengl fmt")
            }
        }
    }

    pub use gl::Magic::Gl;

    use std::{ffi::{CStr, CString}, os::raw::c_void};

    use crate::ndarray_abstractions::MyArray::N as nd_trait;


    const F32_SIZE : usize = std::mem::size_of::<f32>();








    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    // public items
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    

    pub enum UniformType {
        Float,
        Vec3,
        Mat4,
    }


    #[derive(Clone, Copy, Debug)]
    pub enum ShaderType {
        VertexShader,
        FragmentShader,
        AnyShader,
        ShaderProgram,
    }
    

    #[derive(Copy, Clone, PartialEq, Debug)]
    pub enum GlSettings {
        DepthTest,
        Multisample,
        Blend,
        BlendFunc_SRCAlpha_OneMinusSRCAlpha,
        ColourBufferBit,
        DepthBufferBit,
        ArrayBuffer,
        VertexArrayObject,
        VertexBufferObject,
        StaticDraw,
        StreamDraw,
        DynamicDraw,
    }


    pub fn load_with<T>(mut loadfn: T) -> Gl
            where T:  FnMut(&'static str) -> *const c_void {
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


    pub fn set_uniform(opengl:&Gl, program_id:u32,
                       uniform_name:&str, uniform_type:UniformType,
                       value:*const f32) {
        match uniform_type {
            UniformType::Float => set_uniform_float(opengl, program_id, uniform_name, value),
            UniformType::Vec3 => set_uniform_vec3(opengl, program_id, uniform_name, value),
            UniformType::Mat4 => set_uniform_mat4(opengl, program_id, uniform_name, value),
        }
    }


    pub fn create_shader_variant(opengl:&Gl, str_text:&str, shader_type:ShaderType) -> u32 {
        let shader_id = create_shader(opengl, shader_type);

        shader_source(opengl, shader_id, str_text);
        compile_shader(opengl, shader_id);

        let success = get_compilation_error(opengl, shader_id, ShaderType::AnyShader)
                              .expect("failed to compile program");

        shader_id
    }


    pub fn create_shader_program(opengl:&Gl, vertex_id:u32, fragment_id:u32) -> u32 {
        let program_id = create_program(opengl);
        
        attach_shader(opengl, program_id,   vertex_id);
        attach_shader(opengl, program_id, fragment_id);
        link_program(opengl, program_id);
        //delete_shader(opengl, vertex_id);
        //delete_shader(opengl, fragment_id);

        let success = get_compilation_error(opengl, program_id, ShaderType::ShaderProgram)
                              .expect("failed to compile program");

        program_id
    }


    pub struct WithObject<'l> {
        pub opengl:&'l Gl,
        pub object_type:GlSettings,
        pub object_id:u32,
    }
    impl WithObject<'_> {
        pub fn new_vao(opengl:&Gl) -> (u32, WithObject) {
            let vao = gen_vertex_arrays(opengl);
            (vao, WithObject::vao(opengl, vao))
        }
        pub fn new_vbo(opengl:&Gl) -> (u32, WithObject) {
            let vbo = gen_buffers(opengl);
            (vbo, WithObject::vbo(opengl, vbo))
        }
        pub fn vao(opengl:&Gl, vao:u32) -> WithObject {
            bind_vertex_array(opengl, vao);
            WithObject { opengl, object_type:GlSettings::VertexArrayObject, object_id:vao }
        }
        pub fn vbo(opengl:&Gl, vbo:u32) -> WithObject {
            bind_buffer(opengl, GlSettings::ArrayBuffer, vbo);
            WithObject { opengl, object_type:GlSettings::VertexBufferObject, object_id:vbo }
        }
        pub fn buffer_data<N:nd_trait>(&self, target:GlSettings, data:&N, draw_type:GlSettings) {
            let data_size = (data.shape() * F32_SIZE) as isize;
            let data_ptr = data.as_ptr_void();
            buffer_data(self.opengl, target, data_size, data_ptr, draw_type);
        }
        pub fn buffer_sub_data<N:nd_trait>(&self, target:GlSettings, data:&N) {
            let data_size = (data.shape() * F32_SIZE) as isize;
            let data_ptr = data.as_ptr_void();
            buffer_sub_data(self.opengl, GlSettings::ArrayBuffer, data_size, data_ptr);
        }
        pub fn set_vertex_attribs(&self, store_normals:bool) {
            set_vertex_attrib(self.opengl, 0, store_normals);
            set_vertex_attrib(self.opengl, 1, store_normals);
            set_vertex_attrib(self.opengl, 2, store_normals);
            if store_normals { set_vertex_attrib(self.opengl, 3, store_normals); }
        }
        pub fn draw_vao<N:nd_trait>(&self, mode:GlSettings, data:&N) {
            let num_shapes = data.dimension0();
            draw_arrays(self.opengl, mode, num_shapes);
        }
    }
    impl Drop for WithObject<'_> {
        fn drop(&mut self) {
            match self.object_type {
                GlSettings::VertexArrayObject => bind_vertex_array(self.opengl, 0),
                GlSettings::VertexBufferObject => bind_buffer(self.opengl, GlSettings::ArrayBuffer, 0),
                _ => Err("invalid object type".to_owned()).unwrap(),
            }
        }
    }






    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    // private functions
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //



    

    fn create_shader(opengl:&Gl, shader_type:ShaderType) -> u32 {
        let shader_id =  unsafe {
            match shader_type {
                ShaderType::VertexShader => Ok(opengl.CreateShader(gl::VERTEX_SHADER)),
                ShaderType::FragmentShader => Ok(opengl.CreateShader(gl::FRAGMENT_SHADER)),
                _ => Err(0)
            }
        };
        shader_id.expect("invalid shader type")
    }
    fn shader_source(opengl:&Gl, shader_id:u32, source:&str) {
        let binding = CString::new(source).expect("failed to &CStr");
        let source_ptr = binding.as_c_str().as_ptr();
        unsafe {
            opengl.ShaderSource(shader_id, 1, &source_ptr, std::ptr::null());
        }
    }
    fn compile_shader(opengl:&Gl, shader_id:u32) {
        unsafe { opengl.CompileShader(shader_id) } }
    fn get_shader_iv(opengl:&Gl, shader:u32, pname:u32, params:*mut i32) {
        unsafe { opengl.GetShaderiv(shader, pname, params) } ; }
    fn get_program_iv(opengl:&Gl, program:u32, pname:u32, params:*mut i32) {
        unsafe { opengl.GetProgramiv(program, pname, params) } ; }
    fn get_shader_info_log(opengl:&Gl, shader:u32, bufsize:i32, length:*mut i32, infolog:*mut i8) {
        unsafe { opengl.GetShaderInfoLog(shader, bufsize, length, infolog); } ; }
    fn get_program_info_log(opengl:&Gl, program:u32, bufsize:i32, length:*mut i32, infolog:*mut i8) {
        unsafe { opengl.GetProgramInfoLog(program, bufsize, length, infolog); } ; }
        
    fn read_info_log_error(
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
    fn get_compilation_error(opengl:&Gl, id:u32, shader_type:ShaderType)
                    -> Result<String, String> {
        let mut success = 1; // 1 is good, 0 is bad
        let error_msg = match shader_type {
            ShaderType::AnyShader => {
                get_shader_iv(opengl, id, gl::COMPILE_STATUS, &mut success);
                read_info_log_error(opengl, &get_shader_iv, &get_shader_info_log, id)
            },
            ShaderType::ShaderProgram => {
                get_shader_iv(opengl, id, gl::LINK_STATUS, &mut success);
                read_info_log_error(opengl, &get_program_iv, &get_program_info_log, id)
            },
            _ => {success = 0; "error".to_owned()}
        };
        match success {
            0 => Err(error_msg),
            1 => Ok("no error".to_owned()),
            _ => Err("compilation_success is neither 1 nor 0".to_owned()),
        }
    }
    fn create_program(opengl:&Gl) -> u32 { unsafe { opengl.CreateProgram() } }
    fn attach_shader(opengl:&Gl, program_id:u32, shader_id:u32) {
        unsafe { opengl.AttachShader(program_id, shader_id) } }
    fn link_program(opengl:&Gl, program_id:u32) { unsafe { opengl.LinkProgram(program_id) } }
    pub fn delete_shader(opengl:&Gl, shader_id:u32) { unsafe { opengl.DeleteShader(shader_id) } }



    fn get_uniform_location(opengl:&Gl, program_id:u32, uniform_name:&str) -> i32 {
        let cstring = CString::new(uniform_name).expect("failed to turn &str into String");
        let cname = cstring.as_bytes_with_nul().as_ptr() as *const i8;
        unsafe { opengl.GetUniformLocation(program_id, cname) }
    }
    fn set_uniform_float(opengl:&Gl, program_id:u32, uniform_name:&str, float:*const f32) {
        let uniform_loc = get_uniform_location(opengl, program_id, uniform_name);
        unsafe { opengl.Uniform1f(uniform_loc, *float) }
    }
    fn set_uniform_vec3(opengl:&Gl, program_id:u32, uniform_name:&str, vec3_ptr:*const f32) {
        let uniform_loc = get_uniform_location(opengl, program_id, uniform_name);
        unsafe { opengl.Uniform3fv(uniform_loc, 1, vec3_ptr) }
    }
    fn set_uniform_mat4(opengl:&Gl, program_id:u32, uniform_name:&str, mat4_ptr:*const f32) {
        let uniform_loc = get_uniform_location(opengl, program_id, uniform_name);
        unsafe {
            opengl.UniformMatrix4fv(uniform_loc, 1, gl::TRUE, mat4_ptr)
        }
    }



    fn gen_vertex_arrays(opengl:&Gl) -> u32 {
        let mut vao = 0;
        unsafe { opengl.GenVertexArrays(1, &mut vao) }
        vao
    }
    fn gen_buffers(opengl:&Gl) -> u32 {
        let mut vbo = 0;
        unsafe { opengl.GenBuffers(1, &mut vbo) }
        vbo
    }


    fn bind_vertex_array(opengl:&Gl, vao:u32) { unsafe { opengl.BindVertexArray(vao) } }
    fn bind_buffer(opengl:&Gl, target:GlSettings, buffer:u32) {
        match target {
            GlSettings::ArrayBuffer => unsafe {
                opengl.BindBuffer(gl::ARRAY_BUFFER, buffer)
            },
            _ => Err("invalid buffer type".to_owned()).unwrap(),
        }
    }
    fn buffer_data(
        opengl:&Gl,
        target:GlSettings,
        size:gl::types::GLsizeiptr,
        data_ptr:*const gl::types::GLvoid,
        draw_type:GlSettings,
    ) {
        match target {
            GlSettings::ArrayBuffer => {
                match draw_type {
                    GlSettings::StaticDraw => unsafe {
                        opengl.BufferData(gl::ARRAY_BUFFER, size, data_ptr, gl::STATIC_DRAW)
                    },
                    GlSettings::StreamDraw => unsafe {
                        opengl.BufferData(gl::ARRAY_BUFFER, size, data_ptr, gl::STREAM_DRAW)
                    },
                    GlSettings::DynamicDraw => unsafe {
                        opengl.BufferData(gl::ARRAY_BUFFER, size, data_ptr, gl::DYNAMIC_DRAW)
                    },
                    _ => Err("invalid draw usage type").unwrap(),
                }
            },
            _ => Err("invalid buffer_data target").unwrap(),
        }
    }

    fn set_vertex_attrib(opengl:&Gl, layout_location:u32, store_normals:bool) {
        let n_per_vertice : usize = 3;
        let n_per_colour : usize = 3;
        let n_per_opacity : usize = 1;
        let n_per_normal : usize = 3;
        let stride = if store_normals {
            n_per_vertice+n_per_colour+n_per_opacity+n_per_normal
        } else {n_per_vertice+n_per_colour+n_per_opacity} as i32;
        let loc_info = match layout_location {
            0 => Ok([n_per_vertice, (0) * F32_SIZE]),
            1 => Ok([n_per_colour, (n_per_vertice) * F32_SIZE]),
            2 => Ok([n_per_opacity, (n_per_vertice + n_per_colour) * F32_SIZE]),
            3 => if store_normals {
                    Ok([n_per_normal, (n_per_vertice + n_per_colour + n_per_normal) * F32_SIZE])
                } else {Err("normals are not included")},
            _ => Err("invalid layout location"),
        }.unwrap();
        let num_items : i32 = loc_info[0].try_into().unwrap();
        let offset = loc_info[1] as *const gl::types::GLvoid;
        enable_vertex_attrib_array(opengl, layout_location);
        vertex_attrib_pointer(opengl, layout_location, num_items, stride, offset);
    }
    fn enable_vertex_attrib_array(opengl:&Gl, layout_location:u32) {
        unsafe { opengl.EnableVertexAttribArray(layout_location) }
    }
    fn vertex_attrib_pointer(
        opengl:&Gl,
        layout_location:u32,
        num_items_in_location:i32,
        stride_between_points:i32,
        ptr_to_location_in_point:*const c_void) {
        unsafe {
            opengl.VertexAttribPointer(
                layout_location,
                num_items_in_location,
                gl::FLOAT,
                gl::FALSE,
                stride_between_points,
                ptr_to_location_in_point)
        }
    }
    fn buffer_sub_data(opengl:&Gl, target:GlSettings, size:isize, data:*const c_void) {
        match target {
            GlSettings::ArrayBuffer => unsafe {
                opengl.BufferSubData(gl::ARRAY_BUFFER, 0, size, data)},
            _ => Err("invalid buffer target").unwrap(),
        }
    }
    fn draw_arrays(opengl:&Gl, mode:GlSettings, num_shapes:i32) {
        match mode {
            GlSettings::StaticDraw => unsafe {opengl.DrawArrays(gl::STATIC_DRAW, 0, num_shapes)},
            GlSettings::StreamDraw => unsafe {opengl.DrawArrays(gl::STREAM_DRAW, 0, num_shapes)},
            GlSettings::DynamicDraw => unsafe {opengl.DrawArrays(gl::DYNAMIC_DRAW, 0, num_shapes)},
            _ => Err("invalid draw mode").unwrap(),
        }
    }
}