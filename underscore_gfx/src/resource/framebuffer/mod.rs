use crate::bindings::*;
use crate::resource::{
    texture::{Format, Texture},
    Resource, Target,
};
use underscore_64::log;

#[derive(Debug)]
pub struct Framebuffer {
    fb: GLuint,
    w: i32,
    h: i32,
}

impl Framebuffer {
    pub fn new(w: i32, h: i32) -> Self {
        let mut fb = 0;
        unsafe {
            glGenFramebuffers(1, &mut fb);
            glBindFramebuffer(GL_FRAMEBUFFER, fb);
        }

        Self { fb, w, h }
    }

    pub fn with_texture<F: Format>(self, attach: Attachment, tex: &Texture<F>) -> Self {
        unsafe {
            glFramebufferTexture(GL_FRAMEBUFFER, attach as _, **tex, 0);
            glDrawBuffers(1, [attach as _].as_ptr());
        }

        self
    }
}

impl Resource for Framebuffer {
    fn bind(&self) {
        log::debug!("binding framebuffer {}", self.fb);
        unsafe {
            glBindFramebuffer(GL_FRAMEBUFFER, self.fb);
        }
    }
}

impl Target for Framebuffer {}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        log::debug!("dropping framebuffer {}", self.fb);
        unsafe {
            glDeleteFramebuffers(1, &self.fb);
        }
    }
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum Attachment {
    Color0 = GL_COLOR_ATTACHMENT0,
    Stencil = GL_STENCIL_ATTACHMENT,
}
