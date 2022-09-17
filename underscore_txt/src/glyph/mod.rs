use ttf_parser::{Face, OutlineBuilder, Rect};
use underscore_64::{
    data::List,
    math::Spline,
    render::{
        framebuffer::{Attachment, Framebuffer},
        mesh::{Mesh, Topology, Usage},
        program::Program,
        shaders::{POS2D, WHITE},
        target::RenderTarget,
        texture::{Format, Target, Texture},
    },
};

#[derive(Debug)]
pub struct Glyph {
    pub(crate) tex: Option<Texture>,
    pub(crate) x_min: i16,
    pub(crate) x_max: i16,
    pub(crate) y_min: i16,
    pub(crate) y_max: i16,
    pub(crate) h_advance: u16,
}

#[derive(Debug)]
pub struct GlyphMap {
    glyphs: List<Option<Glyph>>,
}

impl GlyphMap {
    pub fn new(file: &[u8]) -> Option<Self> {
        let mut glyphs = List::new(128);

        let face = Face::from_slice(file, 0).ok()?;
        let builder = GlyphBuilder::new(face);
        for ch in 0..128 {
            let glyph = builder.glyph(ch as u8 as char);
            glyphs.push(glyph);
        }

        Some(Self { glyphs })
    }

    pub fn get(&self, idx: u8) -> Option<&Glyph> {
        self.glyphs[idx as _].as_ref()
    }
}

struct SplineBuilder {
    splines: List<Spline>,
    head: [f32; 2],
}

impl SplineBuilder {
    fn new() -> Self {
        Self {
            splines: List::new(1),
            head: [0.0; 2],
        }
    }
}

struct GlyphBuilder<'a> {
    face: Face<'a>,
    stencil: Program,
}

impl<'a> GlyphBuilder<'a> {
    fn new(face: Face<'a>) -> Self {
        Self {
            face,
            stencil: Program::new(POS2D, WHITE),
        }
    }

    fn glyph(&self, ch: char) -> Option<Glyph> {
        let mut outline = SplineBuilder::new();
        self.face.glyph_index(ch).map(|idx| {
            let Rect {
                x_min,
                x_max,
                y_min,
                y_max,
            } = match self.face.outline_glyph(idx, &mut outline) {
                Some(rect) => rect,
                None => Rect {
                    x_min: 0,
                    x_max: 0,
                    y_min: 0,
                    y_max: 0,
                },
            };

            let w = (x_max - x_min) as i32;
            let h = (y_max - y_min) as i32;

            // Build meshes
            let verts = outline.splines.iter().fold(
                List::new(outline.splines.len() * 100 + 1),
                |mut points, spline| {
                    points.push([0.0, 0.0]);
                    spline.points().iter().fold(points, |mut points, point| {
                        let point = [
                            (2.0 * (point[0] - x_min as f32) / w as f32) - 1.0,
                            (2.0 * (point[1] - y_min as f32) / h as f32) - 1.0,
                        ];
                        points.push(point);

                        points
                    })
                },
            );

            let glyph = Mesh::new(&verts, Usage::StaticDraw, Topology::TriFan);
            let quad = Mesh::new(
                &[[-1.0, 1.0], [1.0, 1.0], [-1.0, -1.0], [1.0, -1.0]],
                Usage::StaticDraw,
                Topology::TriStrip,
            );

            // Prepare render target
            let tex = Texture::new(Target::Tex2d, Format::Rgb, w, h);
            let stencil = Texture::new(Target::Tex2d, Format::Stencil, w, h);

            let fb = Framebuffer::new(w, h)
                .with_texture(Attachment::Color0, &tex)
                .with_texture(Attachment::Stencil, &stencil);

            fb.draw(|buf| {
                self.stencil.bind();
                buf.clear_color([0.0, 0.0, 0.0, 1.0]);
                glyph.stencil();
                quad.draw();
            });

            Glyph {
                tex: Some(tex),
                x_min,
                x_max,
                y_min,
                y_max,
                h_advance: self.face.glyph_hor_advance(idx).unwrap(),
            }
        })
    }
}

impl OutlineBuilder for SplineBuilder {
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
