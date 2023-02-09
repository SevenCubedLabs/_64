use log_64 as log;
extern crate alloc;
use {
    alloc::vec::Vec,
    gfx_64::{
        resource::{
            buffer::Usage,
            framebuffer::{Attachment, Framebuffer},
            mesh::{Mesh, Topology},
            program::Program,
            shader::{POS2D, WHITE},
            texture::{Texture, TEX_2D},
        },
        Resource, Target,
    },
    math_64::{Points, Spline},
    ttf_parser::{Face, FaceParsingError, OutlineBuilder, Rect},
};

const PIXELS_PER_EM: f32 = 16.0;

#[derive(Debug)]
pub struct Font {
    glyphs: Vec<Option<Glyph>>,
    pub pixels_per_unit: f32,
    pub line_height: i16,
}

impl Font {
    pub fn new(file: &[u8]) -> Result<Self, FaceParsingError> {
        let mut glyphs = Vec::with_capacity(128);

        let face = Face::from_slice(file, 0)?;
        let builder = GlyphBuilder::new(&face);
        for ch in 0..128u8 {
            let glyph = builder.glyph(ch as char);
            glyphs.push(glyph);
        }

        Ok(Self {
            glyphs,
            pixels_per_unit: PIXELS_PER_EM / face.units_per_em() as f32,
            line_height: face.height(),
        })
    }

    pub fn get(&self, idx: u8) -> Option<&Glyph> {
        self.glyphs[idx as usize].as_ref()
    }

    pub fn draw(&self, text: &[u8], [x, y]: [f32; 2], em: f32, target: &impl Target) {
        let scale = em * self.pixels_per_unit;
        target.bind();

        let mut y = y;
        for line in text.split(|&byte| byte as char == '\n') {
            let mut x = x;
            for &ch in line {
                let glyph = self.get(ch).expect("character not found");
                if let Some(tex) = &glyph.tex {
                    let [w, h] = [glyph.size[0] as f32 * scale, glyph.size[1] as f32 * scale];

                    let xpos = x + glyph.bearing[0] as f32 * scale;
                    let ypos = y - (glyph.size[1] - glyph.bearing[1]) as f32 * scale;

                    log::debug!("rendering {} at ({}, {})", ch as char, xpos, ypos);
                    /*
                    target.viewport(
                        [
                            x + glyph.bearing[0] as f32 * scale,
                            y - ((h as i32 - glyph.size[1] + glyph.bearing[1]) as f32 * scale)
                                as i32,
                        ],
                        [
                            (glyph.h_advance as f32 * scale) as i32,
                            (h as f32 * scale) as i32,
                        ],
                    );
                    */

                    tex.bind();
                    Mesh::new(
                        &[
                            ([xpos, ypos], [0.0, 1.0]),
                            ([xpos + w, ypos], [1.0, 1.0]),
                            ([xpos, ypos - h], [0.0, 0.0]),
                            ([xpos + w, ypos - h], [1.0, 0.0]),
                        ],
                        Usage::StaticDraw,
                        Topology::TriStrip,
                    )
                    .draw();
                }

                x += glyph.h_advance as f32 * scale;
            }

            y -= self.line_height as f32 * scale
        }
    }
}

#[derive(Debug)]
pub struct Glyph {
    pub tex: Option<Texture<[f32; 4]>>,
    pub size: [i32; 2],
    pub bearing: [i32; 2],
    pub h_advance: u16,
}

struct GlyphBuilder<'a> {
    face: &'a Face<'a>,
    stencil: Program,
}

impl<'a> GlyphBuilder<'a> {
    fn new(face: &'a Face<'a>) -> Self {
        Self {
            face,
            stencil: Program::new(POS2D, WHITE),
        }
    }

    fn glyph(&self, ch: char) -> Option<Glyph> {
        log::debug!("rendering '{}'", ch);
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

            let bearing = [
                self.face.glyph_hor_side_bearing(idx).unwrap_or(0) as i32,
                self.face.glyph_ver_side_bearing(idx).unwrap_or(0) as i32,
            ];

            let size = [(x_max - x_min) as i32, (y_max - y_min) as i32];

            let verts = outline.splines.iter().fold(
                Vec::with_capacity(outline.splines.len() * 100 + 1),
                |mut points, spline| {
                    points.push([0.0, 0.0]);
                    spline.points().iter().fold(points, |mut points, point| {
                        let point = [
                            (2.0 * (point[0] - x_min as f32) / size[0] as f32) - 1.0,
                            (2.0 * (point[1] - y_min as f32) / size[1] as f32) - 1.0,
                        ];
                        points.push(point);

                        points
                    })
                },
            );

            log::debug!("calculated glyph vertices: {:?}", verts);
            let glyph = Mesh::new(&verts, Usage::StaticDraw, Topology::TriFan);
            let quad = Mesh::new(
                &[[-1.0, 1.0], [1.0, 1.0], [-1.0, -1.0], [1.0, -1.0]],
                Usage::StaticDraw,
                Topology::TriStrip,
            );

            let tex: Texture<[f32; 4]> = Texture::new(TEX_2D, size);
            let stencil: Texture<i32> = Texture::new(TEX_2D, size);

            let fb = Framebuffer::new();
            fb.attach(Attachment::Color0, &tex);
            fb.attach(Attachment::Stencil, &stencil);

            fb.bind();
            fb.viewport([0, 0], size);
            fb.clear_color([0.0, 0.0, 0.0, 0.0]);

            self.stencil.bind();
            stencil.bind();
            glyph.stencil();
            quad.draw();

            Glyph {
                tex: Some(tex),
                size,
                bearing,
                h_advance: self.face.glyph_hor_advance(idx).unwrap_or(0),
            }
        })
    }
}

struct SplineBuilder {
    splines: Vec<Spline>,
    head: [f32; 2],
}

impl SplineBuilder {
    fn new() -> Self {
        Self {
            splines: Vec::with_capacity(1),
            head: [0.0; 2],
        }
    }
}

impl OutlineBuilder for SplineBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        self.splines.push(Spline::with_capacity(1));
        self.head = [x, y];
    }

    fn line_to(&mut self, x: f32, y: f32) {
        let idx = self.splines.len() - 1;
        self.splines[idx].push([self.head, [x, y]].as_slice().into());
        self.head = [x, y];
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        let idx = self.splines.len() - 1;
        self.splines[idx].push([self.head, [x1, y1], [x, y]].as_slice().into());
        self.head = [x, y];
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        todo!();
        /*
        let idx = self.splines.len() - 1;
        self.splines[idx].push([self.head, [x1, y1], [x2, y2], [x, y]].as_slice().into());
        self.head = [x, y];
        */
    }

    fn close(&mut self) {}
}
