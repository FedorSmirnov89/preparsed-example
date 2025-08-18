//! Lib crate containing code shared between the two crates demonstrating the two options of
//! running a module

mod state;

use anyhow::{Result, bail};
pub use state::ModuleState;
use wasmi::{AsContext, Caller, Engine, Extern, Func, Module, Store};

const MODULE_FUEL: u64 = 1000;

pub fn link_externals(
    module: &Module,
    engine: &Engine,
    externals: &mut Vec<Extern>,
) -> Result<Store<ModuleState>> {
    let mut store = Store::new(engine, ModuleState::default());
    store.set_fuel(MODULE_FUEL)?;
    let imports = module.imports();
    externals.clear();
    externals.reserve(imports.len());
    for import in imports {
        let module_name = import.module();
        let field_name = import.name();
        let host_func = match (module_name, field_name) {
            ("env", "init_led") => {
                Func::wrap(
                    &mut store,
                    move |mut caller: Caller<'_, ModuleState>| -> Result<(), wasmi::Error> {
                        if caller.data().initialized {
                            println!("led already initialized");
                        } else {
                            caller.data_mut().initialized = true;
                            println!("led initialized now");
                        }
                        Ok(())
                    },
                )
            }
            ("env", "set_led") => {
                Func::wrap(
                    &mut store,
                    move |mut caller: Caller<'_, ModuleState>, led_on: i32| {
                        caller.data_mut().set_led(led_on == 1);
                        Ok(())
                    },
                )
            }
            ("logging", "log") => {
                Func::wrap(
                    &mut store,
                    move |caller: Caller<'_, ModuleState>, buffer_ptr: i32, length: i32| {
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
                        Ok(())
                    },
                )
            }
            _ => bail!("unknown import at: {module_name}::{field_name}"),
        };
        externals.push(Extern::Func(host_func));
    }
    Ok(store)
}
