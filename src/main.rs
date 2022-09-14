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

struct Glyph {
    tex: Texture,
    w: i32,
    h: i32,
}

impl Glyph {
    pub fn bind(&self) {
        self.tex.bind();
    }
}

struct GlyphMap {
    glyphs: List<Option<Glyph>>,
}

impl GlyphMap {
    fn new(file: &str) -> Result<Self, String> {
        use std::io::Read;
        let mut glyphs = List::new(128);
        let file = std::fs::File::open(file)
            .map_err(|e| format!("file err: {}", e))?
            .bytes()
            .collect::<Result<Vec<u8>, std::io::Error>>()
            .map_err(|e| format!("file read err:  {}", e))?;

        for ch in 0..128 {
            glyphs.push(GlyphBuilder::new(&file)?.glyph(ch as u8 as char));
        }

        Ok(Self { glyphs })
    }

    fn get(&self, idx: char) -> Option<&Glyph> {
        self.glyphs[idx as _].as_ref()
    }
}

struct GlyphBuilder<'a> {
    file: &'a [u8],
    splines: List<Spline>,
    head: [f32; 2],
}

impl<'a> GlyphBuilder<'a> {
    fn new(file: &'a [u8]) -> Result<Self, String> {
        Ok(Self {
            file,
            splines: List::new(1),
            head: [0.0; 2],
        })
    }

    fn glyph(&'a mut self, ch: char) -> Option<Glyph> {
        let face = ttf_parser::Face::from_slice(self.file, 0).ok()?;
        face.glyph_index(ch).map(|idx| {
            let Rect {
                x_min,
                x_max,
                y_min,
                y_max,
            } = match face.outline_glyph(idx, self) {
                Some(rect) => {
                    println!(
                        "outlined glyph {}: {} {} {} {}",
                        ch, rect.x_min, rect.x_max, rect.y_min, rect.y_max
                    );
                    rect
                }
                None => face.global_bounding_box(),
            };

            let w = (x_max - x_min) as i32;
            let h = (y_max - y_min) as i32;

            // Build meshes
            let verts = self.splines.iter().fold(
                List::new(self.splines.len() * 100 + 1),
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

            // Prepare render target
            let tex = Texture::new(Target::Tex2d, Format::Rgb, w, h);
            let stencil = Texture::new(Target::Tex2d, Format::Stencil, w, h);

            let fb = Framebuffer::new(w, h)
                .with_texture(Attachment::Color0, &tex)
                .with_texture(Attachment::Stencil, &stencil);
            let stencil_prog = Program::new(POS2D, WHITE);

            fb.draw(|| {
                stencil_prog.bind();
                glyph.stencil();
                clear([0.0, 0.0, 0.0, 1.0]);
                quad.draw();
            });

            Glyph { tex, w, h }
        })
    }
}

impl<'a> OutlineBuilder for GlyphBuilder<'a> {
    fn move_to(&mut self, x: f32, y: f32) {
        self.splines.push(Spline::new(1));
        self.head = [x, y];
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.splines
            .tail_mut()
            .push([self.head, [x, y]].as_slice().into());
        self.head = [x, y];
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.splines
            .tail_mut()
            .push([self.head, [x1, y1], [x, y]].as_slice().into());
        self.head = [x, y];
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.splines
            .tail_mut()
            .push([self.head, [x1, y1], [x2, y2], [x, y]].as_slice().into());
        self.head = [x, y];
    }

    fn close(&mut self) {}
}

#[cfg_attr(feature = "minsize", no_mangle)]
pub fn main() {
    let window = Window::new(NAME.as_ptr(), 1920, 1080).expect("window creation failed");
    let _context = window.context();

    let glyphs =
        GlyphMap::new("assets/ttf/Hack-Regular.ttf").expect("Hack-Regular.ttf parse failed");
    println!("Hack-Regular.ttf rendered to glyph map");

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
