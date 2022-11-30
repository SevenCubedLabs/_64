use underscore_64::{
    gfx::{
        mesh::{Mesh, Topology, Usage},
        program::Program,
        shader::{POS2D_RGB, RGB},
        Resource, Target,
    },
    sdl::{
        event::{Event, EventFeed},
        window::Window,
    },
};

const NAME: &str = "_64-triangle";

pub fn main() {
    let window = Window::new(NAME.as_ptr(), 1920, 1080).expect("failed to open sdl2 window");

    let program = Program::new(POS2D_RGB, RGB);
    program.bind();

    let mesh = Mesh::new(
        &[
            ([0.0, 1.0], [1.0, 0.0, 0.0]),
            ([1.0, -1.0], [0.0, 1.0, 0.0]),
            ([-1.0, -1.0], [0.0, 0.0, 1.0]),
        ],
        Usage::StaticDraw,
        Topology::from_indices(&[0, 1, 2], Usage::StaticDraw),
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
                window.clear_color([0.0, 0.0, 0.0, 1.0]);
                program.bind();
                mesh.draw();
                window.swap();
            }
        }
    }
}
