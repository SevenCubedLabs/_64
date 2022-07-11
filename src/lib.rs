#![cfg_attr(feature = "minsize", no_std)]
#![feature(core_intrinsics)]

pub mod event;
pub mod math;
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
