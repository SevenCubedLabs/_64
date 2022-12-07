pub mod buffer;
pub mod framebuffer;
pub mod mesh;
pub mod pipeline;
pub mod shader;
pub mod texture;
pub mod window;

use crate::{gl};
use base_64::math::Matrix;

pub trait Resource {
    fn bind(&self);
}

pub trait Uniform {
    fn bind(&self, location: i32);
}

impl Uniform for Matrix<4, 4> {
    fn bind(&self, location: i32) {
        unsafe {
            gl::UniformMatrix4fv(location, 1, gl::FALSE, self.as_ptr() as _);
        }
    }
}

pub trait Draw {
    fn draw(&self);
}

pub trait Stencil: Draw {
    fn stencil(&self) {
        unsafe {
            gl::Enable(gl::STENCIL_TEST);
            gl::StencilMask(0xFF);
            gl::ColorMask(0, 0, 0, 0);
            gl::StencilFunc(gl::ALWAYS, 1, 0xFF);
            gl::ClearStencil(0);
            gl::Clear(gl::STENCIL_BUFFER_BIT);
            gl::StencilOp(gl::INVERT, gl::INVERT, gl::INVERT);

            self.draw();

            gl::StencilFunc(gl::NOTEQUAL, 0, 0xFF);
            gl::StencilOp(gl::KEEP, gl::KEEP, gl::KEEP);
            gl::ColorMask(0xFF, 0xFF, 0xFF, 0xFF);
            gl::StencilMask(0);
        }
    }
}

pub trait RenderTarget {
    fn clear_color(&self, [r, g, b, a]: [f32; 4]) {
        unsafe {
            gl::ClearColor(r, g, b, a);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    fn clear_stencil(&self, clear: i32) {
        unsafe {
            gl::ClearStencil(clear);
        }
    }

    fn viewport(&self, [x, y]: [i32; 2], [w, h]: [i32; 2]) {
        log::trace!("setting viewport: [{}, {}], [{}, {}]", x, y, w, h);
        unsafe {
            gl::Viewport(x, y, w, h);
        }
    }
}
