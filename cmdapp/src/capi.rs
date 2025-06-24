use crate::*;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::path::Path;

/// Convert an image file to an SVG using default configuration.
/// Returns 0 on success and 1 on failure.
#[no_mangle]
pub extern "C" fn vtracer_convert_image_to_svg(
    input_path: *const c_char,
    output_path: *const c_char,
) -> i32 {
    if input_path.is_null() || output_path.is_null() {
        return 1;
    }
    let input_cstr = unsafe { CStr::from_ptr(input_path) };
    let output_cstr = unsafe { CStr::from_ptr(output_path) };
    let input = match input_cstr.to_str() {
        Ok(s) => Path::new(s),
        Err(_) => return 1,
    };
    let output = match output_cstr.to_str() {
        Ok(s) => Path::new(s),
        Err(_) => return 1,
    };
    match convert_image_to_svg(input, output, Config::default()) {
        Ok(()) => 0,
        Err(_) => 1,
    }
}
