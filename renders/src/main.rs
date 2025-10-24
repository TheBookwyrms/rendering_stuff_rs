#![allow(warnings)]

// uncomment for release
// #![windows_subsystem = "windows"]



use render_context::errors::RenderError;
use render_context::render::Render;
use render_context::enums::{GlError, ProgramSelect, DrawMode};
use matrices::matrix::Matrix;

//use ppm_viewer;




fn error(msg:String) {
    let a = true;
    let _b = match a {
        true =>Err(msg),
        false =>Ok(msg),
    }.unwrap();
}


fn main() -> Result<(), RenderError> {

    //error("halt".to_string());



    //println!("a");
    //ppm_viewer::view_ppm_from_path("C://Users//black//Documents//code_and_related_stuff//my_code//assorted_rust//rendering_stuff_rs//lib//ppm_viewer//src//test.ppm");
    //println!("b");
    ////_ray_tracer::hello_ppm();
    //let a = true;
    //let a = match a {
    //    true =>Err("a"),
    //    false =>Ok("a"),
    //}.unwrap();
    

    let mut render = Render::default()?;
    render.setup_render();


    //let triangle = Matrix2d::from([
    //    [5.0,   1.0, 0.0, 0.9, 0.5, 0.1, 1.0],
    //    [1.0,   0.0, 2.0, 0.1, 0.9, 0.5, 1.0],
    //    [0.0, -18.0, 0.0, 0.5, 0.1, 0.9, 1.0],
    //]);
    let triangle_normals = Matrix::from_2darray([
        [5.0,  1.0, 0.0, 0.9, 0.5, 0.1, 1.0, 0.5, 0.5, 0.5],
        [1.0,  0.0, 0.0, 0.1, 0.9, 0.5, 1.0, 0.5, 0.5, 0.5],
        [0.0, -5.0, 0.0, 0.5, 0.1, 0.9, 1.0, 0.5, 0.5, 0.5],
    ]);



    //let (t_vao, t_vbo) = WithObject::new_vao_vbo(&render.window.opengl, false, &triangle);
    //let (t_vao, t_vbo) = render.create_vao_vbo(&triangle);
    let (t_vao, t_vbo) = render.create_vao_vbo(&triangle_normals)?;
    //let (tn_vao, tn_vbo) = WithObject::new_vao_vbo(&render.window.opengl, true, &triangle_normals);

    while !render.render_over() {
        render.begin_render_actions();


        //render.camera.zoom -= 0.01;
        
        //render.use_program(ProgramType::SimpleOrthographic);
        render.use_program(ProgramSelect::SelectBlinnPhongOrthographic);


        //let with_vao = WithObject::vao(&render.window.opengl, tn_vao);
        //let with_vao = WithObject::vao(&render.window.opengl, t_vao);
        //with_vao.draw_vao(GlSettings::GlTriangles, &triangle);
        //render.draw_vao(GlSettings::GlTriangles, t_vao, &triangle);
        render.draw_vao(DrawMode::GlTriangles, t_vao, &triangle_normals);
        //with_vao.draw_vao(GlSettings::GlTriangles, &triangle_normals);



        //drop(with_vao);
        //drop(with_program);
        render.end_render_actions();
    }

    Ok(())
}