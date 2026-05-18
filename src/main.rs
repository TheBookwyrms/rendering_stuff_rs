#![allow(warnings)]

// uncomment for release
// #![windows_subsystem = "windows"]


mod cube;
mod thing;
mod ntree;


use atmospheric::enums::Object;
use atmospheric::image_processing;
use atmospheric::enums;
use atmospheric::materials::MaterialLightQualities;
use atmospheric::opengl;
//use atmospheric::opengl::abstractions::{self, TextureSetup, Textures, WithObject};
use atmospheric::opengl::abstractions2::{self, TextureSetup, Textures, WithObject};
use atmospheric::enums::{InternalFormat, TextureMagFilter, TextureMinFilter, TextureWrapping, CameraAxis, CameraVector};
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
use crate::ntree::QuadTree;
use crate::ntree::Point;
use crate::ntree::QuadTreeV2;
use crate::ntree::RelativePointPos;
use crate::ntree::SquareBounds;
//use crate::ntree::TreeItem;
use atmospheric::materials::Material;

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


    // //let a = QuadTree {item: &TreeItem::Value(2)};
    // let z = QuadTree::init_value(0);
    // let a = QuadTree::init_value(2);
    // let mut b = a.subdivide_value([
    //     QuadTree::init_value(3),
    //     QuadTree::init_value(4),
    //     QuadTree::init_value(5),
    //     QuadTree::init_value(6)
    //     ]).unwrap();
    // let mut c = z.subdivide_value([
    //     QuadTree::init_value(66),
    //     QuadTree::init_value(77),
    //     QuadTree::init_value(88),
    //     QuadTree::init_value(99),
    // ]).unwrap();
    // let d = b.subdivide_item(ntree::TreeIndex::Two, [
    //     QuadTree::init_value(66),
    //     c.clone(),
    //     QuadTree::init_value(88),
    //     QuadTree::init_value(99),
    // ]).unwrap();
// 
    // println!("{:?}", a);
    // println!("{:?}", b);
    // println!("{}", c);
    // println!("{}", d);
// 
// 
    // let sq = SquareBounds::new(-2., 2., -2., 2.);
    // let qp = Point::new(0.5, 0.5);
    // 
    // let qtree = QuadTree::init_value(sq);

    let p0 = Point::new(0., 0.);
    let p1 = Point::new(1., 0.9);
    let p2 = Point::new(1., 1.5);
    let p3 = Point::new(1.4, 1.75);
    let p4 = Point::new(-1., 1.);
    let p5 = Point::new(-3., 0.);


    let full_square_bounds = SquareBounds::new_simple_on_origin(-2.0, 2.0);
    let mut q1 = QuadTreeV2::new(full_square_bounds);
    println!("{:#?}", q1);
    //let q2 = q1.insert(vec![p0, p1,p2,p3,p4,p5]).unwrap();
    let mut q2 = q1.insert(vec![p1,p2]).unwrap();
    println!("{:#?}", q2);
    let q3 = q2.subdivide_node(RelativePointPos::TopRight).unwrap();
    println!("{:#?}", q3);




    panic!();




    let mut render = Context::default()?;
    render.setup_render();

    
    let cube = cube::colour_cube((0.0, 0.0, 0.0), 5.0, true);
    let (c_vao, c_vbo) = render.create_vao_vbo(&cube, DataFormat::Position3Colour3Alpha1Normal3)?;


    let (vertices_matrix, indices_matrix) = cube::_might_not_even_work_untested_ebo_cube((0.0, 0.0, 0.0), 14.0);
    let (vao, vbo, ebo) = render.create_vao_vbo_ebo(&vertices_matrix, &indices_matrix, DataFormat::Position3Colour3Alpha1)?;
    
    
    let tex_cube = cube::texture_cube((0.0, 0.0, 0.0), 14.0, 1.0);
    let (tc_vao, tc_vbo) = render.create_vao_vbo(&tex_cube, DataFormat::Position3Texture2)?;


    let triangle = Matrix::from_2darray([
        [-5.0,  0.0, 0.0, 0.9, 0.5, 0.1, 1.0, 1.0, 0.0, 0.0],
        [ 0.0, 5.0, 0.0, 0.1, 0.9, 0.5, 1.0, 1.0, 0.0, 0.0],
        [ 5.0, 0.0, 0.0, 0.5, 0.1, 0.9, 1.0, 1.0, 0.0, 0.0],
    ]);
    let (t_vao, t_vbo) = render.create_vao_vbo(&triangle, DataFormat::Position3Colour3Alpha1Normal3)?;


    let texture_triangle = Matrix::from_2darray([
          // positions      // texture coords
        [ 35.,  35., -30.0,   1.0, 1.0],   // top right
        [ 35., -35., -30.0,   1.0, 0.0],   // bottom right
        [-35., -35., -30.0,   0.0, 0.0],   // bottom left
        [-35.,  35., -30.0,   0.0, 1.0],   // top left 
    ]);
    let triangle_indices = Matrix::from_2darray([
        [0, 1, 3],
        [1, 2, 3],
    ]);
    let (tex_vao, tex_vbo, tex_ebo) = render.create_vao_vbo_ebo(&texture_triangle, &triangle_indices, DataFormat::Position3Texture2)?;


    let zero = Matrix::from_2darray([[0., 0., 0., 0., 0., 0., 1.]]);
    let x    = Matrix::from_2darray([[5., 0., 0., 1., 0., 0., 1.]]);
    let y    = Matrix::from_2darray([[0., 5., 0., 0., 1., 0., 1.]]);
    let z    = Matrix::from_2darray([[0., 0., 5., 0., 0., 1., 1.]]);
    let (zero_vao, zero_vbo) = render.create_vao_vbo(&zero, DataFormat::Position3Colour3Alpha1)?;
    let (x_vao, z_vbo) = render.create_vao_vbo(&x, DataFormat::Position3Colour3Alpha1)?;
    let (y_vao, z_vbo) = render.create_vao_vbo(&y, DataFormat::Position3Colour3Alpha1)?;
    let (z_vao, z_vbo) = render.create_vao_vbo(&z, DataFormat::Position3Colour3Alpha1)?;
    

    fn target_to_matrix(target:Vector<f32>, col:(f32, f32, f32), a:f32) -> Matrix<f32> {
        let (r, g, b) = col;
        let mut new_arr = target.multiply_by_constant(1.0).array;
        new_arr.extend(&[r, g, b, a]);
        Matrix { shape: vec![7, 1], array: new_arr }
    }
    let (target_vao, target_vbo) = render.create_vao_vbo(
        &target_to_matrix(
            render.camera.camera_info_matrix.get_camera(CameraVector::Target),
            (1.0, 1.0, 1.0),
            1.0),
        DataFormat::Position3Colour3Alpha1
    )?;

    fn get_light_matrix(render:&Context) -> Matrix<f32> {
        let (p0, p1, p2) = render.lighting.light_source_pos;
        let (c0, c1, c2) = render.lighting.light_diffuse_colour;
        let light_source    = Matrix::from_2darray([[p0, p1, p2, c0, c1, c2, 1.0]]);
        light_source
    }
    let (light_vao, light_vbo) = render.create_vao_vbo(&get_light_matrix(&render), DataFormat::Position3Colour3Alpha1)?;

    

    let awesomeface = Image::decode_from_path("images/awesomeface.png", ImageFormat::PNG, true);
    let bluefaces   = Image::decode_from_path("images/bluefaces.png", ImageFormat::PNG, true);
    let container   = Image::decode_from_path("images/container.jpg", ImageFormat::JPEG, true);
    let wall        = Image::decode_from_path("images/wall.jpg", ImageFormat::JPEG, true);
    let ppm         = Image::decode_from_path("ray_tracer/test.txt", ImageFormat::PPMP3, true);


    let prepared_bluefaces = TextureSetup::get_prepared(
        &render.window.opengl, TextureTarget::Texture2D,
        bluefaces,
        TextureWrapping::MirroredRepeat, TextureWrapping::MirroredRepeat,
        TextureMinFilter::LinearMipmapNearest, TextureMagFilter::Linear,
        0);

    let prepared_ppm = TextureSetup::get_prepared(
        &render.window.opengl, TextureTarget::Texture2D,
        ppm,
        TextureWrapping::Repeat, TextureWrapping::Repeat,
        TextureMinFilter::LinearMipmapNearest, TextureMagFilter::Linear,
        0);

    let prepared_awesomeface = TextureSetup::get_prepared(
        &render.window.opengl, TextureTarget::Texture2D,
        awesomeface,
        TextureWrapping::Repeat, TextureWrapping::Repeat,
        TextureMinFilter::LinearMipmapNearest, TextureMagFilter::Linear,
        0);

    let prepared_container = TextureSetup::get_prepared(
        &render.window.opengl, TextureTarget::Texture2D,
        container,
        TextureWrapping::Repeat, TextureWrapping::Repeat,
        TextureMinFilter::LinearMipmapNearest, TextureMagFilter::Linear,
        0);



        let vertex_text   = std::fs::read("src/two_texture_vertex.glsl").unwrap().iter().map(|a| *a as char).collect::<String>();
        let fragment_text = std::fs::read("src/two_texture_fragment.glsl").unwrap().iter().map(|a| *a as char).collect::<String>();
        let shader_id = render.compile_custom_program(vertex_text.as_str(), fragment_text.as_str())?;

        let mut count = 0;
    while !render.render_over() {
        count += 1;
        render.begin_render_actions()?;

        let time = render.window.get_time_since_glfw_init();
        let sin_t = time.sin();
        let cos_t = time.cos();
        
        //render.use_program(ProgramSelect::SelectSimpleOrthographic);
        //let with_relevant = WithObject::existing(&render.window.opengl, opengl::enums::Object::VAO, c_vao, DataFormat::Position3Colour3Alpha1);
        //render.programs.draw(with_relevant, DrawCall::Arrays, DrawMode::GlTriangles, &cube)?;

        //render.use_program(ProgramSelect::SelectBlinnPhongOrthographic);
        //let with_relevant = WithObject::existing(&render.window.opengl, opengl::enums::Object::VAO, t_vao, DataFormat::Position3Colour3Alpha1Normal3);
        //render.programs.draw(with_relevant, DrawCall::Arrays, DrawMode::GlTriangles, &triangle)?;

        render.use_program(ProgramSelect::SelectSimpleTexture);
        &render.textures.activate(
            &render.window.opengl, OpenglTexture::Texture0, &prepared_container, &render.programs
        )?;


        // panic!();
        // /// fix this to use WithObject.draw() by passing render.programs to it
        // /// that way you just call draw on the object itself
        // /// and then just move the one DataFormat check into WithObject
        // panic!();
        // let with_relevant = WithObject::existing(&render.window.opengl, enums::Object::VAO, tc_vao, DataFormat::Position3Texture2);
        // render.programs.draw(with_relevant, DrawCall::Arrays, DrawMode::GlTriangles, &tex_cube)?;


    //    render.use_custom_program(shader_id);
    //    render.set_orthographic_camera_uniforms()?;
    //    //render.set_custom_uniform(sa, uniform, value)
//
    //    
    //    &render.textures.activate(
    //        &render.window.opengl, OpenglTexture::Texture0, &prepared_bluefaces, &render.programs
    //    )?;
    //    &render.textures.activate(
    //        &render.window.opengl, OpenglTexture::Texture1, &prepared_ppm, &render.programs
    //    )?;
    //    //&render.textures.activate(
    //    //    &render.window.opengl, OpenglTexture::Texture2, &prepared_awesomeface, &render.programs
    //    //)?;
    //
//
//
    //    let with_relevant = WithObject::existing(&render.window.opengl, enums::Object::VAO, tex_vao, DataFormat::Position3Texture2)
    //                                                    .add(enums::Object::EBO, tex_ebo)?;
    //                                                    //.add(opengl::enums::Object::Texture2D, texture_id)?;
    //    render.programs.draw(with_relevant, DrawCall::Elements, DrawMode::GlTriangles, &triangle_indices)?;
//
//
//
//
        render.use_program(ProgramSelect::SelectSimpleOrthographic)?;
////
    //    //// origin, x, y, and z points
    //    //let with_zero = WithObject::existing(&render.window.opengl, enums::Object::VAO, zero_vao, DataFormat::Position3Colour3Alpha1);
    //    //render.programs.draw(with_zero, DrawCall::Arrays, DrawMode::GlPoints, &zero)?;
    //    //let with_x = WithObject::existing(&render.window.opengl, enums::Object::VAO, x_vao, DataFormat::Position3Colour3Alpha1);
    //    //render.programs.draw(with_x, DrawCall::Arrays, DrawMode::GlPoints, &x)?;
    //    //let with_y = WithObject::existing(&render.window.opengl, enums::Object::VAO, y_vao, DataFormat::Position3Colour3Alpha1);
    //    //render.programs.draw(with_y, DrawCall::Arrays, DrawMode::GlPoints, &y)?;
    //    //let with_z = WithObject::existing(&render.window.opengl, enums::Object::VAO, z_vao, DataFormat::Position3Colour3Alpha1);
    //    //render.programs.draw(with_z, DrawCall::Arrays, DrawMode::GlPoints, &z)?;
        //let with_target = WithObject::existing(&render.window.opengl, enums::Object::VAO, target_vao, DataFormat::Position3Colour3Alpha1)
        //         .add(Object::VBO, target_vbo)?;
        //let data = target_to_matrix(render.camera.camera_info_matrix.get_camera(CameraVector::Target), (1.0, 1.0, 1.0), 1.0);
        //with_target.buffer_sub_data(&data, Object::VBO)?;
        //render.programs.draw(with_target, DrawCall::Arrays, DrawMode::GlPoints, &data)?;
        

        render.lighting.light_diffuse_colour.0 = f32::sin(0.75*time as f32);
        render.lighting.light_diffuse_colour.1 = f32::sin(0.25*time as f32);
        render.lighting.light_diffuse_colour.2 = f32::sin(0.65*time as f32);



        let lighting_material = Material::Default;
        let lighting_material_qualities = lighting_material.get_material_qualities();

        render.programs.set_uniform(&render.window.opengl,"object_material.ambient_reflected_colour", UniformType::Vec3,
            Matrix::from_1darray(lighting_material_qualities.get_ambient()))?;
        render.programs.set_uniform(&render.window.opengl,"object_material.diffuse_reflected_colour", UniformType::Vec3,
            Matrix::from_1darray(lighting_material_qualities.get_diffuse()))?;
        render.programs.set_uniform(&render.window.opengl,"object_material.specular_reflected_colour", UniformType::Vec3,
            Matrix::from_1darray(lighting_material_qualities.get_specular()))?;
        render.programs.set_uniform(&render.window.opengl,"object_material.shininess", UniformType::Float,
            Matrix::from_scalar(lighting_material_qualities.get_shininess()))?;

            
//lightingShader.setVec3("material.ambient", 1.0f, 0.5f, 0.31f);
//lightingShader.setVec3("material.diffuse", 1.0f, 0.5f, 0.31f);
//lightingShader.setVec3("material.specular", 0.5f, 0.5f, 0.5f);
//lightingShader.setFloat("material.shininess", 32.0f);


        let with_light_source = WithObject::existing(&render.window.opengl, enums::Object::VAO, light_vao, DataFormat::Position3Colour3Alpha1)
                 .add(Object::VBO, light_vbo)?;
        let data = get_light_matrix(&render);
        with_light_source.buffer_sub_data(&data, Object::VBO)?;
        render.programs.draw(with_light_source, DrawCall::Arrays, DrawMode::GlPoints, &data)?;
        //drop(with_light_source);

        render.lighting.light_source_pos.0 += 0.005 * sin_t as f32 * cos_t as f32;
        render.lighting.light_source_pos.1 -= 0.015 * sin_t as f32 * cos_t as f32;
        render.lighting.light_source_pos.2 += 0.005 * sin_t as f32 * cos_t as f32;

        
        //render.lighting.light_source_pos.0 += 0.1 * sin_t as f32 * cos_t as f32;
        //render.lighting.light_source_pos.1 -= 0.15 * sin_t as f32 * cos_t as f32;
        //render.lighting.light_source_pos.2 += 0.15 * sin_t as f32 * cos_t as f32;


        //if count % 15 == 13 {
        //render.lighting.light_source_pos.0 += 1. * sin_t as f32 * cos_t as f32;
        //render.lighting.light_source_pos.1 -= 1.5 * sin_t as f32 * cos_t as f32;
        //render.lighting.light_source_pos.2 += 1.5 * sin_t as f32 * cos_t as f32;
        //}

        ////        
        
        
        render.use_program(ProgramSelect::SelectBlinnPhongOrthographic)?;


        let default_material = Material::Default;
        let custom_material = Material::Custom(
            MaterialLightQualities::assign(
                [1.0, 0.5, 0.31],
                [1.0, 0.5, 0.31],
                [0.5, 0.5,  0.5],
                32.,
            )
        );
        let black_rubber = Material::BlackRubber;
        let brass = Material::Brass;
        let gold = Material::Gold;
        let polished_gold = Material::PolishedGold;

        let material_qualities = default_material.get_material_qualities();
        let material_qualities = custom_material.get_material_qualities();
        //let material_qualities = black_rubber.get_material_qualities();
        //let material_qualities = brass.get_material_qualities();
        //let material_qualities = gold.get_material_qualities();
        //let material_qualities = polished_gold.get_material_qualities();

        render.programs.set_uniform(&render.window.opengl,"object_material.ambient_reflected_colour", UniformType::Vec3,
            Matrix::from_1darray(material_qualities.get_ambient()))?;
        render.programs.set_uniform(&render.window.opengl,"object_material.diffuse_reflected_colour", UniformType::Vec3,
            Matrix::from_1darray(material_qualities.get_diffuse()))?;
        render.programs.set_uniform(&render.window.opengl,"object_material.specular_reflected_colour", UniformType::Vec3,
            Matrix::from_1darray(material_qualities.get_specular()))?;
        render.programs.set_uniform(&render.window.opengl,"object_material.shininess", UniformType::Float,
            Matrix::from_scalar(material_qualities.get_shininess()))?;


    //    // // cube 1
    //    
         let with_cube = WithObject::existing(&render.window.opengl, enums::Object::VAO, c_vao, DataFormat::Position3Colour3Alpha1Normal3);
         render.programs.draw(with_cube, DrawCall::Arrays, DrawMode::GlTriangles, &cube)?;
    //    // 
    //    // // cube 2
    //    // let tr = Matrix::translate(Vector::from_1darray([-16.0, 0.0, 0.0]));
    //    // render.programs.set_uniform(&render.window.opengl, "world_transform", UniformType::Mat4,
    //    //     Matrix::opengl_to_right_handed().matmul(&tr).unwrap())?;
    //    // let with_cube = WithObject::existing(&render.window.opengl, enums::Object::VAO, c_vao, DataFormat::Position3Colour3Alpha1);
    //    // render.programs.draw(with_cube, DrawCall::Arrays, DrawMode::GlTriangles, &cube)?;
////
//
        render.end_render_actions()?;
    }

    Ok(())
}