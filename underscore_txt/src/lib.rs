use ttf_parser::{OutlineBuilder, Rect};
use underscore_64::{
    data::List,
    math::Spline,
    render::{
        clear,
        framebuffer::{Attachment, Framebuffer},
        mesh::{Mesh, Topology},
        program::Program,
        shaders::{POS2D, WHITE},
        target::RenderTarget,
        texture::{Format, Target, Texture},
    },
};

#[derive(Debug)]
pub struct Glyph {
    tex: Texture,
    w: i32,
    h: i32,
    x_min: i16,
    y_min: i16,
    h_advance: u16,
}

impl Glyph {
    pub fn bind(&self) {
        self.tex.bind();
    }
}

#[derive(Debug)]
pub struct GlyphMap {
    y_origin: i16,
    glyphs: List<Option<Glyph>>,
}

impl GlyphMap {
    pub fn new(file: &str) -> Result<Self, String> {
        use std::io::Read;
        let mut glyphs = List::new(128);
        let file = std::fs::File::open(file)
            .map_err(|e| format!("file err: {}", e))?
            .bytes()
            .collect::<Result<Vec<u8>, std::io::Error>>()
            .map_err(|e| format!("file read err:  {}", e))?;
        let y_origin = -GlyphBuilder::new(&file)?
            .descender()
            .ok_or(format!("no descender found"))?;

        for ch in 0..128 {
            let glyph = GlyphBuilder::new(&file)?.glyph(ch as u8 as char);
            println!("{:?}", glyph);
            glyphs.push(glyph);
        }

        Ok(Self { y_origin, glyphs })
    }

    pub fn get(&self, idx: char) -> Option<&Glyph> {
        self.glyphs[idx as _].as_ref()
    }
}

struct GlyphBuilder<'a> {
    file: &'a [u8],
    splines: List<Spline>,
    head: [f32; 2],
    stencil: Program,
}

impl<'a> GlyphBuilder<'a> {
    fn new(file: &'a [u8]) -> Result<Self, String> {
        Ok(Self {
            file,
            splines: List::new(1),
            head: [0.0; 2],
            stencil: Program::new(POS2D, WHITE),
        })
    }

    fn descender(&self) -> Option<i16> {
        Some(ttf_parser::Face::from_slice(self.file, 0).ok()?.descender())
    }

    fn glyph(&'a mut self, ch: char) -> Option<Glyph> {
        let face = ttf_parser::Face::from_slice(self.file, 0).ok()?;
        face.glyph_index(ch).map(|idx| {
            let Rect {
                x_min,
                x_max,
                y_min,
                y_max,
            } = match face.outline_glyph(idx, self) {
                Some(rect) => {
                    println!(
                        "outlined glyph {}: {} {} {} {}",
                        ch, rect.x_min, rect.x_max, rect.y_min, rect.y_max
                    );
                    rect
                }
                None => face.global_bounding_box(),
            };

            let w = (x_max - x_min) as i32;
            let h = (y_max - y_min) as i32;
            println!("width: {}", w);
            println!("height: {}", h);

            // Build meshes
            let verts = self.splines.iter().fold(
                List::new(self.splines.len() * 100 + 1),
                |mut points, spline| {
                    points.push([0.0, 0.0]);
                    spline.points().iter().fold(points, |mut points, point| {
                        let point = [
                            (2.0 * (point[0] - x_min as f32) / w as f32) - 1.0,
                            (2.0 * (point[1] - y_min as f32) / h as f32) - 1.0,
                        ];
                        points.push(point);

                        points
                    })
                },
            );

            let glyph = Mesh::new(&verts, Topology::TriFan);
            let quad = Mesh::new(
                &[[-1.0, 1.0], [1.0, 1.0], [-1.0, -1.0], [1.0, -1.0]],
                Topology::TriStrip,
            );

            // Prepare render target
            let tex = Texture::new(Target::Tex2d, Format::Rgb, w, h);
            let stencil = Texture::new(Target::Tex2d, Format::Stencil, w, h);

            let fb = Framebuffer::new(w, h)
                .with_texture(Attachment::Color0, &tex)
                .with_texture(Attachment::Stencil, &stencil);

            fb.draw(|| {
                self.stencil.bind();
                glyph.stencil();
                clear([0.0, 0.0, 0.0, 1.0]);
                quad.draw();
            });

            Glyph {
                tex,
                w,
                h,
                y_min,
                x_min,
                h_advance: face.glyph_hor_advance(idx).unwrap(),
            }
        })
    }
}

impl<'a> OutlineBuilder for GlyphBuilder<'a> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
