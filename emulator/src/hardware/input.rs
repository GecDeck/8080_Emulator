use raylib::prelude::KeyboardKey;
use super::*;

#[derive(Debug, Clone, Copy)]
pub struct InputConfig {
    p1_start: KeyboardKey,
    p1_shoot: KeyboardKey,
    p1_left: KeyboardKey,
    p1_right: KeyboardKey,
    p2_start: KeyboardKey,
    p2_shoot: KeyboardKey,
    p2_left: KeyboardKey,
    p2_right: KeyboardKey,
    tilt_button: KeyboardKey,
    coin: KeyboardKey,
}
impl InputConfig {
    fn new() -> Self {
        Self {
            p1_start: KeyboardKey::KEY_Q,
            p1_shoot: KeyboardKey::KEY_S,
            p1_left: KeyboardKey::KEY_A,
            p1_right: KeyboardKey::KEY_D,
            p2_start: KeyboardKey::KEY_U,
            p2_shoot: KeyboardKey::KEY_K,
            p2_left: KeyboardKey::KEY_J,
            p2_right: KeyboardKey::KEY_L,
            tilt_button: KeyboardKey::KEY_TAB,
            coin: KeyboardKey::KEY_ENTER,
        }
    }
}
impl Default for InputConfig {
    fn default() -> Self {
        Self::new()
    }
}

pub fn read_input(raylib_handle: &raylib::prelude::RaylibHandle, hardware: &mut Hardware, input_config: InputConfig) {
    // Reads keys based on what has been assigned in the config, then sets the bits in the input
    //  ports based on which keys are pressed

    if raylib_handle.is_key_down(input_config.coin) {
        // TODO: Should the coin input be a toggle? not sure
        hardware.ports.input_1 = hardware.ports.input_1 | 0b10000000;
    }

}
