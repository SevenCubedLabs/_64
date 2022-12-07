#![no_std]
#![cfg_attr(test, feature(test))]
#![feature(core_intrinsics)]

pub mod graph;
pub mod math;
pub mod mem;

#[cfg(feature = "log")]
#[macro_export]
extern crate log;

extern "C" {
    pub fn exit(_: i32);
}
