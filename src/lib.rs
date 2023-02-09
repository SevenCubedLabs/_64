#![cfg_attr(
    not(feature = "std"),
    no_std,
    feature(lang_items),
    feature(naked_functions),
    feature(default_alloc_error_handler),
    feature(const_cstr_methods)
)]
pub use {gfx_64, gui_64, math_64, sdl_64};

#[cfg(not(feature = "std"))]
pub mod min_build {
    #[lang = "eh_personality"]
    fn eh_personality() {}

    #[panic_handler]
    fn panic(_info: &core::panic::PanicInfo) -> ! {
        loop {}
    }

    #[cfg(not(feature = "std"))]
    #[no_mangle]
    #[naked]
    pub unsafe extern "C" fn _start() {
        use core::arch::asm;

        extern "C" {
            fn exit(_: core::ffi::c_int);
        }

        asm!(
            "mov rdi, rsp",
            "call main",
            "mov rax, 0",
            "call exit",
            options(noreturn)
        )
    }
}
