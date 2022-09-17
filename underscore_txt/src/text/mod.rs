use crate::glyph::GlyphMap;
use underscore_64::{
    data::List,
    render::{
        framebuffer::{Attachment, Framebuffer},
        mesh::{Mesh, Topology, Usage},
        program::Program,
        shaders::{POS2D_TEX2D, TEX2D},
        target::RenderTarget,
        texture::{Format, Target, Texture},
    },
};

pub struct TextBox {
    line: List<u8>,
}

impl TextBox {
    pub fn new(n: usize) -> Self {
        Self { line: List::new(n) }
    }

    pub fn dimensions(&self, glyphs: &GlyphMap) -> (i32, i32, i32) {
        self.line.iter().fold((0, 0, 0), |(w, h, y_origin), &ch| {
            let glyph = glyphs.get(ch).unwrap();
            (
                w + glyph.h_advance as i32,
                if (glyph.y_max - glyph.y_min) as i32 > h {
                    (glyph.y_max - glyph.y_min) as i32
                } else {
                    h
                },
                if -glyph.y_min as i32 > y_origin {
                    -glyph.y_min as i32
                } else {
                    y_origin
                },
            )
        })
    }

    pub fn draw(&self, glyphs: &GlyphMap, line_width: i32) -> Texture {
        let (w, h, y_origin) = self.dimensions(glyphs);

        let scale = line_width as f32 / w as f32;
        let w = (w as f32 * scale).ceil() as i32;
        let h = (h as f32 * scale).ceil() as i32;
        let y_origin = (y_origin as f32 * scale).ceil() as i32;
        log::debug!("computed texture size of {}x{}", w, h);

        let tex_quad = Mesh::new(
            &[
                ([-1.0, 1.0], [0.0, 1.0]),
                ([1.0, 1.0], [1.0, 1.0]),
                ([-1.0, -1.0], [0.0, 0.0]),
                ([1.0, -1.0], [1.0, 0.0]),
            ],
            Usage::StaticDraw,
            Topology::TriStrip,
        );

        let render = Texture::new(Target::Tex2d, Format::Rgb, w, h);
        let shader = Program::new(POS2D_TEX2D, TEX2D);

        Framebuffer::new(w, h)
            .with_texture(Attachment::Color0, &render)
            .draw(|buf| {
                buf.clear_color([0.0, 0.0, 0.0, 1.0]);

                shader.bind();
                let mut advance = 0;
                for &ch in self.line.iter() {
                    let glyph = glyphs.get(ch).unwrap();
                    let n_advance = advance + (glyph.h_advance as f32 * scale) as i32;
                    let x_min = advance + (glyph.x_min as f32 * scale) as i32;
                    let y_min = (glyph.y_min as f32 * scale) as i32;
                    let x_max = advance + (glyph.x_max as f32 * scale) as i32;
                    let y_max = (glyph.y_max as f32 * scale) as i32;
                    log::debug!(
                        "drawing glyph {} with viewort {}, {}, {}, {}",
                        ch as char,
                        x_min,
                        y_origin + y_min,
                        x_max - x_min,
                        y_max - y_min,
                    );

                    if let Some(tex) = &glyph.tex {
                        buf.set_viewport(x_min, y_origin + y_min, x_max - x_min, y_max - y_min);

                        tex.bind();
                        tex_quad.draw();
                    }

                    advance = n_advance
                }
            });

        render
    }
}

impl From<&[u8]> for TextBox {
    fn from(bytes: &[u8]) -> Self {
        Self { line: bytes.into() }
    }
}
