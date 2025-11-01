#![allow(warnings)]

// uncomment for release
// #![windows_subsystem = "windows"]


mod cube;


use opengl::high_level_abstractions::{self, WithObject};
use render_context::errors::RenderError;
use render_context::render::Render;
use render_context::enums::{GlError, ProgramSelect, DrawMode, DrawCall};
use numeracy::matrices::matrix::Matrix;

//use ppm_viewer;




fn main() -> Result<(), RenderError> {

    let cube = cube::cube((0.0, 0.0, 0.0), 8.0);

    let (vertices_matrix, indices_matrix) = cube::ebo_cube((0.0, 0.0, 0.0), 8.0);



    let mut render = Render::default()?;
    render.setup_render();


    let (vao, vbo, ebo) = render.create_vao_vbo_ebo(&vertices_matrix, &indices_matrix)?;

    let (c_vao, c_vbo) = render.create_vao_vbo(&cube)?;

    while !render.render_over() {
        render.begin_render_actions();


        
        render.use_program(ProgramSelect::SelectSimpleOrthographic);

        let x:usize = indices_matrix.shape.iter().product();
        //render.draw(DrawCall::Elements, DrawMode::GlTriangles, vao, &indices_matrix);
        render.draw(DrawCall::Arrays, DrawMode::GlTriangles, c_vao, &cube);



        //drop(with_vao);
        //drop(with_program);
        render.end_render_actions();
    }

    Ok(())
}