use atmospheric::context;
use numeracy::matrices2::Matrix;

use crate::c_void;

use atmospheric::enums::{
    BufferObject, ContextError, DataFormat,
    DrawType, Object as OpenglObject, UpdateVertexAttrib,
    DrawCall, DrawMode, ArrayObject
};
use atmospheric::opengl::{raw_opengl, intermediate_opengl};
use atmospheric::opengl::abstractions::WithObject;
use atmospheric::opengl::gl::Gl;

//#[repr(C)]
//pub struct InstancingTestObject<const N:usize> {
pub struct InstancingTestObject {
    /// position 3, colour 3, alpha 1, normal 3, texture 2
    pub vertices:Matrix<f32, 2>,
    //pub transformation_matrices:[Matrix<f32, 2>; N],
    pub transformation_matrices_vec:Vec<Matrix<f32, 2>>,
    object_vao:u32,
    vertices_vbo:u32,
    transformations_vbo:u32,
}

//impl<const N:usize> InstancingTestObject<N> {
impl InstancingTestObject {
    //pub fn new(vertices:Matrix<f32, 2>, transformations:[Matrix<f32, 2>; N]) -> Self {
    pub fn new(opengl:&Gl, vertices:Matrix<f32, 2>, transformations:Vec<Matrix<f32, 2>>) -> Self {
        let (vertices_vao, vertices_vbo, transformations_vbo) = Self::prepare(opengl, vertices.clone(), transformations.clone()).unwrap();
        Self {vertices, transformation_matrices_vec:transformations, object_vao: vertices_vao, vertices_vbo, transformations_vbo}
        //Self {vertices, transformation_matrices:transformations}
    }
    pub fn get_matrix_of_transformation_matrices(transformations:&Vec<Matrix<f32, 2>>) -> Matrix<f32, 1> {
        
        let vec_of_items = transformations.iter()
                                             .map(|m| m.get_view_of_array())
                                             .collect::<Vec<&[f32]>>()
                                             .concat();
        let mat = Matrix::from_vec(vec_of_items);
        mat        
        
        
        //let a = transformations.clone().into_iter().map(|a| a.array).collect::<Vec<Vec<f32>>>();
        ////let a = self.transformation_matrices.clone().map(|a| a.array);
        //let b = a.concat();
        //let c = Matrix::from_vec(b);
        //c
    }
    fn prepare(opengl:&Gl, vertices:Matrix<f32, 2>, transformations:Vec<Matrix<f32, 2>>) -> Result<(u32, u32, u32), ContextError> {

        let with_vertices_vao = WithObject::new(opengl, OpenglObject::VAO, DataFormat::Position3Colour3Alpha1Normal3Texture2);
        
        let with_vertices_vbo = WithObject::new(opengl, OpenglObject::VBO, DataFormat::Position3Colour3Alpha1Normal3Texture2);
        with_vertices_vbo.buffer_data(&vertices, DrawType::DynamicDraw, OpenglObject::VBO)?;
        with_vertices_vao.set_vertex_attribs(vertices.dtype_memsize() as i32)?;

        let transformation_data = Self::get_matrix_of_transformation_matrices(&transformations);
        let with_transformations_vbo = WithObject::new(opengl, OpenglObject::VBO, DataFormat::Position3Colour3Alpha1Normal3Texture2);
        with_transformations_vbo.buffer_data(&transformation_data, DrawType::DynamicDraw, OpenglObject::VBO)?;
        with_vertices_vao.set_vertex_attrib_mat4_per_instance(transformation_data.dtype_memsize() as i32)?;

        //let vertices_vao = intermediate_opengl::generate(opengl, OpenglObject::VAO);
        //intermediate_opengl::bind_vertex_array(opengl, ArrayObject::VertexArrayObject, vertices_vao);
        
        //let vertices_vbo_id = intermediate_opengl::generate(opengl, OpenglObject::VBO);
        //intermediate_opengl::bind_buffer(opengl, BufferObject::VertexBufferObject, vertices_vbo_id);

        //let vertices_data_size = vertices.memory_size() as isize;
        //let vertices_data_ptr = vertices.as_ptr() as *const c_void;
        //intermediate_opengl::buffer_data(
        //    opengl,
        //    BufferObject::VertexBufferObject,
        //    vertices_data_size, vertices_data_ptr, DrawType::DynamicDraw
        //);

        //let float_size = std::mem::size_of::<f32>() as i32;

        //let vertices_stride = 12;
        //let vertices_stride = 12+16;

        //let transformations_stride = 16;
        //let transformations_stride = 12+16;

        //let transformations_num_items = 4;
        //let transformations_num_items = 16;


        //intermediate_opengl::set_vertex_attrib(opengl, 0, 3, vertices_stride, 0, float_size);
        //intermediate_opengl::set_vertex_attrib_divisor(opengl, 0, UpdateVertexAttrib::PerVertex);
        //
        //intermediate_opengl::set_vertex_attrib(opengl, 1, 3, vertices_stride, 3, float_size);
        //intermediate_opengl::set_vertex_attrib_divisor(opengl, 1, UpdateVertexAttrib::PerVertex);
        //
        //intermediate_opengl::set_vertex_attrib(opengl, 2, 3, vertices_stride, 3+3, float_size);
        //intermediate_opengl::set_vertex_attrib_divisor(opengl, 2, UpdateVertexAttrib::PerVertex);
        //
        //intermediate_opengl::set_vertex_attrib(opengl, 3, 3, vertices_stride, 3+3+1, float_size);
        //intermediate_opengl::set_vertex_attrib_divisor(opengl, 3, UpdateVertexAttrib::PerVertex);
        //
        //intermediate_opengl::set_vertex_attrib(opengl, 4, 3, vertices_stride, 3+3+1+3, float_size);
        //intermediate_opengl::set_vertex_attrib_divisor(opengl, 4, UpdateVertexAttrib::PerVertex);
        

        //intermediate_opengl::bind_buffer(opengl, BufferObject::VertexBufferObject, 0);







        //let transformations_vbo_id = intermediate_opengl::generate(opengl, OpenglObject::VBO);
        //intermediate_opengl::bind_buffer(opengl, BufferObject::VertexBufferObject, transformations_vbo_id);
        
        // get and buffer data for transformations vbo
        //let data_size = transformation_data.memory_size() as isize;
        //let data_ptr = transformation_data.as_ptr() as *const c_void;
        
        //intermediate_opengl::buffer_data(
        //    opengl,
        //    BufferObject::VertexBufferObject,
        //    data_size, data_ptr, DrawType::DynamicDraw
        //);



        //intermediate_opengl::set_vertex_attrib(opengl, 5, 4, 16, 0, float_size);
        //intermediate_opengl::set_vertex_attrib_divisor(opengl, 5, UpdateVertexAttrib::PerInstance(1));
//
        //intermediate_opengl::set_vertex_attrib(opengl, 6, 4, 16, 0+4, float_size);
        //intermediate_opengl::set_vertex_attrib_divisor(opengl, 6, UpdateVertexAttrib::PerInstance(1));
//
        //intermediate_opengl::set_vertex_attrib(opengl, 7, 4, 16, 0+4+4, float_size);
        //intermediate_opengl::set_vertex_attrib_divisor(opengl, 7, UpdateVertexAttrib::PerInstance(1));
//
        //intermediate_opengl::set_vertex_attrib(opengl, 8, 4, 16, 0+4+4+4, float_size);
        //intermediate_opengl::set_vertex_attrib_divisor(opengl, 8, UpdateVertexAttrib::PerInstance(1));

        //intermediate_opengl::set_vertex_attrib(opengl, 5, 16, 16, 0, data_size as i32);
        ////intermediate_opengl::set_vertex_attrib(opengl, 5, 16, 12+16, 0, data_size as i32);
        ////intermediate_opengl::set_vertex_attrib(opengl, 5, 16, 16, 3+3+1+3+2, data_size as i32);
        //intermediate_opengl::set_vertex_attrib(opengl, 5, 16, 12+16, 3+3+1+3+2, data_size as i32);
        //
        //intermediate_opengl::set_vertex_attrib_divisor(opengl, 5, UpdateVertexAttrib::PerInstance(1));






        //intermediate_opengl::bind_buffer(opengl, BufferObject::VertexBufferObject, 0);


        //intermediate_opengl::bind_vertex_array(opengl, ArrayObject::VertexArrayObject, 0);




        Ok((with_vertices_vao.get_vao(), with_vertices_vbo.get_vbo(), with_transformations_vbo.get_vbo()))
    }



    pub fn draw(&self, opengl:&Gl) -> Result<(), ContextError> {

        let with_vao = WithObject::existing(opengl, OpenglObject::VAO, self.object_vao, DataFormat::Position3Colour3Alpha1Normal3Texture2);
        with_vao.draw_instanced(DrawCall::Arrays, DrawMode::GlTriangles, &self.vertices, self.transformation_matrices_vec.len().try_into().unwrap())?;

        //intermediate_opengl::bind_vertex_array(opengl, ArrayObject::VertexArrayObject, self.object_vao);
//
        //let (call, mode, data) = (DrawCall::Arrays, DrawMode::GlTriangles, &self.vertices);
        //let instance_count = self.transformation_matrices_vec.len() as i32;
//
//
        //let count : i32 = self.vertices.shape[1].try_into().unwrap();
//
        //intermediate_opengl::draw_arrays_instanced(opengl, mode, count, instance_count);
//
//
        //intermediate_opengl::bind_vertex_array(opengl, ArrayObject::VertexArrayObject, 0);


        Ok(())
    }
}