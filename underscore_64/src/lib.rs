#![no_std]
#![feature(core_intrinsics)]

pub use log;
pub mod data;
pub mod event;
pub mod math;

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(improper_ctypes)]
#[allow(dead_code)]
pub mod bindings {
    mod c_types {
        pub type c_short = i16;
        pub type c_int = i32;
        pub type c_schar = i8;
        pub type c_char = i8;
        pub type c_uchar = u8;
        pub type c_ushort = u16;
        pub type c_uint = u32;
        pub type c_long = i64;
        pub type c_longlong = i64;
        pub type c_ulong = u64;
        pub type c_ulonglong = i64;
        pub type c_void = core::ffi::c_void;
    }

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

macro_rules! include_c_str {
    ($src:literal) => {
        concat!(include_str!($src), "\0")
    };
}
