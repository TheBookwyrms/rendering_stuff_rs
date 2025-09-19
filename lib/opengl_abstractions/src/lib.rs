#![allow(warnings)]

#[allow(non_camel_case_types)]

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

use std::{ffi::CString, os::raw::c_void};
//use std::{ffi::{CStr, CString}, os::raw::c_void};


use matrices::Matrix2d;

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
    Program,
    Vertex_ArrayObject_BufferObject,
    StaticDraw,
    StreamDraw,
    DynamicDraw,
    GlTriangles,
    GlPoints,
    GlLines,
}


pub fn load_with<T>(loadfn: T) -> Gl
//pub fn load_with<T>(mut loadfn: T) -> Gl
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
            //let mut start = u32::MAX;
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

    get_compilation_error(opengl, shader_id, ShaderType::AnyShader)
        .expect("failed to compile program");

    shader_id
}


pub fn create_shader_program(opengl:&Gl, vertex_id:u32, fragment_id:u32) -> u32 {
    let program_id = create_program(opengl);
    
    attach_shader(opengl, program_id,   vertex_id);
    attach_shader(opengl, program_id, fragment_id);
    link_program(opengl, program_id);
    //detach_shader(opengl, program_id, vertex_id);
    //detach_shader(opengl, program_id, fragment_id);
    //delete_shader(opengl, vertex_id);
    //delete_shader(opengl, fragment_id);

    get_compilation_error(opengl, program_id, ShaderType::ShaderProgram)
        .expect("failed to compile program");

    program_id
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
        let vao = gen_vertex_arrays(opengl);
        (vao, WithObject::vao(opengl, vao))
    }
    pub fn new_vbo(opengl:&Gl) -> (u32, WithObject) {
        let vbo = gen_buffers(opengl);
        (vbo, WithObject::vbo(opengl, vbo))
    }
    pub fn new_vao_vbo(opengl:&Gl, store_normals:bool, data:&Matrix2d) -> (u32, u32) {
        let (vao, with_vao) = WithObject::new_vao(opengl);
        let (vbo, with_vbo) = WithObject::new_vbo(opengl);

        with_vbo.buffer_data(GlSettings::ArrayBuffer, data, GlSettings::DynamicDraw);
        with_vao.set_vertex_attribs(store_normals);

        (vao, vbo)
    }
    pub fn vao_vbo(opengl:&Gl, vao:u32, vbo:u32) -> WithObject {
        bind_vertex_array(opengl, vao);
        bind_buffer(opengl, GlSettings::ArrayBuffer, vbo);
        WithObject { opengl, object_type:GlSettings::Vertex_ArrayObject_BufferObject,
                        vao_id:vao, vbo_id:vbo, program_id:0 }
    }
    pub fn vao(opengl:&Gl, vao:u32) -> WithObject {
        bind_vertex_array(opengl, vao);
        WithObject { opengl, object_type:GlSettings::VertexArrayObject,
                     vao_id:vao, vbo_id:0, program_id:0 }
    }
    pub fn vbo(opengl:&Gl, vbo:u32) -> WithObject {
        bind_buffer(opengl, GlSettings::ArrayBuffer, vbo);
        WithObject { opengl, object_type:GlSettings::VertexBufferObject,
                     vao_id:0, vbo_id:vbo, program_id:0 }
    }
    pub fn update_vbo(&self, data:&Vec<f32>) {
        self.buffer_sub_data(GlSettings::ArrayBuffer, data);
    }
    pub fn program(opengl:&Gl, program:u32) -> WithObject {
        use_program(opengl, program);
        WithObject { opengl, object_type:GlSettings::Program,
                     vao_id:0, vbo_id:0, program_id:program }
    }
    pub fn buffer_data(&self, target:GlSettings, data:&Matrix2d, draw_type:GlSettings) {
        let data_size = (data.size() * F32_SIZE) as gl::types::GLsizeiptr;
        let data_ptr = data.clone().as_ptr() as *const c_void;
        buffer_data(self.opengl, target, data_size, data_ptr, draw_type);
    }
    pub fn buffer_sub_data(&self, target:GlSettings, data:&Vec<f32>) {
        let data_size = (data.len() * std::mem::size_of::<f32>()).try_into().unwrap();
        let data_ptr = data.as_ptr() as *const c_void;
        match target {
            GlSettings::ArrayBuffer => buffer_sub_data(
                                            self.opengl,
                                            GlSettings::ArrayBuffer,
                                            data_size,
                                            data_ptr),
            _ => {},
        }
    }
    pub fn set_vertex_attribs(&self, store_normals:bool) {
        set_vertex_attrib(self.opengl, 0, store_normals);
        set_vertex_attrib(self.opengl, 1, store_normals);
        set_vertex_attrib(self.opengl, 2, store_normals);
        if store_normals { set_vertex_attrib(self.opengl, 3, store_normals); }
    }
    pub fn draw_vao(&self, mode:GlSettings, data:&Matrix2d) {
        //let num_shapes = 1;
        //let num_shapes = 3;
        let num_shapes = data.nrows.try_into().unwrap();
        draw_arrays(self.opengl, mode, num_shapes);
    }
    pub fn set_uniform(&self, uniform_name:&str, uniform_type:UniformType, value:Matrix2d) {
        set_uniform(self.opengl, self.program_id, uniform_name, uniform_type, value.as_ptr());
    }
}
impl Drop for WithObject<'_> {
    fn drop(&mut self) {
        match self.object_type {
            GlSettings::VertexArrayObject => bind_vertex_array(self.opengl, 0),
            GlSettings::VertexBufferObject => bind_buffer(self.opengl, GlSettings::ArrayBuffer, 0),
            GlSettings::Vertex_ArrayObject_BufferObject => {
                bind_vertex_array(self.opengl, 0);
                bind_buffer(self.opengl, GlSettings::ArrayBuffer, 0);
            },
            GlSettings::Program => {
                /* this is being done in render clear_bindings by binding to 0*/
                //use_program(self.opengl, 0)
            },
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
            ShaderType::VertexShader   => Ok(opengl.CreateShader(gl::VERTEX_SHADER)),
            ShaderType::FragmentShader => Ok(opengl.CreateShader(gl::FRAGMENT_SHADER)),
            _ => Err(0)
        }
    };
    
    shader_id.expect("invalid shader type")
}
fn shader_source(opengl:&Gl, shader_id:u32, source:&str) {
    let binding = CString::new(source).expect("failed to CString");
    let source_ptr = binding.as_c_str().as_ptr();
    unsafe {
        opengl.ShaderSource(shader_id, 1, &source_ptr, std::ptr::null());
    }
    
}
fn compile_shader(opengl:&Gl, shader_id:u32) {
    unsafe { opengl.CompileShader(shader_id) }
        }
fn get_shader_iv(opengl:&Gl, shader:u32, pname:u32, params:*mut i32) {
    unsafe { opengl.GetShaderiv(shader, pname, params) }
        ; }
fn get_program_iv(opengl:&Gl, program:u32, pname:u32, params:*mut i32) {
    unsafe { opengl.GetProgramiv(program, pname, params) }
        ; }
fn get_shader_info_log(opengl:&Gl, shader:u32, bufsize:i32, length:*mut i32, infolog:*mut i8) {
    unsafe { opengl.GetShaderInfoLog(shader, bufsize, length, infolog); }
        ; }
fn get_program_info_log(opengl:&Gl, program:u32, bufsize:i32, length:*mut i32, infolog:*mut i8) {
    unsafe { opengl.GetProgramInfoLog(program, bufsize, length, infolog); }
        ; }
    
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
    let mut success = 0; // this defaults error unless it worked // 1 is good, 0 is bad
    let error_msg = match shader_type {
        ShaderType::AnyShader => {
            get_shader_iv(opengl, id, gl::COMPILE_STATUS, &mut success);
            read_info_log_error(opengl, &get_shader_iv, &get_shader_info_log, id)
        },
        ShaderType::ShaderProgram => {
            get_program_iv(opengl, id, gl::LINK_STATUS, &mut success);
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
    unsafe { opengl.AttachShader(program_id, shader_id) }
        }
fn link_program(opengl:&Gl, program_id:u32) { unsafe { opengl.LinkProgram(program_id) }
        }
fn delete_shader(opengl:&Gl, shader_id:u32) { unsafe { opengl.DeleteShader(shader_id) }
        }
fn detach_shader(opengl:&Gl, program_id:u32, shader_id:u32) {
    unsafe { opengl.DetachShader(program_id, shader_id) }
        }


fn use_program(opengl:&Gl, program_id:u32) {
    unsafe { opengl.UseProgram(program_id) }
    
}


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
    let n_per_colour  : usize = 3;
    let n_per_opacity : usize = 1;
    let n_per_normal  : usize = 3;
    let len_ptr = n_per_vertice + n_per_colour +
                            n_per_opacity + if store_normals
                            {n_per_normal} else {0};
    let stride = (len_ptr * F32_SIZE).try_into().unwrap();
    let (num_items, offset) = match layout_location {
        0 => Ok((n_per_vertice.try_into().unwrap(), 0 as *const c_void)),
        1 => Ok(( n_per_colour.try_into().unwrap() , ((n_per_vertice) * F32_SIZE) as *const c_void)),
        2 => Ok((n_per_opacity.try_into().unwrap(), ((n_per_vertice + n_per_colour) * F32_SIZE) as *const c_void)),
        3 => if store_normals {
                Ok((n_per_normal.try_into().unwrap(), ((n_per_vertice + n_per_colour + n_per_opacity) * F32_SIZE) as *const c_void))
            } else {Err("normals are not included")},
        _ => Err("invalid layout location"),
    }.unwrap();
    //let num_items : i32 = loc_info[0].try_into().unwrap();
    //let offset = loc_info[1] as *const gl::types::GLvoid;
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
    offset:*const c_void) {
    unsafe {
        opengl.VertexAttribPointer(
            layout_location,
            num_items_in_location,
            gl::FLOAT,
            gl::FALSE,
            stride_between_points,
            offset)
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
    unsafe {opengl.PointSize(10.0)}
    match mode {
        GlSettings::GlPoints => unsafe {opengl.DrawArrays(gl::POINTS, 0, num_shapes)},
        GlSettings::GlLines => unsafe {opengl.DrawArrays(gl::LINES, 0, num_shapes)},
        GlSettings::GlTriangles => unsafe {opengl.DrawArrays(gl::TRIANGLES, 0, num_shapes)},
        _ => Err("invalid draw mode").unwrap(),
    }
}
fn viewport(opengl:&Gl, width:i32, height:i32) {
    unsafe { opengl.Viewport(0, 0, width, height);}
}