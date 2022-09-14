use underscore_64::{
    assets::shaders::{POS2D_RGB, RGB},
    event::{Event, EventFeed},
    render::{
        clear,
        mesh::{Mesh, Topology},
        program::Program,
    },
    window::Window,
};

const NAME: &str = "_64-triangle\0";

pub fn main() {
    let window = Window::new(NAME.as_ptr(), 1920, 1080).expect("test");
    let _context = window.context();

    let program = Program::new(POS2D_RGB, RGB);
    program.bind();

    let mesh = Mesh::new(
        &[
            ([0.0, 1.0], [1.0, 0.0, 0.0]),
            ([1.0, -1.0], [0.0, 1.0, 0.0]),
            ([-1.0, -1.0], [0.0, 0.0, 1.0]),
        ],
        Topology::from_indices(&[0, 1, 2]),
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
}
