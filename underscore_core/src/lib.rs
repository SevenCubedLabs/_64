#![no_std]
#![feature(core_intrinsics)]

pub mod math;
pub mod alloc {
    #[cfg(feature = "default_alloc")]
    extern crate alloc;
    #[cfg(feature = "default_alloc")]
    pub use alloc::*;

    #[cfg(not(feature = "default_vec"))]
    pub mod vec;
}
