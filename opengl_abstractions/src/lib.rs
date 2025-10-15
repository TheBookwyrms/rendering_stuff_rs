// gets gl from the build.rs and khronos API
mod gl;

// provides handles for the base opengl API
mod raw_opengl;

// provides safe(r) handles for the opengl API exposed in raw_opengl
mod intermediate_opengl;

pub mod enums;

pub mod high_level_abstractions;