# wasi plugins on wapm.io
❗Experimental!❗

This is an experiment to see how one can make wasi plugins that are published on the wapm registry.

The runner crate will load wasm at runtime from the wapm_packages directory. 
The plugin passes data with the bincode serialization. 
The plugin is built for the wasi target so it can do things like open files.

There is currently a single plugin called `foo` that is deployed on the [wapm registry][plugin_wapm_io]. 

To run this, first install packages:
`> wapm install`

Then run the runner crate:
`> cargo run --package wasm-plugin-runner --bin wasm-plugin-runner --no-default-features --features prod`

## Attributions
This is heavily inspired by [FreeMasen][free_masen] and his work:
- [wasmer plugin][wasmer_plugin]
- [blog posts][wasmer_plugin_blog]

[free_masen]: https://github.com/FreeMasen
[wasmer_plugin]: https://github.com/FreeMasen/wasmer-plugin
[wasmer_plugin_blog]: https://wiredforge.com/blog/wasmer-plugin-pt-1/index.html
[plugin_wapm_io]: https://wapm.io/package/xmclark/rust_wasi_plugin_experiment_plugin_foo
