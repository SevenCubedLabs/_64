use super::buffer::Buffer;
use underscore_sys::*;

mod vertex;
use vertex::Vertex;

pub struct Mesh {
    vao: GLuint,
    vertices: Buffer,
    topology: Topology,
}

impl Mesh {
    pub fn new<Verts: Vertex>(verts: &[Verts], topology: Topology) -> Self {
        let mut vao = 0;
        unsafe {
            glGenVertexArrays(1, &mut vao);
            glBindVertexArray(vao);
        }

        match &topology {
            Topology::TriIndexed(indices) => {
                indices.bind();
            }
            _ => {}
        }

        let vertices = Buffer::new(GL_ARRAY_BUFFER, verts);
        Verts::bind();

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
                Topology::TriIndexed(indices) => {
                    glDrawElements(
                        GL_TRIANGLES,
                        indices.len() as _,
                        GL_UNSIGNED_BYTE,
                        core::ptr::null(),
                    );
                }

                Topology::Lines => {
                    glDrawArrays(GL_LINE_STRIP, 0, self.vertices.len() as _);
                }

                Topology::Points => {
                    glDrawArrays(GL_POINTS, 0, self.vertices.len() as _);
                }

                Topology::TriFan => {
                    glDrawArrays(GL_TRIANGLE_FAN, 0, self.vertices.len() as _);
                }

                Topology::TriStrip => {
                    glDrawArrays(GL_TRIANGLE_STRIP, 0, self.vertices.len() as _);
                }
            }
        }
    }

    pub fn stencil(&self) {
        unsafe {
            glEnable(GL_STENCIL_TEST);
            glStencilFunc(GL_ALWAYS, 1, 0xFF);
            glClear(GL_STENCIL_BUFFER_BIT);
            glColorMask(0, 0, 0, 0);
            glStencilMask(0xFF);
            glStencilOp(GL_INVERT, GL_INVERT, GL_INVERT);

            self.draw();

            glStencilFunc(GL_NOTEQUAL, 0, 0xFF);
            glStencilOp(GL_KEEP, GL_KEEP, GL_KEEP);
            glColorMask(0xFF, 0xFF, 0xFF, 0xFF);
            glStencilMask(0);
        }
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
    pub fn from_indices(idx: &[u8]) -> Self {
        Self::TriIndexed(Buffer::new(GL_ELEMENT_ARRAY_BUFFER, idx))
    }
}
