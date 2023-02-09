#![no_std]
extern crate alloc;

#[cfg(feature = "vec_64")]
pub mod vec;
#[cfg(feature = "vec_64")]
pub use vec::Vec;

#[cfg(not(feature = "vec_64"))]
pub use alloc::vec::Vec;

pub fn init() {
    #[global_allocator]
    static MALLOC: StdAlloc = StdAlloc {};
}

struct StdAlloc;

unsafe impl core::alloc::GlobalAlloc for StdAlloc {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        extern "C" {
            fn malloc(_: usize) -> *mut core::ffi::c_void;
        }

        malloc(layout.size() as _) as _
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _: core::alloc::Layout) {
        extern "C" {
            fn free(_: *mut core::ffi::c_void);
        }

        free(_ptr as _);
    }
}
