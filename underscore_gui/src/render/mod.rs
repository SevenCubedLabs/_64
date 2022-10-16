pub mod glyph;

use glyph::GlyphMap;
use ttf_parser::{FaceParsingError, OutlineBuilder};
use underscore_64::{
    data::List,
    math::Spline,
    render::{
        framebuffer::{Attachment, Framebuffer},
        program::Program,
        shaders::{POS2D_TEX2D, TEX2D},
        target::RenderTarget,
        texture::{Format, Target, Texture},
    },
};

pub struct TtfSystem {
    fonts: List<GlyphMap>,
}

impl TtfSystem {
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

    pub fn draw(
        &mut self,
        text: &str,
        font: usize,
        [w, mut h]: [i32; 2],
        ch_width: i32,
    ) -> Texture {
        let shader = Program::new(POS2D_TEX2D, TEX2D);
        shader.bind();

        let render = Texture::new(Target::Tex2d, Format::Rgb, w, h);
        let mut txt_buf = Framebuffer::new(w, h).with_texture(Attachment::Color0, &render);
        txt_buf.clear_color([0.0, 0.0, 0.0, 0.0]);

        for line in text.lines() {
            h -= (1.3
                * self.fonts[font].draw(line, line.len() as i32 * ch_width, h, &mut txt_buf) as f32)
                as i32;
        }
        render
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
