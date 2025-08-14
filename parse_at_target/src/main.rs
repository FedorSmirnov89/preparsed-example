//! This crate represents the "current approach" to run a Wasm module with an embedding of wasmi:
//!
//! 1) The .wasm file is provided to the target device running wasmi
//! 2) The .wasm file is parsed, validated, and transformed into a wasm module in wasmi's IR
//! 3) wasmi runs the module
//!
//! This way of running a module requires to have the whole parsing and validation machinery on the target, resuling in larger memory requirements

use anyhow::{Result, anyhow};
use wasmi::{AsContext, Caller, Config, Engine, Linker, Module, Store};

const MODULE_FUEL: u64 = 1000;

fn main() -> Result<()> {
    // Read in the wasm module (assuming it wasm compiled)
    let wasm_bytes =
        std::fs::read("../wasm_module/target/wasm32-unknown-unknown/release/wasm_module.wasm")
            .map_err(|e| anyhow!("failed to read wasm: {e}"))?;

    // set up runtime
    let mut cfg = Config::default();
    cfg.consume_fuel(true);
    cfg.compilation_mode(wasmi::CompilationMode::Eager);
    let engine = Engine::new(&cfg);

    // create linker + link host functions
    let mut linker = <Linker<ModuleState>>::new(&engine);
    link_host_functions(&mut linker)?;

    // create state and module store
    let state = ModuleState::default();
    let mut store = Store::new(&engine, state);
    store.set_fuel(MODULE_FUEL)?;

    // Parse module from the .wasm file
    let module =
        Module::new(&engine, &wasm_bytes).map_err(|e| anyhow!("failed to load module: {e}"))?;

    // instantiate module
    let instantiated = linker
        .instantiate(&mut store, &module)
        .map_err(|e| anyhow!("failed to instantiate module: {e}"))?;
    let started = instantiated
        .start(&mut store)
        .map_err(|e| anyhow!("failed to start module: {e}"))?;

    let led_fn = started
        .get_typed_func::<(), ()>(&mut store, "run")
        .expect("failed to get function");

    println!("First call");
    led_fn.call(&mut store, ())?;
    println!("Second call");
    led_fn.call(&mut store, ())?;

    println!("Module terminated");

    Ok(())
}

/// Just a placeholder for sth holding some host-side state on the target, e.g., primitives for peripheral access
#[derive(Debug, Default)]
struct ModuleState {
    initialized: bool,
    led_state: bool,
}

impl ModuleState {
    fn set_led(&mut self, led_on: bool) {
        let cur_status = if self.led_state { "ON" } else { "OFF" };
        println!("led is {cur_status}");

        assert!(self.initialized, "led was not initialized");

        let status = if led_on { "ON" } else { "OFF" };
        self.led_state = led_on;
        println!("Led turned {status}");
    }
}

fn link_host_functions(linker: &mut Linker<ModuleState>) -> Result<()> {
    link_output_init(linker)?;
    link_set_led(linker)?;
    link_logging(linker)?;
    Ok(())
}

fn link_logging(linker: &mut Linker<ModuleState>) -> Result<()> {
    linker
        .func_wrap(
            "logging",
            "log",
            |caller: Caller<'_, ModuleState>, buffer_ptr: u32, length: u32| {
                let memory = caller
                    .get_export("memory")
                    .expect("module does not export memory")
                    .into_memory()
                    .expect("failed to get memory");
                let store = caller.as_context();
                let data_start = buffer_ptr as usize;
                let data_end = data_start + (length as usize);
                let data = &memory.data(&store)[data_start..data_end];

                let log_msg = str::from_utf8(data).expect("failed to convert string");
                println!("module log: {log_msg}");
            },
        )
        .map_err(|e| anyhow!("failed to link log function: {e}"))?;
    Ok(())
}

fn link_output_init(linker: &mut Linker<ModuleState>) -> Result<()> {
    linker
        .func_wrap("env", "init_led", |mut caller: Caller<'_, ModuleState>| {
            if caller.data().initialized {
                println!("led already initialized");
            } else {
                caller.data_mut().initialized = true;
                println!("led initialized now");
            }
        })
        .map_err(|e| anyhow!("failed to link init led function: {e}"))?;
    Ok(())
}

fn link_set_led(linker: &mut Linker<ModuleState>) -> Result<()> {
    linker
        .func_wrap(
            "env",
            "set_led",
            |mut caller: Caller<'_, ModuleState>, led_on: i32| {
                caller.data_mut().set_led(led_on == 1);
            },
        )
        .map_err(|e| anyhow!("failed to link set led function: {e}"))?;
    Ok(())
}
