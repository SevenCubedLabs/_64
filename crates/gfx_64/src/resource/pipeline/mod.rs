use crate::{gl, gl::types::*};
use crate::{resource::Resource, Shader};

pub struct Pipeline(GLuint);

impl Pipeline {
    pub fn new(vert_src: &str, frag_src: &str) -> Pipeline {
        let pipeline = unsafe { gl::CreateProgram() };
        log::info!("creating pipeline {pipeline}");

        let vert = Shader::new(vert_src, gl::VERTEX_SHADER);
        let frag = Shader::new(frag_src, gl::FRAGMENT_SHADER);
        vert.attach(pipeline);
        frag.attach(pipeline);

        unsafe {
            gl::LinkProgram(pipeline);
        }

        Self(pipeline)
    }
}

impl Resource for Pipeline {
    fn bind(&self) {
        unsafe {
            gl::UseProgram(self.0);
        }
    }
}

impl core::ops::Deref for Pipeline {
    type Target = GLuint;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for Pipeline {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.0);
        }
    }
}
