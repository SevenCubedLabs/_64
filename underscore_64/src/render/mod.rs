mod buffer;
pub mod framebuffer;
pub mod mesh;
pub mod program;
pub mod shaders;
pub mod target;
pub mod texture;
pub mod window;
use underscore_sys::*;

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
