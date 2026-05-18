use atmospheric::enums::DataFormat;
use atmospheric::materials::Material;
use numeracy::{matrices::Matrix, vectors::Vector};
 
pub struct Thing {
    pub vertices:Matrix<f32>,
    pub data_format:DataFormat,
    pub model_transform:Matrix<f32>,
    pub colour:Vector<f32>,
    pub material:Material,
    pub vao:Option<u32>,
    pub vbo:Option<u32>,
    pub ebo:Option<u32>,
}