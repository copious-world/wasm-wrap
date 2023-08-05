use std::mem;
use std::ffi::CString;
//use std::ffi::CStr;
use std::os::raw::{c_char};


/*
cargo build --target wasm32-unknown-unknown --release

https://github.com/radu-matei/wasm-memory/blob/main/src/lib.rs
https://depth-first.com/articles/2020/07/07/rust-and-webassembly-from-scratch-hello-world-with-strings/
*/

#[link(wasm_import_module = "mod")]
extern "C" {
    fn message_js(s: *mut i8, size: usize);
}


unsafe fn convert_str(input: &str) -> *mut c_char {
    let c_str = CString::new(input).unwrap().into_raw();
    return c_str;
}


/// message_js is avalable for module use. 
/// ex_message calls it
#[no_mangle]
pub fn ex_message(alertable: &String) {
    unsafe {
        let pointer = convert_str(alertable.as_str());
        message_js(pointer,alertable.len());    
    }
}


/// Allocate memory into the module's linear memory
/// and return the offset to the start of the block.
#[no_mangle]
pub fn alloc(len: usize) -> *mut u8 {
    // create a new mutable buffer with capacity `len`
    let mut buf = Vec::with_capacity(len);
    // take a mutable pointer to the buffer
    let ptr = buf.as_mut_ptr();
    // take ownership of the memory block and
    // ensure the its destructor is not
    // called when the object goes out of scope
    // at the end of the function
    mem::forget(buf);
    // return the pointer so the runtime
    // can write data at this offset
    return ptr;
}


/*
#[no_mangle]
pub unsafe extern "C" fn dealloc(ptr: *mut c_void) {
    let _ = Vec::from_raw_parts(ptr, 0, 1024);
}
//
let data = Vec::from_raw_parts(ptr, size, size);
mem::drop(data);
*/

/// free memory taken up in linear memory.
#[no_mangle]
pub unsafe fn dealloc(ptr: *mut u8, size: usize) {
    let _ = Vec::from_raw_parts(ptr, 0, size);  // length < capacity
}




static PLUGIN_NAME: &'static str = "Basic Crate";

#[no_mangle]
pub extern "C" fn plugin_name() -> *mut c_char {
    let s = CString::new(PLUGIN_NAME).unwrap();
    s.into_raw()
}

#[no_mangle]
pub fn plugin_name_len() -> usize {
    PLUGIN_NAME.len()
}





#[no_mangle]
pub unsafe fn test_callback() {
    let str = String::from("we are calling back");
    ex_message(&str);
}

