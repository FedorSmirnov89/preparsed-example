//! Crate representing the firmware actually put on a low-resource target.
//! Contains only the code necessary to run a pre-parsed module, witout any of wasmi's parsing/validation/translation machinery.
//!
//! Note that wasmi is pulled in with only the 'deserialization' feature enabled

use anyhow::{Context, Result, anyhow};
use shared::Runtime;
use wasmi::preparsed::deserialize_module;

fn main() -> Result<()> {
    // Read in the preparsed bytes (assuming that we compiled the wasm module and then preparsed it)
    let preparsed_bytes =
        std::fs::read("../preparse/preparsed.wi").context("failed to read preparsed bytes")?;

    let (module, engine) =
        deserialize_module(&preparsed_bytes).map_err(|e| anyhow!("failed to deser module: {e}"))?;

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
