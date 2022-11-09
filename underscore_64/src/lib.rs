#![no_std]
#![feature(core_intrinsics)]

pub mod data;
pub mod event;
pub mod math;

pub fn exit(code: i32) {
    unsafe {
        underscore_sys::exit(code);
    }
}
