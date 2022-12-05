use crate::gl::types::*;
use crate::{resource::Resource, GfxSystem};

pub type Pipeline = GLuint;

impl Resource for Pipeline {
    fn bind(&self, ctx: &GfxSystem) {
        unsafe {
            ctx.UseProgram(*self);
        }
    }
}
