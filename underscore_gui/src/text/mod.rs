use crate::font::Font;
use underscore_64::data::List;
use underscore_gfx::{
    assets::shaders::{POS2D_TEX2D, TEX2D},
    resource::{
        framebuffer::{Attachment, Framebuffer},
        mesh::{Mesh, Topology, Usage},
        program::Program,
        texture::{Texture, TextureRgba, TEX_2D},
        Resource, Target,
    },
};

pub type TextView = underscore_gfx::resource::texture::TextureRgba;

#[derive(Debug)]
pub struct Text {
    pub(crate) columns: i32,
    pub(crate) w: i32,
    pub(crate) h: i32,
    pub(crate) text: List<u8>,
    pub(crate) tex: TextureRgba,
    pub(crate) buf: Framebuffer,
}

impl Text {
    pub fn new(columns: i32, [w, h]: [i32; 2]) -> Self {
        let tex = Texture::new(TEX_2D, [w, h]);
        let buf = Framebuffer::new();
        buf.attach(Attachment::Color0, &tex);
        Self {
            columns,
            w,
            h,
            text: List::new(1),
            tex,
            buf,
        }
    }

    pub fn update(&mut self, text: &str) {
        self.text = text.as_bytes().into();
    }

    pub fn view(&self) -> &TextView {
        &self.tex
    }
}
