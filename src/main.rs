#![allow(warnings)]

// uncomment for release
// #![windows_subsystem = "windows"]





mod camera;
mod render_context;
mod shaders;

//use crate::window_loader::WindowLoader::init_window_and_opengl;
use crate::shaders::shaders::{ProgramHolder, ProgramType};
use crate::shaders::shaders::{create_vao_vbo, update_vbo, draw_vao};
use crate::camera::Camera::{Camera, Lighting};
use crate::render_context::RenderContext::Render;
use opengl::{clear_colour, GlSettings};
use opengl;
use matrices::Matrix2d;
use window::init_window_and_opengl;




use rust_embed::Embed;



#[derive(Embed)]
#[folder = "src/shaders_glsl/"]
struct Asset;


fn main() {

    

    let mut window = init_window_and_opengl();
    let mut camera = Camera::new();
    let mut lighting = Lighting::new();
    let program_holder = ProgramHolder::new(
        &window.opengl,
        [ProgramType::SimpleOrthographic, ProgramType::BlinnPhongOrthographic]
    );


    let mut render = Render::new(window, camera, lighting, program_holder);
    render.setup_render();

    //window.set_polling();
    //window.make_current();
    //window.default_gl_settings();



    let triangle = Matrix2d::from([
        [5.0,   1.0, 0.0, 0.9, 0.5, 0.1, 1.0],
        [1.0,   0.0, 2.0, 0.1, 0.9, 0.5, 1.0],
        [0.0, -18.0, 0.0, 0.5, 0.1, 0.9, 1.0],
    ]);



    let (t_vao, t_vbo) = create_vao_vbo(&render.window.opengl, false, &triangle);


    let mut world_transform = Matrix2d::from([
            [1.,0.,0.,0.],
            [0.,0.,1.,0.],
            [0.,1.,0.,0.],
            [0.,0.,0.,1.],
            ]);


    let m = Matrix2d::from([
        [1., 2., 3.],
        [4., 5., 6.],
        [7., 8., 9.]
        ]);
    println!("{:?}", m.transpose());



    while !render.render_over() {
        render.begin_render_actions();


        //render.camera.zoom -= 0.001;




        //let with_program = render.programs.use_program(&render.window.opengl, ProgramType::SimpleOrthographic);
        render.use_program(ProgramType::SimpleOrthographic);
        //render.use_program(ProgramType::BlinnPhongOrthographic);



        draw_vao(&render.window.opengl, GlSettings::GlTriangles, t_vao, &triangle);

        render.end_render_actions();
    }


}


fn get_shader_text(filename:&str) -> String {
    let mut file = filename.to_owned();
    file.push_str(".glsl");
    let file = file.as_str();

    let glsl = Asset::get(file).unwrap();
    let shader_text = std::str::from_utf8(glsl.data.as_ref()).unwrap().to_owned();
    shader_text
}