#![no_std]
pub mod event;
pub mod window;

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(improper_ctypes)]
#[allow(dead_code)]
mod sdl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
