use opengl::enums::InternalFormat;

pub enum ImageFormat {
    JPEG,
    PNG,
}

//impl From<InternalFormat> for ImageFormat {
//    fn from(value: InternalFormat) -> Self {
//        match value {
//            InternalFormat::RGB => ImageFormat::JPEG,
//            InternalFormat::RGBA => ImageFormat::PNG,
//        }
//    }
//}
impl Into<InternalFormat> for ImageFormat {
    fn into(self) -> InternalFormat {
        match self {
            ImageFormat::JPEG => InternalFormat::RGB,
            ImageFormat::PNG => InternalFormat::RGBA,
        }
    }
}