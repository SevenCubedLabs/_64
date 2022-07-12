pub mod shaders {
    macro_rules! shader_src {
        ($src:literal) => {
            concat!(include_str!($src), "\0")
        };
    }

    pub const POS3D: &str = shader_src!("shaders/pos3d.vert");
    pub const POS3D_RGB: &str = shader_src!("shaders/pos3d_rgb.vert");
    pub const WHITE: &str = shader_src!("shaders/white.frag");
    pub const RGB: &str = shader_src!("shaders/rgb.frag");
}
