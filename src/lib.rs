#![no_std]
#![feature(core_intrinsics)]

extern crate alloc;

pub mod event;
pub mod render;
mod sys;
pub mod window;
pub use sys::utils;
