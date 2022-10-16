use crate::render::{target::RenderTarget, Context};
use underscore_sys::*;

pub struct Window {
    window: *mut SDL_Window,
    w: i32,
    h: i32,
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

    pub fn context(&self) -> Context {
        unsafe {
            SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 4);
            SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 6);

            Context::new(self.window)
        }
    }

    pub fn swap(&self) {
        unsafe {
            SDL_GL_SwapWindow(self.window);
        }
    }
}

impl RenderTarget for Window {
    fn draw<T, F: FnMut(&mut Self) -> T>(&mut self, mut f: F) -> T {
        unsafe {
            glViewport(0, 0, self.w, self.h);
            glBindFramebuffer(GL_FRAMEBUFFER, 0);
        }
        f(self)
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            SDL_QuitSubSystem(SDL_INIT_VIDEO);
        }
    }
}
