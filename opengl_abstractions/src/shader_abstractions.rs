use crate::gl::Gl;
use crate::enums::{GlError, ProgramSelect, ProgramVariant, UniformType};
use crate::{high_level_abstractions, intermediate_opengl};

use crate::shaders;

use numeracy::matrices::matrix::Matrix;


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




pub fn create_program(opengl:&Gl, program_type:ProgramSelect) -> Result<ProgramVariant, GlError> {
    match program_type {
        ProgramSelect::SelectBlinnPhongOrthographic => {
            let vertex_text = shaders::BLINN_PHONG_ORTHOGRAPHIC_VERTEX;
            let fragment_text = shaders::BLINN_PHONG_ORTHOGRAPHIC_FRAGMENT;
            let shader_id = high_level_abstractions::create_shader_program(
                opengl, vertex_text, fragment_text
            )?;
            Ok(ProgramVariant::BlinnPhongOrthographic(shader_id))
        },
        ProgramSelect::SelectSimpleOrthographic => {
            let vertex_text = shaders::SIMPLE_ORTHOGRAPHIC_VERTEX;
            let fragment_text = shaders::SIMPLE_ORTHOGRAPHIC_FRAGMENT;
            let shader_id = high_level_abstractions::create_shader_program(
                opengl, vertex_text, fragment_text
            )?;
            Ok(ProgramVariant::SimpleOrthographic(shader_id))
        },
    }
}