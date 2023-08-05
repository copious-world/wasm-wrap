#wasm-nopackage

A simple package that provides alloc, dealloc, and a way to set a method that sends a string to JavaScript.

## Use Rust


The next code snippet uses `go_live` and `set_plugin_name` allowing JavaScript to call and be called effectively extending the package's default values. Functions have been used to make the base module aware of the values that this application uses.

The JavaScript will only call `startup`.


```
use wasm_nopackage::{go_live,set_plugin_name};

// In the final module that exposes itself to JavaScript, 
// this method will be mapped to the JavaScript caller.
// This might have been included in the base crate, but 
// the crate publisher looks for the symbol and fails to publish.
// It is hoped that this is not too much for those who use 
// the crate to include these fiew lines.

#[link(wasm_import_module = "mod")]
extern "C" {
    fn message_js(s: *mut i8, size: usize);
}


// This function allows a mutable static reference to be set
// so that ex_message(alertable: &String) can be used generally.
// This module might just use this method. But, there may be other modules
// that can include wasm_nopackage::ex_message.
//
fn output_string(s: *mut i8, size: usize) -> () {
    unsafe {
        message_js(s,size);
    }
}


// The module name is a function here rather than in a global variable.
// This function will be called when plugin_name and plugin_name_len
// are called by JavaScript.
fn my_plugin_name() -> &'static str {
    let my_str : &str = "Test Lib";  // <- the name of the lib here.
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

```


## JavaScript


The following code is a test program that uses a class that works with the module derived from wasm-nopackage.


```
<script src="js/wasm_wrap.js" ></script>
<script>
  async function main() {
    //
    let wasmod = new WASMInterface('/wasm/testup')
    await wasmod.init("mod")
    wasmod.startup()  // entrypoint is from the test module call tables
    wasmod.test_callback();  // wasm-nopackage provides this entry point.
    console.log(wasmod.plugin_name_str())
    //
  }
  //
  setTimeout(main,1000)

</script
```


In the above example, `wasm_wrap.js` defines WASMInterface at the window level of the browser. So, this example is not using modules. But, a version using modules is possible.


The class definition for WASMInterface includes this definition. All the methods that are expose by `wasm-nopackage` and the WASM lib that uses it are added to the class and can be called as methods on the JavaScript class.


```
async init(module_name) {
    //
    let importObject = {};
    //
    importObject[module_name] = {
        __memory_base: 0,
        __table_base: 0,
        memory: new WebAssembly.Memory({ initial: 1 }),
        "message_js": (str_offset, size) => { this.wasm_alert(str_offset, size); }
    };
    this.mod = await this.get_wasm_module(this._mod_path, importObject);
    let self = this;
    for (let ky in this.mod) {
        self[ky] = this.mod[ky];
    }
    //
}
```
 
 
The WASMInterface JavaScript class provides some methods that read and write strings in the WASM memory, making appropriate allocations. If can be helpful to use the JavaScript methods to setup calls that take the references they manage as pointers to strings or data in memory.




