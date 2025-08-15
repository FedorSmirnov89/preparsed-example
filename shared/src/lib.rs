//! Lib crate containing code shared between the two crates demonstrating the two options of
//! running a module

mod state;

use anyhow::{Result, bail};
pub use state::ModuleState;
use wasmi::{AsContext, Caller, Engine, Extern, Func, FuncType, Module, Store};

const MODULE_FUEL: u64 = 1000;

pub fn link_externals(
    module: &Module,
    engine: &Engine,
) -> Result<(Vec<Extern>, Store<ModuleState>)> {
    let state = ModuleState::default();
    let mut store = Store::new(engine, state);
    store.set_fuel(MODULE_FUEL)?;

    let module_imports = module.imports();
    let mut externals = Vec::with_capacity(module_imports.len());

    for import in module_imports {
        let module_name = import.module();
        let field_name = import.name();
        let ty = import.ty();

        match ty {
            wasmi::ExternType::Func(func_ty) => {
                let host_func_ty =
                    FuncType::new(func_ty.params().to_vec(), func_ty.results().to_vec());

                let host_func = match (module_name, field_name) {
                    ("env", "init_led") => Func::new(
                        &mut store,
                        host_func_ty,
                        move |mut caller: Caller<'_, ModuleState>, _args, _results| {
                            if caller.data().initialized {
                                println!("led already initialized");
                            } else {
                                caller.data_mut().initialized = true;
                                println!("led initialized now");
                            }
                            Ok(())
                        },
                    ),

                    ("env", "set_led") => Func::new(
                        &mut store,
                        host_func_ty,
                        move |mut caller: Caller<'_, ModuleState>, args, _results| {
                            let led_on = args[0].i32().expect("led_on has to be an int");
                            caller.data_mut().set_led(led_on == 1);
                            Ok(())
                        },
                    ),

                    ("logging", "log") => Func::new(
                        &mut store,
                        host_func_ty,
                        move |caller: Caller<'_, ModuleState>, args, _results| {
                            let buffer_ptr = args[0].i32().expect("buffer ptr has to be an int");
                            let length = args[1].i32().expect("buffer len has to be an int");

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
                    ),

                    (unknown_module, unknown_field) => bail!(
                        "unexpected function import: module {unknown_module}, name: {unknown_field}"
                    ),
                };

                externals.push(Extern::Func(host_func));
            }

            _ => bail!("unexpected import (not a function)"), // just for the small example
        }
    }

    Ok((externals, store))
}
