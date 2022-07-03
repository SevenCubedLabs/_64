#![no_std]
#![feature(core_intrinsics)]

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
mod sys;

pub fn exit(code: i32) {
    unsafe { sys::exit(code) };
}
