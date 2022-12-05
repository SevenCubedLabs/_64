#![no_std]
#![cfg_attr(test, feature(test))]
#![feature(core_intrinsics)]

pub mod math;
pub mod mem;
pub mod system;

pub type Handle<T> = core::sync::atomic::AtomicPtr<T>;

#[cfg(feature = "log")]
#[macro_export]
extern crate log;

extern "C" {
    pub fn exit(_: i32);
}
