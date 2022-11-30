#![no_std]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(improper_ctypes)]
#[allow(dead_code)]
mod underscore_sys {
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

    include!(concat!(env!("OUT_DIR"), "/underscore_sys.rs"));
}

pub use underscore_sys::*;
