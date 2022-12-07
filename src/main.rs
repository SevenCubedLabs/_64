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

use base_64::graph::Node;
use gfx_64::{
    resource::{
        mesh::{Mesh, Topology, Usage},
        pipeline::Pipeline,
        shader::{POS2D_TEX2D, TEX2D},
    },
    Draw, GfxSystem, RenderTarget, Resource,
};
use gui_64::{GuiSystem, HACK_TTF};
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
            metadata.level() <= Level::Trace
        }

        fn log(&self, record: &Record) {
            if self.enabled(record.metadata()) {
                println!("{:>7} |{}", format!("[{}]", record.level()), record.args());
            }
        }

        fn flush(&self) {}
    }

    pub fn init() {
        log::set_logger(&LOGGER)
            .map(|()| log::set_max_level(LevelFilter::Debug))
            .expect("failed to init logs");
        log::info!("logging enabled");
    }
}

#[cfg_attr(not(feature = "std"), no_mangle)]
pub fn main() {
    #[cfg(feature = "log")]
    simple_log::init();

    let gfx = Node::new(GfxSystem::new(NAME, 1920, 1080).expect("couldn't open SDL2/GL window"));
    let gui = Node::new(GuiSystem::new([WIDTH, HEIGHT]));

    let mut gfx = gfx.handle();
    let mut gui = gui.handle();

    let hack = gui.load_font(HACK_TTF).expect("load font failed");
    gui.draw_text(hack, "hello\nworld", [0.0, HEIGHT as f32], 3.0);

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

    let glyph_prog = Pipeline::new(POS2D_TEX2D, TEX2D);

    let mut events = EventFeed::new();
    events.text_input(true);

    #[cfg(feature = "log")]
    log::info!("main setup completed");
    #[cfg(feature = "log")]
    log::set_max_level(log::LevelFilter::Off);

    let mut frame = 0;
    loop {
        #[cfg(feature = "log")]
        log::info!("starting frame {}", frame);
        match events.next() {
            Some(event) => match event {
                Event::Quit => {
                    #[cfg(feature = "log")]
                    log::info!("quit event received");
                    break;
                }

                _ => {}
            },

            None => {}
        };

        gfx.draw(|| gui.draw());
        frame += 1;
    }

    #[cfg(not(feature = "std"))]
    unsafe {
        base_64::exit(0);
    }
}
