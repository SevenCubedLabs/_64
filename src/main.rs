#![cfg_attr(
    not(feature = "std"),
    no_std,
    no_main,
    feature(lang_items),
    feature(naked_functions),
    feature(default_alloc_error_handler),
    feature(const_cstr_methods)
)]
use core::ffi::{c_char, CStr};

const NAME_PTR: *const c_char = {
    const BYTES: &[u8] = b"Hello, world!\0";
    BYTES.as_ptr().cast()
};
const NAME: &CStr = unsafe { CStr::from_ptr(NAME_PTR) };
const WIDTH: i32 = 1920;
const HEIGHT: i32 = 1080;

use {
    gfx_64::{
        resource::{
            mesh::{Mesh, Topology, Usage},
            program::Program,
            shader::{POS2D_TEX2D, TEX2D},
        },
        Resource, Target, SWAP_CHAIN,
    },
    gui_64::text::TextSystem,
    sdl_64::{
        event::{Event, EventFeed},
        window::Window,
    },
};

#[cfg(not(feature = "std"))]
use project_64::min_build::*;

#[cfg_attr(not(feature = "std"), no_mangle)]
pub fn main() {
    #[cfg(feature = "log")]
    simple_log::init();

    #[cfg(not(feature = "std"))]
    mem_64::init();

    let window = Window::new(&NAME, 1920, 1080).expect("window creation failed");

    let text = TextSystem::default();

    let mut greets = gui_64::text::Text::new([1920, 1080]);
    greets.update("Greetz!\n  it builds:D");
    text.draw(&greets, [1920, 1080], 0, 10.0);

    let tex_quad = Mesh::new(
        &[
            ([-1.0, 1.0], [0.0, 1.0]),
            ([1.0, 1.0], [1.0, 1.0]),
            ([-1.0, -1.0], [0.0, 0.0]),
            ([1.0, -1.0], [1.0, 0.0]),
        ],
        Usage::StaticDraw,
        Topology::TriStrip,
    );

    let glyph_prog = Program::new(POS2D_TEX2D, TEX2D);
    glyph_prog.bind();

    let mut events = EventFeed;
    events.text_input(true);

    #[cfg(feature = "log")]
    simple_log::set_max_level(simple_log::LevelFilter::Off);

    loop {
        match events.next() {
            Some(event) => match event {
                Event::Quit => {
                    break;
                }

                _ => {}
            },

            None => {}
        };

        SWAP_CHAIN.bind();
        SWAP_CHAIN.clear_color([0.0, 0.0, 0.0, 1.0]);
        SWAP_CHAIN.viewport([0, 0], [WIDTH, HEIGHT]);
        greets.view().bind();
        tex_quad.draw();
        window.swap();
    }

    /*
    #[cfg(not(feature = "std"))]
    unsafe {
        exit(0);
    }
    */
}
