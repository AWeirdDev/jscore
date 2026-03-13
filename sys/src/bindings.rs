#![allow(nonstandard_style)]

#[cfg(any(target_os = "macos", target_os = "linux"))]
pub type PidT = std::os::raw::c_int;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
