#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]
#![allow(dead_code)]

pub mod utils {
    pub struct Allocator;

    unsafe impl core::alloc::GlobalAlloc for Allocator {
        unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
            super::malloc(layout.size() as _) as _
        }

        unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
            super::free(ptr as _);
        }
    }

    pub const STDOUT: LinuxFd = LinuxFd(1);

    pub struct LinuxFd(i32);

    impl LinuxFd {
        pub fn write(&self, bytes: &str) -> Result<(), i64> {
            unsafe {
                let res;
                core::arch::asm!(
                    "syscall",
                    in("rax") 1,
                    in("rdi") self.0,
                    in("rsi") bytes.as_ptr(),
                    in("rdx") bytes.len(),
                    lateout("rax") res,
                );

                if res == bytes.len() as _ {
                    Ok(())
                } else {
                    Err(res)
                }
            }
        }
    }

    pub fn exit(code: i32) {
        unsafe {
            super::exit(code);
        }
    }
}

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
