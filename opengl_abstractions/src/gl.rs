#![allow(warnings)]

include!(concat!(env!("OUT_DIR"), "\\gl_bindings.rs"));

use std::fmt;

impl fmt::Debug for Gl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "opengl fmt")
    }
}