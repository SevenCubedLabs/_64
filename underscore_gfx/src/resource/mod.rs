pub mod buffer;
pub mod framebuffer;
pub mod mesh;
pub mod program;
pub mod texture;
pub mod window;

use crate::bindings::*;
use underscore_64::log;

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
