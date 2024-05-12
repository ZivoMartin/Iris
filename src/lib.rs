use std::os::raw::c_char;
use std::ffi::CStr;


#[no_mangle]
pub extern "C" fn iris_new_request(req: *const c_char) {
    // build a Rust string from C string
    let s = unsafe { CStr::from_ptr(req).to_string_lossy().into_owned() };

    println!("rust_string() is called, value passed = <{:?}>", s);
}
