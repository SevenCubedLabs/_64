mod glyph;

use glyph::GlyphMap;
use ttf_parser::FaceParsingError;
use underscore_64::data::List;
use underscore_gfx::{
    framebuffer::{Attachment, Framebuffer},
    program::Program,
    shaders::{POS2D_TEX2D, TEX2D},
    target::RenderTarget,
    texture::{Format, Target, Texture},
};

pub struct Text {
    columns: i32,
    w: i32,
    h: i32,
    lines: List<List<u8>>,
    font: usize,
    tex: Texture,
    buf: Framebuffer,
}

pub struct TextSystem {
    fonts: List<GlyphMap>,
}

impl TextSystem {
    pub fn new() -> Self {
        Self {
            fonts: List::new(1),
        }
    }

    pub fn load_font(&mut self, file: &[u8]) -> Result<usize, FaceParsingError> {
        let new_glyphs = GlyphMap::new(file)?;
        self.fonts.push(new_glyphs);
        Ok(self.fonts.len() - 1)
    }

    pub fn draw<'a>(&'a mut self, text: &'a mut Text) -> &'a Texture {
        let shader = Program::new(POS2D_TEX2D, TEX2D);
        shader.bind();

        text.buf.clear_color([0.0, 0.0, 0.0, 0.0]);

        let mut h = text.h;
        let ch_width = text.w / text.columns;
        for line in text.lines.iter() {
            h -= (1.3
                * self.fonts[text.font].draw(line, line.len() as i32 * ch_width, h, &mut text.buf)
                    as f32) as i32;
        }

        &text.tex
    }
}

impl Text {
    pub fn new(columns: i32, [w, h]: [i32; 2]) -> Self {
        let tex = Texture::new(Target::Tex2d, Format::Rgba, w, h);
        let buf = Framebuffer::new(w, h).with_texture(Attachment::Color0, &tex);
        Self {
            columns,
            w,
            h,
            lines: List::new(1),
            tex,
            buf,
            font: 0,
        }
    }

    pub fn update(&mut self, text: &str) {
        self.lines = text
            .split('\n')
            .map(|line| {
                let bytes: List<u8> = line.as_bytes().into();
                bytes
            })
            .collect();
    }
}
