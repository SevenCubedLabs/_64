use crate::sys::*;

pub struct Buffer {
    _type: GLenum,
    buf: GLuint,
    len: usize,
}

impl Buffer {
    pub fn new(_type: GLenum) -> Self {
        unsafe {
            let mut buf = 0;
            glGenBuffers(1, &mut buf);
            Self { _type, buf, len: 0 }
        }
    }

    pub fn bind(&self) {
        unsafe { glBindBuffer(self._type, self.buf) }
    }

    pub fn copy<Data>(&mut self, data: &[Data]) {
        self.bind();
        self.len = data.len();

        unsafe {
            glBufferData(
                self._type,
                (core::mem::size_of::<Data>() * data.len()) as _,
                data.as_ptr() as _,
                GL_STATIC_DRAW,
            )
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            glDeleteBuffers(1, [self.buf].as_ptr());
        }
    }
}
