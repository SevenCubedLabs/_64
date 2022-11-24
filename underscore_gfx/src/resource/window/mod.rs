use crate::{
    resource::{Resource, Target},
    GfxSystem,
};
use underscore_64::bindings::*;

pub struct Window {
    window: *mut SDL_Window,
    pub(crate) w: i32,
    pub(crate) h: i32,
}

impl Window {
    pub fn new(name: *const u8, w: i32, h: i32) -> Result<Self, ()> {
        unsafe {
            SDL_InitSubSystem(SDL_INIT_VIDEO);

            let window = SDL_CreateWindow(
                name as _,
                SDL_WINDOWPOS_UNDEFINED_MASK as _,
                SDL_WINDOWPOS_UNDEFINED_MASK as _,
                w,
                h,
                SDL_WINDOW_OPENGL | SDL_WINDOW_SHOWN,
            );

            if !window.is_null() {
                Ok(Self { window, w, h })
            } else {
                SDL_QuitSubSystem(SDL_INIT_VIDEO);
                Err(())
            }
        }
    }

    pub fn context(&self) -> GfxSystem {
        unsafe {
            SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 4);
            SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 6);

            GfxSystem::new(self.window)
        }
    }

    pub fn swap(&self) {
        unsafe {
            SDL_GL_SwapWindow(self.window);
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            SDL_QuitSubSystem(SDL_INIT_VIDEO);
        }
    }
}

impl Resource for Window {
    fn bind(&self) {
        unsafe {
            glBindFramebuffer(GL_FRAMEBUFFER, 0);
            glViewport(0, 0, self.w, self.h);
        }
    }
}

impl Target for Window {}
