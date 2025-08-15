//! Lib crate containing code shared between the two crates demonstrating the two options of
//! running a module

mod host_functions;
mod state;

use anyhow::{Result, anyhow};
pub use state::ModuleState;
use wasmi::{Engine, Instance, Linker, Module, Store};

use crate::host_functions::link_host_functions;

const MODULE_FUEL: u64 = 1000;

/// Represents the structs required for funning a module
pub struct Runtime {
    store: Store<ModuleState>,
    linker: Linker<ModuleState>,
}

impl Runtime {
    pub fn new(engine: &Engine) -> Result<Self> {
        let mut linker = <Linker<ModuleState>>::new(engine);
        link_host_functions(&mut linker)?;

        // create state and module store
        let state = ModuleState::default();
        let mut store = Store::new(engine, state);
        store.set_fuel(MODULE_FUEL)?;
        Ok(Self { store, linker })
    }

    pub fn start_module(&mut self, module: &Module) -> Result<Instance> {
        let started = self
            .linker
            .instantiate_and_start(&mut self.store, module)
            .map_err(|e| anyhow!("failed to instantiate module: {e}"))?;
        Ok(started)
    }

    pub fn run_function(&mut self, instance: &Instance) -> Result<()> {
        let led_fn = instance.get_typed_func::<(), ()>(&mut self.store, "run")?;
        led_fn.call(&mut self.store, ())?;
        Ok(())
    }
}
