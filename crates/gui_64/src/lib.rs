#![no_std]
pub mod font;

use base_64::{
    math::{ortho, Points},
    mem::Vec,
};
use font::{Face, Font, FontId, Glyph, Rect, SplineBuilder};
use gfx_64::{
    resource::{
        framebuffer::{Attachment, Framebuffer},
        mesh::{Mesh, Topology, Usage},
        shader::{POS2D, POS2D_TEX2D, TEX2D, WHITE},
        texture::{Format, Target, Texture},
    },
    Draw, Pipeline, RenderTarget, Resource, Stencil, Uniform,
};
use ttf_parser::FaceParsingError;

pub const HACK_TTF: &[u8] = include_bytes!("../assets/Hack/build/ttf/Hack-Regular.ttf");
const TEXT_VERT: &str = concat!(include_str!("../assets/shaders/text.vert"), "\0");
const TEXT_FRAG: &str = concat!(include_str!("../assets/shaders/text.frag"), "\0");
const PIXELS_PER_EM: f32 = 16.0;

pub struct GuiSystem {
    fonts: Vec<Font>,
    txt_pipe: Pipeline,
    gui_pipe: Pipeline,
    stencil_pipe: Pipeline,
    view: Texture,
    fb: Framebuffer,
    quad: Mesh,
    dim: [f32; 2],
}

impl GuiSystem {
    pub fn new(dim: [i32; 2]) -> Self {
        log::info!("initializing GuiSystem");
        let txt_pipe = Pipeline::new(TEXT_VERT, TEXT_FRAG);
        let gui_pipe = Pipeline::new(POS2D_TEX2D, TEX2D);
        let stencil_pipe = Pipeline::new(POS2D, WHITE);

        let view = Texture::new(Target::Tex2d, dim, Format::Rgba);
        let fb = Framebuffer::new(&[Attachment::Color0], &[&view]);

        let quad = Mesh::new(
            &[
                ([-1.0, 1.0], [0.0, 1.0]),
                ([1.0, 1.0], [1.0, 1.0]),
                ([-1.0, -1.0], [0.0, 0.0]),
                ([1.0, -1.0], [1.0, 0.0]),
            ],
            Usage::StaticDraw,
            Topology::TriStrip,
        );

        let dim = [dim[0] as f32, dim[1] as f32];
        Self {
            fonts: Vec::new(),
            txt_pipe,
            gui_pipe,
            stencil_pipe,
            view,
            fb,
            quad,
            dim,
        }
    }

    pub fn load_font(&mut self, file: &[u8]) -> Result<FontId, FaceParsingError> {
        log::debug!("loading font");
        let face = Face::from_slice(file, 0)?;

        let mut glyphs = Vec::with_capacity(128);
        for ch in 0..128u8 {
            glyphs.push(self.draw_glyph(&face, ch as char));
        }

        self.fonts.push(Font {
            glyphs,
            pixels_per_unit: PIXELS_PER_EM / face.units_per_em() as f32,
            line_height: face.height(),
        });
        Ok(self.fonts.len() - 1)
    }

    pub fn draw_text(&mut self, font_id: FontId, text: &str, [x, y]: [f32; 2], em: f32) {
        let font = &self.fonts[font_id];
        let scale = em * font.pixels_per_unit;
        self.fb.bind();
        self.fb
            .viewport([0, 0], [self.dim[0] as i32, self.dim[1] as i32]);
        self.txt_pipe.bind();

        ortho([0.0, 0.0], self.dim).bind(0);
        let mut y = y;
        for line in text.split(|byte| byte == '\n') {
            y -= font.line_height as f32 * scale;

            let mut x = x;
            for ch in line.chars() {
                let glyph = font.get(ch).expect("character not found");
                if let Some(tex) = &glyph.tex {
                    let [w, h] = [glyph.size[0] as f32 * scale, glyph.size[1] as f32 * scale];
                    let [dx, dy] = [
                        glyph.bearing[0] as f32 * scale,
                        glyph.bearing[1] as f32 * scale,
                    ];

                    let left = x + dx;
                    let right = left + w;
                    let bottom = y + dy;
                    let top = bottom + h;

                    tex.bind();
                    Mesh::new(
                        &[
                            ([left, top], [0.0, 1.0]),
                            ([right, top], [1.0, 1.0]),
                            ([left, bottom], [0.0, 0.0]),
                            ([right, bottom], [1.0, 0.0]),
                        ],
                        Usage::StaticDraw,
                        Topology::TriStrip,
                    )
                    .draw();
                }

                x += glyph.h_advance as f32 * scale;
            }
        }
    }

    fn draw_glyph(&mut self, face: &Face, ch: char) -> Option<Glyph> {
        let mut outline = SplineBuilder::new();
        face.glyph_index(ch).map_or(None, |idx| {
            face.outline_glyph(idx, &mut outline).map_or(
                None,
                |Rect {
                     x_max,
                     x_min,
                     y_max,
                     y_min,
                 }| {
                    log::debug!("rendering glyph '{}' ({}u8)", ch, ch as u8);
                    let bearing = [
                        face.glyph_hor_side_bearing(idx).unwrap_or(0) as i32,
                        face.glyph_ver_side_bearing(idx).unwrap_or(0) as i32,
                    ];

                    let size = [(x_max - x_min) as i32, (y_max - y_min) as i32];

                    // Build meshes
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

                    log::trace!("calculated glyph vertices: {:?}", verts);
                    let glyph = Mesh::new(&verts, Usage::StaticDraw, Topology::TriFan);

                    log::trace!("initializing full screen quad");
                    let quad = Mesh::new(
                        &[[-1.0, 1.0], [1.0, 1.0], [-1.0, -1.0], [1.0, -1.0]],
                        Usage::StaticDraw,
                        Topology::TriStrip,
                    );

                    log::trace!("preparing glyph render target");
                    let tex = Texture::new(Target::Tex2d, size, Format::Rgba);
                    let stencil = Texture::new(Target::Tex2d, size, Format::Stencil);
                    let fb = Framebuffer::new(
                        &[Attachment::Color0, Attachment::Stencil],
                        &[&tex, &stencil],
                    );

                    fb.bind();
                    fb.viewport([0, 0], size);
                    fb.clear_color([0.0, 0.0, 0.0, 0.0]);

                    self.stencil_pipe.bind();
                    glyph.stencil();
                    quad.draw();

                    Some(Glyph {
                        tex: Some(tex),
                        size,
                        bearing,
                        h_advance: face.glyph_hor_advance(idx).unwrap_or(0),
                    })
                },
            )
        })
    }
}

impl Draw for GuiSystem {
    fn draw(&self) {
        log::debug!("drawing gui");
        self.gui_pipe.bind();
        self.view.bind();
        self.quad.draw();
    }
}
