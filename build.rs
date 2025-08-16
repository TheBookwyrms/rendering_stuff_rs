extern crate gl_generator;
//
use gl_generator::{Registry, Api, Profile, Fallbacks};
use std::env;
use std::fs::File;
use std::path::Path;

//use std::process::Command;
//use std::env;
//use std::path::Path;


fn main() {
    
    let out_dir = env::var("OUT_DIR").unwrap();
    
    let dest = Path::new(&out_dir);
    
    let mut file = File::create(&dest.join("gl_bindings.rs")).unwrap();


    Registry::new(Api::Gl, (3, 3), Profile::Core, Fallbacks::All, [])
    .write_bindings(gl_generator::StructGenerator, &mut file)
    .unwrap();

     println!("cargo:rustc-link-lib=static=glfw3");

//    let out_dir = env::var("OUT_DIR").unwrap();
//
//    cc::Build::new()
//
//        .file("glfw3.dll")
//        .compile("glfw");
//    println!("cargo::rerun-if-changed=src/glfw3.dll");
//}
//
//fn main() {
}