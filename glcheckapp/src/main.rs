#[macro_use]
extern crate glchecklib;

use glchecklib::{GlProfile, Robustness};

fn main() {
    use std::process::exit;

    exit(if require_gl!{3, 3, GlProfile::Core, Robustness::NotRobust,
                        GL_ARB_texture_float} {
        0
    } else {
        1
    });
}
