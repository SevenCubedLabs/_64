#![no_std]
#![feature(core_intrinsics)]

extern crate underscore_sys as _sys;
pub mod event;
pub mod render;
pub mod window;

#[cfg(feature = "minsize")]
mod data;
#[cfg(not(feature = "minsize"))]
mod data {
    extern crate alloc;

    pub type List<Item> = alloc::vec::Vec<Item>;
}

pub fn exit(code: i32) {
    unsafe { _sys::exit(code) };
}
