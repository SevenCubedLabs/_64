use underscore_64::{
    c_str,
    gfx::{
        mesh::{Mesh, Topology},
        program::Program,
        shader::{POS2D, WHITE},
        Resource, Target,
    },
    math::{sin, Curve},
    sdl::{
        event::{Event, EventFeed},
        window::Window,
    },
};

static NAME: &[u8] = c_str!("_64-triangle");

pub fn main() {
    let window = Window::new(NAME, 1920, 1080).expect("test");

    let program = Program::new(POS2D, WHITE);
    program.bind();

    let new_sin = |x: f32| sin(x * 6.28);
    let sin_plot = new_sin.plot(-1.0, 1.0, 100);

    let mesh = Mesh::static_draw(&sin_plot, Topology::Lines);

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
