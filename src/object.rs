use atmospheric::context;
use atmospheric::enums::{AttributeLoc::{self, At}, OpenglTexture};
use atmospheric::enums::UpdateVertexAttrib::{PerInstance, PerVertex};
use atmospheric::image_processing::Image;
use atmospheric::materials::Material;
use numeracy::matrices::{Matrix, S1, S2, ShapeTrait};

use crate::c_void;

use atmospheric::enums::{
    ArrayObject, BufferObject, ContextError, DataFormat, DrawCall, DrawMode, DrawType, ImageFormat, Object as OpenglObject, UpdateVertexAttrib
};
use atmospheric::opengl::{raw_opengl, intermediate_opengl};
//use atmospheric::opengl::abstractions::WithObject;
use atmospheric::opengl::abstractions::{PreparedTexture, Programs, TextureSetup, Textures, WithEbo, WithVao, WithVaoEbo, WithVaoVbo, WithVbo};
use atmospheric::opengl::gl::Gl;

 pub enum ObjectColour<const NUM_VERTICES:usize> {
    None,
    Constant( Matrix<f32, 2, S2<4, 1>>),
    PerVertex(Matrix<f32, 2, S2<4, NUM_VERTICES>>),
 }

 pub enum ObjectTexture<const NUM_VERTICES:usize> {
    None,
    PerVertex(Image, Image, Matrix<f32, 2, S2<2, NUM_VERTICES>>),
 }
 pub enum ObjectMaterials<const NUM_VERTICES:usize> {
    None,
    Constant( Material),
    PerVertex(Matrix<Material, 1, S1<NUM_VERTICES>>),
 }
 
 
 pub struct ObjectForVaoDraws<const NUM_VERTICES:usize> {
    position_matrix:Matrix<f32, 2, S2<3, NUM_VERTICES>>,
    normals_matrix:Matrix<f32, 2, S2<3, NUM_VERTICES>>,
    
    colour_matrix:ObjectColour<NUM_VERTICES>,
    //texture_coords_matrix:ObjectTextureCoords<NUM_VERTICES>,
    materials_matrix:ObjectMaterials<NUM_VERTICES>,

    /// vec of mat4, length N, for position and normals
    transformation_matrices:Vec<Matrix<f32, 2, S2<4, 4>>>,
    
    object_vao:u32,
    position_matrix_vbo:u32,
    colour_matrix_vbo:u32,
    texture_matrix_vbo:u32,
    normals_matrix_vbo:u32,
    materials_matrix_vbo:u32,
    transformation_matrices_vbo:u32,

    diffuse_texture:PreparedTexture,
    specular_texture:PreparedTexture,
 }
 
 impl<const NUM_VERTICES:usize> ObjectForVaoDraws<NUM_VERTICES> {

    pub fn assert_vaos_vbos_set(&self) { 
         assert_ne!(
             self.object_vao * self.position_matrix_vbo * self.colour_matrix_vbo *
             self.texture_matrix_vbo * self.normals_matrix_vbo * self.materials_matrix_vbo, 0
         );
     }

    fn get_matrix_of_transformation_matrices(transformations:&Vec<Matrix<f32, 2, S2<4, 4>>>) -> Matrix<f32, 1, S1<1>> {
        const FAKE_LEN:usize = 1;
        let vec_of_items = transformations.iter()
                                             .map(|m| m.get_view_of_array())
                                             .collect::<Vec<&[f32]>>()
                                             .concat();
        let mat: Matrix<f32, 1, S1<FAKE_LEN>> = Matrix::from_vec(vec_of_items);
        mat        
    }

    pub fn new(
        opengl:&Gl,
        positions:Matrix<f32, 2, S2<3, NUM_VERTICES>>,
        normals:Matrix<f32, 2, S2<3, NUM_VERTICES>>,
        colour:ObjectColour<NUM_VERTICES>,
        // //texture_coords:ObjectTextureCoords<NUM_VERTICES>,
        materials:ObjectMaterials<NUM_VERTICES>,
        texture:ObjectTexture<NUM_VERTICES>,
        // diffuse_texture:ObjectTexture<NUM_VERTICES>,
        // specular_texture:ObjectTexture<NUM_VERTICES>,
        // //diffuse_image:Option<Image>,
        // //specular_image:Option<Image>,
        transformations:Vec<Matrix<f32, 2, S2<4, 4>>>,
     ) -> Self {
 

        let dtype_size = positions.dtype_memsize() as i32;

        let with_object_vao = WithVao::new(opengl);

        let with_positions_vbo = WithVbo::new(opengl);
        with_positions_vbo.buffer_data(&positions, DrawType::DynamicDraw);
        DataFormat::Position3(At(0), PerVertex).set_vertex_attribs(opengl, dtype_size);

        let with_colours_vbo = WithVbo::new(opengl);
        match colour {
            ObjectColour::None => {
                with_colours_vbo.buffer_data(&Matrix::from_1darray([1.0; 4]), DrawType::DynamicDraw);
                DataFormat::Colour4(At(1), PerInstance(transformations.len() as u32)).set_vertex_attribs(opengl, dtype_size);
            },
            ObjectColour::Constant(ref mat) => {
                with_colours_vbo.buffer_data(&mat, DrawType::DynamicDraw);
                DataFormat::Colour4(At(1), PerInstance(transformations.len() as u32)).set_vertex_attribs(opengl, dtype_size);

            },
            ObjectColour::PerVertex(ref mat) => {
                with_colours_vbo.buffer_data(&mat, DrawType::DynamicDraw);
                DataFormat::Colour4(At(1), PerVertex).set_vertex_attribs(opengl, dtype_size);
            },
        }

        let with_normals_vbo = WithVbo::new(opengl);
        with_normals_vbo.buffer_data(&normals, DrawType::DynamicDraw);
        DataFormat::Normal3(At(2), PerVertex).set_vertex_attribs(opengl, dtype_size);

        //let diffuse_texture = match diffuse_image {
        //    None => {
        //        let white = Image::decode_from_path("../rendering_stuff_rs/src/black.ppm", ImageFormat::PPMP3, false);
        //        TextureSetup::get_prepared_default(opengl, white)
        //    },
        //    Some(image) => {
        //        let prepared_image = TextureSetup::get_prepared_default(opengl, image);
        //        prepared_image
        //    },
        //};
        //let specular_texture = match specular_image {
        //    None => {
        //        ;
        //        TextureSetup::get_prepared_default(opengl, white)
        //    },
        //    Some(image) => {
        //        let prepared_image = TextureSetup::get_prepared_default(opengl, image);
        //        prepared_image
        //    },
        //};
        let with_texture_coords_vbo = WithVbo::new(opengl);
        let (diffuse_texture, specular_texture) = match texture {
            ObjectTexture::None => {
                let default_diffuse  = TextureSetup::get_prepared_default(opengl, Image::decode_from_path("../rendering_stuff_rs/src/white.ppm", ImageFormat::PPMP3, false));
                let default_specular = TextureSetup::get_prepared_default(opengl, Image::decode_from_path("../rendering_stuff_rs/src/white.ppm", ImageFormat::PPMP3, false));
                
                with_texture_coords_vbo.buffer_data(&Matrix::from_1darray([0.5; 2]), DrawType::DynamicDraw);
                DataFormat::Texture2(At(3), PerInstance(transformations.len() as u32)).set_vertex_attribs(opengl, dtype_size);
                
                (default_diffuse, default_specular)
            },
            ObjectTexture::PerVertex(diffuse_image, specular_image, ref mat) => {
                let diffuse  = TextureSetup::get_prepared_default(opengl, diffuse_image);
                let specular = TextureSetup::get_prepared_default(opengl, specular_image);

                with_texture_coords_vbo.buffer_data(&mat, DrawType::DynamicDraw);
                DataFormat::Texture2(At(3), PerVertex).set_vertex_attribs(opengl, dtype_size);

                (diffuse, specular)
            },
        };

        let with_materials_vbo = WithVbo::new(opengl);
        match materials {
            ObjectMaterials::None => {
                let data = Matrix::from_1darray([1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 32.0]);
                //let data = Matrix::from_1darray([1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 32.0]);
                //let data = Matrix::from_1darray([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 32.0]);
                with_materials_vbo.buffer_data(&data, DrawType::DynamicDraw);
                /// takes up locations 4, 5, 6, 7
                DataFormat::Material3331(At(4), PerInstance(transformations.len() as u32)).set_vertex_attribs(opengl, dtype_size);
            },
            ObjectMaterials::Constant(material) => {
                let data = Matrix::from_1darray(material.get_material_qualities().get_components_array());
                with_materials_vbo.buffer_data(&data, DrawType::DynamicDraw);
                /// takes up locations 4, 5, 6, 7
                DataFormat::Material3331(At(4), PerInstance(transformations.len() as u32)).set_vertex_attribs(opengl, dtype_size);

            },
            ObjectMaterials::PerVertex(ref materials_matrix) => {
                let mut material_values = vec![];
                materials_matrix.array.iter().enumerate().for_each(
                    |(idx, material)| {
                        material_values.extend(
                            material.get_material_qualities().get_components_array()
                        );
                    }
                );

                /// takes up locations 4, 5, 6, 7
                /// also lying about data length, because no const generics
                with_materials_vbo.buffer_data(&Matrix::<f32, 1, S1<1>>::from_vec(material_values), DrawType::DynamicDraw);
                DataFormat::Material3331(At(4), PerVertex).set_vertex_attribs(opengl, dtype_size);
            },
        }

        let transformation_data = Self::get_matrix_of_transformation_matrices(&transformations);
        let with_transformations_vbo = WithVbo::new(opengl);
        with_transformations_vbo.buffer_data(&transformation_data, DrawType::DynamicDraw);
        /// takes up locations 8, 9, 10, 11
        DataFormat::TranformationMatrix4x4(At(8), PerInstance(1)).set_vertex_attribs(opengl, dtype_size);





 
 
        Self {
            position_matrix: positions,
            colour_matrix: colour,
            //texture_coords_matrix: texture_coords,
            normals_matrix: normals,
            materials_matrix: materials,
            transformation_matrices: transformations,
 
            object_vao: with_object_vao.get_vao(),
            position_matrix_vbo: with_positions_vbo.get_vbo(),
            colour_matrix_vbo: with_colours_vbo.get_vbo(),
            texture_matrix_vbo: with_texture_coords_vbo.get_vbo(),
            normals_matrix_vbo: with_normals_vbo.get_vbo(),
            materials_matrix_vbo: with_materials_vbo.get_vbo(),
            transformation_matrices_vbo: with_transformations_vbo.get_vbo(),
 
            diffuse_texture,
            specular_texture,
         }
    }
    
    pub fn draw<'a>(&'a self, opengl:&Gl, textures:&mut Textures<'a>, programs:&Programs) -> Result<(), ContextError> {

        textures.activate(
            opengl, OpenglTexture::Texture0, &self.diffuse_texture, programs
        )?;
        textures.activate(
            opengl, OpenglTexture::Texture1, &self.specular_texture, programs
        )?;

        let with_vao = WithVao::existing(opengl, self.object_vao);
        with_vao.draw_instanced(DrawMode::GlTriangles, &self.position_matrix, self.transformation_matrices.len().try_into().unwrap());
    
        Ok(())
    }
}




//pub struct InstancingTestObject<const N:usize> {
pub struct InstancingTestObject<const N_VERTICES:usize, const N_ITEMS_PER_VERTEX:usize> {
    /// position 3, colour 3, alpha 1, normal 3, texture 2
    pub vertices:Matrix<f32, 2, S2<N_ITEMS_PER_VERTEX, N_VERTICES>>,
    //pub transformation_matrices:[Matrix<f32, 2>; N],
    pub transformation_matrices_vec:Vec<Matrix<f32, 2, S2<4, 4>>>,
    object_vao:u32,
    vertices_vbo:u32,
    transformations_vbo:u32,
}

//impl<const N:usize> InstancingTestObject<N> {
impl<const N_VERTICES:usize, const N_ITEMS_PER_VERTEX:usize> InstancingTestObject<N_VERTICES, N_ITEMS_PER_VERTEX> {
    //pub fn new(vertices:Matrix<f32, 2>, transformations:[Matrix<f32, 2>; N]) -> Self {
    //pub fn new(opengl:&Gl, vertices:Matrix<f32, 2, S2<N_ITEMS_PER_VERTEX, N_VERTICES>>, transformations:Vec<Matrix<f32, 2, S2<4, 4>>>) -> Result<Self, ContextError> {
    pub fn new(opengl:&Gl, vertices:Matrix<f32, 2, S2<N_ITEMS_PER_VERTEX, N_VERTICES>>, transformations:Vec<Matrix<f32, 2, S2<4, 4>>>) -> Self {
        
        //let (vertices_vao, vertices_vbo, transformations_vbo) = Self::prepare(opengl, &vertices, &transformations)?;
        let (vertices_vao, vertices_vbo, transformations_vbo) = Self::prepare(opengl, &vertices, &transformations);
        //Ok(
            Self {
                vertices, transformation_matrices_vec:transformations,
                object_vao: vertices_vao, vertices_vbo, transformations_vbo
            }
        //)

    }

    fn get_matrix_of_transformation_matrices(transformations:&Vec<Matrix<f32, 2, S2<4, 4>>>) -> Matrix<f32, 1, S1<1>> {
        const FAKE_LEN:usize = 1;
        let vec_of_items = transformations.iter()
                                             .map(|m| m.get_view_of_array())
                                             .collect::<Vec<&[f32]>>()
                                             .concat();
        let mat: Matrix<f32, 1, S1<FAKE_LEN>> = Matrix::from_vec(vec_of_items);
        mat        
    }

    fn prepare(opengl:&Gl, vertices:&Matrix<f32, 2, S2<N_ITEMS_PER_VERTEX, N_VERTICES>>, transformations:&Vec<Matrix<f32, 2, S2<4, 4>>>) -> (u32, u32, u32) {

        let with_vertices_vao = WithVao::new(opengl);
        
        let with_vertices_vbo = WithVbo::new(opengl);
        with_vertices_vbo.buffer_data(&vertices, DrawType::DynamicDraw);
        with_vertices_vao.set_vertex_attribs_per_vertex(vertices.dtype_memsize() as i32, DataFormat::Position3Colour3Alpha1Normal3Texture2);

        let transformation_data = Self::get_matrix_of_transformation_matrices(&transformations);
        let with_transformations_vbo = WithVbo::new(opengl);
        with_transformations_vbo.buffer_data(&transformation_data, DrawType::DynamicDraw);
        with_vertices_vao.set_vertex_attrib_mat4_per_instance(transformation_data.dtype_memsize() as i32, DataFormat::Position3Colour3Alpha1Normal3Texture2);

        (with_vertices_vao.get_vao(), with_vertices_vbo.get_vbo(), with_transformations_vbo.get_vbo())
    }



    pub fn draw(&self, opengl:&Gl) {
        let with_vao = WithVao::existing(opengl, self.object_vao);
        with_vao.draw_instanced(DrawMode::GlTriangles, &self.vertices, self.transformation_matrices_vec.len().try_into().unwrap());
    }

    //fn prepare(opengl:&Gl, vertices:&Matrix<f32, 2, S2<N_ITEMS_PER_VERTEX, N_VERTICES>>, transformations:&Vec<Matrix<f32, 2, S2<4, 4>>>) -> Result<(u32, u32, u32), ContextError> {
//
    //    let with_vertices_vao = WithObject::new(opengl, OpenglObject::VAO, DataFormat::Position3Colour3Alpha1Normal3Texture2);
    //    
    //    let with_vertices_vbo = WithObject::new(opengl, OpenglObject::VBO, DataFormat::Position3Colour3Alpha1Normal3Texture2);
    //    with_vertices_vbo.buffer_data(&vertices, DrawType::DynamicDraw, OpenglObject::VBO)?;
    //    with_vertices_vao.set_vertex_attribs(vertices.dtype_memsize() as i32)?;
//
    //    let transformation_data = Self::get_matrix_of_transformation_matrices(&transformations);
    //    let with_transformations_vbo = WithObject::new(opengl, OpenglObject::VBO, DataFormat::Position3Colour3Alpha1Normal3Texture2);
    //    with_transformations_vbo.buffer_data(&transformation_data, DrawType::DynamicDraw, OpenglObject::VBO)?;
    //    with_vertices_vao.set_vertex_attrib_mat4_per_instance(transformation_data.dtype_memsize() as i32)?;
//
    //    Ok((with_vertices_vao.get_vao(), with_vertices_vbo.get_vbo(), with_transformations_vbo.get_vbo()))
    //}
//
//
//
    //pub fn draw(&self, opengl:&Gl) -> Result<(), ContextError> {
//
    //    let with_vao = WithObject::existing(opengl, OpenglObject::VAO, self.object_vao, DataFormat::Position3Colour3Alpha1Normal3Texture2);
    //    with_vao.draw_instanced(DrawCall::Arrays, DrawMode::GlTriangles, &self.vertices, self.transformation_matrices_vec.len().try_into().unwrap())?;
//
    //    Ok(())
    //}
}