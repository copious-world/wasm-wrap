use wasm_nopackage::go_live;


#[link(wasm_import_module = "mod")]
extern "C" {
    fn message_js(s: *mut i8, size: usize);
}




///
/// free memory taken up in linear memory.
/// The original pointer offset will be found in ptr, and the size of the previously allocated block is required.
#[no_mangle]
pub fn startup() {
    unsafe {
        go_live(message_js);
    }
}



static PLUGIN_NAME: &'static str = "Test Crate";

#[no_mangle]
pub extern "C" fn plugin_name() -> *mut c_char {
    let s = CString::new(PLUGIN_NAME).unwrap();
    s.into_raw()
}


#[no_mangle]
pub fn plugin_name_len() -> usize {
    PLUGIN_NAME.len()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
