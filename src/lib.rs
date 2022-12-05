#![no_std]
pub use base_64::math;
pub use gfx_64 as gfx;
pub use gui_64 as gui;

#[macro_export]
macro_rules! c_str {
    ($exp: expr) => {
        concat!($exp, "\0").as_bytes()
    };
}
