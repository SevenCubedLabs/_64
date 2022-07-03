use super::{buffer::Buffer, vertex::Vertex};
use crate::sys::*;
use alloc::vec::Vec;

pub struct Mesh {
    vao: GLuint,
    _vertices: Vec<Buffer>,
    indices: Buffer,
}

impl Mesh {
    pub fn builder() -> MeshBuilder {
        MeshBuilder {
            vao: unsafe {
                let mut vao = 0;
                glGenVertexArrays(1, &mut vao);
                glBindVertexArray(vao);
                vao
            },
            vertices: Vec::new(),
            indices: None,
        }
    }

    pub fn draw(&self) {
        unsafe {
            glBindVertexArray(self.vao);
            glDrawElements(
                GL_TRIANGLES,
                self.indices.len() as _,
                GL_UNSIGNED_BYTE,
                core::ptr::null(),
            );
        }
    }
}

pub struct MeshBuilder {
    vao: GLuint,
    vertices: Vec<Buffer>,
    indices: Option<Buffer>,
}

impl MeshBuilder {
    pub fn with_verts<V: Vertex>(mut self, verts: &[V]) -> Self {
        self.vertices.push({
            let mut buf = Buffer::new(GL_ARRAY_BUFFER);
            buf.copy(verts);
            V::enable(self.vertices.len() as _);
            buf
        });
        self
    }

    pub fn with_indices(mut self, ids: &[u8]) -> Self {
        self.indices = Some({
            let mut buf = Buffer::new(GL_ELEMENT_ARRAY_BUFFER);
            buf.copy(ids);
            buf
        });
        self
    }

    pub fn build(self) -> Mesh {
        Mesh {
            vao: self.vao,
            _vertices: self.vertices,
            indices: self.indices.unwrap(),
        }
    }
}
