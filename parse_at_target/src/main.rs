//! This crate represents the "current approach" to run a Wasm module with an embedding of wasmi:
//!
//! 1) The .wasm file is provided to the target device running wasmi
//! 2) The .wasm file is parsed, validated, and transformed into a wasm module in wasmi's IR
//! 3) wasmi runs the module
//!
//! This way of running a module requires to have the whole parsing and validation machinery on the target, resuling in larger memory requirements

use anyhow::{Result, anyhow};
use shared::Runtime;
use wasmi::{Config, Engine, Module};

fn main() -> Result<()> {
    // Read in the wasm module (assuming it wasm compiled)
    let wasm_bytes =
        std::fs::read("../wasm_module/target/wasm32-unknown-unknown/release/wasm_module.wasm")
            .map_err(|e| anyhow!("failed to read wasm: {e}"))?;

    // set up engine
    let mut cfg = Config::default();
    cfg.consume_fuel(true);
    cfg.compilation_mode(wasmi::CompilationMode::Eager);
    let engine = Engine::new(&cfg);
    // Parse module from the .wasm file
    let module =
        Module::new(&engine, &wasm_bytes).map_err(|e| anyhow!("failed to load module: {e}"))?;

    // from here on, both option do the same
    let mut runtime = Runtime::new(&engine)?;
    let started = runtime.start_module(&module)?;

    println!("First call");
    runtime.run_function(&started)?;
    println!("Second call");
    runtime.run_function(&started)?;

    println!("Module terminated");

    Ok(())
}
