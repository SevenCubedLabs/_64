use base_64::{
    math::{Points, Spline},
    mem::Vec,
};
use gfx_64::{
    resource::{
        buffer::Usage,
        framebuffer::{Attachment, Framebuffer},
        mesh::{Mesh, Topology},
        pipeline::Pipeline,
        shader::{POS2D, WHITE},
        texture::Texture,
    },
    Resource,
};
use log;
pub use ttf_parser::{Face, FaceParsingError, OutlineBuilder, Rect};

pub type FontId = usize;

#[derive(Debug)]
pub struct Font {
    pub(crate) glyphs: Vec<Option<Glyph>>,
    pub(crate) pixels_per_unit: f32,
    pub(crate) line_height: i16,
}

impl Font {
    pub fn get(&self, idx: char) -> Option<&Glyph> {
        self.glyphs[idx as usize].as_ref()
    }
}

#[derive(Debug)]
pub struct Glyph {
    pub tex: Option<Texture>,
    pub size: [i32; 2],
    pub bearing: [i32; 2],
    pub h_advance: u16,
}

pub struct SplineBuilder {
    pub(crate) splines: Vec<Spline>,
    head: [f32; 2],
}

impl SplineBuilder {
    pub fn new() -> Self {
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
        let idx = self.splines.len() - 1;
        self.splines[idx].push([self.head, [x1, y1], [x2, y2], [x, y]].as_slice().into());
        self.head = [x, y];
    }

    fn close(&mut self) {}
}
