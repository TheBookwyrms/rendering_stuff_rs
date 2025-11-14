#![allow(warnings)]

// uncomment for release
// #![windows_subsystem = "windows"]


mod cube;
mod image_processing;
mod enums;


use opengl::abstractions::{self, WithObject, TextureSetup};
use opengl::enums::{InternalFormat, TextureMagFilter, TextureMinFilter, TextureWrapping};
use opengl::{gl, intermediate_opengl, raw_opengl};
use render_context::errors::RenderError;
use render_context::render::Render;
use render_context::enums::{GlError, ProgramSelect, DrawMode, DrawCall, DataFormat, TextureTarget, UniformType};
use numeracy::matrices::Matrix;
use zune_jpeg;
use zune_png;
use enums::ImageFormat;


use std::ffi::{CStr, CString};
use std::io::Read;
use std::os::raw::c_void;

use crate::image_processing::Image;

//use ppm_viewer;




fn main() -> Result<(), RenderError> {

    let cube = cube::cube((0.0, 0.0, 0.0), 8.0);

    let (vertices_matrix, indices_matrix) = cube::ebo_cube((0.0, 0.0, 0.0), 8.0);
    //let (vertices_matrix, indices_matrix) = cube::ebo_cube((0.0, 0.0, 0.0), 50.0);


    let triangle = Matrix::from_2darray([
        [-5.0,  0.0, 0.0, 0.9, 0.5, 0.1, 1.0, 1.0, 0.0, 0.0],
        [ 0.0, 5.0, 0.0, 0.1, 0.9, 0.5, 1.0, 1.0, 0.0, 0.0],
        [ 5.0, 0.0, 0.0, 0.5, 0.1, 0.9, 1.0, 1.0, 0.0, 0.0],
    ]);

    let texture_triangle = Matrix::from_2darray([
          // positions      // texture coords
        [ 35.,  35., 0.0,   5.0, 5.0],   // top right
        [ 35., -35., 0.0,   5.0, 0.0],   // bottom right
        [-35., -35., 0.0,   0.0, 0.0],   // bottom left
        [-35.,  35., 0.0,   0.0, 5.0],   // top left 
    ]);
    //let texture_triangle = Matrix::from_2darray([
    //      // positions      // colors             // texture coords
    //    [ 5.0,  5.0, 0.0,   1.0, 0.0, 0.0, 1.0,   1.0, 1.0],   // top right
    //    [ 5.0, -5.0, 0.0,   0.0, 1.0, 0.0, 1.0,   1.0, 0.0],   // bottom right
    //    [-5.0, -5.0, 0.0,   0.0, 0.0, 1.0, 1.0,   0.0, 0.0],   // bottom left
    //    [-5.0,  5.0, 0.0,   1.0, 1.0, 0.0, 1.0,   0.0, 1.0],   // top left 
    //]);
    let triangle_indices = Matrix::from_2darray([
        [0, 1, 3],
        [1, 2, 3],
    ]);


    let mut render = Render::default()?;
    render.setup_render();


    let (vao, vbo, ebo) = render.create_vao_vbo_ebo(&vertices_matrix, &indices_matrix, DataFormat::Position3Colour3Alpha1)?;

    let (c_vao, c_vbo) = render.create_vao_vbo(&cube, DataFormat::Position3Colour3Alpha1)?;


    let (t_vao, t_vbo) = render.create_vao_vbo(&triangle, DataFormat::Position3Colour3Alpha1Normal3)?;
    let (tex_vao, tex_vbo, tex_ebo) = render.create_vao_vbo_ebo(&texture_triangle, &triangle_indices, DataFormat::Position3Texture2)?;
    
    
    //let (t_vao, t_vbo, t_ebo) = render.create_vao_vbo_ebo(&triangle, &triangle_indices)?;


    let awesomeface = Image::decode("images/awesomeface.png", ImageFormat::PNG, true);
    let bluefaces = Image::decode("images/bluefaces.png", ImageFormat::PNG, true);
    
    //let texture_id = intermediate_opengl::texture_test_1(
    //    &render.window.opengl, bluefaces.width as *mut i32, bluefaces.height as *mut i32, bluefaces.pixels.as_ptr() as *const u8
    //);

    let texture_id = TextureSetup::get(
            &render.window.opengl, TextureTarget::Texture2D,
            bluefaces.width as i32, bluefaces.height as i32, bluefaces.pixels, bluefaces.format.into()
        )
        .set_texture_image_and_mipmap(0)
        //.create_mipmap()
        .set_filters(TextureMinFilter::LinearMipmapNearest, TextureMagFilter::Linear)
        //.set_st_wrapping(TextureWrapping::ClampToBorder(1.0, 1.0, 1.0, 1.0), TextureWrapping::ClampToEdge)
        .set_st_wrapping(TextureWrapping::MirroredRepeat, TextureWrapping::MirroredRepeat)
        .get_prepared_texture()?.texture;


    raw_opengl::active_texture(&render.window.opengl, gl::TEXTURE0);
    render.use_program(ProgramSelect::SelectSimpleTexture);
    intermediate_opengl::set_uniform(
        &render.window.opengl, render.programs.current_program.unwrap(),
        "texture1", UniformType::Int, Matrix::from_scalar(0).as_ptr()
    )?;
    render.programs.disuse_program(&render.window.opengl);
    intermediate_opengl::bind_texture(&render.window.opengl, TextureTarget::Texture2D, 0);
    //raw_opengl::bind_texture(&render.window.opengl, gl::TEXTURE_2D, 0);


    while !render.render_over() {
        render.begin_render_actions()?;


        
        //render.use_program(ProgramSelect::SelectSimpleOrthographic);
        //let with_relevant = WithObject::existing(&render.window.opengl, opengl::enums::Object::VAO, c_vao, DataFormat::Position3Colour3Alpha1);
        //render.programs.draw(with_relevant, DrawCall::Arrays, DrawMode::GlTriangles, &cube)?;

        //render.use_program(ProgramSelect::SelectBlinnPhongOrthographic);
        //let with_relevant = WithObject::existing(&render.window.opengl, opengl::enums::Object::VAO, t_vao, DataFormat::Position3Colour3Alpha1Normal3);
        //render.programs.draw(with_relevant, DrawCall::Arrays, DrawMode::GlTriangles, &triangle)?;

        render.use_program(ProgramSelect::SelectSimpleTexture);
        let with_relevant = WithObject::existing(&render.window.opengl, opengl::enums::Object::VAO, tex_vao, DataFormat::Position3Texture2)
                                                        .add(opengl::enums::Object::EBO, tex_ebo)?
                                                        .add(opengl::enums::Object::Texture2D, texture_id)?;
        render.programs.draw(with_relevant, DrawCall::Elements, DrawMode::GlTriangles, &triangle_indices)?;





        // unknown data type
        //render.draw(DrawCall::Elements, DrawMode::GlTriangles, vao, &indices_matrix)?;
        //render.draw(DrawCall::Elements, DrawMode::GlTriangles, t_vao, &triangle_indices)?;


        render.end_render_actions()?;
    }

    Ok(())
}