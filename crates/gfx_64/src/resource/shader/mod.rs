mod defaults;

use crate::{gl, gl::types::*, Pipeline};
pub use defaults::*;

pub struct Shader(GLuint);

impl Shader {
    pub fn new(source: &str, stage: GLenum) -> Self {
        unsafe {
            let shader = gl::CreateShader(stage);
            log::info!("building shader {}", shader);

            gl::ShaderSource(shader, 1, &(source.as_ptr() as _), core::ptr::null());
            gl::CompileShader(shader);
            let err = gl::GetError();

            log::info!("compile results: {}", err);

            let mut success = 1;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

            if success == gl::FALSE as _ {
                let mut log_len = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut log_len);
                log::debug!("received {log_len} bytes err msg");

                let mut err = [0u8; 150];
                let mut len = 0;
                gl::GetShaderInfoLog(shader, 150, &mut len, err.as_mut_ptr() as *mut GLchar);

                log::error!(
                    "received {len} bytes: {}",
                    core::str::from_utf8(&err).unwrap()
                );
                panic!();
            }

            Self(shader)
        }
    }

    pub fn attach(&self, pipeline: GLuint) {
        unsafe {
            gl::AttachShader(pipeline, self.0);
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        log::info!("dropping shader {}", self.0);
        unsafe {
            gl::DeleteShader(self.0);
        }
    }
}
