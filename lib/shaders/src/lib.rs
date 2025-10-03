use opengl;
use opengl::{Gl, ShaderType};
use opengl::{WithObject, GlSettings, UniformType};
use matrices::Matrix2d;
//use crate::ndarray_abstractions::MyArray::{Arr1D, Arr2D, Arr3D, Arr4D};
//use crate::ndarray_abstractions::MyArray::N as nd_trait;

//use crate::vertices::Vertices::Matrix2d;

use std::vec;
use std::{error::Error, fmt};
use std::os::raw;

use rust_embed::Embed;



#[derive(Embed)]
#[folder = "src/shaders_glsl/"]
struct Asset;



#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ProgramType {
    BlinnPhongOrthographic,
    SimpleOrthographic,
}

pub struct ProgramHolder {
    pub programs:Vec<ShaderProgram>
}
impl ProgramHolder {
    pub fn new<const K:usize>(opengl:&Gl, program_types:[ProgramType;K]) -> ProgramHolder {

        let mut programs = vec![];
        for ptype in program_types {
            programs.push(ShaderProgram::new(opengl, ptype));
        }

        ProgramHolder { programs }
    }
    pub fn add(&mut self, program:ShaderProgram) {
        self.programs.push(program);
    }
    pub fn get_program_id(&self, program_type:ProgramType) -> Result<u32, &str> {
        let mut valid_programs = vec![];
        for program in &self.programs {
            if program_type==program.program_type { valid_programs.push(program); }
        }
        match valid_programs.len() {
            0 => Err("no valid programs of proper type"),
            1 => {
                let program_id = valid_programs[0].program_id;
                Ok(program_id)
            },
            _ => Err("too many program of proper type"),
        }
    }
    pub fn use_program<'a>(&self, opengl:&'a Gl, program_type:ProgramType) -> WithObject<'a> {
        let mut valid_programs = vec![];
        for program in &self.programs {
            if program_type==program.program_type { valid_programs.push(program); }
        }
        let use_result: WithObject = match valid_programs.len() {
            0 => { Err("no valid programs of proper type") },
            1 => {
                let program_id = valid_programs[0].program_id;
                let with_program = WithObject::program(opengl, program_id);
                Ok(with_program)
            },
            _ => { Err("too many program of proper type") },
        }.unwrap();

        use_result
    }
    //pub fn set_program_uniform(&self,
    //                           opengl:&Gl,
    //                           program_type:ProgramType,
    //                           uniform_name:&str,
    //                           uniform_type:UniformType,
    //                           value:Matrix2d) {
    //    let mut valid_programs = vec![];
    //    for program in &self.programs {
    //        if program_type==program.program_type { valid_programs.push(program); }
    //    }
    //    let uniform_set = match valid_programs.len() {
    //        0 => { Err("no valid programs of proper type") },
    //        1 => {
    //            let program_id = valid_programs[0].program_id;
    //            opengl::set_uniform(opengl, program_id, uniform_name, uniform_type, value.as_ptr());
    //            Ok("all good")
    //        },
    //        _ => { Err("too many program of proper type") },
    //    };
    //    uniform_set.unwrap();
    //}
}


#[derive(Debug)]
pub struct ShaderProgram {
    pub program_id : u32,
    pub program_type : ProgramType,
}

impl ShaderProgram {
    pub fn new(opengl:&Gl, program_type:ProgramType)  -> ShaderProgram {

        let (vertex, fragment) = match program_type {
            ProgramType::BlinnPhongOrthographic => (
                Shader::new(opengl, get_shader_text("blinn_phong_orthographic_vertex"), ShaderType::VertexShader),
                //Shader::new(opengl, get_shader_text("simple_orthographic_vertex"), ShaderType::VertexShader),
                Shader::new(opengl, get_shader_text("blinn_phong_orthographic_fragment"), ShaderType::FragmentShader)
                //Shader::new(opengl, get_shader_text("simple_orthographic_fragment"), ShaderType::FragmentShader)
            ),
            ProgramType::SimpleOrthographic => (
                Shader::new(opengl, get_shader_text("simple_orthographic_vertex"), ShaderType::VertexShader),
                Shader::new(opengl, get_shader_text("simple_orthographic_fragment"), ShaderType::FragmentShader)
            ),
        };

        let program_id = opengl::create_shader_program(opengl, vertex.shader_id, fragment.shader_id);

        ShaderProgram { program_id:program_id, program_type:program_type }
    }
}


#[derive(Debug)]
pub struct Shader {
    pub shader_type : ShaderType,
    pub shader_id : u32,
}
impl Shader {
    pub fn new(opengl:&Gl, shader_text:String, shader_type : ShaderType) -> Shader {
        let str_text = shader_text.as_str();

        let shader_id = opengl::create_shader_variant(opengl, str_text, shader_type);

        Shader {shader_type:shader_type, shader_id:shader_id}
    }
}

fn get_shader_text(filename:&str) -> String {
    let mut file = filename.to_owned();
    file.push_str(".glsl");
    let file = file.as_str();

    let glsl = Asset::get(file).unwrap();
    let shader_text = std::str::from_utf8(glsl.data.as_ref()).unwrap().to_owned();
    shader_text
}