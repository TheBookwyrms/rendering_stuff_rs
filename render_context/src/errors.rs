use std::num::TryFromIntError;

use glfw::{InitError, WindowEvent};
use opengl::enums::GlError;

#[derive(Debug)]
pub enum RenderError {
    NewGLFWEventDetected(WindowEvent),
    GLFWinitError(InitError),
    GLFWNoWindowCreated,
    GLFWResizeBoundsError((i32, i32)),
    GLError(GlError),
    TryFromIntError(TryFromIntError),
    DataLengthError(usize),
}

impl From<GlError> for RenderError {
    fn from(value: GlError) -> Self {
        Self::GLError(value)
    }
}