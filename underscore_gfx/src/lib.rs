pub mod assets;
pub mod resource;

#[macro_use]
extern crate underscore_64;
use underscore_64::bindings::*;

pub struct GfxSystem(SDL_GLContext);

impl GfxSystem {
    pub fn new(window: *mut SDL_Window) -> Self {
        unsafe { GfxSystem(SDL_GL_CreateContext(window)) }
    }
}

impl Drop for GfxSystem {
    fn drop(&mut self) {
        unsafe {
            SDL_GL_DeleteContext(self.0 as _);
        }
    }
}
