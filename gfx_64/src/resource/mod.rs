pub mod buffer;
pub mod framebuffer;
pub mod mesh;
pub mod pipeline;
pub mod shader;
pub mod texture;

use crate::{gl, GfxSystem};
use base_64::math::Matrix;

pub trait Resource {
    fn bind(&self, _: &GfxSystem);
}

pub trait Uniform {
    fn bind(&self, ctx: &GfxSystem, location: i32);
}

impl Uniform for Matrix<4, 4> {
    fn bind(&self, ctx: &GfxSystem, location: i32) {
        unsafe {
            ctx.UniformMatrix4fv(location, 1, gl::FALSE, self.as_ptr() as _);
        }
    }
}

pub trait Draw {
    fn draw(&self, ctx: &GfxSystem);

    fn stencil(&self, ctx: &GfxSystem) {
        unsafe {
            ctx.Enable(gl::STENCIL_TEST);
            ctx.StencilMask(0xFF);
            ctx.ColorMask(0, 0, 0, 0);
            ctx.StencilFunc(gl::ALWAYS, 1, 0xFF);
            ctx.ClearStencil(0);
            ctx.Clear(gl::STENCIL_BUFFER_BIT);
            ctx.StencilOp(gl::INVERT, gl::INVERT, gl::INVERT);

            self.draw(ctx);

            ctx.StencilFunc(gl::NOTEQUAL, 0, 0xFF);
            ctx.StencilOp(gl::KEEP, gl::KEEP, gl::KEEP);
            ctx.ColorMask(0xFF, 0xFF, 0xFF, 0xFF);
            ctx.StencilMask(0);
        }
    }
}
