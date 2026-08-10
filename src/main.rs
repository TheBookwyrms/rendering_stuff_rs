#![allow(warnings)]

// uncomment for release
// #![windows_subsystem = "windows"]


mod cube;
mod quadtree;
mod octree;
mod ntree;
//mod quadtreev2;


use atmospheric::config::RenderInitialConfig;
use atmospheric::enums::CameraMode;
use atmospheric::enums::Object;
use atmospheric::image_processing;
use atmospheric::enums;
use atmospheric::lighting::LightCounter;
use atmospheric::lighting::LightingGenerator;
use atmospheric::lighting::{PointLight, DirectionalLight, SpotLight};
use atmospheric::materials::MaterialLightQualities;
use atmospheric::opengl;
//use atmospheric::opengl::abstractions::{self, TextureSetup, Textures, WithObject};
//use atmospheric::opengl::abstractions::{self, TextureSetup, Textures, WithObject};
use atmospheric::opengl::abstractions::{self, TextureSetup, Textures, WithVao, WithVbo, WithEbo, WithVaoVbo, WithVaoEbo};
use atmospheric::enums::{InternalFormat, TextureMagFilter, TextureMinFilter, TextureWrapping, CameraAxis, CameraVector};
use atmospheric::opengl::{gl, intermediate_opengl, raw_opengl};
use atmospheric::enums::ContextError;
use atmospheric::enums::ImageFormat;
use atmospheric::context::Context;
use atmospheric::enums::{DataFormat, DrawCall, DrawMode, GlError, OpenglTexture, ProgramSelect, TextureTarget, UniformType, LightForm};
//use numeracy::matrices::Matrix;
use numeracy::matrices::Matrix;
use numeracy::matrices::S1;
use numeracy::matrices::S2;
use numeracy::vectors::Vector;

mod object;

use core::panic;
use std::f32::consts::PI;
use std::ffi::{CStr, CString};
use std::fs::exists;
use std::io::Read;
use std::os::raw::c_void;
use std::{time::{SystemTime, UNIX_EPOCH, Duration}, thread};

use crate::image_processing::Image;
use crate::object::InstancingTestObject;
use crate::object::ObjectColour;
use crate::object::ObjectForVaoDraws;
use crate::object::ObjectMaterials;
use crate::object::ObjectTexture;
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


static intial_config : RenderInitialConfig = RenderInitialConfig {
    window_name   : "hello, window!",
    window_height : 1080,
    window_width  : 1920,
    camera_mode   : CameraMode::Encompassing,
    max_lights    : LightCounter::max_values(1, 18, 1),
};


fn main() -> Result<(), ContextError> {


    let mut lighting_generator = LightingGenerator::init(&intial_config.max_lights);

    let mut render = Context::default(intial_config)?;
    render.setup_render();



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
    let quadtree_lines : Matrix<f32, 1, numeracy::matrices::S1<1>>  = Matrix::from_vec(qtl);
    let quadtree_points: Matrix<f32, 1, numeracy::matrices::S1<1>>  = Matrix::from_vec(qtp);
    //let quadtree_lines  = Matrix { shape: [7, qtl.len()/7], array: qtl };
    //let quadtree_points = Matrix { shape: [7, qtp.len()/7], array: qtp };

    let (qtl_vao, qtl_vbo) = render.create_vao_vbo(&quadtree_lines, DataFormat::Position3Colour3Alpha1);//?;
    let (qtp_vao, qtp_vbo) = render.create_vao_vbo(&quadtree_points, DataFormat::Position3Colour3Alpha1);//?;


    
    let mut point_light = lighting_generator.generate_point_light(&render, [0., 0., 10.], [0.75, 0.95, 0.65])?;
    let mut point_light2 = lighting_generator.generate_point_light(&render, [-10., 0., 0.], [0.75, 0.95, 0.65])?;
    let mut point_light3 = lighting_generator.generate_point_light(&render, [0., -10., 0.], [0.75, 0.95, 0.65])?;
    //let mut dir_light = lighting_generator.generate_directional_light([0., 0., 1.], [0.75, 0.95, 0.65])?;
    let mut dir_light = lighting_generator.generate_directional_light([0., 0., 1.], [1.;3])?;
    let mut spot_light = lighting_generator.generate_spot_light([0., 0., 10.], [0., 0., -1.], [0.75, 0.95, 0.65], PI/6., PI/3.)?;

    //let mut point_light = PointLight::new([0., 0., 10.], [0.75, 0.95, 0.65]);
    //let mut dir_light = DirectionalLight::new();
    //let mut spot_light = SpotLight::new();

    //let mut lighting_manager = LightingManager::instantiate(
    //    &render, Vector::from_scalar(point_light),
    //    Vector::from_scalar(dir_light), Vector::from_scalar(spot_light)
    //)?;


    
    let cube = cube::colour_cube((0.0, 0.0, 0.0), 5.0, true);
    let (c_vao, c_vbo) = render.create_vao_vbo(&cube, DataFormat::Position3Colour3Alpha1Normal3);//?;


    //let (vertices_matrix, indices_matrix) = cube::_might_not_even_work_untested_ebo_cube((0.0, 0.0, 0.0), 14.0);
    //let (vao, vbo, ebo) = render.create_vao_vbo_ebo(&vertices_matrix, &indices_matrix, DataFormat::Position3Colour3Alpha1)?;
    

    let tex_cube = cube::texture_cube((0.0, 0.0, 0.0), 14.0, 1.0);
    let (tc_vao, tc_vbo) = render.create_vao_vbo(&tex_cube, DataFormat::Position3Texture2);//?;
    
    let tex_col_cube = cube::texture_colour_cube((0.0, 0.0, 0.0), 6.0, 1.0);
    let (tcc_vao, tcc_vbo) = render.create_vao_vbo(&tex_col_cube, DataFormat::Position3Colour3Alpha1Normal3Texture2);//?;


    let (cc_pos, cc_col) = cube::colour_cube_new((0., 0., 0.), 6.);
    let (ct_pos, ct_tex) = cube::texture_cube_new((0., 0., 0.), 6., 1.);
    let (ccn_pos, ccn_col, ccn_norm) = cube::colour_normal_cube_new((0., 0., 0.), 6.);
    let (ccnt_pos, ccnt_col, ccnt_norm, ccnt_tex) = cube::texture_colour_cube_new((0., 0., 0.), 6., 1.);















    let base_original = cube::texture_colour_cube((0.0, 0.0, 0.0), 6.0, 1.0);    
    let mut base = cube::texture_colour_cube((0.0, 0.0, 0.0), 6.0, 1.0);    
    const NUM_INSTANCES:usize = 100;
    let spawn_range = 25.;

    let mut all_x = [0.; NUM_INSTANCES];
    let mut all_y = [0.; NUM_INSTANCES];
    let mut all_z = [0.; NUM_INSTANCES];
    
    let bases = vec![cube::texture_colour_cube((0.0, 0.0, 0.0), 6.0, 1.0); NUM_INSTANCES];
    let mut transformation_matrices = vec![];
    for i in 0..NUM_INSTANCES {
        let tx = pseudo_randf64(-spawn_range, spawn_range, 100) as f32;
        let ty = pseudo_randf64(-spawn_range, spawn_range, 100) as f32;
        let tz = pseudo_randf64(-spawn_range, spawn_range, 100) as f32;
        let rx = pseudo_randf64(-45., 45., 100) as f32;
        let ry = pseudo_randf64(-45., 45., 100) as f32;
        let rz = pseudo_randf64(-45., 45., 100) as f32;

        all_x[i] = tx;
        all_y[i] = ty;
        all_z[i] = tz;

        let rotate = Matrix::rotate(Vector::from_1darray([rx, ry, rz]));
        let translate = Matrix::translate(Vector::from_1darray([tx, ty, tz]));
        let translate_transposed = translate.transpose();
        let rotate_transposed = rotate.clone().transpose();


        //let t_r = translate.matmul(&rotate)?; // NOPE
        //let t_r = rotate.matmul(&translate)?; // NOPE
        let t_r = translate_transposed.matmul(&rotate);
        let t_r = rotate.matmul(&translate_transposed);
        //let t_r = translate.matmul(&rotate_transposed)?; // NOPE
        //let t_r = rotate_transposed.matmul(&translate)?; // NOPE
        let t_r = translate_transposed.matmul(&rotate_transposed);
        let t_r = rotate_transposed.matmul(&translate_transposed);

        //let t_r = translate_transposed;
        
        //let t_r = translate_transposed.matmul(&rotate)?;
        //let t_r = rotate.matmul(&translate_transposed)?;
        //let t_r = rotate.matmul(&translate)?;


        transformation_matrices.push(t_r);
    }

    //let mut low_x = 0;
    //let mut mid_x = 0;
    //let mut high_x = 0;
    //let mut low_y = 0;
    //let mut mid_y = 0;
    //let mut high_y = 0;
    //let mut low_z = 0;
    //let mut mid_z = 0;
    //let mut high_z = 0;
//
    //all_x.map(|p| {if p <= -12.5 {low_x+=1} else if p >= 12.5 {high_x+=1} else {mid_x+=1}});
    //all_y.map(|p| {if p <= -12.5 {low_y+=1} else if p >= 12.5 {high_y+=1} else {mid_y+=1}});
    //all_z.map(|p| {if p <= -12.5 {low_z+=1} else if p >= 12.5 {high_z+=1} else {mid_z+=1}});
//
    //println!("low x {}, mid x {}, high x {}", low_x, mid_x, high_x);
    //println!("low y {}, mid y {}, high y {}", low_y, mid_y, high_y);
    //println!("low z {}, mid z {}, high z {}", low_z, mid_z, high_z);


    //for i in 0..NUM_INSTANCES {
//
    //    let pos  = bases[i].get_submatrix([0..3, 0..36])?;
    //    let rest = bases[i].get_submatrix([3..12, 0..36])?;
    //    let pos4 = pos.expand_horizontally(Matrix { shape: [1, 36], array: [1.; 36].to_vec() })?;
    //    let translated_rotated = pos4.matmul(&transformation_matrices[i])?;
    //    let new_pos = translated_rotated.get_submatrix([0..3, 0..36])?;
    //    let new_full = new_pos.expand_horizontally(rest)?;
//
    //    //base = base.expand_vertically(new_full)?;
    //}
    //let (tcc_vaos, tcc_vbos) = render.create_vao_vbo(&base, DataFormat::Position3Colour3Alpha1Normal3Texture2)?;



    let triangle = Matrix::from_2darray([
        [-5.0,  0.0, 0.0, 0.9, 0.5, 0.1, 1.0, 1.0, 0.0, 0.0],
        [ 0.0, 5.0, 0.0, 0.1, 0.9, 0.5, 1.0, 1.0, 0.0, 0.0],
        [ 5.0, 0.0, 0.0, 0.5, 0.1, 0.9, 1.0, 1.0, 0.0, 0.0],
    ]);
    let (t_vao, t_vbo) = render.create_vao_vbo(&triangle, DataFormat::Position3Colour3Alpha1Normal3);//?;


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
    let (tex_vao, tex_vbo, tex_ebo) = render.create_vao_vbo_ebo(&texture_triangle, &triangle_indices, DataFormat::Position3Texture2);//?;


    let zero = Matrix::from_2darray([[0., 0., 0., 0., 0., 0., 1.]]);
    let x    = Matrix::from_2darray([[5., 0., 0., 1., 0., 0., 1.]]);
    let y    = Matrix::from_2darray([[0., 5., 0., 0., 1., 0., 1.]]);
    let z    = Matrix::from_2darray([[0., 0., 5., 0., 0., 1., 1.]]);
    let (zero_vao, zero_vbo) = render.create_vao_vbo(&zero, DataFormat::Position3Colour3Alpha1);//?;
    let (x_vao, z_vbo) = render.create_vao_vbo(&x, DataFormat::Position3Colour3Alpha1);//?;
    let (y_vao, z_vbo) = render.create_vao_vbo(&y, DataFormat::Position3Colour3Alpha1);//?;
    let (z_vao, z_vbo) = render.create_vao_vbo(&z, DataFormat::Position3Colour3Alpha1);//?;
    

    fn target_to_matrix(target:Vector<f32, 3>, col:(f32, f32, f32), a:f32) -> Matrix<f32, 2, S2<7, 1>> {
        let (r, g, b) = col;
        let mut new_arr = target.multiply_by_constant(1.0).array;
        new_arr.extend(&[r, g, b, a]);
        Matrix { shape: S2::<7, 1>, array: new_arr }
    }
    let (target_vao, target_vbo) = render.create_vao_vbo(
        &target_to_matrix(
            render.camera.camera_info_matrix.get_camera(CameraVector::Target),
            (1.0, 1.0, 1.0),
            1.0),
        DataFormat::Position3Colour3Alpha1
    );//?;

    let (light_vao, light_vbo) = render.create_vao_vbo(&point_light.get_vertex_data(), DataFormat::Position3Colour3Alpha1);//?;

    

    let awesomeface = Image::decode_from_path("images/awesomeface.png", ImageFormat::PNG, true);
    let bluefaces   = Image::decode_from_path("images/bluefaces.png", ImageFormat::PNG, true);
    let container   = Image::decode_from_path("images/container.jpg", ImageFormat::JPEG, true);
    let wall        = Image::decode_from_path("images/wall.jpg", ImageFormat::JPEG, true);
    let ppm         = Image::decode_from_path("ray_tracer/test.txt", ImageFormat::PPMP3, true);
    let container_diffuse_map    = Image::decode_from_path("images/container_diffuse_map.png", ImageFormat::PNG, true);
    let container_specular_map   = Image::decode_from_path("images/container_specular_map.png", ImageFormat::PNG, true);



    let prepared_bluefaces = TextureSetup::get_prepared_default(
        &render.window.opengl, bluefaces
    );

    let prepared_ppm = TextureSetup::get_prepared_default(
        &render.window.opengl, ppm
    );

    let prepared_awesomeface = TextureSetup::get_prepared_default(
        &render.window.opengl, awesomeface
    );

    let prepared_container = TextureSetup::get_prepared_default(
        &render.window.opengl, container
    );
    let prepared_container_diffuse_map = TextureSetup::get_prepared_default(
        &render.window.opengl, container_diffuse_map.clone()
    );
    let prepared_container_specular_map = TextureSetup::get_prepared_default(
        &render.window.opengl, container_specular_map.clone()
    );




    //let vertex_text   = std::fs::read("src/two_texture_vertex.glsl").unwrap().iter().map(|a| *a as char).collect::<String>();
    //let fragment_text = std::fs::read("src/two_texture_fragment.glsl").unwrap().iter().map(|a| *a as char).collect::<String>();
    //let vertex_text   = std::fs::read("../atmospheric/shaders_glsl/phong_texture_vertex.glsl").unwrap().iter().map(|a| *a as char).collect::<String>();
    //let fragment_text = std::fs::read("../atmospheric/shaders_glsl/phong_texture_fragment.glsl").unwrap().iter().map(|a| *a as char).collect::<String>();
        let dir_max   = &intial_config.max_lights.get_light_count(enums::LightSourceForm::Directional);
        let point_max = &intial_config.max_lights.get_light_count(enums::LightSourceForm::Point);
        let spot_max  = &intial_config.max_lights.get_light_count(enums::LightSourceForm::Spot);

    let vertex_text   = std::fs::read("src/instancing_phong_texture_vertex.glsl").unwrap().iter().map(|a| *a as char).collect::<String>();
    let fragment_text = std::fs::read("src/instancing_phong_texture_fragment.glsl").unwrap().iter().map(|a| *a as char).collect::<String>()
                .replace("find_and_replace_with_max_number_of_point_lights", &point_max.to_string())
                .replace("find_and_replace_with_max_number_of_directional_lights", &dir_max.to_string())
                .replace("find_and_replace_with_max_number_of_spot_lights", &spot_max.to_string());;
    let shader_id = render.compile_custom_program(vertex_text.as_str(), fragment_text.as_str())?;
        
    let vertex_text   = std::fs::read("src/instancing_full_vertex.glsl").unwrap().iter().map(|a| *a as char).collect::<String>();
    let fragment_text = std::fs::read("src/instancing_full_fragment.glsl").unwrap().iter().map(|a| *a as char).collect::<String>()
                .replace("find_and_replace_with_max_number_of_point_lights", &point_max.to_string())
                .replace("find_and_replace_with_max_number_of_directional_lights", &dir_max.to_string())
                .replace("find_and_replace_with_max_number_of_spot_lights", &spot_max.to_string());;
    let instancing_shader_id = render.compile_custom_program(vertex_text.as_str(), fragment_text.as_str())?;
        
        //panic!();
    
    let testing_instancing_object = InstancingTestObject::new(
        &render.window.opengl, base_original, transformation_matrices.clone()
    );//?;



    let real_instancing_object = ObjectForVaoDraws::new(
        &render.window.opengl,
        ccnt_pos,
        ccnt_norm,
        //ObjectColour::None,
        ObjectColour::PerVertex(ccnt_col),
        //ObjectTextureCoords::PerVertex(ccnt_tex),
        //ObjectMaterials::None,
        ObjectMaterials::Constant(Material::Emerald),
        //ObjectTexture::None,
        ObjectTexture::PerVertex(container_diffuse_map, container_specular_map, ccnt_tex),
        // //Some(container_diffuse_map),
        // None,
        // //Some(container_specular_map),
        // None,
        transformation_matrices
    );




    let mut sign = true;
    let mut time_last_changed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    

    let mut time_prior = render.window.get_time_since_glfw_init() as f32;
    
    while !render.render_over() {

        render.begin_render_actions()?;

        let time = render.window.get_time_since_glfw_init() as f32;
        let sin_t = time.sin();
        let cos_t = time.cos();

        let delta_time = time-time_prior;
        time_prior = time;
        
        //lighting_manager.per_loop_rule(light_loop_rule);

        
        // simple shader for light source
        render.use_program(ProgramSelect::SelectSimpleOrthographic)?;

        // // light source's diffuse colour changes over time
        point_light.set_light(LightForm::Diffuse, [
            f32::sin(0.75*time), f32::sin(0.25*time), f32::sin(0.65*time),
        ]);

        point_light2.set_light(LightForm::Diffuse, [
            f32::sin(2.*0.75*time), f32::sin(2.*0.25*time), f32::sin(2.*0.65*time),
        ]);
        point_light2.set_position([10.*f32::sin(time), 10.*f32::cos(time), 10.]);

        point_light3.set_light(LightForm::Diffuse, [
            f32::sin(2.*0.65*time), f32::sin(2.*0.75*time), f32::sin(2.*0.25*time),
        ]);
        point_light3.set_position([-10.*f32::cos(time), -10.*f32::sin(time), 10.]);

        //point_light.draw(&render)?;
        point_light2.draw(&render)?;
        point_light3.draw(&render)?;
        //lighting_manager.draw_point_lights(&render)?;

        spot_light.rotate([0., -0.75, 0.])?;

        //// draw light source
        //let with_light_source = WithObject::existing(&render.window.opengl, enums::Object::VAO, light_vao, DataFormat::Position3Colour3Alpha1)
        //         .add(Object::VBO, light_vbo)?;
        //let data = point_light.get_vertex_data();
        //with_light_source.buffer_sub_data(&data, Object::VBO)?;
        //render.programs.draw(with_light_source, DrawCall::Arrays, DrawMode::GlPoints, &data)?;
        
        // // move light source
        let pz = point_light.get_position()[2];
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
            point_light.translate([0., 0., change]);
         } else {
            point_light.translate([0., 0., -change]);
         }

        
        


        // // cube with diffuse and specular colour maps
        // render.use_program(ProgramSelect::SelectPhongTexture);
        // &render.textures.activate(
        //     &render.window.opengl, OpenglTexture::Texture0, &prepared_container_diffuse_map, &render.programs
        // )?;
        // &render.textures.activate(
        //     &render.window.opengl, OpenglTexture::Texture1, &prepared_container_specular_map, &render.programs
        // )?;
        // render.programs.set_uniform::<f32>(&render.window.opengl,"object_shininess", UniformType::Float,
        // //Matrix::from_scalar(16.0))?;
        // Matrix::from_2darray([[56.0]]))?;
//
        // ////render.programs.set_uniform::<i32>(&render.window.opengl,"test", UniformType::Int,
        // ////Matrix::from_scalar(2))?;
        // //render.programs.set_uniform::<f32>(&render.window.opengl,"hi[0]", UniformType::Int,
        // //Matrix::from_scalar(0.))?;
        // //render.programs.set_uniform::<f32>(&render.window.opengl,"hi[1]", UniformType::Int,
        // //Matrix::from_scalar(1.))?;
//
        // //point_light.set_lighting_uniforms(&render)?;
        // //point_light2.set_lighting_uniforms(&render)?;
        // //point_light3.set_lighting_uniforms(&render)?;
        // //dir_light.set_lighting_uniforms(&render)?;
        // spot_light.set_lighting_uniforms(&render)?;
        // //let with_relevant = WithObject::existing(&render.window.opengl, enums::Object::VAO, tcc_vao, DataFormat::Position3Colour3Alpha1Normal3Texture2);
        // //render.programs.draw(with_relevant, DrawCall::Arrays, DrawMode::GlTriangles, &tex_col_cube)?;
        // let with_relevant = WithObject::existing(&render.window.opengl, enums::Object::VAO, tcc_vaos, DataFormat::Position3Colour3Alpha1Normal3Texture2);
        // render.programs.draw(with_relevant, DrawCall::Arrays, DrawMode::GlTriangles, &base)?;
        // &render.textures.deactivate_all(&render.window.opengl);







        if true {
            render.use_custom_program(instancing_shader_id);

            render.set_orthographic_camera_uniforms()?;
            render.set_blinn_phong_uniforms()?;
            
            //point_light.set_lighting_uniforms(&render)?;
            //point_light2.set_lighting_uniforms(&render)?;
            //point_light3.set_lighting_uniforms(&render)?;
            //spot_light.set_lighting_uniforms(&render)?;
            dir_light.set_lighting_uniforms(&render)?;

            real_instancing_object.draw(&render.window.opengl, &mut render.textures, &render.programs)?;

        } else {
            // cube with diffuse and specular colour maps
            render.use_custom_program(shader_id);
            &render.textures.activate(
                &render.window.opengl, OpenglTexture::Texture0, &prepared_container_diffuse_map, &render.programs
            )?;
            &render.textures.activate(
                &render.window.opengl, OpenglTexture::Texture1, &prepared_container_specular_map, &render.programs
            )?;
            render.programs.set_uniform::<f32, 1, S1<1>>(
                &render.window.opengl,
                "object_shininess",
                UniformType::Float,
                Matrix::from_scalar(56.0)
            )?;



            render.set_orthographic_camera_uniforms()?;
            render.set_blinn_phong_uniforms()?;

            //point_light.set_lighting_uniforms(&render)?;
            //point_light2.set_lighting_uniforms(&render)?;
            //point_light3.set_lighting_uniforms(&render)?;
            dir_light.set_lighting_uniforms(&render)?;
            //spot_light.set_lighting_uniforms(&render)?;
            //let with_relevant = WithObject::existing(&render.window.opengl, enums::Object::VAO, tcc_vao, DataFormat::Position3Colour3Alpha1Normal3Texture2);
            //render.programs.draw(with_relevant, DrawCall::Arrays, DrawMode::GlTriangles, &tex_col_cube)?;
            testing_instancing_object.draw(&render.window.opengl);//?;
            //let with_relevant = WithObject::existing(&render.window.opengl, enums::Object::VAO, tcc_vaos, DataFormat::Position3Colour3Alpha1Normal3Texture2);
            //render.programs.draw(with_relevant, DrawCall::Arrays, DrawMode::GlTriangles, &base)?;
            &render.textures.deactivate_all(&render.window.opengl);
        }














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
        //point_light.light_diffuse_colour.0 = f32::sin(0.75*time as f32);
        //point_light.light_diffuse_colour.1 = f32::sin(0.25*time as f32);
        //point_light.light_diffuse_colour.2 = f32::sin(0.65*time as f32);
//
        //// draw light source
        //let with_light_source = WithObject::existing(&render.window.opengl, enums::Object::VAO, light_vao, DataFormat::Position3Colour3Alpha1)
        //         .add(Object::VBO, light_vbo)?;
        //let data = get_light_matrix(&render);
        //with_light_source.buffer_sub_data(&data, Object::VBO)?;
        //render.programs.draw(with_light_source, DrawCall::Arrays, DrawMode::GlPoints, &data)?;
        //
        //// move light source
        //point_light.light_source_pos.0 += 0.005 * sin_t as f32 * cos_t as f32;
        //point_light.light_source_pos.1 -= 0.015 * sin_t as f32 * cos_t as f32;
        //point_light.light_source_pos.2 += 0.005 * sin_t as f32 * cos_t as f32;
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