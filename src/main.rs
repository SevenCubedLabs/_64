#![cfg_attr(
    not(feature = "std"),
    no_std,
    no_main,
    feature(lang_items),
    feature(naked_functions)
)]

#[cfg(not(feature = "std"))]
#[lang = "eh_personality"]
fn eh_personality() {}

#[cfg(not(feature = "std"))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[cfg(not(feature = "std"))]
#[no_mangle]
#[naked]
pub unsafe extern "C" fn _start() {
    use core::arch::asm;

    asm!("mov rdi, rsp", "call main", options(noreturn))
}

static NAME: &[u8] = c_str!("_64");
const WIDTH: i32 = 1920;
const HEIGHT: i32 = 1080;

use gfx_64::{
    resource::{
        mesh::{Mesh, Topology, Usage},
        program::Program,
        shader::{POS2D_TEX2D, TEX2D},
    },
    GfxSystem, Resource, Target,
};
use gui_64::text::TextSystem;
use sdl_64::{
    event::{Event, EventFeed},
    window::Window,
};
use underscore_64::c_str;

#[cfg(feature = "log")]
mod simple_log {
    pub use log::{set_max_level, LevelFilter};
    use log::{Level, Metadata, Record};

    struct SimpleLogger;
    static LOGGER: SimpleLogger = SimpleLogger;

    impl log::Log for SimpleLogger {
        fn enabled(&self, metadata: &Metadata) -> bool {
            metadata.level() <= Level::Debug
        }

        fn log(&self, record: &Record) {
            if self.enabled(record.metadata()) {
                println!("[{:>5}] {}", record.level(), record.args());
            }
        }

        fn flush(&self) {}
    }

    pub fn init() {
        log::set_logger(&LOGGER)
            .map(|()| log::set_max_level(LevelFilter::Debug))
            .expect("failed to init logs");
        log::info!("---BEGIN LOG---");
    }
}

#[cfg_attr(not(feature = "std"), no_mangle)]
pub fn main() {
    #[cfg(feature = "log")]
    simple_log::init();

    let window = Window::new(NAME, 1920, 1080).expect("window creation failed");

    let text = TextSystem::default();

    let mut greets = gui_64::text::Text::new([1920, 1080]);
    greets.update("hello\nworld");
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

        window.bind();
        window.clear_color([0.0, 0.0, 0.0, 1.0]);
        window.viewport([0, 0], [WIDTH, HEIGHT]);
        greets.view().bind();
        tex_quad.draw();
        window.swap();
    }

    #[cfg(not(feature = "std"))]
    unsafe {
        base_64::exit(0);
    }
}
