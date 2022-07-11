use super::{buffer::Buffer, vertex::Vertex};
use _sys::*;

pub enum Topology {
    Curve,
    IdxTriangles(Buffer),
}

impl Topology {
    pub fn idx_triangles(idx: &[u8]) -> Self {
        Self::IdxTriangles(Buffer::new(GL_ELEMENT_ARRAY_BUFFER, idx))
    }
}

pub struct Mesh {
    vao: GLuint,
    vertices: Buffer,
    topology: Topology,
}

impl Mesh {
    pub fn new<V: Vertex>(verts: &[V], topology: Topology) -> Self {
        let mut vao = 0;
        unsafe {
            glGenVertexArrays(1, &mut vao);
            glBindVertexArray(vao);
        }

        match &topology {
            Topology::IdxTriangles(indices) => {
                indices.bind();
            }
            _ => {}
        }

        let vertices = Buffer::new(GL_ARRAY_BUFFER, verts);
        V::enable(0);

        Self {
            vao,
            vertices,
            topology,
        }
    }

    pub fn draw(&self) {
        unsafe {
            glBindVertexArray(self.vao);
            match &self.topology {
                Topology::IdxTriangles(indices) => {
                    glDrawElements(
                        GL_TRIANGLES,
                        indices.len() as _,
                        GL_UNSIGNED_BYTE,
                        core::ptr::null(),
                    );
                }

                Topology::Curve => {
                    glDrawArrays(GL_LINE_STRIP, 0, self.vertices.len() as _);
                }
            }
        }
    }
}
