use crate::render::Context;
use _sys::*;

pub struct Window(*mut SDL_Window);

impl Window {
    pub fn new(name: *const u8, width: u32, height: u32) -> Result<Self, ()> {
        unsafe {
            SDL_InitSubSystem(SDL_INIT_VIDEO);

            let window = SDL_CreateWindow(
                name as _,
                SDL_WINDOWPOS_UNDEFINED_MASK as _,
                SDL_WINDOWPOS_UNDEFINED_MASK as _,
                width as _,
                height as _,
                SDL_WINDOW_OPENGL | SDL_WINDOW_SHOWN,
            );

            if !window.is_null() {
                Ok(Self(window))
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

            Context::new(self.0)
        }
    }

    pub fn swap(&self) {
        unsafe {
            SDL_GL_SwapWindow(self.0);
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
