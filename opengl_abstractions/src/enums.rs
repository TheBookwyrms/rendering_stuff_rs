use std::ffi::NulError;

#[derive(Clone, Debug)] // Copy
pub enum GlError {
    CStringError(NulError),
    InvalidShaderType(ShaderType),
    InvalidBufferType(BufferType),
    InvalidDrawType(DrawType),
    InvalidLayoutLocation(u32),
    InvalidDrawMode(DrawMode),
    CompilationSuccessFailed(String),
    InvalidDataDims(usize),
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
    AnyShader,
    ShaderProgram,
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
pub enum GlSettings {
    DepthTest,
    Multisample,
    Blend,
    BlendFunc_SRCAlpha_OneMinusSRCAlpha,
    ColourBufferBit,
    DepthBufferBit,
    //ArrayBuffer,
    VertexArrayObject,
    VertexBufferObject,
    Program,
    Vertex_ArrayObject_BufferObject,
    //StaticDraw,
    //StreamDraw,
    //DynamicDraw,
    //GlTriangles,
    //GlPoints,
    //GlLines,
}