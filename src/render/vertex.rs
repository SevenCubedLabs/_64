use _sys::*;
use core::mem::size_of;

pub trait Vertex: Sized {
    const NUM: GLint;
    const SIZE: GLint = size_of::<Self>() as _;
    const TYPE: GLenum = GL_FLOAT;
    const NORM: GLboolean = GL_FALSE as _;

    fn enable(idx: u32) {
        unsafe {
            glEnableVertexAttribArray(idx);
            glVertexAttribPointer(
                idx,
                Self::NUM,
                Self::TYPE,
                Self::NORM,
                Self::SIZE,
                core::ptr::null(),
            );
        }
    }
}

impl Vertex for [f32; 3] {
    const NUM: GLint = 3;
}
