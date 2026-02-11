#![allow(warnings)]

// uncomment for release
// #![windows_subsystem = "windows"]


mod cube;


use atmospheric::image_processing;
use atmospheric::enums;
use atmospheric::opengl;
use atmospheric::opengl::abstractions::{self, TextureSetup, Textures, WithObject};
use atmospheric::enums::{InternalFormat, TextureMagFilter, TextureMinFilter, TextureWrapping};
use atmospheric::opengl::{gl, intermediate_opengl, raw_opengl};
use atmospheric::enums::ContextError;
use atmospheric::enums::ImageFormat;
use atmospheric::context::Context;
use atmospheric::enums::{DataFormat, DrawCall, DrawMode, GlError, OpenglTexture, ProgramSelect, TextureTarget, UniformType};
use numeracy::matrices::Matrix;
use numeracy::vectors::Vector;


use std::ffi::{CStr, CString};
use std::io::Read;
use std::os::raw::c_void;

use crate::image_processing::Image;

//use ppm_viewer;
use ray_tracer;

fn error(msg:String) {
    let a = true;
    let _b = match a {
        true =>Err(msg),
        false =>Ok(msg),
    }.unwrap();
}


fn main() -> Result<(), ContextError> {

    //let ppm = ray_tracer::hello_ppm();
    //let b = ray_tracer::ppm_to_file("ppm_viewer/src/test.ppm", ppm).unwrap();
    //error("hi".to_string());

    let cube = cube::cube((0.0, 0.0, 0.0), 14.0);

    let (vertices_matrix, indices_matrix) = cube::ebo_cube((0.0, 0.0, 0.0), 8.0);
    //let (vertices_matrix, indices_matrix) = cube::ebo_cube((0.0, 0.0, 0.0), 50.0);


    let triangle = Matrix::from_2darray([
        [-5.0,  0.0, 0.0, 0.9, 0.5, 0.1, 1.0, 1.0, 0.0, 0.0],
        [ 0.0, 5.0, 0.0, 0.1, 0.9, 0.5, 1.0, 1.0, 0.0, 0.0],
        [ 5.0, 0.0, 0.0, 0.5, 0.1, 0.9, 1.0, 1.0, 0.0, 0.0],
    ]);

    let texture_triangle = Matrix::from_2darray([
          // positions      // texture coords
        [ 35.,  35., -30.0,   1.0, 1.0],   // top right
        [ 35., -35., -30.0,   1.0, 0.0],   // bottom right
        [-35., -35., -30.0,   0.0, 0.0],   // bottom left
        [-35.,  35., -30.0,   0.0, 1.0],   // top left 
    ]);

    let zero = Matrix::from_2darray([[0., 0., 0., 0., 0., 0., 1.]]);
    let x    = Matrix::from_2darray([[5., 0., 0., 1., 0., 0., 1.]]);
    let y    = Matrix::from_2darray([[0., 5., 0., 0., 1., 0., 1.]]);
    let z    = Matrix::from_2darray([[0., 0., 10., 0., 0., 1., 1.]]);

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


    let mut render = Context::default()?;
    render.setup_render();


    let (vao, vbo, ebo) = render.create_vao_vbo_ebo(&vertices_matrix, &indices_matrix, DataFormat::Position3Colour3Alpha1)?;

    let (c_vao, c_vbo) = render.create_vao_vbo(&cube, DataFormat::Position3Colour3Alpha1)?;


    let (t_vao, t_vbo) = render.create_vao_vbo(&triangle, DataFormat::Position3Colour3Alpha1Normal3)?;
    let (tex_vao, tex_vbo, tex_ebo) = render.create_vao_vbo_ebo(&texture_triangle, &triangle_indices, DataFormat::Position3Texture2)?;
    let (zero_vao, zero_vbo) = render.create_vao_vbo(&zero, DataFormat::Position3Colour3Alpha1)?;
    let (x_vao, z_vbo) = render.create_vao_vbo(&x, DataFormat::Position3Colour3Alpha1)?;
    let (y_vao, z_vbo) = render.create_vao_vbo(&y, DataFormat::Position3Colour3Alpha1)?;
    let (z_vao, z_vbo) = render.create_vao_vbo(&z, DataFormat::Position3Colour3Alpha1)?;

    
    //let (t_vao, t_vbo, t_ebo) = render.create_vao_vbo_ebo(&triangle, &triangle_indices)?;


    let awesomeface = Image::decode_from_path("images/awesomeface.png", ImageFormat::PNG, true);
    let bluefaces   = Image::decode_from_path("images/bluefaces.png", ImageFormat::PNG, true);
    //let ppm = Image::decode("ppm_viewer/src/test.ppm", ImageFormat::PPMP3, false);
    let ppm = Image::decode_from_path("ray_tracer/test.txt", ImageFormat::PPMP3, true);
    //let ppm = Image::decode("ray_tracer/hey.txt", ImageFormat::PPMP3, false);


    let prepared_bluefaces = TextureSetup::get(
            &render.window.opengl, TextureTarget::Texture2D,
            bluefaces
            //bluefaces.width, bluefaces.height, bluefaces.pixels, bluefaces.format.into()
        )
        .set_texture_image_and_mipmap(0)
        .set_filters(TextureMinFilter::LinearMipmapNearest, TextureMagFilter::Linear)
        //.set_st_wrapping(TextureWrapping::ClampToBorder(1.0, 1.0, 1.0, 1.0), TextureWrapping::ClampToEdge)
        .set_st_wrapping(TextureWrapping::MirroredRepeat, TextureWrapping::MirroredRepeat)
        .get_prepared_texture()?;
    let prepared_ppm = TextureSetup::get(
            &render.window.opengl, TextureTarget::Texture2D,
            ppm
            //bluefaces.width, bluefaces.height, bluefaces.pixels, bluefaces.format.into()
        )
        .set_texture_image_and_mipmap(0)
        .set_filters(TextureMinFilter::LinearMipmapNearest, TextureMagFilter::Linear)
        //.set_st_wrapping(TextureWrapping::ClampToBorder(1.0, 1.0, 1.0, 1.0), TextureWrapping::ClampToEdge)
        .set_st_wrapping(TextureWrapping::Repeat, TextureWrapping::Repeat)
        .get_prepared_texture()?;

    let prepared_awesomeface = TextureSetup::get(
            &render.window.opengl, TextureTarget::Texture2D,
            awesomeface
            //bluefaces.width, bluefaces.height, bluefaces.pixels, bluefaces.format.into()
        )
        .set_texture_image_and_mipmap(0)
        .set_filters(TextureMinFilter::LinearMipmapNearest, TextureMagFilter::Linear)
        //.set_st_wrapping(TextureWrapping::ClampToBorder(1.0, 1.0, 1.0, 1.0), TextureWrapping::ClampToEdge)
        .set_st_wrapping(TextureWrapping::Repeat, TextureWrapping::Repeat)
        .get_prepared_texture()?;


        let vertex_text   = std::fs::read("src/two_texture_vertex.glsl").unwrap().iter().map(|a| *a as char).collect::<String>();
        let fragment_text = std::fs::read("src/two_texture_fragment.glsl").unwrap().iter().map(|a| *a as char).collect::<String>();
        let shader_id = render.compile_custom_program(vertex_text.as_str(), fragment_text.as_str())?;

    while !render.render_over() {
        render.begin_render_actions()?;


        
        //render.use_program(ProgramSelect::SelectSimpleOrthographic);
        //let with_relevant = WithObject::existing(&render.window.opengl, opengl::enums::Object::VAO, c_vao, DataFormat::Position3Colour3Alpha1);
        //render.programs.draw(with_relevant, DrawCall::Arrays, DrawMode::GlTriangles, &cube)?;

        //render.use_program(ProgramSelect::SelectBlinnPhongOrthographic);
        //let with_relevant = WithObject::existing(&render.window.opengl, opengl::enums::Object::VAO, t_vao, DataFormat::Position3Colour3Alpha1Normal3);
        //render.programs.draw(with_relevant, DrawCall::Arrays, DrawMode::GlTriangles, &triangle)?;

        //render.use_program(ProgramSelect::SelectSimpleTexture);

        render.use_custom_program(shader_id);
        render.set_orthographic_camera_uniforms()?;
        //render.set_custom_uniform(sa, uniform, value)

        
        &render.textures.activate(
            &render.window.opengl, OpenglTexture::Texture0, &prepared_bluefaces, &render.programs
        )?;
        //&render.textures.activate(
        //    &render.window.opengl, OpenglTexture::Texture1, &prepared_ppm, &render.programs
        //)?;
        &render.textures.activate(
            &render.window.opengl, OpenglTexture::Texture1, &prepared_awesomeface, &render.programs
        )?;
    
        //let with_relevant = WithObject::existing(&render.window.opengl, enums::Object::VAO, tex_vao, DataFormat::Position3Texture2)
        //                                                .add(enums::Object::EBO, tex_ebo)?;
        //                                                //.add(opengl::enums::Object::Texture2D, texture_id)?;
        //render.programs.draw(with_relevant, DrawCall::Elements, DrawMode::GlTriangles, &triangle_indices)?;


        // if !render.paused {
        //     render.camera.angle_xyz += Vector::from_1darray([0.0, 0.50, 0.0]);
        // }


         render.use_program(ProgramSelect::SelectSimpleOrthographic)?;

        // origin, x, y, and z points
        // let with_zero = WithObject::existing(&render.window.opengl, enums::Object::VAO, zero_vao, DataFormat::Position3Colour3Alpha1);
        // render.programs.draw(with_zero, DrawCall::Arrays, DrawMode::GlPoints, &zero)?;
        // let with_x = WithObject::existing(&render.window.opengl, enums::Object::VAO, x_vao, DataFormat::Position3Colour3Alpha1);
        // render.programs.draw(with_x, DrawCall::Arrays, DrawMode::GlPoints, &x)?;
        // let with_y = WithObject::existing(&render.window.opengl, enums::Object::VAO, y_vao, DataFormat::Position3Colour3Alpha1);
        // render.programs.draw(with_y, DrawCall::Arrays, DrawMode::GlPoints, &y)?;
        // let with_z = WithObject::existing(&render.window.opengl, enums::Object::VAO, z_vao, DataFormat::Position3Colour3Alpha1);
        // render.programs.draw(with_z, DrawCall::Arrays, DrawMode::GlPoints, &z)?;

        //let cam_x = -8.;
        //render.camera.camera_target[0] = cam_x;
        //render.camera.camera_position[0] = cam_x;
        //render.camera.camera_position[2] = -20.0;
        let with_cube = WithObject::existing(&render.window.opengl, enums::Object::VAO, c_vao, DataFormat::Position3Colour3Alpha1);
        render.programs.draw(with_cube, DrawCall::Arrays, DrawMode::GlTriangles, &cube)?;


        let tr = Matrix::translate(Vector::from_1darray([-16.0, 0.0, 0.0]));
        render.programs.set_uniform(&render.window.opengl, "world_transform", UniformType::Mat4,
            Matrix::opengl_to_right_handed().matmul(&tr).unwrap())?;
        let with_cube = WithObject::existing(&render.window.opengl, enums::Object::VAO, c_vao, DataFormat::Position3Colour3Alpha1);
        render.programs.draw(with_cube, DrawCall::Arrays, DrawMode::GlTriangles, &cube)?;


        //render.camera.translate_independant_of_external_information(Vector::from_1darray([0.0, 0.0, 0.05]));
        
        // forwards testing
        //render.camera.translate_relative_to_the_target(-0.05, 0.00, 0.0)?;
        //render.camera.camera_position += Vector::from_1darray([0.0, 0., 0.05]);

        // right testing
        render.camera.translate_relative_to_the_target(0., 0.05, 0.0)?;
        //render.camera.translate_relative_to_the_target(0., 0.5, 0.0)?;
        render.camera.translate_relative_to_the_target(0., 0.0, 0.05)?;

        render.end_render_actions()?;
    }

    Ok(())
}