use sdl2_sys::*;

pub struct Window {
    window: *mut SDL_Window,
}

impl Window {
    pub fn new(name: &[u8], w: i32, h: i32) -> Result<Self, ()> {
        unsafe {
            SDL_InitSubSystem(SDL_INIT_VIDEO);

            let window = SDL_CreateWindow(
                name.as_ptr() as *const i8,
                SDL_WINDOWPOS_UNDEFINED_MASK as _,
                SDL_WINDOWPOS_UNDEFINED_MASK as _,
                w,
                h,
                SDL_WindowFlags::SDL_WINDOW_OPENGL as u32
                    | SDL_WindowFlags::SDL_WINDOW_SHOWN as u32,
            );

            if !window.is_null() {
                SDL_GL_SetAttribute(SDL_GLattr::SDL_GL_CONTEXT_MAJOR_VERSION, 4);
                SDL_GL_SetAttribute(SDL_GLattr::SDL_GL_CONTEXT_MINOR_VERSION, 6);

                Ok(Self { window })
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
