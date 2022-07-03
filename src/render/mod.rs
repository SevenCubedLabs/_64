mod buffer;
pub mod mesh;
pub mod program;
pub mod shader;
pub mod vertex;
use _sys::*;

pub struct Context(SDL_GLContext);

impl Context {
    pub fn new(window: *mut SDL_Window) -> Self {
        unsafe { Context(SDL_GL_CreateContext(window)) }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            SDL_GL_DeleteContext(self.0 as _);
        }
    }
}

pub fn clear([r, g, b, a]: [f32; 4]) {
    unsafe {
        glClearColor(r, g, b, a);
        glClear(GL_COLOR_BUFFER_BIT);
    }
}
