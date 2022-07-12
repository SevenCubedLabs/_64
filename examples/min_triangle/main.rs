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

#[cfg_attr(feature = "minsize", no_mangle)]
pub fn main() {
    use underscore_64::{
        assets::shaders::{POS3D_RGB, RGB},
        event::{Event, EventFeed},
        render::{
            clear,
            mesh::{Mesh, Topology},
            program::Program,
        },
        window::Window,
    };

    let window = Window::new(NAME.as_ptr(), 1920, 1080).expect("test");
    let _context = window.context();

    let program = Program::new(POS3D_RGB, RGB);
    program.bind();

    let mesh = Mesh::new(
        &[
            ([0.0, 1.0, 0.0], [1.0, 0.0, 0.0]),
            ([1.0, -1.0, 0.0], [0.0, 1.0, 0.0]),
            ([-1.0, -1.0, 0.0], [0.0, 0.0, 1.0]),
        ],
        Topology::idx_triangles(&[0, 1, 2]),
    );

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
    underscore_64::exit(0);
}
