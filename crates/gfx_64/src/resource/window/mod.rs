pub use sdl_64::window::Window;

use crate::{
    gl, {RenderTarget, Resource},
};
use log;

impl RenderTarget for Window {}

impl Resource for Window {
    fn bind(&self) {
        unsafe {
            log::info!("binding window");
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }
}
