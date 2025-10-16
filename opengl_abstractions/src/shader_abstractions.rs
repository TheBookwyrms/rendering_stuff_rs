use std::fs::File;
use std::io::Read;

use crate::gl::Gl;
use crate::enums::{GlError, ProgramSelect, ProgramVariant, UniformType};
use crate::{high_level_abstractions, intermediate_opengl};


use matrices::matrix::Matrix;

use rust_embed::Embed;


#[derive(Embed)]
#[folder = "src/shaders_glsl/"]
struct Asset;

#[derive(Clone, Copy)]
pub struct ProgramHolder {
    pub simple_orthographic:ProgramVariant,
    pub blinn_phone_orthographic:ProgramVariant
}
impl ProgramHolder {
    pub fn new(
        simple_orthographic_shader:ProgramVariant,
        blinn_phone_orthographic_shader:ProgramVariant
    ) -> Result<ProgramHolder, GlError> {

        let simple_orthographic = match simple_orthographic_shader {
            ProgramVariant::SimpleOrthographic(id) => Ok(ProgramVariant::SimpleOrthographic(id)),
            ProgramVariant::BlinnPhongOrthographic(id) => Err(GlError::InvalidProgramVariantUsage(ProgramVariant::BlinnPhongOrthographic(id))),
        }?;

        let blinn_phone_orthographic = match blinn_phone_orthographic_shader {
            ProgramVariant::SimpleOrthographic(id) => Err(GlError::InvalidProgramVariantUsage(ProgramVariant::SimpleOrthographic(id))),
            ProgramVariant::BlinnPhongOrthographic(id) => Ok(ProgramVariant::BlinnPhongOrthographic(id)),
        }?;

        Ok(ProgramHolder { simple_orthographic, blinn_phone_orthographic })
    }

    pub fn use_program(&self, opengl:&Gl, program:ProgramSelect) -> Result<(), GlError> {
        match program {
            ProgramSelect::SelectSimpleOrthographic => intermediate_opengl::use_program(opengl, self.simple_orthographic),
            ProgramSelect::SelectBlinnPhongOrthographic => intermediate_opengl::use_program(opengl, self.blinn_phone_orthographic),
        }
    }

}

pub struct WithProgram<'l> {
    pub opengl:&'l Gl,
    pub program_variant:ProgramVariant,
}
impl WithProgram<'_> {
    pub fn program(opengl:&Gl, program:ProgramSelect, programs:ProgramHolder) -> WithProgram<'_> {
        match program {
            ProgramSelect::SelectSimpleOrthographic => WithProgram { opengl, program_variant: programs.simple_orthographic },
            ProgramSelect::SelectBlinnPhongOrthographic => WithProgram { opengl, program_variant: programs.blinn_phone_orthographic },
        }
    }

    pub fn use_program(&self) -> Result<(), GlError> {
        intermediate_opengl::use_program(self.opengl, self.program_variant)
    }

    pub fn set_uniform(&self, uniform_name:&str, uniform_type:UniformType, value:Matrix<f32>) -> Result<(), GlError> {
        high_level_abstractions::set_uniform(self.opengl, self.program_variant, uniform_name, uniform_type, value.as_ptr())
    }
}




pub fn create_program(opengl:&Gl, program_type:ProgramSelect, embed:bool) -> Result<ProgramVariant, GlError> {
    let text_getter = match embed {
        true  => get_shader_text_embed,
        false => get_shader_text_dynamic,
    };
    
    match program_type {
        ProgramSelect::SelectBlinnPhongOrthographic => {
            let vertex_text = text_getter("blinn_phong_orthographic_vertex")?;
            let fragment_text = text_getter("blinn_phong_orthographic_fragment")?;
            let shader_id = high_level_abstractions::create_shader_program(
                opengl, vertex_text.as_str(), fragment_text.as_str()
            )?;
            Ok(ProgramVariant::BlinnPhongOrthographic(shader_id))
        },
        ProgramSelect::SelectSimpleOrthographic => {
            let vertex_text = text_getter("simple_orthographic_vertex")?;
            let fragment_text = text_getter("simple_orthographic_fragment")?;
            let shader_id = high_level_abstractions::create_shader_program(
                opengl, vertex_text.as_str(), fragment_text.as_str()
            )?;
            Ok(ProgramVariant::SimpleOrthographic(shader_id))
        },
    }
}



fn error(msg:String) {
    let a = true;
    let _b = match a {
        true =>Err(msg),
        false =>Ok(msg),
    }.unwrap();
}

pub fn get_shader_text_dynamic(filename:&str) -> Result<String, GlError> {
    let mut folder = "shaders_glsl/".to_owned();
    folder.push_str(filename);
    folder.push_str(".glsl");
    let file_path = folder.as_str();

    println!("{:?}", file_path);
    println!("{:?}", std::env::current_dir());

    match File::open(file_path) {
        Ok(mut file_handle) => {
            let mut shader_text = String::new();
            match file_handle.read_to_string(&mut shader_text) {
                Ok(_) => Ok(shader_text),
                Err(error) => Err(GlError::FileError(error)),
            }
        },
        Err(error) => Err(GlError::FileError(error)),
    }
}

pub fn get_shader_text_embed(filename:&str) -> Result<String, GlError> {
    let mut file = filename.to_owned();
    file.push_str(".glsl");
    let file = file.as_str();

    match Asset::get(file) {
        Some(glsl) => {
            match std::str::from_utf8(glsl.data.as_ref()) {
                Ok(shader_text_str) => Ok(shader_text_str.to_owned()),
                Err(error) => Err(GlError::TextError(error)),
            }
        },
        None => Err(GlError::EmbedError),
    }
}