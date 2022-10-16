use underscore_sys::*;

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum Target {
    Tex2d = GL_TEXTURE_2D,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum Format {
    Rgb = GL_RGB,
    Rgba = GL_RGBA,
    Stencil = GL_STENCIL_INDEX,
}

#[derive(Debug)]
pub struct Texture {
    id: GLuint,
    target: Target,
}

impl Texture {
    pub fn new(target: Target, format: Format, w: i32, h: i32) -> Self {
        let mut id = 0;
        unsafe {
            glGenTextures(1, &mut id);
            glBindTexture(target as _, id);
            glTexImage2D(
                target as _,
                0,
                format as _,
                w,
                h,
                0,
                format as _,
                GL_UNSIGNED_BYTE,
                core::ptr::null(),
            );
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST as _);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST as _);
        }

        Self { id, target }
    }

    pub fn bind(&self) {
        unsafe {
            glBindTexture(self.target as _, self.id);
        }
    }
}

impl core::ops::Deref for Texture {
    type Target = GLuint;

    fn deref(&self) -> &Self::Target {
        &self.id
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            glDeleteTextures(1, &self.id);
        }
    }
}
