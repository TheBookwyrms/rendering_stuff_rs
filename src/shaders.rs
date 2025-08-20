pub mod shaders {

    use crate::gl_abstractions::OpenGl::{self, create_shader_variant};
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




    

    #[derive(Debug)]
    pub enum ProgramType {
        Object,
        Lighting,
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
                    vertex   = Shader::new(opengl, get_shader_text("object_vertex_shader"), ShaderType::VertexShader);
                    fragment = Shader::new(opengl, get_shader_text("object_fragment_shader"), ShaderType::FragmentShader);
                },
                ProgramType::Lighting=> {
                    vertex   = Shader::new(opengl, get_shader_text("lighting_vertex_shader"), ShaderType::VertexShader);
                    fragment = Shader::new(opengl, get_shader_text("lighting_fragment_shader"), ShaderType::FragmentShader);
                },
            }

            let program_id = OpenGl::create_shader_program(opengl, vertex.shader_id, fragment.shader_id);

            ShaderProgram { program_id:program_id, program_type:program_type }
        }
        pub fn set_uniform_float(&self, opengl:&Gl, uniform_name:&str, float:f32) {
            let float_ptr = &float as *const f32;
            OpenGl::set_uniform(opengl, self.program_id, uniform_name, UniformType::Float, float_ptr);
        }
        pub fn set_uniform_vec3(&self, opengl:&Gl, uniform_name:&str, vec3:(f32, f32, f32)) {
            let vec3_ptr = Arr1D::from([vec3.0, vec3.1, vec3.2]).as_ptr();
            OpenGl::set_uniform(opengl, self.program_id, uniform_name, UniformType::Vec3, vec3_ptr);
        }
        pub fn set_uniform_mat4(&self, opengl:&Gl, uniform_name:&str, mat4:Arr2D) {
            let mat4_ptr = mat4.as_ptr();
            OpenGl::set_uniform(opengl, self.program_id, uniform_name, UniformType::Mat4, mat4_ptr);
        }
    }

    //impl Drop for Shader<'_> {
    //    fn drop(&mut self) {
    //        OpenGl::delete_shader(self.opengl, self.shader_id);
    //    }
    //}

    impl Shader<'_> {
        pub fn new<'a>(opengl:&'a Gl, shader_text:String, shader_type : ShaderType
                            ) -> Shader<'a> {
            let str_text = shader_text.as_str();

            let shader_id = create_shader_variant(opengl, str_text, shader_type);

            Shader {opengl:opengl, shader_type:shader_type, shader_id:shader_id}
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