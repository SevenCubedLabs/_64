#![no_std]
#![feature(core_intrinsics)]

pub mod event;
pub mod render;
pub mod window;
pub use sys::utils;

mod data;
mod sys;
