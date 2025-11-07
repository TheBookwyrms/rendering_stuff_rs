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
    MatrixError(MatrixError),
    TryFromIntError(TryFromIntError),
    DataLengthError(usize),
    InvalidObjectType,
    NotImplementedYet,
    InvalidProgramID,
    InvalidProgramType,
    InvalidDataFormat,
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
pub enum ProgramSelect {
    SelectBlinnPhongOrthographic,
    SelectSimpleOrthographic,
    SelectSimpleTexture,
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

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DrawCall {
    Vertices,
    Arrays,
    Elements,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DataFormat {
    //Position3,
    //Position3Colour3,
    Position3Texture2,
    Position3Colour3Alpha1,
    Position3Colour3Alpha1Normal3,
    //Position3Colour3Texture2,
    //Position3Colour3Alpha1Texture2,
}



//#[derive(Copy, Clone, PartialEq, Debug)]
//pub enum TextureWrapping {
//    Repeat,
//    MirroredRepeat,
//    ClampToEdge,
//    ClampToBorder,
//}
//
//#[derive(Copy, Clone, PartialEq, Debug)]
//pub enum TextureTarget {
//    Texture1D,
//    Texture1DArray,
//    Texture2D,
//    Texture2DArray,
//    Texture2DMultisample,
//    Texture2DMultisampleArray,
//    Texture3D,
//    TextureCubeMap,
//    TextureCubeMapArray,
//    TextureRectangle,
//}
//
//#[derive(Copy, Clone, PartialEq, Debug)]
//pub enum TexturePName {
//    DEPTH_STENCIL_TEXTURE_MODE,
//    Texture_BASE_LEVEL,
//    TextureCompare_FUNC,
//    TextureCompare_MODE,
//    TextureLod_BIAS,
//    TextureMin_FILTER,
//    TextureMag_FILTER,
//    TextureMin_LOD,
//    TextureMax_LOD,
//    TextureMax_LEVEL,
//    TextureSwizzle_R,
//    TextureSwizzle_G,
//    TextureSwizzle_B,
//    TextureSwizzle_A,
//    TextureWrap_S,
//    TextureWrap_T,
//    TextureWrap_R,
//
//    // for vector TexParameter's only:
//    Texture_BORDER_COLOR,
//    Texture_SWIZZLE_RGBA,
//}