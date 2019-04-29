use wasmer_plugin::wasmer_plugin;
use wasm_data::*;

#[wasmer_plugin]
pub extern fn pass_data(mut foo: Foo) -> Foo {
    foo.foo = "fooooooo".to_string();
    foo
}

fn main() {

}
