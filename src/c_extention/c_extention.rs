use std::os::raw::c_char;
use std::ffi::CStr;
use crate::{
    Interpreteur,
    Tokenizer,
    TokenizerMessage,
    OneQuery,
};
use std::sync::mpsc::{Sender, Receiver, channel};

use std::thread::spawn;
static mut SENDER: Option<Sender<String>> = None;

#[no_mangle]
pub extern "C" fn iris_new_request(dangerous_req: *const c_char) {
    unsafe {
        let req = CStr::from_ptr(dangerous_req)
                 .to_string_lossy()
                 .into_owned();
        SENDER.as_mut().unwrap().send(req).expect("Failed to send the request");
    };

}

#[no_mangle]
pub extern "C" fn iris_init() {
    let (sender, receiver) = channel::<String>();
    unsafe {
        SENDER = Some(sender.clone());
        spawn(move || iris_run(receiver));
    }
}



 fn iris_run(request_receiver: Receiver<String>)  {
     let (sender, receiver) = channel::<TokenizerMessage>();
     let mut interp = Interpreteur::new();
     let mut tokenizer = Tokenizer::new(sender);
     loop {
         match request_receiver.recv() {
             Ok(req) => tokenizer = OneQuery::new(req).execute(tokenizer, &mut interp, &receiver),
             _ => break         
         }
     }
 }
