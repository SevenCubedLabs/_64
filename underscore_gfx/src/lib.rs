pub mod assets;
pub mod resource;

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(improper_ctypes)]
#[allow(dead_code)]
mod bindings {
    mod c_types {
        pub type c_short = i16;
        pub type c_int = i32;
        pub type c_schar = i8;
        pub type c_char = i8;
        pub type c_uchar = u8;
        pub type c_ushort = u16;
        pub type c_uint = u32;
        pub type c_long = i64;
        pub type c_longlong = i64;
        pub type c_ulong = u64;
        pub type c_ulonglong = i64;
        pub type c_void = core::ffi::c_void;
    }

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use crate::bindings::*;
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
