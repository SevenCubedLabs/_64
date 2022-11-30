#![no_std]
#![feature(core_intrinsics)]

pub mod alloc;
pub mod math;

#[macro_export]
macro_rules! c_str {
    ($exp: expr) => {
        concat!($exp, "\0").as_ptr() as *const u8
    };
}
