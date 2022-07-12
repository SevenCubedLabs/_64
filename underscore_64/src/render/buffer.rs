use underscore_sys::*;

pub struct Buffer {
    _type: GLenum,
    buf: GLuint,
    len: usize,
}

impl Buffer {
    pub fn new<Data>(_type: GLenum, data: &[Data]) -> Self {
        unsafe {
            let mut buf = 0;
            glGenBuffers(1, &mut buf);
            glBindBuffer(_type, buf);

            glBufferData(
                _type,
                (core::mem::size_of::<Data>() * data.len()) as _,
                data.as_ptr() as _,
                GL_STATIC_DRAW,
            );

            Self {
                _type,
                buf,
                len: data.len(),
            }
        }
    }

    pub fn bind(&self) {
        unsafe { glBindBuffer(self._type, self.buf) }
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
