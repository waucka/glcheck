#[macro_use]
extern crate glchecklib;

use glchecklib::{GlProfile, Robustness};

fn main() {
    require_gl!{3, 3, GlProfile::Core, Robustness::NotRobust,
                GL_ARB_texture_float};
}
