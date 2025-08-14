/// Just a placeholder for sth holding some host-side state on the target, e.g., primitives for peripheral access
#[derive(Debug, Default)]
pub struct ModuleState {
    pub initialized: bool,
    led_state: bool,
}

impl ModuleState {
    pub fn set_led(&mut self, led_on: bool) {
        let cur_status = if self.led_state { "ON" } else { "OFF" };
        println!("led is {cur_status}");

        assert!(self.initialized, "led was not initialized");

        let status = if led_on { "ON" } else { "OFF" };
        self.led_state = led_on;
        println!("Led turned {status}");
    }
}
