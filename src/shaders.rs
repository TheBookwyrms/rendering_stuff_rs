pub mod shaders {

    use crate::gl_abstractions::OpenGl;
    use crate::gl_abstractions::OpenGl::{Gl, ShaderType};
    use crate::gl_abstractions::OpenGl::{WithObject, GlSettings, UniformType};
    use crate::ndarray_abstractions::MyArray::{Arr1D, Arr2D, Arr3D, Arr4D};
    use crate::ndarray_abstractions::MyArray::N as nd_trait;

    use std::vec;
    use std::{error::Error, fmt};
    use std::os::raw;

    use rust_embed::Embed;

    

    #[derive(Embed)]
    #[folder = "src/shaders_glsl/"]
    struct Asset;


    pub fn create_vao_vbo<N:nd_trait>(opengl:&Gl, store_normals:bool, data:N) -> (u32, u32) {
        let (vao, with_vao) = WithObject::new_vao(opengl);
        let (vbo, with_vbo) = WithObject::new_vbo(opengl);

        with_vbo.buffer_data(GlSettings::ArrayBuffer, &data, GlSettings::DynamicDraw);
        with_vao.set_vertex_attribs(true);

        (vao, vbo)
    }
    pub fn update_vbo<N:nd_trait>(opengl:&Gl, vbo:u32, data:N) {
        let with_vbo = WithObject::vbo(opengl, vbo);
        with_vbo.buffer_sub_data(GlSettings::ArrayBuffer, &data);
    }
    pub fn draw_vao<N:nd_trait>(opengl:&Gl, vao:u32, data:N) {
        let with_vao = WithObject::vao(opengl, vao);
        with_vao.draw_vao(GlSettings::DynamicDraw, &data);
    }




    

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum ProgramType {
        ForObject,
        ForLighting,
    }

    pub struct ProgramHolder {
        pub programs:Vec<ShaderProgram>
    }
    impl ProgramHolder {
        pub fn new(opengl:&Gl, program_types:Vec<ProgramType>) -> ProgramHolder {

            let mut programs = vec![];
            for ptype in program_types {
                programs.push(ShaderProgram::new(opengl, ptype));
            }

            ProgramHolder { programs }
        }
        pub fn add(&mut self, program:ShaderProgram) {
            self.programs.push(program);
        }
        pub fn set_program_uniform<N:nd_trait>(&self,
                                   opengl:&Gl,
                                   program_type:ProgramType,
                                   uniform_name:&str,
                                   uniform_type:UniformType,
                                   value:&N) {
            let mut valid_programs = vec![];
            for program in &self.programs {
                if program_type==program.program_type { valid_programs.push(program); }
            }
            match valid_programs.len() {
                0 => { Err::<N, &str>("no valid programs of proper type"); },
                1 => {
                    let program_id = valid_programs[0].program_id;
                    OpenGl::set_uniform(opengl, program_id, uniform_name, uniform_type, value.as_ptr());
                },
                _ => { Err::<N, &str>("too many program of proper type").unwrap(); },
            }
        }
    }


    #[derive(Debug)]
    pub struct ShaderProgram {
        pub program_id : u32,
        pub program_type : ProgramType,
    }

    impl ShaderProgram {
        pub fn new(opengl:&Gl, program_type:ProgramType)  -> ShaderProgram {

            let (vertex, fragment) = match program_type {
                ProgramType::ForObject => (
                    Shader::new(opengl, get_shader_text("object_vertex_shader"), ShaderType::VertexShader),
                    Shader::new(opengl, get_shader_text("object_fragment_shader"), ShaderType::FragmentShader)
                ),
                ProgramType::ForLighting => (
                    Shader::new(opengl, get_shader_text("lighting_vertex_shader"), ShaderType::VertexShader),
                    Shader::new(opengl, get_shader_text("lighting_fragment_shader"), ShaderType::FragmentShader)
                ),
            };

            let program_id = OpenGl::create_shader_program(opengl, vertex.shader_id, fragment.shader_id);

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

            let shader_id = OpenGl::create_shader_variant(opengl, str_text, shader_type);

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
}