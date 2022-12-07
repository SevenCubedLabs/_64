use crate::{gl, gl::types::*, RenderTarget, Resource, Texture};

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum Attachment {
    Color0 = gl::COLOR_ATTACHMENT0,
    Stencil = gl::STENCIL_ATTACHMENT,
}

#[derive(Clone, Debug)]
pub struct Framebuffer {
    id: GLuint,
}

impl Framebuffer {
    pub fn new(attachments: &[Attachment], tex: &[&Texture]) -> Self {
        let mut id = 0;
        unsafe {
            gl::GenFramebuffers(1, &mut id);
            gl::BindFramebuffer(gl::FRAMEBUFFER, id);

            for (&attachment, tex) in attachments.iter().zip(tex.iter()) {
                gl::FramebufferTexture(gl::FRAMEBUFFER, attachment as u32, tex.id, 0);
            }
        }

        Self { id }
    }
}

impl Resource for Framebuffer {
    fn bind(&self) {
        log::trace!("binding framebuffer {}", self.id);
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);
            gl::DrawBuffers(1, [Attachment::Color0].as_ptr() as *const u32);
        }
    }
}

impl RenderTarget for Framebuffer {}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &self.id);
        }
    }
}
