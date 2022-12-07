#![no_std]
pub mod resource;
pub mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use base_64::mem::Vec;
use gl::types::*;

pub use resource::{
    buffer::{Buffer, Usage},
    framebuffer::{Attachment, Framebuffer},
    mesh::{Mesh, MeshId, Topology, Vertex},
    pipeline::Pipeline,
    shader::Shader,
    texture::{Format, Target, Texture},
    window::Window,
    Draw, RenderTarget, Resource, Stencil, Uniform,
};

#[derive(Debug)]
pub struct GfxSystem {
    win: Window,
}

impl GfxSystem {
    pub fn new(name: &[u8], w: i32, h: i32) -> Result<Self, ()> {
        log::info!("initializing GfxSystem");
        let win = Window::new(name, w, h)?;

        let gl_fn = |s: &str| unsafe {
            let gl = sdl_64::load_gl(s.as_ptr() as *const i8);

            if gl.is_null() {
                panic!()
            } else {
                gl as *const _
            }
        };

        log::info!("window created");

        unsafe {
            gl::Enable(gl::DEBUG_OUTPUT);
            gl::DebugMessageCallback(Some(debug_log), core::ptr::null());
        }

        Ok(Self { win })
    }

    pub fn draw(&self, draw_fn: impl Fn()) {
        log::debug!("drawing to window");
        self.win.bind();
        self.win.clear_color([0.0, 0.0, 0.0, 1.0]);
        draw_fn();
        self.win.swap();
    }
}

pub extern "system" fn debug_log(
    source: GLenum,
    _type: GLenum,
    _id: GLuint,
    severity: GLenum,
    len: GLsizei,
    msg: *const GLchar,
    usr_param: *mut GLvoid,
) {
    let msg = unsafe { core::ffi::CStr::from_ptr(msg) };
    log::debug!("{}", msg.to_str().expect("couldn't parse as utf8"));
}

/*
impl Drop for GfxSystem {
    fn drop(&mut self) {
        log::info!("dropping Gfx");
        unsafe {
            log::debug!("dropping textures");
            let tex_len = self.textures.len();
            );

            log::debug!("dropping framebuffers");
            let fb_len = self.framebuffers.len() as i32;

            log::debug!("dropping buffers");
            let buf_len = self.buffers.len() as i32;

            log::debug!("dropping meshes");
            let meshes_len = self.meshes.len() as i32;
        }
    }
}
*/
