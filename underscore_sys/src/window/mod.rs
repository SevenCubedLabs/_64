use crate::sdl::*;
use underscore_gfx::{gl, Resource, Target};

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
                SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 4);
                SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 6);

                Ok(Self {
                    window,
                    w,
                    h,
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
            SDL_QuitSubSystem(SDL_INIT_VIDEO);
        }
    }
}

impl Resource for Window {
    fn bind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            gl::Viewport(0, 0, self.w, self.h);
        }
    }
}

impl Target for Window {}
