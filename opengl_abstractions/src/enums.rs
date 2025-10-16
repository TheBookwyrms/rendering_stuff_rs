use std::{ffi::NulError, str::Utf8Error};


#[derive(Debug)] // Copy
pub enum GlError {
    CStringError(NulError),
    InvalidShaderType(ShaderType),
    InvalidBufferType(BufferType),
    InvalidDrawType(DrawType),
    InvalidLayoutLocation(u32),
    InvalidDrawMode(DrawMode),
    CompilationSuccessFailed(String),
    InvalidDataDims(usize),
    InvalidColour(f32, f32, f32, f32),
    FileError(std::io::Error),
    EmbedError,
    TextError(Utf8Error),
    InvalidProgramVariantUsage(ProgramVariant),
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
pub enum BufferType {
    ArrayBuffer,
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
pub enum VertexObject {
    Array,
    Buffer,
    ArrayAndBuffer,
}