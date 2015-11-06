extern crate glutin;
extern crate gl;
extern crate libc;

use gl::types::*;
use std::ffi::CStr;
use std::collections::HashSet;

use glutin::{WindowBuilder, Window, GlRequest, Api, CreationError};

pub use glutin::{GlProfile, Robustness};

pub struct GLChecker {
    pub major_ver: u8,
    pub minor_ver: u8,
    pub profile: GlProfile,
    pub robustness: Robustness,
    pub extensions: Vec<&'static str>,
}

#[macro_export]
macro_rules! require_gl {
    ($major_ver:expr, $minor_ver:expr, $profile:expr, $robustness:expr, $($extension:ident),+) => (
        {
            use std::process::exit;

            let checker = $crate::GLChecker{
                major_ver: $major_ver,
                minor_ver: $minor_ver,
                profile: $profile,
                robustness: $robustness,
                extensions: vec![
                    $(
                        stringify!($extension),
                        )+
                        ],
            };

            if checker.check() {
                exit(0);
            } else {
                exit(1);
            }
        }
        )
}

impl GLChecker {
    fn check_extensions(self: &Self, available: HashSet<&str>) -> bool {
        let mut success = true;

        for ext in self.extensions.iter() {
            if !available.contains(ext) {
                success = false;
                println!("Extension {} is not available!", ext);
            }
        }

        return success;
    }

    fn handle_creation_error(self: &Self, err: CreationError) {
        match err {
            CreationError::OsError(msg) => {
                println!("OS error: {}", msg);
                println!("Most likely cause: OpenGL version {}.{} not available", self.major_ver, self.minor_ver);
            },
            CreationError::NotSupported => {
                println!("OpenGL not available");
            },
            CreationError::NoBackendAvailable(_) => {
                println!("OpenGL not available");
            },
            CreationError::RobustnessNotSupported => {
                println!("Robustness not available");
            },
            CreationError::OpenGlVersionNotSupported => {
                println!("OpenGL {}.{} not available", self.major_ver, self.minor_ver);
            },
            CreationError::NoAvailablePixelFormat => {
                println!("No pixel format available");
            },
        }
    }

    fn check_version(self: &Self, gl_major_ver: u8, gl_minor_ver: u8) -> bool {
        if gl_major_ver > self.major_ver {
            println!("WARNING: asked for version {}.{}, got version {}.{}",
                     self.major_ver, self.minor_ver,
                     gl_major_ver, gl_minor_ver);
            println!("This is probably OK, but good to be aware of.");
            true
        } else if gl_major_ver == self.major_ver && gl_minor_ver >= self.minor_ver {
            true
        } else {
            false
        }
    }

    fn check_robustness(self: &Self, is_robust: bool) -> bool {
        match self.robustness {
            Robustness::NotRobust => !is_robust,
            Robustness::NoError => !is_robust,
            Robustness::RobustNoResetNotification => is_robust,
            Robustness::TryRobustNoResetNotification => true,
            Robustness::RobustLoseContextOnReset => is_robust,
            Robustness::TryRobustLoseContextOnReset => true,
        }
    }

    //TODO: this should be a compile-time thing.
    fn check_sanity(self: &Self) -> bool {
        if self.major_ver < 3 {
            match self.profile {
                GlProfile::Core => {
                    println!("ERROR: Asked for core profile on version < 3.0.  That makes no sense.");
                    false
                },
                GlProfile::Compatibility => true
            }
        } else {
            true
        }
    }

    fn check_profile(self: &Self, is_core: bool, is_compat: bool) -> bool {
        if is_core && is_compat {
            println!("Profile is core and compat?  What?");
            return false;
        }
        match self.profile {
            GlProfile::Core => is_core,
            GlProfile::Compatibility => is_compat
        }
    }

    fn get_extensions(self: &Self) -> HashSet<&str> {
        let mut exts = HashSet::new();
        let mut num_extensions = 0 as GLint;

        unsafe {
            gl::GetIntegerv(gl::NUM_EXTENSIONS, &mut num_extensions);
        }

        for i in 0..num_extensions {
            unsafe {
                let ext_string_ptr = gl::GetStringi(gl::EXTENSIONS, i as u32);
                if ext_string_ptr == std::ptr::null() {
                    panic!("FATAL: Failed to get extension {}!", i);
                }
                let ext_string = match CStr::from_ptr(ext_string_ptr as *const i8).to_str() {
                    Ok(ext) => ext,
                    Err(_) => panic!("FATAL: Failed to get list of extensions!")
                };
                exts.insert(ext_string);
            }
        }

        return exts;
    }

    fn build_window(self: &Self) -> Result<Window, CreationError> {
        WindowBuilder::new()
            .with_gl(GlRequest::Specific(Api::OpenGl, (self.major_ver, self.minor_ver)))
            .with_gl_profile(self.profile)
            .with_gl_robustness(self.robustness)
            .with_visibility(false)
            .build_strict()
    }

    fn check_context(self: &Self, window: &Window) -> bool {
        unsafe {
            window.make_current().unwrap();
        }
        gl::load_with(|s| window.get_proc_address(s));
        let mut gl_major_ver = 0 as GLint;
        let mut gl_minor_ver = 0 as GLint;
        let mut ctx_flags = 0 as GLint;
        let mut profile_mask = 0 as GLint;
        unsafe {
            gl::GetIntegerv(gl::MAJOR_VERSION, &mut gl_major_ver);
            gl::GetIntegerv(gl::MINOR_VERSION, &mut gl_minor_ver);
            gl::GetIntegerv(gl::CONTEXT_FLAGS, &mut ctx_flags);
            gl::GetIntegerv(gl::CONTEXT_PROFILE_MASK, &mut profile_mask);
        }
        let ctx_flags = ctx_flags as u32;
        let profile_mask = profile_mask as u32;
        if !self.check_robustness(ctx_flags & gl::CONTEXT_FLAG_ROBUST_ACCESS_BIT != 0) {
            println!("Context has incorrect robustness");
            return false;
        }
        if !self.check_profile(profile_mask & gl::CONTEXT_CORE_PROFILE_BIT != 0, profile_mask & gl::CONTEXT_COMPATIBILITY_PROFILE_BIT != 0) {
            println!("Wrong profile (wanted {})", match self.profile {
                GlProfile::Core => "Core",
                GlProfile::Compatibility => "Compatibility"
            });
            return false;
        }
        if !self.check_version(gl_major_ver as u8, gl_minor_ver as u8) {
            println!("Obtained version: {}.{}", gl_major_ver, gl_minor_ver);
            println!("Required version: {}.{}", self.major_ver, self.minor_ver);
            return false;
        }
        let exts = self.get_extensions();
        self.check_extensions(exts)
    }

    pub fn check(self: &Self) -> bool {
        if !self.check_sanity() {
            return false;
        }
        match self.build_window() {
            Ok(window) =>  self.check_context(&window),
            Err(creation_error) => {
                self.handle_creation_error(creation_error);
                false
            }
        }
    }
}
