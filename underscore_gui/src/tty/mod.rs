use underscore_64::{
    data::List,
    render::{
        framebuffer::{Attachment, Framebuffer},
        texture::{Format, Target, Texture},
    },
};

pub struct TextBox {
    columns: i32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    lines: List<List<u8>>,
    tex: Texture,
    buf: Framebuffer,
}

impl TextBox {
    fn new(columns: i32, [x, y]: [i32; 2], [w, h]: [i32; 2]) -> Self {
        let tex = Texture::new(Target::Tex2d, Format::Rgba, w, h);
        let buf = Framebuffer::new(w, h).with_texture(Attachment::Color0, &tex);
        Self {
            columns,
            x,
            y,
            w,
            h,
            lines: List::new(1),
            tex,
            buf,
        }
    }

    fn draw(&self) {}
}
