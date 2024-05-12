use std::os::raw::c_char;
use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn iris_new_request(req: *mut c_char) {
    unsafe {
        println!("{}", CStr::from_ptr(req).to_str().expect("Failed to convert dangerous c string in safe rust string"));
    }
}
