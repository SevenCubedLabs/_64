#![no_std]
pub mod resource;
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(improper_ctypes)]
#[allow(dead_code)]
mod gl {

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use resource::*;

use {crate::gl::*, log_64 as log, math_64::Matrix};

pub use resource::framebuffer::SWAP_CHAIN;

pub trait Resource {
    fn bind(&self);
}

pub trait Target: Resource {
    fn clear_color(&self, [r, g, b, a]: [f32; 4]) {
        unsafe {
            glClearColor(r, g, b, a);
        }
    }

    fn clear_stencil(&self, clear: i32) {
        unsafe {
            glClearStencil(clear);
        }
    }

    fn viewport(&self, [x, y]: [i32; 2], [w, h]: [i32; 2]) {
        log::debug!("setting viewport: [{}, {}], [{}, {}]", x, y, w, h);
        unsafe {
            glViewport(x, y, w, h);
        }
    }
}

pub trait Uniform {
    fn bind(&self, location: i32);
}

impl Uniform for Matrix<4, 4> {
    fn bind(&self, location: i32) {
        unsafe {
            glUniformMatrix4fv(location, 1, GL_FALSE as _, self.as_ptr() as _);
        }
    }
}
