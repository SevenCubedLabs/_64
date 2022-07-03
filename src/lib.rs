#![no_std]
#![feature(core_intrinsics)]

pub mod event;
pub mod render;
pub mod window;
pub use sys::utils;

#[cfg(feature = "minsize")]
mod data;
#[cfg(not(feature = "minsize"))]
mod data {
    extern crate alloc;

    pub type List<Item> = alloc::vec::Vec<Item>;
}
mod sys;
