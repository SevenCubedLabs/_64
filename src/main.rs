#![cfg_attr(
    feature = "minsize",
    no_std,
    no_main,
    feature(lang_items),
    feature(naked_functions)
)]

#[cfg(feature = "minsize")]
#[lang = "eh_personality"]
fn eh_personality() {}

#[cfg(feature = "minsize")]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[cfg(feature = "minsize")]
#[no_mangle]
#[naked]
pub unsafe extern "C" fn _start() {
    use core::arch::asm;

    asm!("mov rdi, rsp", "call main", options(noreturn))
}

const NAME: &str = "_64\0";
const WIDTH: i32 = 1920;
const HEIGHT: i32 = 1080;

use underscore_64::log;
use underscore_gfx::{
    resource::{
        mesh::{Mesh, Topology, Usage},
        program::Program,
        shader::{POS2D_TEX2D, TEX2D},
    },
    Resource, Target,
};
use underscore_gui::TextSystem;
use underscore_sdl::{
    event::{Event, EventFeed},
    window::Window,
};

use log::{Level, LevelFilter, Metadata, Record};

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

#[cfg_attr(feature = "minsize", no_mangle)]
pub fn main() {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Debug))
        .expect("failed to init logs");
    log::info!("---BEGIN LOG---");
    let window = Window::new(NAME.as_ptr(), 1920, 1080).expect("window creation failed");

    let text = TextSystem::default();

    let mut greets = underscore_gui::text::Text::new(120, [1920, 1080]);
    greets.update("hello\nworld");
    text.draw(&greets, 0, 10.0);

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

    log::debug!("end setup");
    log::set_max_level(log::LevelFilter::Off);

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

    log::set_max_level(log::LevelFilter::Debug);
    #[cfg(feature = "minsize")]
    underscore_64::exit(0);
}
