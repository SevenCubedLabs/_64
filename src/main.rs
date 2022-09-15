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

use underscore_64::{
    data::List,
    event::{Event, EventFeed},
    render::{
        clear,
        mesh::{Mesh, Topology},
        program::Program,
        shaders::{POS2D_TEX2D, TEX2D},
        target::RenderTarget,
        window::Window,
    },
};
use underscore_txt::GlyphMap;

#[cfg_attr(feature = "minsize", no_mangle)]
pub fn main() {
    let window = Window::new(NAME.as_ptr(), 1920, 1080).expect("window creation failed");
    let _context = window.context();

    let glyphs =
        GlyphMap::new("assets/ttf/Hack-Regular.ttf").expect("Hack-Regular.ttf parse failed");
    println!("Hack-Regular.ttf rendered to glyph map");
    println!("{:?}", glyphs);

    let tex_quad = Mesh::new(
        &[
            ([-1.0, 1.0], [0.0, 1.0]),
            ([1.0, 1.0], [1.0, 1.0]),
            ([-1.0, -1.0], [0.0, 0.0]),
            ([1.0, -1.0], [1.0, 0.0]),
        ],
        Topology::TriStrip,
    );

    let glyph_prog = Program::new(POS2D_TEX2D, TEX2D);

    let mut events = EventFeed;
    events.text_input(true);

    let mut ch = List::new(1);
    ch.push('A' as u8);
    loop {
        match events.next() {
            Some(event) => match event {
                Event::Quit => {
                    break;
                }

                Event::Keyboard { .. } => {}

                Event::TextInput { text } => ch = text.iter().map(|ch| *ch as u8).collect(),
            },

            None => {}
        };

        window.draw(|| {
            glyph_prog.bind();
            clear([0.0, 0.0, 0.0, 1.0]);
            glyphs
                .get(ch[0] as char)
                .expect(&format!("glyph for {} not found", ch[0] as char))
                .bind();
            tex_quad.draw();
        });
        window.swap();
    }

    #[cfg(feature = "minsize")]
    underscore_64::exit(0);
}
