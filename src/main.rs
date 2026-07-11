#![allow(warnings)]

// uncomment for release
// #![windows_subsystem = "windows"]


mod cube;
mod thing;
mod quadtree;
mod octree;
mod ntree;
//mod quadtreev2;


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
use std::fs::exists;
use std::io::Read;
use std::os::raw::c_void;
use std::{time::{SystemTime, UNIX_EPOCH, Duration}, thread};

use crate::image_processing::Image;
use crate::octree::{Octree};
use crate::ntree::{Point, PointThing};
use crate::quadtree::{QuadTree};
//use crate::quadtreev2::{QuadTree as QuadTreeV2, SquareBounds as SquareBoundsV2, Tree};
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


pub fn pseudo_randf64(min:f64, max:f64, microsecond_delay:u8) -> f64 {
    let num_zero_to_ten_thousand = (
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos()%10000
    ) as f64;
    let old_max = 10000.;
    let old_min = 0.;
    let old_range = (old_max - old_min);
    let new_range = (max - min);
    let rescaled_value = ( (num_zero_to_ten_thousand - old_min) / (old_range) ) * (new_range) + min;
    thread::sleep(Duration::from_micros(microsecond_delay as u64)); // to allow direct re-uses
    rescaled_value
}



fn main() -> Result<(), ContextError> {


    let mut points = vec![];
    for i in 0..299 {
        let x = pseudo_randf64(-2., 2., 100) as f32;
        let y = pseudo_randf64(-2., 2., 100) as f32;
        let z = pseudo_randf64(-2., 2., 100) as f32;
        points.push(PointThing::new(x, y, z));
    }

    let q1 = Octree::new_on_origin(4.);
    //let q1 = QuadTree::new_on_origin(4.);
    let q2 = q1.insert(points, 5).unwrap();
    

    let quadtree_final = q2;
    let qtl = quadtree_final.get_all_lines();
    let qtp = quadtree_final.get_all_points();
    let quadtree_lines  = Matrix { shape: vec![7, qtl.len()/7], array: qtl };
    let quadtree_points = Matrix { shape: vec![7, qtp.len()/7], array: qtp };






    let mut render = Context::default()?;
    render.setup_render();


    let (qtl_vao, qtl_vbo) = render.create_vao_vbo(&quadtree_lines, DataFormat::Position3Colour3Alpha1)?;
    let (qtp_vao, qtp_vbo) = render.create_vao_vbo(&quadtree_points, DataFormat::Position3Colour3Alpha1)?;

    
    let cube = cube::colour_cube((0.0, 0.0, 0.0), 5.0, true);
    let (c_vao, c_vbo) = render.create_vao_vbo(&cube, DataFormat::Position3Colour3Alpha1Normal3)?;


    let (vertices_matrix, indices_matrix) = cube::_might_not_even_work_untested_ebo_cube((0.0, 0.0, 0.0), 14.0);
    let (vao, vbo, ebo) = render.create_vao_vbo_ebo(&vertices_matrix, &indices_matrix, DataFormat::Position3Colour3Alpha1)?;
    
    
    let tex_cube = cube::texture_cube((0.0, 0.0, 0.0), 14.0, 1.0);
    let (tc_vao, tc_vbo) = render.create_vao_vbo(&tex_cube, DataFormat::Position3Texture2)?;
    
    let tex_col_cube = cube::texture_colour_cube((0.0, 0.0, 0.0), 6.0, 1.0);
    let (tcc_vao, tcc_vbo) = render.create_vao_vbo(&tex_col_cube, DataFormat::Position3Colour3Alpha1Normal3Texture2)?;

    let mut base = cube::texture_colour_cube((0.0, 0.0, 0.0), 6.0, 1.0);    
    let spawn_range = 25.;
    for i in 0..100 {
        let tx = pseudo_randf64(-spawn_range, spawn_range, 100) as f32;
        let ty = pseudo_randf64(-spawn_range, spawn_range, 100) as f32;
        let tz = pseudo_randf64(-spawn_range, spawn_range, 100) as f32;
        let rx = pseudo_randf64(-45., 45., 100) as f32;
        let ry = pseudo_randf64(-45., 45., 100) as f32;
        let rz = pseudo_randf64(-45., 45., 100) as f32;


        let mut tcc_cube_i = cube::texture_colour_cube((0., 0., 0.), 5., 1.);
        let rotate = Matrix::rotate(Vector::from_1darray([rx, ry, rz]))?;
        let translate = Matrix::translate(Vector::from_1darray([tx, ty, tz]));

        let pos = tcc_cube_i.get_submatrix([0..3, 0..36])?;
        let rest = tcc_cube_i.get_submatrix([3..12, 0..36])?;
        let pos4 = pos.expand_along_axis(Matrix { shape: vec![1, 36], array: [1.; 36].to_vec() }, 0)?;
        let np4 = pos4.matmul(&rotate)?;
        let np5 = np4.matmul(&translate.transpose()?)?;
        let npos = np5.get_submatrix([0..3, 0..36])?;
        let new = npos.expand_along_axis(rest, 0)?;

        base = base.expand_along_axis(new, 1)?;
    }
    let (tcc_vaos, tcc_vbos) = render.create_vao_vbo(&base, DataFormat::Position3Colour3Alpha1Normal3Texture2)?;



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
    let container_diffuse_map    = Image::decode_from_path("images/container_diffuse_map.png", ImageFormat::PNG, true);
    let container_specular_map   = Image::decode_from_path("images/container_specular_map.png", ImageFormat::PNG, true);



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
    let prepared_container_diffuse_map = TextureSetup::get_prepared(
        &render.window.opengl, TextureTarget::Texture2D,
        container_diffuse_map,
        TextureWrapping::Repeat, TextureWrapping::Repeat,
        TextureMinFilter::LinearMipmapNearest, TextureMagFilter::Linear,
        0);
    let prepared_container_specular_map = TextureSetup::get_prepared(
        &render.window.opengl, TextureTarget::Texture2D,
        container_specular_map,
        TextureWrapping::Repeat, TextureWrapping::Repeat,
        TextureMinFilter::LinearMipmapNearest, TextureMagFilter::Linear,
        0);



        //let vertex_text   = std::fs::read("src/two_texture_vertex.glsl").unwrap().iter().map(|a| *a as char).collect::<String>();
        //let fragment_text = std::fs::read("src/two_texture_fragment.glsl").unwrap().iter().map(|a| *a as char).collect::<String>();
        let vertex_text   = std::fs::read("../atmospheric/shaders_glsl/phong_texture_vertex.glsl").unwrap().iter().map(|a| *a as char).collect::<String>();
        let fragment_text = std::fs::read("../atmospheric/shaders_glsl/phong_texture_fragment.glsl").unwrap().iter().map(|a| *a as char).collect::<String>();
        let shader_id = render.compile_custom_program(vertex_text.as_str(), fragment_text.as_str())?;
                

        let mut sign = true;
        let mut time_last_changed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    while !render.render_over() {
        render.begin_render_actions()?;

        let time = render.window.get_time_since_glfw_init();
        let sin_t = time.sin();
        let cos_t = time.cos();
        

        
        // simple shader for light source
        render.use_program(ProgramSelect::SelectSimpleOrthographic)?;

        // // light source's diffuse colour changes over time
         render.lighting.light_diffuse_colour.0 = f32::sin(0.75*time as f32);
         render.lighting.light_diffuse_colour.1 = f32::sin(0.25*time as f32);
         render.lighting.light_diffuse_colour.2 = f32::sin(0.65*time as f32);

        // draw light source
        let with_light_source = WithObject::existing(&render.window.opengl, enums::Object::VAO, light_vao, DataFormat::Position3Colour3Alpha1)
                 .add(Object::VBO, light_vbo)?;
        let data = get_light_matrix(&render);
        with_light_source.buffer_sub_data(&data, Object::VBO)?;
        render.programs.draw(with_light_source, DrawCall::Arrays, DrawMode::GlPoints, &data)?;
        
        // // move light source
         //render.lighting.light_source_pos.0 += 0.005 * sin_t as f32 * cos_t as f32;
         //render.lighting.light_source_pos.1 -= 0.015 * sin_t as f32 * cos_t as f32;
         //render.lighting.light_source_pos.2 += 0.005 * sin_t as f32 * cos_t as f32;
         //render.lighting.light_source_pos.0 += 0.15 * sin_t as f32 * cos_t as f32;
         //render.lighting.light_source_pos.1 -= 0.15 * sin_t as f32 * cos_t as f32;
         //render.lighting.light_source_pos.2 += 0.15 * sin_t as f32 * cos_t as f32;
         //render.lighting.light_source_pos.0 += 0.05 * sin_t as f32;
         //render.lighting.light_source_pos.1 += 0.05 * sin_t as f32;
         render.lighting.light_source_pos.0 = 0.;
         render.lighting.light_source_pos.1 = 0.;
         let pz = render.lighting.light_source_pos.2;
         fn gettime() -> Duration {
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap()
         }
         let change = 0.005 * 2.*sin_t.abs() as f32;
         if ((pz > 100.) || (pz < 0.)) && (
            Duration::abs_diff(time_last_changed, gettime()).as_secs_f64() > 1.
        ) {
            sign = !sign;
            time_last_changed = gettime();
        }
         if sign {
            render.lighting.light_source_pos.2 += change;
         } else {
            render.lighting.light_source_pos.2 -= change;
         }

         //render.lighting.light_source_pos.2 = 5.5;
        
        


        // cube with diffuse and specular colour maps
        render.use_program(ProgramSelect::SelectPhongTexture);
        &render.textures.activate(
            &render.window.opengl, OpenglTexture::Texture0, &prepared_container_diffuse_map, &render.programs
        )?;
        &render.textures.activate(
            &render.window.opengl, OpenglTexture::Texture1, &prepared_container_specular_map, &render.programs
        )?;
        render.programs.set_uniform(&render.window.opengl,"object_material.shininess", UniformType::Float,
        //Matrix::from_scalar(16.0))?;
        Matrix::from_scalar(56.0))?;
        //let with_relevant = WithObject::existing(&render.window.opengl, enums::Object::VAO, tcc_vao, DataFormat::Position3Colour3Alpha1Normal3Texture2);
        //render.programs.draw(with_relevant, DrawCall::Arrays, DrawMode::GlTriangles, &tex_col_cube)?;
        let with_relevant = WithObject::existing(&render.window.opengl, enums::Object::VAO, tcc_vaos, DataFormat::Position3Colour3Alpha1Normal3Texture2);
        render.programs.draw(with_relevant, DrawCall::Arrays, DrawMode::GlTriangles, &base)?;
        &render.textures.deactivate_all(&render.window.opengl);






         //// cube with textures on all sides testing
         //render.use_program(ProgramSelect::SelectSimpleTexture);
         //&render.textures.activate(
         //    &render.window.opengl, OpenglTexture::Texture0, &prepared_container, &render.programs
         //)?;
         //// panic!();
         //// /// fix this to use WithObject.draw() by passing render.programs to it
         //// /// that way you just call draw on the object itself
         //// /// and then just move the one DataFormat check into WithObject
         //// panic!();
         //let with_relevant = WithObject::existing(&render.window.opengl, enums::Object::VAO, tc_vao, DataFormat::Position3Texture2);
         //render.programs.draw(with_relevant, DrawCall::Arrays, DrawMode::GlTriangles, &tex_cube)?;


        //// mixing two textures testing
        //render.use_custom_program(shader_id);
        //render.set_orthographic_camera_uniforms()?;       
        //&render.textures.activate(
        //    &render.window.opengl, OpenglTexture::Texture0, &prepared_bluefaces, &render.programs
        //)?;
        //&render.textures.activate(
        //    &render.window.opengl, OpenglTexture::Texture1, &prepared_ppm, &render.programs
        //)?;
        //&render.textures.activate(
        //    &render.window.opengl, OpenglTexture::Texture2, &prepared_awesomeface, &render.programs
        //)?;
        //let with_relevant = WithObject::existing(&render.window.opengl, enums::Object::VAO, tex_vao, DataFormat::Position3Texture2)
        //                                                .add(enums::Object::EBO, tex_ebo)?;
        //                                                //.add(opengl::enums::Object::Texture2D, texture_id)?;
        //render.programs.draw(with_relevant, DrawCall::Elements, DrawMode::GlTriangles, &triangle_indices)?;





        // quadtree and octree testing
        //render.use_program(ProgramSelect::SelectSimpleOrthographic)?;
        //let with_qtl = WithObject::existing(&render.window.opengl, enums::Object::VAO, qtl_vao, DataFormat::Position3Colour3Alpha1);
        //render.programs.draw(with_qtl, DrawCall::Arrays, DrawMode::GlLines, &quadtree_lines)?;
        //let with_qtp = WithObject::existing(&render.window.opengl, enums::Object::VAO, qtp_vao, DataFormat::Position3Colour3Alpha1);
        //render.programs.draw(with_qtp, DrawCall::Arrays, DrawMode::GlPoints, &quadtree_points)?;











        ///// LIGHTING TESTING
        ///// LIGHTING TESTING
        ///// LIGHTING TESTING
        ///// LIGHTING TESTING
        ///// LIGHTING TESTING
        ///// LIGHTING TESTING
        ///// 
//
        //// simple shader for light source
        //render.use_program(ProgramSelect::SelectSimpleOrthographic)?;
//
        //// light source's diffuse colour changes over time
        //render.lighting.light_diffuse_colour.0 = f32::sin(0.75*time as f32);
        //render.lighting.light_diffuse_colour.1 = f32::sin(0.25*time as f32);
        //render.lighting.light_diffuse_colour.2 = f32::sin(0.65*time as f32);
//
        //// draw light source
        //let with_light_source = WithObject::existing(&render.window.opengl, enums::Object::VAO, light_vao, DataFormat::Position3Colour3Alpha1)
        //         .add(Object::VBO, light_vbo)?;
        //let data = get_light_matrix(&render);
        //with_light_source.buffer_sub_data(&data, Object::VBO)?;
        //render.programs.draw(with_light_source, DrawCall::Arrays, DrawMode::GlPoints, &data)?;
        //
        //// move light source
        //render.lighting.light_source_pos.0 += 0.005 * sin_t as f32 * cos_t as f32;
        //render.lighting.light_source_pos.1 -= 0.015 * sin_t as f32 * cos_t as f32;
        //render.lighting.light_source_pos.2 += 0.005 * sin_t as f32 * cos_t as f32;
        //
        //
//        // 
////
//        //// phong shader for objects hit by light 
 //       render.use_program(ProgramSelect::SelectBlinnPhongOrthographic)?;
//
//        // get Materials
//        let default_material = Material::Default;
//        let custom_material = Material::Custom(
//            MaterialLightQualities::assign(
//                [1.0, 0.5, 0.31],
//                [1.0, 0.5, 0.31],
//                [0.5, 0.5,  0.5],
//                32.,
//            )
//        );
//        let black_rubber = Material::BlackRubber;
  //      let brass = Material::Brass;
//        let gold = Material::Gold;
//        let polished_gold = Material::PolishedGold;
//
//        // get material qualities to use
//        //let material_qualities = default_material.get_material_qualities();
//        //let material_qualities = custom_material.get_material_qualities();
//        //let material_qualities = black_rubber.get_material_qualities();
   //     let material_qualities = brass.get_material_qualities();
//        //let material_qualities = gold.get_material_qualities();
//        //let material_qualities = polished_gold.get_material_qualities();
//
//        // set material qualities uniforms
   //     render.programs.set_uniform(&render.window.opengl,"object_material.ambient_reflected_colour", UniformType::Vec3,
   //         Matrix::from_1darray(material_qualities.get_ambient()))?;
   //     render.programs.set_uniform(&render.window.opengl,"object_material.diffuse_reflected_colour", UniformType::Vec3,
   //         Matrix::from_1darray(material_qualities.get_diffuse()))?;
   //     render.programs.set_uniform(&render.window.opengl,"object_material.specular_reflected_colour", UniformType::Vec3,
   //         Matrix::from_1darray(material_qualities.get_specular()))?;
   //     render.programs.set_uniform(&render.window.opengl,"object_material.shininess", UniformType::Float,
   //         Matrix::from_scalar(material_qualities.get_shininess()))?;
//
//        // draw object
  //      let with_cube = WithObject::existing(&render.window.opengl, enums::Object::VAO, c_vao, DataFormat::Position3Colour3Alpha1Normal3);
  //      render.programs.draw(with_cube, DrawCall::Arrays, DrawMode::GlTriangles, &cube)?;
//



        render.end_render_actions()?;
    }

    Ok(())
}