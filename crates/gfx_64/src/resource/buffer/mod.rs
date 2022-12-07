use crate::{gl, gl::types::*, resource::Resource, GfxSystem};

#[derive(Clone, Copy, Debug)]
pub struct Buffer {
    pub(crate) id: GLuint,
    pub(crate) buf_type: GLenum,
    pub(crate) len: usize,
}

impl Buffer {
    pub fn new<Data>(buf_type: GLenum, usage: Usage, data: &[Data]) -> Buffer {
        unsafe {
            let mut id = 0;
            gl::GenBuffers(1, &mut id);
            gl::BindBuffer(buf_type, id);

            gl::BufferData(
                buf_type,
                (core::mem::size_of::<Data>() * data.len()) as _,
                data.as_ptr() as _,
                usage as _,
            );

            Buffer {
                buf_type,
                id,
                len: data.len(),
            }
        }
    }

    pub fn update<Data>(&mut self, data: &[Data]) {
        self.bind();
        unsafe {
            gl::BufferSubData(
                self.buf_type,
                0,
                (core::mem::size_of::<Data>() * data.len()) as _,
                data.as_ptr() as _,
            );
        }
        self.len = data.len();
    }
}

impl Resource for Buffer {
    fn bind(&self) {
        unsafe { gl::BindBuffer(self.buf_type, self.id) }
    }
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum Usage {
    StaticDraw = gl::STATIC_DRAW,
    StreamDraw = gl::STREAM_DRAW,
}
