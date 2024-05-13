use std::os::raw::c_char;
use std::ffi::CStr;
use crate::{
    Interpreteur,
    Tokenizer,
    TokenizerMessage,
    OneFile,
    OneQuery,
    pre_init_database
};
use std::sync::mpsc::{Receiver, channel};
static mut TOKENIZER: Option<Tokenizer> = None;
static mut INTERPRETEUR: Option<Interpreteur> = None;
static mut RECEIVER: Option<Receiver<TokenizerMessage>> = None;

#[no_mangle]
pub extern "C" fn iris_init() {
    pre_init_database();
    let (sender, receiver) = channel::<TokenizerMessage>();
    unsafe {
        INTERPRETEUR = Some(Interpreteur::new());
        TOKENIZER = Some(Tokenizer::new(sender));
        RECEIVER = Some(receiver);
    }
}

unsafe fn try_to_init() {
    if TOKENIZER.is_none() {
        iris_init();
    }
}

unsafe fn extract_rust_string(dangerous_string: *const c_char) -> String {
    CStr::from_ptr(dangerous_string)
        .to_string_lossy()
        .into_owned()
}

#[no_mangle]
pub unsafe extern "C" fn iris_new_request(dangerous_req: *const c_char) {
    try_to_init();
    let req = extract_rust_string(dangerous_req);
    TOKENIZER = Some(OneQuery::new(req).execute(
        TOKENIZER.take().unwrap(),
        INTERPRETEUR.as_mut().unwrap(),
        RECEIVER.as_ref().unwrap()
    ));

}
#[no_mangle]
pub unsafe extern "C" fn iris_load_file(dangerous_file_path: *const c_char) {
    try_to_init();
    let path = extract_rust_string(dangerous_file_path);
    TOKENIZER = Some(OneFile::new(path).execute(
        TOKENIZER.take().unwrap(),
        INTERPRETEUR.as_mut().unwrap(),
        RECEIVER.as_ref().unwrap()
    ))
}

