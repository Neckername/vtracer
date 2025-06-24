use crate::*;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::path::Path;

/// Convert an image file to an SVG using default settings.
///
/// Returns `true` on success, `false` otherwise.
#[no_mangle]
pub extern "C" fn vtracer_convert_image_to_svg_default(
    input_path: *const c_char,
    output_path: *const c_char,
) -> bool {
    if input_path.is_null() || output_path.is_null() {
        return false;
    }
    let input = unsafe { CStr::from_ptr(input_path) };
    let output = unsafe { CStr::from_ptr(output_path) };
    let input = match input.to_str() {
        Ok(s) => s,
        Err(_) => return false,
    };
    let output = match output.to_str() {
        Ok(s) => s,
        Err(_) => return false,
    };
    let config = Config::default();
    match convert_image_to_svg(Path::new(input), Path::new(output), config) {
        Ok(()) => true,
        Err(_) => false,
    }
}
