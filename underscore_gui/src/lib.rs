pub mod font;
pub mod text;

use font::Font;
use text::Text;
use underscore_64::{data::List, log, math::ortho};
use underscore_gfx::{
    resource::{
        mesh::{Mesh, Topology, Usage},
        program::Program,
    },
    Resource, Target, Uniform,
};

pub type FontId = usize;

pub struct TextSystem {
    fonts: List<Font>,
    program: Program,
}

static TEXT_VERT: &str = concat!(include_str!("shaders/text.vert"), "\0");
static TEXT_FRAG: &str = concat!(include_str!("shaders/text.frag"), "\0");

impl TextSystem {
    pub fn new() -> Self {
        Self {
            fonts: List::new(1),
            program: Program::new(TEXT_VERT, TEXT_FRAG),
        }
    }

    pub fn load_font(&mut self, file: &[u8]) -> Result<FontId, String> {
        let new_glyphs = Font::new(file).map_err(|e| e.to_string())?;
        self.fonts.push(new_glyphs);
        Ok(self.fonts.len() - 1)
    }

    pub fn draw(&self, text: &Text, font: FontId, em: f32) {
        log::debug!("rendering {:?}", text);
        self.program.bind();

        text.buf.bind();
        text.buf.viewport([0, 0], [text.w, text.h]);
        text.buf.clear_color([0.0, 0.0, 0.0, 1.0]);

        ortho([0.0, 0.0], [text.w as f32, text.h as f32]).bind(0);

        let font = &self.fonts[font];
        let scale = em * font.pixels_per_unit;

        let mut y = text.h as f32;
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
        let mut gui = TextSystem::new();
        gui.load_font(include_bytes!("../assets/ttf/Hack-Regular.ttf"))
            .expect("couldn't parse ../assets/ttf/Hack-Regular.ttf");

        gui
    }
}
