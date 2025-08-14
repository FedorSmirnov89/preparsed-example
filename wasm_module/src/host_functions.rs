// Import the host function for LED control
unsafe extern "C" {
    fn set_led(led_on: u32);
}

pub(super) enum LedState {
    On,
    Off,
}

// Safe wrapper to use the function from Rust
pub(super) fn set_led_state(state: LedState) {
    match state {
        LedState::On => unsafe { set_led(1) },
        LedState::Off => unsafe { set_led(0) },
    }
}

unsafe extern "C" {
    fn init_led();
}

// Safe wrapper to use function from Rust
pub(super) fn init_output_pin() {
    unsafe {
        init_led();
    }
}

#[link(wasm_import_module = "logging")]
unsafe extern "C" {
    fn log(buffer: *const u8, length: i32);
}

// Safe wrapper for logging
pub(crate) fn log_msg(msg: &str) {
    unsafe {
        log(msg.as_ptr(), msg.len() as i32);
    }
}
