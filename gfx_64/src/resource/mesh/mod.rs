mod vertex;

pub use crate::resource::buffer::Usage;
use crate::{
    gl,
    gl::types::*,
    resource::{buffer::Buffer, Draw, Resource},
    GfxSystem,
};
pub use vertex::Vertex;

pub struct Mesh {
    pub(crate) vao: GLuint,
    pub(crate) vertices: Buffer,
    pub(crate) topology: Topology,
}

impl Draw for Mesh {
    fn draw(&self, ctx: &GfxSystem) {
        unsafe {
            self.bind(ctx);
            match &self.topology {
                Topology::TriIndexed(indices) => {
                    ctx.DrawElements(
                        gl::TRIANGLES,
                        indices.len as _,
                        gl::UNSIGNED_BYTE,
                        core::ptr::null(),
                    );
                }

                Topology::Lines => {
                    ctx.DrawArrays(gl::LINE_STRIP, 0, self.vertices.len as _);
                }

                Topology::Points => {
                    ctx.DrawArrays(gl::POINTS, 0, self.vertices.len as _);
                }

                Topology::TriFan => {
                    ctx.DrawArrays(gl::TRIANGLE_FAN, 0, self.vertices.len as _);
                }

                Topology::TriStrip => {
                    ctx.DrawArrays(gl::TRIANGLE_STRIP, 0, self.vertices.len as _);
                }
            }
        }
    }
}

impl Resource for Mesh {
    fn bind(&self, ctx: &GfxSystem) {
        unsafe { ctx.BindVertexArray(self.vao) }
    }
}

pub enum Topology {
    Points,
    Lines,
    TriFan,
    TriStrip,
    TriIndexed(Buffer),
}

impl Topology {
    pub fn from_index_buffer(buf: Buffer) -> Self {
        Self::TriIndexed(buf)
    }
}
