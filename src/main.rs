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

use std::io::Read;
use underscore_64::event::{Event, EventFeed};
use underscore_gfx::{
    mesh::{Mesh, Topology, Usage},
    program::Program,
    shaders::{POS2D_TEX2D, TEX2D},
    target::RenderTarget,
    window::Window,
};
use underscore_gui::text::TextSystem;

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
    let mut window = Window::new(NAME.as_ptr(), 1920, 1080).expect("window creation failed");
    let _context = window.context();

    let file = std::fs::File::open("assets/ttf/Hack-Regular.ttf")
        .expect("couldn't open ./assets/ttf/Hack-Regular.ttf")
        .bytes()
        .collect::<Result<Vec<u8>, std::io::Error>>()
        .expect("couldn't read ./assets/ttf/Hack-Regular.ttf");

    let mut ttf = TextSystem::new();
    let font = ttf.load_font(&file).expect("Hack-Regular.ttf parse failed");

    let mut greets = underscore_gui::Text::new(120, [1920, 1080]);
    greets.update("hello world");
    let text = ttf.draw(&mut greets);

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

    let mut events = EventFeed;
    events.text_input(true);

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

        window.draw(|win| {
            glyph_prog.bind();
            win.clear_color([0.0, 0.0, 0.0, 1.0]);
            text.bind();
            tex_quad.draw();
        });
        window.swap();
    }

    #[cfg(feature = "minsize")]
    underscore_64::exit(0);
}
