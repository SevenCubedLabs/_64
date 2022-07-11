#![cfg_attr(
    feature = "minsize",
    no_std,
    no_main,
    feature(lang_items),
    feature(naked_functions),
    feature(default_alloc_error_handler)
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
const POS: &str = concat!(include_str!("../../src/shaders/pos.vert"), "\0");
const WHITE: &str = concat!(include_str!("../../src/shaders/white.frag"), "\0");

#[cfg_attr(feature = "minsize", no_mangle)]
pub fn main() {
    use underscore_64::{
        event::{Event, EventFeed},
        math::{sin, Curve},
        render::{
            clear,
            mesh::{Mesh, Topology},
            program::Program,
        },
        window::Window,
    };

    let window = Window::new(NAME.as_ptr(), 1920, 1080).expect("test");
    let _context = window.context();

    let program = Program::new(POS, WHITE);
    program.bind();

    let new_sin = |x: f32| sin(x * 6.28);
    let sin_plot = new_sin.plot(-1.0, 1.0, 100);

    let mesh = Mesh::new(&sin_plot, Topology::Curve);

    let mut events = EventFeed;
    loop {
        match events.next() {
            Some(event) => match event {
                Event::Quit => {
                    break;
                }

                _ => {}
            },

            None => {
                clear([0.0, 0.0, 0.0, 1.0]);
                program.bind();
                mesh.draw();
                window.swap();
            }
        }
    }

    #[cfg(feature = "minsize")]
    _64::exit(0);
}
