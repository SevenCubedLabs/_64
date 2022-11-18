pub mod font;
pub mod text;

use font::Font;
use text::Text;
use underscore_64::data::List;
use underscore_gfx::{
    assets::shaders::{POS2D_TEX2D, TEX2D},
    resource::{program::Program, Resource, Target},
};

pub type FontId = usize;
pub type TextView = underscore_gfx::resource::texture::TextureRgba;

pub struct TextSystem {
    fonts: List<Font>,
    program: Program,
}

impl TextSystem {
    pub fn new() -> Self {
        Self {
            fonts: List::new(1),
            program: Program::new(POS2D_TEX2D, TEX2D),
        }
    }

    pub fn load_font(&mut self, file: &[u8]) -> Result<FontId, String> {
        let new_glyphs = Font::new(file).map_err(|e| e.to_string())?;
        self.fonts.push(new_glyphs);
        Ok(self.fonts.len() - 1)
    }

    pub fn render(&self, font: FontId, text: &mut Text) {
        log::debug!("rendering {:?}", text);
        self.program.bind();

        text.buf.clear_color([0.0, 0.0, 0.0, 1.0]);

        let mut h = text.h;
        let ch_width = text.w / text.columns;
        for line in text.lines.iter() {
            log::debug!("drawing line: {}", unsafe {
                core::str::from_utf8_unchecked(line)
            });

            h -= (1.3
                * self.fonts[font].draw(line, line.len() as i32 * ch_width, h, &text.buf) as f32)
                as i32;
        }
    }
}

impl Default for TextSystem {
    fn default() -> Self {
        let mut gui = TextSystem::new();
        gui.load_font(include_bytes!("../assets/ttf/Hack-Regular.ttf"))
            .expect("couldn't parse ../assets/ttf/Hack-Regular.ttf");

        gui
    }
}
