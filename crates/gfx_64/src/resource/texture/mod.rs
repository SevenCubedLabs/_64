use crate::{gl, gl::types::*, resource::Resource, GfxSystem};

#[derive(Clone, Debug)]
pub struct Texture {
    pub(crate) id: GLuint,
    pub(crate) target: Target,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum Target {
    Tex2d = gl::TEXTURE_2D,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum Format {
    Rgba = gl::RGBA,
    Stencil = gl::STENCIL_INDEX,
}

impl Texture {
    pub fn new(target: Target, [w, h]: [i32; 2], format: Format) -> Texture {
        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            log::info!("creating texture {}", id);
            gl::BindTexture(target as _, id);
            gl::TexImage2D(
                target as _,
                0,
                format as i32,
                w,
                h,
                0,
                format as u32,
                gl::UNSIGNED_BYTE,
                core::ptr::null(),
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as _);
        }

        Texture { id, target }
    }
}

impl Resource for Texture {
    fn bind(&self) {
        log::info!("binding texture {}", self.id);
        unsafe {
            gl::BindTexture(self.target as _, self.id);
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        log::info!("dropping texture {}", self.id);
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}
