# Repo exemplifying the proposed "preparsed" feature for wasmi

## Repo contents:

### wasm_module

Contains Rust code that can be compiled to a Wasm module which represents something we would want to run on an embedded, low-resource target.


## Running the example

### Compiling the Wasm module

Given that you have the `wasm32-unknown-unknown` target installed on your machine, you should be able to build the module by running `cargo build --release` from the `wasm_module` directory. This produce the module `wasm_module.wasm` in `wasm_module/target/wasm32-unknown-unknown/release`.

### Option 1 - parsing and running on same target

Go to the directory `parse_at_target` and run `cargo run --release` there. You should see output similar to this:

```
First call
module log: starting
led initialized now
module log: initialized
module log: led on
led is OFF
Led turned ON
module log: updating state
module log: counter: 1
Second call
module log: starting
led already initialized
module log: initialized
module log: led off
led is ON
Led turned OFF
module log: updating state
module log: counter: 2
Module terminated
```

Check the binary size using

```
ls -lh target/release/parse_at_target
```

I am getting a binary of around 2.3MiB on my machine for this exact case - a similar project compiled for an embedded platform requires about 480 KiB.

### Option 2 - pre-parsing on a remote (powerful) node; running on the target

#### Preparsing the module

Go to the directory `preparse` and preparse the module by running `cargo run --release`. The preparsed module should be available under `preparse/preparsed.wi`.