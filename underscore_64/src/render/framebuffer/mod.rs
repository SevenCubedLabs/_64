use super::{target::RenderTarget, texture::Texture};
use underscore_sys::*;

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

    pub fn with_texture(self, attach: Attachment, tex: &Texture) -> Self {
        unsafe {
            glFramebufferTexture(GL_FRAMEBUFFER, attach as _, **tex, 0);
            glDrawBuffers(1, [attach as _].as_ptr());
        }

        self
    }

    fn bind(&self) {
        unsafe {
            glBindFramebuffer(GL_FRAMEBUFFER, self.fb);
        }
    }
}

impl RenderTarget for Framebuffer {
    fn draw<T, F: FnMut(&mut Self) -> T>(&mut self, mut f: F) -> T {
        self.bind();
        self.viewport([0, 0], [self.w, self.h]);
        f(self)
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
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
