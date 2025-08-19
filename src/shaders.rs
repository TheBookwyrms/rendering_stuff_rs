pub mod shaders {

    use crate::gl_abstractions::OpenGl;
    use crate::gl_abstractions::OpenGl::{Gl, ShaderType, ShaderVariant};
    use crate::gl_abstractions::OpenGl::{WithObject, GlSettings};
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
        let vao = OpenGl::gen_vertex_arrays(opengl);
        let vbo = OpenGl::gen_buffers(opengl);

        let with_vao = WithObject::vao(opengl, vao); // unbinds object after use finished
        let with_vbo = WithObject::vbo(opengl, vbo); // unbinds object after use finished

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
                    vertex   = Shader::new(opengl, get_shader_text("object_vertex_shader"), ShaderType::Vertex);
                    fragment = Shader::new(opengl, get_shader_text("object_fragment_shader"), ShaderType::Fragment);
                },
                ProgramType::Lighting=> {
                    vertex   = Shader::new(opengl, get_shader_text("lighting_vertex_shader"), ShaderType::Vertex);
                    fragment = Shader::new(opengl, get_shader_text("lighting_fragment_shader"), ShaderType::Fragment);
                },
            }

            let program_id = create_program(opengl, vertex.shader_id, fragment.shader_id);

            OpenGl::get_compilation_error(opengl, program_id, ShaderVariant::Program);
            ShaderProgram { program_id:program_id, program_type:program_type }
        }
        pub fn set_uniform_float(&self, opengl:&Gl, uniform_name:&str, float:f32) {
            OpenGl::set_uniform_float(opengl, self.program_id, uniform_name, float);
        }
        pub fn set_uniform_vec3(&self, opengl:&Gl, uniform_name:&str, vec3:(f32, f32, f32)) {
            let vec3_ptr = Arr1D::from([vec3.0, vec3.1, vec3.2]).as_ptr();
            OpenGl::set_uniform_vec3(opengl, self.program_id, uniform_name, vec3_ptr);
        }
        pub fn set_uniform_mat4(&self, opengl:&Gl, uniform_name:&str, mat4:Arr2D) {
            let mat4_ptr = mat4.as_ptr();
            OpenGl::set_uniform_mat4(opengl, self.program_id, uniform_name, mat4_ptr);
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

            let shader_id = OpenGl::create_shader(opengl, shader_type);
            OpenGl::shader_source(opengl, shader_id, str_text);
            OpenGl::compile_shader(opengl, shader_id);

            OpenGl::get_compilation_error(opengl, shader_id, ShaderVariant::Shader);
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

    fn create_program<'b>(opengl:&'b Gl, vertex_id:u32, fragment_id:u32) -> u32 {
        let program_id = OpenGl::create_program(opengl);
        OpenGl::attach_shader(opengl, program_id,   vertex_id);
        OpenGl::attach_shader(opengl, program_id, fragment_id);
        OpenGl::link_program(opengl, program_id);
        //OpenGl::delete_shader(opengl, vertex_id);
        //OpenGl::delete_shader(opengl, fragment_id);
        program_id
    }
}