#![allow(warnings)]

// uncomment for release
// #![windows_subsystem = "windows"]


mod cube;


use opengl::abstractions::{WithObject};
use opengl::{gl, intermediate_opengl, raw_opengl};
use render_context::errors::RenderError;
use render_context::render::Render;
use render_context::enums::{GlError, ProgramSelect, DrawMode, DrawCall, DataFormat};
use numeracy::matrices::Matrix;
use zune_jpeg;

use std::ffi::{CStr, CString};
use std::os::raw::c_void;

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
        [ 350.,  350., 0.0,   1.0, 1.0],   // top right
        [ 350., -350., 0.0,   1.0, 0.0],   // bottom right
        [-350., -350., 0.0,   0.0, 0.0],   // bottom left
        [-350.,  350., 0.0,   0.0, 1.0],   // top left 
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

    let wall_data = include_bytes!("wall.jpg");
    //let wall_data = include_bytes!("../images/shazi.jpg");
    //let wall_data = include_bytes!("1bafc004e22b8b5fbd9a2616be16ff02.jpg");
    //let wall_data = include_bytes!("green dragon.jpg");
    //let wall_data = include_bytes!("unknown_by_korbox_d2scgt2-pre.jpg");
    //let wall_data = include_bytes!("zudarts-lee-180308(1).jpg");

    //let (tex_width, tex_height, maybe_pixel_data, _) = opengl::image_decoding::jpeg::get_jpeg_width_length_from_multiple(wall_data).unwrap();


    let cwd_and_file = std::env::current_dir().unwrap().join("images").join("wall.jpg");
    let cwd_and_file = std::env::current_dir().unwrap().join("images").join("Shazi.jpg");

    let be = std::fs::read(cwd_and_file).unwrap();
    let mut decoder = zune_jpeg::JpegDecoder::new(&be);
    let pixels = decoder.decode().unwrap();

    let (width, height) = decoder.dimensions().unwrap();
    let nchannels = decoder.get_output_colorspace().unwrap().num_components();
    
    let data = Matrix {shape:vec![width*nchannels, height], array:pixels, dtype:numeracy::enums::MatrixDataTypes::U8};
    let flipped_data = data.flip_vertically()?;
    
    let texture_id = intermediate_opengl::texture_test_1(
        &render.window.opengl, width as *mut i32, height as *mut i32, flipped_data.array.as_ptr() as *const u8
    );




    render.use_program(ProgramSelect::SelectSimpleTexture);
    raw_opengl::set_uniform_int(
        &render.window.opengl,
        intermediate_opengl::get_uniform_location(
            &render.window.opengl, render.programs.current_program.unwrap(), "texture1"
        ).unwrap(),
        0);
    render.programs.disuse_program(&render.window.opengl);

    while !render.render_over() {
        render.begin_render_actions()?;


        
        //render.use_program(ProgramSelect::SelectSimpleOrthographic);
        //render.draw(DrawCall::Arrays, DrawMode::GlTriangles, c_vao, &cube, DataFormat::Position3Colour3Alpha1)?;

        //render.use_program(ProgramSelect::SelectBlinnPhongOrthographic);
        //render.draw(DrawCall::Arrays, DrawMode::GlTriangles, t_vao, &triangle, DataFormat::Position3Colour3Alpha1Normal3)?;

        render.use_program(ProgramSelect::SelectSimpleTexture);
        //raw_opengl::set_uniform_int(
        //    &render.window.opengl,
        //    intermediate_opengl::get_uniform_location(
        //        &render.window.opengl, render.programs.current_program.unwrap(), "texture1"
        //    ).unwrap(),
        //    0);
        raw_opengl::active_texture(&render.window.opengl, gl::TEXTURE0);
        raw_opengl::bind_texture(&render.window.opengl, gl::TEXTURE_2D, texture_id);
        render.draw(DrawCall::Elements, DrawMode::GlTriangles, tex_vao, &triangle_indices, DataFormat::Position3Texture2)?;

        // unknown data type
        //render.draw(DrawCall::Elements, DrawMode::GlTriangles, vao, &indices_matrix)?;
        //render.draw(DrawCall::Elements, DrawMode::GlTriangles, t_vao, &triangle_indices)?;


        render.end_render_actions()?;
    }

    Ok(())
}