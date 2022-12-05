use crate::{gl, gl::types::*, resource::Resource, GfxSystem};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Framebuffer {
    pub(crate) id: GLuint,
}

impl Resource for Framebuffer {
    fn bind(&self, ctx: &GfxSystem) {
        log::debug!("binding framebuffer {}", self.id);
        unsafe {
            ctx.BindFramebuffer(gl::FRAMEBUFFER, self.id);
        }
    }
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum Attachment {
    Color0 = gl::COLOR_ATTACHMENT0,
    Stencil = gl::STENCIL_ATTACHMENT,
}
