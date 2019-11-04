use std::ffi::{CString, c_void};
use std::os::raw::{c_char, c_int};
use libc::{size_t, c_float};
use crate::ffi;


#[link(name = "cbits")]
extern {
    pub fn webp_config_init(config: *mut ffi::WebPConfig);
    pub fn webp_config_preset(
        config: *mut ffi::WebPConfig,
        preset: ffi::WebPPreset,
        quality: c_float,
    );
    pub fn webp_validate_config(config: *mut ffi::WebPConfig);
}