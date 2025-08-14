use anyhow::{Result, anyhow};
use wasmi::{AsContext, Caller, Linker};

use crate::ModuleState;

pub(crate) fn link_host_functions(linker: &mut Linker<ModuleState>) -> Result<()> {
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
