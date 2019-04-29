use wasm_data::{Foo, serialize, deserialize};
use wasmer_runtime::instantiate;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;

const START: usize = 5;

pub fn foo_command(_foo: Foo) -> Foo {
    #[cfg(feature = "dev")]
    let wasm_path: PathBuf = PathBuf::from("./target/wasm32-unknown-wasi/release/wasm-plugin-foo.wasm");
    #[cfg(feature = "prod")]
    let wasm_path: PathBuf = PathBuf::from("./wapm_packages/xmclark/rust_wasi_plugin_experiment_plugin_foo@0.1.1/wasm-plugin-foo.wasm");

    let mut wasm = Vec::new();
    let mut f = File::open(&wasm_path).expect("failed to open wasm");
    f.read_to_end(&mut wasm).expect("failed to read wasm");
    println!("instantiating the wasm module");
    let imports = wasmer_wasi::generate_import_object(vec![], vec![], vec![]);
    let instance = instantiate(&wasm, &imports).expect("Could not instantiate wasm.");
    let context = instance.context();
    let memory = context.memory(0);

    let view = memory.view::<u8>();
    // Zero our the first 4 bytes of memory
    for cell in view[1..5].iter() {
        cell.set(0);
    }
    let foo: Foo = Foo {foo: "foo?".to_string() };
    let bytes = serialize(foo);
    let len = bytes.len();
    for (cell, byte) in view[START..len + START].iter().zip(bytes.iter()) {
        cell.set(*byte)
    }
    let func = instance.func::<(i32, i32), i32>("_pass_data").expect("Could not get func _pass_data");
    let ptr = func.call(START as _, len as i32).expect("could not call _pass_data");

    let updated_mem = instance.context().memory(0);
    let view = updated_mem.view();
    let mut new_len_bytes: [u8; 4] = [0;4];
    for i in 0..4 {
        new_len_bytes[i] = view.get(1 + i).map(|c| c.get()).expect("unable to get new length");
    }
    let new_len = u32::from_ne_bytes(new_len_bytes);
    let end: usize = ptr as usize + new_len as usize;
    let buf: Vec<u8> = view[ptr as usize..end].iter().map(|c| c.get()).collect();

    let update_foo: Foo = deserialize(&buf);
    update_foo
}
