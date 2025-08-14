# Repo exemplifying the proposed "preparsed" feature for wasmi

## Repo contents:

### wasm_module

Contains Rust code that can be compiled to a Wasm module which represents something we would want to run on an embedded, low-resource target.


## Running the example

### Compiling the Wasm module

Given that you have the `wasm32-unknown-unknown` target installed on your machine, you should be able to build the module by running `cargo build --release` from the `wasm_module` directory. This produce the module `wasm_module.wasm` in `wasm_module/target/wasm32-unknown-unknown/release`.