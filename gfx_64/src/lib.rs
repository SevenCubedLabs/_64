#![no_std]
pub mod resource;
mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use base_64::mem::Vec;
use gl::{types::*, Gl};
pub use resource::{
    buffer::{Buffer, Usage},
    framebuffer::{Attachment, Framebuffer},
    mesh::{Mesh, Topology, Vertex},
    pipeline::Pipeline,
    shader::Shader,
    texture::{Target, Texture},
    Resource,
};

pub struct GfxSystem {
    ctx: Gl,
    textures: Vec<Texture>,
    shaders: Vec<Shader>,
    pipelines: Vec<Pipeline>,
    buffers: Vec<Buffer>,
    framebuffers: Vec<Framebuffer>,
}

impl GfxSystem {
    pub fn new() -> Self {
        Self {
            ctx: Gl::load_with(|s| unsafe { sdl_64::load_gl(s.as_ptr() as *const i8) }),
            textures: Vec::new(),
            shaders: Vec::new(),
            pipelines: Vec::new(),
            buffers: Vec::new(),
            framebuffers: Vec::new(),
        }
    }

    pub fn new_mesh<V: Vertex>(&self, verts: &[V], usage: Usage, topology: Topology) -> Mesh {
        let mut vao = 0;
        unsafe {
            self.ctx.GenVertexArrays(1, &mut vao);
            self.ctx.BindVertexArray(vao);
        }

        match &topology {
            Topology::TriIndexed(indices) => {
                indices.bind(self);
            }
            _ => {}
        }

        let vertices = self.new_buffer(gl::ARRAY_BUFFER, usage, verts);
        V::bind(self);

        Mesh {
            vao,
            vertices,
            topology,
        }
    }

    pub fn new_texture(&mut self, target: Target, [w, h]: [i32; 2], format: GLenum) -> Texture {
        let mut id = 0;
        unsafe {
            self.ctx.GenTextures(1, &mut id);
            self.ctx.BindTexture(target as _, id);
            self.ctx.TexImage2D(
                target as _,
                0,
                format as _,
                w,
                h,
                0,
                format,
                gl::UNSIGNED_BYTE,
                core::ptr::null(),
            );
            self.ctx
                .TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as _);
            self.ctx
                .TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as _);
        }

        let tex = Texture { id, target };
        self.textures.push(tex);

        tex
    }

    pub fn new_program(&mut self, vert_src: &str, frag_src: &str) -> Pipeline {
        let pipeline = unsafe { self.ctx.CreateProgram() };

        let vert = self.new_shader(vert_src, gl::VERTEX_SHADER);
        let frag = self.new_shader(frag_src, gl::FRAGMENT_SHADER);
        self.link_shader(pipeline, vert);
        self.link_shader(pipeline, frag);

        unsafe {
            self.ctx.LinkProgram(pipeline);
        }

        self.pipelines.push(pipeline);

        pipeline
    }

    fn clear_color(&self, [r, g, b, a]: [f32; 4]) {
        unsafe {
            self.ctx.ClearColor(r, g, b, a);
        }
    }

    fn clear_stencil(&self, clear: i32) {
        unsafe {
            self.ctx.ClearStencil(clear);
        }
    }

    fn viewport(&self, [x, y]: [i32; 2], [w, h]: [i32; 2]) {
        log::debug!("setting viewport: [{}, {}], [{}, {}]", x, y, w, h);
        unsafe {
            self.ctx.Viewport(x, y, w, h);
        }
    }

    pub fn new_shader(&mut self, source: &str, stage: GLenum) -> Shader {
        unsafe {
            let shader = self.ctx.CreateShader(stage);
            self.ctx
                .ShaderSource(shader, 1, &(source.as_ptr() as _), core::ptr::null());
            self.ctx.CompileShader(shader);
            self.shaders.push(shader);

            shader
        }
    }

    pub fn new_framebuffer(&mut self) -> Framebuffer {
        let mut id = 0;
        unsafe {
            self.ctx.GenFramebuffers(1, &mut id);
            self.ctx.BindFramebuffer(gl::FRAMEBUFFER, id);
        }

        let fb = Framebuffer { id };
        self.framebuffers.push(fb);
        fb
    }

    pub fn attach_to_framebuffer(&mut self, attach: Attachment, tex: &Texture) {
        unsafe {
            self.ctx
                .FramebufferTexture(gl::FRAMEBUFFER, attach as _, tex.id, 0);
            self.ctx.DrawBuffers(1, [attach as _].as_ptr());
        }
    }

    pub fn link_shader(&self, pipeline: Pipeline, shader: Shader) {
        unsafe {
            self.ctx.AttachShader(pipeline, shader);
        }
    }

    pub fn new_buffer<Data>(&self, buf_type: GLenum, usage: Usage, data: &[Data]) -> Buffer {
        unsafe {
            let mut id = 0;
            self.ctx.GenBuffers(1, &mut id);
            self.ctx.BindBuffer(buf_type, id);

            self.ctx.BufferData(
                buf_type,
                (core::mem::size_of::<Data>() * data.len()) as _,
                data.as_ptr() as _,
                usage as _,
            );

            Buffer {
                buf_type,
                id,
                len: data.len(),
            }
        }
    }

    pub fn update_buffer<Data>(&mut self, buf_idx: usize, data: &[Data]) {
        let mut buf = self.buffers[buf_idx];
        buf.bind(self);
        unsafe {
            self.BufferSubData(
                buf.buf_type,
                0,
                (core::mem::size_of::<Data>() * data.len()) as _,
                data.as_ptr() as _,
            );
        }
        buf.len = data.len();
    }
}

impl core::ops::Deref for GfxSystem {
    type Target = gl::Gl;

    fn deref(&self) -> &Self::Target {
        &self.ctx
    }
}

impl Drop for GfxSystem {
    fn drop(&mut self) {
        unsafe {
            log::trace!("dropping shaders");
            for shader in self.shaders.iter() {
                self.ctx.DeleteShader(*shader);
            }

            let tex_len = self.textures.len();
            log::trace!("dropping textures {}", tex_len);
            self.ctx.DeleteTextures(
                tex_len as _,
                self.textures
                    .iter()
                    .map(|tex| tex.id)
                    .collect::<Vec<u32>>()
                    .as_ptr(),
            );

            let fb_len = self.framebuffers.len() as i32;
            log::trace!("dropping {} framebuffers", fb_len);
            self.ctx.DeleteFramebuffers(
                fb_len,
                self.framebuffers
                    .iter()
                    .map(|fb| fb.id)
                    .collect::<Vec<u32>>()
                    .as_ptr(),
            );

            let buf_len = self.buffers.len() as i32;
            log::trace!("dropping {} buffers", buf_len);

            self.ctx.DeleteBuffers(
                buf_len,
                self.buffers
                    .iter()
                    .map(|buf| buf.id)
                    .collect::<Vec<u32>>()
                    .as_ptr(),
            );
        }
    }
}
