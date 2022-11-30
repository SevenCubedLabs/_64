#![no_std]
pub use underscore_core::math;
pub use underscore_gfx as gfx;
pub use underscore_gui as gui;
pub use underscore_sdl as sdl;

#[macro_export]
macro_rules! c_str {
    ($exp: expr) => {
        concat!($exp, "\0").as_bytes()
    };
}
