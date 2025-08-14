#![no_std] // required for wasm32-unknown-unknown
use core::panic::PanicInfo;

use core::fmt::Write;

use heapless::String;
use spin::Mutex;

use crate::host_functions::{LedState, init_output_pin, log_msg, set_led_state};

#[panic_handler] // required when you drop std
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

mod host_functions;

struct ModuleState {
    counter: u32,
}

static STATE: Mutex<ModuleState> = Mutex::new(ModuleState { counter: 0 });

#[unsafe(no_mangle)]
pub extern "C" fn run() {
    log_msg("starting");

    init_output_pin(28);
    log_msg("initialized");

    // Since WASM is always single-threaded, the locking is a no-op
    let mut state = STATE.lock();

    if state.counter.is_multiple_of(2) {
        log_msg("led on");
        set_led_state(LedState::On);
    } else {
        log_msg("led off");
        set_led_state(LedState::Off);
    }

    log_msg("updating state");
    state.counter += 1;

    // complicated way to have a formatted string :)
    let mut buf: String<32> = String::new();
    write!(buf, "counter: {}", state.counter).unwrap();
    log_msg(&buf);
}
