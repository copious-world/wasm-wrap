
use wasm_nopackage::{go_live,set_plugin_name};


#[link(wasm_import_module = "mod")]
extern "C" {
    fn message_js(s: *mut i8, size: usize);
}


fn output_string(s: *mut i8, size: usize) -> () {
    unsafe {
        message_js(s,size);
    }
}

fn my_plugin_name() -> &'static str {
    let my_str : &str = "Test Crate";
    return my_str;
}


///
/// free memory taken up in linear memory.
/// The original pointer offset will be found in ptr, and the size of the previously allocated block is required.
#[no_mangle]
pub fn startup() {
    go_live(output_string);
    set_plugin_name(my_plugin_name);
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
