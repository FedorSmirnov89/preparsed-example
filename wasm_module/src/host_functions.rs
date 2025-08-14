// Import the host function for LED control
extern "C" {
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

extern "C" {
    fn init_led(led_pin: u32);
}

// Safe wrapper to use function from Rust
pub(super) fn init_output_pin(pin_number: u32) {
    unsafe {
        init_led(pin_number);
    }
}

#[link(wasm_import_module = "logging")]
extern "C" {
    fn log(buffer: *const u8, length: i32);
}

// Safe wrapper for logging
pub(crate) fn log_msg(msg: &str) {
    unsafe {
        log(msg.as_ptr(), msg.len() as i32);
    }
}
