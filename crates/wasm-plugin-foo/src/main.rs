use wasm_plugin_macro::wasm_plugin;

//use wasmer_plugin::wasmer_plugin;
use wasm_data::*;

#[wasm_plugin]
pub extern fn pass_data(mut foo: Foo) -> Foo {
    foo.foo = "fooooooo".to_string();
    foo
}

fn main() {

}
