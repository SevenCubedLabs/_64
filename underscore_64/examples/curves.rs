use underscore_64::{
    assets::shaders::{POS2D, WHITE},
    event::{Event, EventFeed},
    math::{sin, Curve},
    render::{
        clear,
        mesh::{Mesh, Topology},
        program::Program,
    },
    window::Window,
};

const NAME: &str = "_64-curves\0";

pub fn main() {
    let window = Window::new(NAME.as_ptr(), 1920, 1080).expect("test");
    let _context = window.context();

    let program = Program::new(POS2D, WHITE);
    program.bind();

    let new_sin = |x: f32| sin(x * 6.28);
    let sin_plot = new_sin.plot(-1.0, 1.0, 100);

    let mesh = Mesh::new(&sin_plot, Topology::Points);

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
