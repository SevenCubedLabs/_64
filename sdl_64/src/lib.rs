#![no_std]
pub mod event;
pub mod window;

pub use sdl2_sys::SDL_GL_GetProcAddress as load_gl;
