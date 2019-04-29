use wasm_data::{Foo};

mod commands;

use commands::foo::foo_command;

fn main() {
    let foo = Foo { foo: "foo?".to_string() };
    println!("current foo: {}", foo.foo);
    let updated_foo = foo_command(foo);
    println!("foo updated: {}", updated_foo.foo);
}
