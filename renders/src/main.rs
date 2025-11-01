#![allow(warnings)]

// uncomment for release
// #![windows_subsystem = "windows"]


mod cube;


use opengl::high_level_abstractions::{self, WithObject};
use render_context::errors::RenderError;
use render_context::render::Render;
use render_context::enums::{GlError, ProgramSelect, DrawMode};
use numeracy::matrices::matrix::Matrix;

//use ppm_viewer;




fn main() -> Result<(), RenderError> {

    let cube = cube::cube((0.0, 0.0, 0.0), 8.0);

    let mut cubes = vec![];
    for i in cube {
        cubes.push((i, 0, 0));
    }

    let (vertices_matrix, indices_matrix) = cube::ebo_cube((0.0, 0.0, 0.0), 8.0);



    let mut render = Render::default()?;
    render.setup_render();


    let (vao, vbo, ebo) = render.create_vao_vbo_ebo(&vertices_matrix, &indices_matrix)?;
    println!("{}, {}, {}", vao, vbo, ebo);

    //for (i, j) in cubes.clone().into_iter().enumerate() {
    //    let (c_vao, c_vbo) = render.create_vao_vbo(&j.0)?;
    //    cubes[i] = (j.0, c_vao, c_vbo);
    //}
    //let (c_vao, c_vbo) = render.create_vao_vbo(&cube)?;

    while !render.render_over() {
        render.begin_render_actions();


        
        render.use_program(ProgramSelect::SelectSimpleOrthographic);

        let x:usize = indices_matrix.shape.iter().product();
        //render.draw_vao_ebo(DrawMode::GlTriangles, vao, &vertices_matrix);
        render.draw_vao_ebo(DrawMode::GlTriangles, vao, x as i32);
        //let with_vao = high_level_abstractions::WithObject::existing(&render.window.opengl, opengl::enums::Object::VAO, vao);
        //with_vao.draw(DrawMode::GlTriangles, &indices_matrix);
        //for c in cubes.clone() {
        //    render.draw_vao(DrawMode::GlTriangles, c.1, &c.0);
        //}
        //render.draw_vao(DrawMode::GlTriangles, c_vao, &cube);



        //drop(with_vao);
        //drop(with_program);
        render.end_render_actions();
    }

    Ok(())
}