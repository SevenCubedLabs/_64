use underscore_gfx::{Resource, Target};
use underscore_sys::*;

pub struct Window {
    window: *mut SDL_Window,
    size: [i32; 2],
    ctx: SDL_GLContext,
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
                SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 4);
                SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 6);

                Ok(Self {
                    window,
                    size: [w, h],
                    ctx: SDL_GL_CreateContext(window),
                })
            } else {
                SDL_QuitSubSystem(SDL_INIT_VIDEO);
                Err(())
            }
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
            SDL_GL_DeleteContext(self.ctx as _);
            SDL_QuitSubSystem(SDL_INIT_VIDEO);
        }
    }
}

impl Resource for Window {
    fn bind(&self) {
        unsafe {
            glBindFramebuffer(GL_FRAMEBUFFER, 0);
        }
    }
}

impl Target for Window {}
