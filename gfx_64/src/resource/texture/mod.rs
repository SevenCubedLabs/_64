use crate::{gl, gl::types::*, resource::Resource, GfxSystem};

#[derive(Clone, Copy, Debug)]
pub struct Texture {
    pub(crate) id: GLuint,
    pub(crate) target: Target,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum Target {
    Tex2d = gl::TEXTURE_2D,
}

impl Resource for Texture {
    fn bind(&self, ctx: &GfxSystem) {
        log::debug!("binding texture {}", self.id);
        unsafe {
            ctx.BindTexture(self.target as _, self.id);
        }
    }
}
