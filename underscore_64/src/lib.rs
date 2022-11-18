#![no_std]
#![feature(core_intrinsics)]

pub use log;
pub mod data;
pub mod event;
pub mod math;

extern "C" {
    pub fn exit(code: i32);
}
