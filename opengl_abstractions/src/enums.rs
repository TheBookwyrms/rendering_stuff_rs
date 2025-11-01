use std::str::Utf8Error;
use std::num::TryFromIntError;
use std::ffi::NulError;

use numeracy::enums::MatrixError;


#[derive(Debug)] // Copy
pub enum GlError {
    CStringError(NulError),
    InvalidShaderType(ShaderType),
    InvalidBufferType(BufferObject),
    InvalidDrawType(DrawType),
    InvalidLayoutLocation(u32),
    InvalidDrawMode(DrawMode),
    CompilationSuccessFailed(String),
    InvalidDataDims(usize),
    InvalidColour(f32, f32, f32, f32),
    FileError(std::io::Error),
    TextError(Utf8Error),
    InvalidProgramVariantUsage(ProgramVariant),
    MatrixError(MatrixError),
    TryFromIntError(TryFromIntError),
    DataLengthError(usize),
    InvalidObjectType,
    NotImplementedYet,
}

#[derive(Clone, Copy, Debug)]
pub enum UniformType {
    Float,
    Vec3,
    Mat4,
}

#[derive(Clone, Copy, Debug)]
pub enum ShaderType {
    VertexShader,
    FragmentShader,
    ShaderProgram,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ProgramVariant {
    BlinnPhongOrthographic(u32),
    SimpleOrthographic(u32),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ProgramSelect {
    SelectBlinnPhongOrthographic,
    SelectSimpleOrthographic,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum BufferObject {
    VertexBufferObject,
    ElementBufferObject,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ArrayObject {
    VertexArrayObject,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DrawType {
    StaticDraw,
    StreamDraw,
    DynamicDraw,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DrawMode {
    GlTriangles,
    GlPoints,
    GlLines,
    GlTriangleStrip,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum BlendFunc {
    SRCAlphaOneMinusSRCAlpha,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum BufferBit {
    ColourBufferBit,
    DepthBufferBit,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum GlEnable {
    DepthTest,
    Multisample,
    Blend,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Object {
    VAO,
    VBO,
    EBO,
}