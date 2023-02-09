pub mod font;

extern crate alloc;
use {
    alloc::vec::Vec,
    font::Font,
    gfx_64::{
        resource::{
            framebuffer::{Attachment, Framebuffer},
            mesh::{Mesh, Topology, Usage},
            program::Program,
            texture::{Texture, TextureRgba, TEX_2D},
        },
        Resource, Target, Uniform,
    },
    log_64 as log,
    math_64::ortho,
    ttf_parser::FaceParsingError,
};

pub type FontId = usize;

pub struct TextSystem {
    fonts: Vec<Font>,
    program: Program,
}

static TEXT_VERT: &str = concat!(include_str!("shaders/text.vert"), "\0");
static TEXT_FRAG: &str = concat!(include_str!("shaders/text.frag"), "\0");

impl TextSystem {
    pub fn new() -> Self {
        Self {
            fonts: Vec::with_capacity(1),
            program: Program::new(TEXT_VERT, TEXT_FRAG),
        }
    }

    pub fn load_font(&mut self, file: &[u8]) -> Result<FontId, FaceParsingError> {
        let new_glyphs = Font::new(file)?;
        self.fonts.push(new_glyphs);
        Ok(self.fonts.len() - 1)
    }

    pub fn draw(&self, text: &Text, [w, h]: [i32; 2], font: FontId, em: f32) {
        log::debug!("rendering {:?}", text);
        self.program.bind();

        text.buf.bind();
        text.buf.viewport([0, 0], [w, h]);
        text.buf.clear_color([0.0, 0.0, 0.0, 1.0]);

        ortho([0.0, 0.0], [w as f32, h as f32]).bind(0);

        let font = &self.fonts[font];
        let scale = em * font.pixels_per_unit;

        let mut y = h as f32;
        for line in text.text.split(|&byte| byte as char == '\n') {
            y -= font.line_height as f32 * scale;

            let mut x = 0.0;
            for &ch in line {
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

                    log::debug!(
                        "rendering {} with bottom left ({}, {}) and top right ({}, {})",
                        ch as char,
                        left,
                        bottom,
                        right,
                        top
                    );

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
}

impl Default for TextSystem {
    fn default() -> Self {
        let mut txt = TextSystem::new();
        txt.load_font(include_bytes!("ttf/Hack-Regular.ttf"))
            .unwrap();

        txt
    }
}

#[derive(Debug)]
pub struct Text {
    text: Vec<u8>,
    tex: TextureRgba,
    buf: Framebuffer,
}

impl Text {
    pub fn new(size: [i32; 2]) -> Self {
        let tex = Texture::new(TEX_2D, size);
        let buf = Framebuffer::new();
        buf.attach(Attachment::Color0, &tex);

        Self {
            text: Vec::with_capacity(1),
            tex,
            buf,
        }
    }

    pub fn update(&mut self, text: &str) {
        self.text = text.as_bytes().into();
    }

    pub fn view(&self) -> &TextureRgba {
        &self.tex
    }
}
