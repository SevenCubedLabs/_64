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
    math::Spline,
    render::{
        clear,
        framebuffer::{Attachment, Framebuffer},
        mesh::{Mesh, Topology},
        program::Program,
        shaders::{POS2D, POS2D_TEX2D, TEX2D, WHITE},
        target::RenderTarget,
        texture::{Format, Target, Texture},
        window::Window,
    },
};

use ttf_parser::{OutlineBuilder, Rect};

struct GlyphBuilder {
    curves: List<Spline>,
    head: [f32; 2],
}

impl GlyphBuilder {
    fn new() -> Self {
        Self {
            curves: List::new(1),
            head: [0.0; 2],
        }
    }

    fn splines(&self) -> &List<Spline> {
        &self.curves
    }
}

impl OutlineBuilder for GlyphBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        self.curves.push(Spline::new(1));
        self.head = [x, y];
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.curves
            .tail_mut()
            .push([self.head, [x, y]].as_slice().into());
        self.head = [x, y];
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.curves
            .tail_mut()
            .push([self.head, [x1, y1], [x, y]].as_slice().into());
        self.head = [x, y];
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.curves
            .tail_mut()
            .push([self.head, [x1, y1], [x2, y2], [x, y]].as_slice().into());
        self.head = [x, y];
    }

    fn close(&mut self) {
        /*
        let begin = self.curves.tail()[0][0];
        self.curves
            .tail_mut()
            .push([self.head, begin].as_slice().into());
            */
    }
}

#[cfg_attr(feature = "minsize", no_mangle)]
pub fn main() {
    let window = Window::new(NAME.as_ptr(), 1920, 1080).expect("window creation failed");
    let _context = window.context();

    let font = ttf_parser::Face::from_slice(include_bytes!("../assets/ttf/Hack-Regular.ttf"), 0)
        .expect("Hack-Regular.ttf parse failed");

    let mut glyph = GlyphBuilder::new();
    let idx = font.glyph_index('A').expect("glyph data not found for A");

    let Rect {
        x_min,
        x_max,
        y_min,
        y_max,
    } = font
        .outline_glyph(idx, &mut glyph)
        .expect("glyph outline failed");

    println!(
        "glyph outlined: x_min {} x_max {} y_min {} y_max {}",
        x_min, x_max, y_min, y_max
    );

    for spline in glyph.splines().iter() {
        println!("extracted spline:");

        for point in spline.points().iter() {
            println!("\t{},{}", point[0], point[1]);
        }
    }

    // Prepare render target
    let w = x_max - x_min;
    let h = y_max - y_min;

    let color = Texture::new(Target::Tex2d, Format::Rgb, w as _, h as _);
    let stencil = Texture::new(Target::Tex2d, Format::Stencil, w as _, h as _);

    let fb = Framebuffer::new(w as _, h as _)
        .with_texture(Attachment::Color0, &color)
        .with_texture(Attachment::Stencil, &stencil);
    println!("render target created");

    let verts = glyph.splines().iter().fold(
        List::new(glyph.splines().len() * 100 + 1),
        |mut points, spline| {
            points.push([0.0, 0.0]);
            spline.points().iter().fold(points, |mut points, point| {
                points.push([
                    (2.0 * (point[0] - x_min as f32) / w as f32) - 1.0,
                    (2.0 * (point[1] - y_min as f32) / h as f32) - 1.0,
                ]);

                points
            })
        },
    );

    let glyph = Mesh::new(&verts, Topology::TriFan);
    let quad = Mesh::new(
        &[[-1.0, 1.0], [1.0, 1.0], [-1.0, -1.0], [1.0, -1.0]],
        Topology::TriStrip,
    );
    let tex_quad = Mesh::new(
        &[
            ([-1.0, 1.0], [0.0, 1.0]),
            ([1.0, 1.0], [1.0, 1.0]),
            ([-1.0, -1.0], [0.0, 0.0]),
            ([1.0, -1.0], [1.0, 0.0]),
        ],
        Topology::TriStrip,
    );

    let stencil_prog = Program::new(POS2D, WHITE);
    let glyph_prog = Program::new(POS2D_TEX2D, TEX2D);

    fb.draw(|| {
        stencil_prog.bind();
        glyph.stencil();
        clear([0.0, 0.0, 0.0, 1.0]);
        quad.draw();
    });

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
                window.draw(|| {
                    glyph_prog.bind();
                    clear([0.0, 0.0, 0.0, 1.0]);
                    color.bind();
                    tex_quad.draw();
                });
                window.swap();
            }
        }
    }

    #[cfg(feature = "minsize")]
    underscore_64::exit(0);
}
