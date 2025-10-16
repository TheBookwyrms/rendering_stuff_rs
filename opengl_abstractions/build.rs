extern crate gl_generator;

use gl_generator::{Registry, Api, Profile, Fallbacks};
use std::env;
use std::fs::File;
use std::path::Path;

fn main() {
    
    let out_dir = env::var("OUT_DIR").unwrap();
    
    let dest = Path::new(&out_dir);
    
    let mut opengl_api_file = File::create(&dest.join("gl_bindings.rs")).unwrap();


    Registry::new(Api::Gl, (3, 3), Profile::Core, Fallbacks::All, [])
    .write_bindings(gl_generator::StructGenerator, &mut opengl_api_file)
    .unwrap();


    //println!("cargo:rustc-link-lib=static=glfw3");
}