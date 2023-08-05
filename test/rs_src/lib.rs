

#[link(wasm_import_module = "mod")]
extern "C" {
    fn message_js(s: *mut i8, size: usize);
}
