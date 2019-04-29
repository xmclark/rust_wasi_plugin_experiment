# wasmer-plugin-macro

Taken mostly from [FreeMasen's `wasmer_plugin` macro](https://github.com/FreeMasen/wasmer-plugin).

This has been changed to serialize/deserialize JSON instead of bincode.

I think this change could make it easier to write WebAssembly in other 
languages. Not all languages understand [bincode](https://github.com/TyOverby/bincode).
