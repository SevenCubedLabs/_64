use crate::glyph::{Glyph, GlyphMap};
use underscore_64::{
    data::List,
    render::{
        clear_color,
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

    pub fn dimensions(&self, glyphs: &GlyphMap) -> (i32, i32) {
        self.line.iter().fold((0, 0), |(w, h), &ch| {
            let glyph = glyphs.get(ch).unwrap();
            (
                w + glyph.h_advance as i32,
                if (glyph.y_max - glyph.y_min) as i32 > h {
                    (glyph.y_max - glyph.y_min) as i32
                } else {
                    h
                },
            )
        })
    }

    pub fn draw(&self, glyphs: &GlyphMap, w: i32) -> Texture {
        let (raw_w, raw_h) = self.dimensions(glyphs);

        let scale = w as f32 / raw_w as f32;
        let h = (raw_h as f32 * scale) as i32;

        log::debug!("computed texture size of {}x{}", w, h);
        let mut tex_quad = Mesh::new(
            &[
                ([-1.0, 1.0], [0.0, 1.0]),
                ([1.0, 1.0], [1.0, 1.0]),
                ([-1.0, -1.0], [0.0, 0.0]),
                ([1.0, -1.0], [1.0, 0.0]),
            ],
            Usage::StreamDraw,
            Topology::TriStrip,
        );

        let render = Texture::new(Target::Tex2d, Format::Rgb, w, h);
        let shader = Program::new(POS2D_TEX2D, TEX2D);

        Framebuffer::new(w, h)
            .with_texture(Attachment::Color0, &render)
            .draw(|_| {
                clear_color([0.0, 0.0, 0.0, 1.0]);

                shader.bind();
                let mut advance = -1.0;
                for &ch in self.line.iter() {
                    let glyph = glyphs.get(ch).unwrap();
                    log::debug!("glyph {}: {:?}", ch as char, glyph);
                    let n_advance = advance + glyph.h_advance as f32 / raw_w as f32 * 2.0;

                    // Don't draw spaces, idk why their bounding box is so wonky
                    if ch != ' ' as _ {
                        let left = advance + glyph.x_min as f32 / raw_w as f32 * 2.0;
                        let right = advance + glyph.x_max as f32 / raw_w as f32 * 2.0;

                        glyph.tex.bind();
                        tex_quad.update(&[
                            ([left, 1.0], [0.0, 1.0]),   // top left
                            ([right, 1.0], [1.0, 1.0]),  // top right
                            ([left, -1.0], [0.0, 0.0]),  // bottom left
                            ([right, -1.0], [1.0, 0.0]), // bottom right
                        ]);
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
