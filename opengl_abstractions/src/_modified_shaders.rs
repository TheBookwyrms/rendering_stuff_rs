use opengl;
use opengl::{Gl, ShaderType};
use opengl::WithObject;//{WithObject, GlSettings, UniformType};

use std::vec;

use rust_embed::Embed;



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

    }
}