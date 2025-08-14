//! This crate represents the part which would happen on a node with more resources than the resource-restricted target.
//! Here, we load the .wasm file, parse and validate it, and then serialize the wasmi IR into a format that can be later
//! provided to the resource-limited target.
//!
//! Note that serializing/preparsing the modules requires using wasmi with the 'parser' and 'serialization' features enabled (we could also use the other default features, since the node where we preparse is supposed to be powerful)

use anyhow::{Context, Result, anyhow};
use wasmi::{Config, Engine, Module, preparsed::serialize_module};

const OUTPUT_PATH: &str = "./preparsed.wi";

fn main() -> Result<()> {
    // Read in the .wasm file (assuming that the wasm_module crate was compiled)
    let wasm_bytes =
        std::fs::read("../wasm_module/target/wasm32-unknown-unknown/release/wasm_module.wasm")
            .map_err(|e| anyhow!("failed to read wasm: {e}"))?;
    println!("wasm read");

    // Create and configure the engine (must correspond to what we will be using on the target)
    let mut cfg = Config::default();
    cfg.consume_fuel(true);
    cfg.compilation_mode(wasmi::CompilationMode::Eager); // the only legal compilation mode here, since we won't have the parsing/validation/translation machinery on the target
    let engine = Engine::new(&cfg);

    let module = Module::new(&engine, &wasm_bytes).context("failed to parse module")?;
    println!("module parsed");
    let preparsed_bytes = serialize_module(&module, &engine)
        .map_err(|e| anyhow!("failed to serialize module: {e}"))?;
    println!("module serialized");

    // Write the bytes onto disk (these are the bytes we would flash/load onto the target node)
    std::fs::write(OUTPUT_PATH, &preparsed_bytes)
        .context("failed writing preparsed bytes to disk")?;
    println!("preparsed module written to {OUTPUT_PATH}");
    Ok(())
}
