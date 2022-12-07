mod vertex;

pub use crate::resource::buffer::Usage;
use crate::{
    gl,
    gl::types::*,
    resource::{buffer::Buffer, Draw, Resource},
    GfxSystem, Stencil,
};
pub use vertex::Vertex;

pub type MeshId = GLuint;

#[derive(Clone, Debug)]
pub struct Mesh {
    pub(crate) id: MeshId,
    pub(crate) vertices: Buffer,
    pub(crate) topology: Topology,
}

impl Mesh {
    pub fn new<V: Vertex>(verts: &[V], usage: Usage, topology: Topology) -> Mesh {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
            log::trace!("vao {:?} created", id);
            gl::BindVertexArray(id);
        }
        log::info!("mesh {:?} created", id);

        match &topology {
            Topology::TriIndexed(indices) => {
                indices.bind();
            }
            _ => {}
        }

        let vertices = Buffer::new(gl::ARRAY_BUFFER, usage, verts);
        V::bind();

        let mesh = Mesh {
            id,
            vertices,
            topology,
        };
        mesh
    }
}

impl Draw for Mesh {
    fn draw(&self) {
        unsafe {
            self.bind();
            match &self.topology {
                Topology::TriIndexed(indices) => {
                    gl::DrawElements(
                        gl::TRIANGLES,
                        indices.len as _,
                        gl::UNSIGNED_BYTE,
                        core::ptr::null(),
                    );
                }

                Topology::Lines => {
                    gl::DrawArrays(gl::LINE_STRIP, 0, self.vertices.len as _);
                }

                Topology::Points => {
                    gl::DrawArrays(gl::POINTS, 0, self.vertices.len as _);
                }

                Topology::TriFan => {
                    gl::DrawArrays(gl::TRIANGLE_FAN, 0, self.vertices.len as _);
                }

                Topology::TriStrip => {
                    gl::DrawArrays(gl::TRIANGLE_STRIP, 0, self.vertices.len as _);
                }
            }
        }
    }
}

impl Stencil for Mesh {}

impl Resource for Mesh {
    fn bind(&self) {
        unsafe { gl::BindVertexArray(self.id) }
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
