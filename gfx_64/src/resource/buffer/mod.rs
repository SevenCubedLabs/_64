use crate::{gl, gl::types::*, resource::Resource, GfxSystem};

#[derive(Clone, Copy, Debug)]
pub struct Buffer {
    pub(crate) id: GLuint,
    pub(crate) buf_type: GLenum,
    pub(crate) len: usize,
}

impl Resource for Buffer {
    fn bind(&self, ctx: &GfxSystem) {
        unsafe { ctx.BindBuffer(self.buf_type, self.id) }
    }
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum Usage {
    StaticDraw = gl::STATIC_DRAW,
    StreamDraw = gl::STREAM_DRAW,
}
