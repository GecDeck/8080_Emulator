use raylib::prelude::KeyboardKey;
use super::*;

const COIN_BIT: u8 = 0;
const P2_START_BIT: u8 = 1;
const P1_START_BIT: u8 = 2;
const P1_SHOOT_BIT: u8 = 4;
const P1_LEFT_BIT: u8 = 5;
const P1_RIGHT_BIT: u8 = 6;
// Input 1 but order

const TILT_BIT: u8 = 2;
const P2_SHOOT_BIT: u8 = 4;
const P2_LEFT_BIT: u8 = 5;
const P2_RIGHT_BIT: u8 = 6;

#[derive(Debug, Clone, Copy)]
pub struct InputConfig {
    coin: KeyboardKey,
    p2_start: KeyboardKey,
    p1_start: KeyboardKey,
    p1_shoot: KeyboardKey,
    p1_left: KeyboardKey,
    p1_right: KeyboardKey,
    tilt_button: KeyboardKey,
    p2_shoot: KeyboardKey,
    p2_left: KeyboardKey,
    p2_right: KeyboardKey,
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

    // INPUT 1
    if raylib_handle.is_key_down(input_config.coin) {
        hardware.ports.input_1 |= 1 << COIN_BIT;
    } else { hardware.ports.input_1 &= 0b11111110_u8.rotate_left(COIN_BIT as u32) }

    if raylib_handle.is_key_down(input_config.p2_start) {
        hardware.ports.input_1 |= 1 << P2_START_BIT;
    } else { hardware.ports.input_1 &= 0b11111110_u8.rotate_left( P2_START_BIT as u32) }

    if raylib_handle.is_key_down(input_config.p1_start) {
        hardware.ports.input_1 |= 1 << P1_START_BIT;
    } else { hardware.ports.input_1 &= 0b11111110_u8.rotate_left(P1_START_BIT as u32) }

    if raylib_handle.is_key_down(input_config.p1_shoot) {
        hardware.ports.input_1 |= 1 << P1_SHOOT_BIT;
    } else { hardware.ports.input_1 &= 0b11111110_u8.rotate_left(P1_SHOOT_BIT as u32) }

    if raylib_handle.is_key_down(input_config.p1_left) {
        hardware.ports.input_1 |= 1 << P1_LEFT_BIT;
    } else { hardware.ports.input_1 &= 0b11111110_u8.rotate_left(P1_LEFT_BIT as u32) }

    if raylib_handle.is_key_down(input_config.p1_right) {
        hardware.ports.input_1 |= 1 << P1_RIGHT_BIT;
    } else { hardware.ports.input_1 &= 0b11111110_u8.rotate_left(P1_RIGHT_BIT as u32) }

    // INPUT 2
    if raylib_handle.is_key_down(input_config.tilt_button) {
        hardware.ports.input_2 |= 1 << TILT_BIT;
    } else { hardware.ports.input_2 &= 0b11111110_u8.rotate_left(TILT_BIT as u32) }

    if raylib_handle.is_key_down(input_config.p2_shoot) {
        hardware.ports.input_2 |= 1 << P2_SHOOT_BIT;
    } else { hardware.ports.input_2 &= 0b11111110_u8.rotate_left(P2_SHOOT_BIT as u32) }

    if raylib_handle.is_key_down(input_config.p2_left) {
        hardware.ports.input_2 |= 1 << P2_LEFT_BIT;
    } else { hardware.ports.input_2 &= 0b11111110_u8.rotate_left(P2_LEFT_BIT as u32) }

    if raylib_handle.is_key_down(input_config.p2_right) {
        hardware.ports.input_2 |= 1 << P2_RIGHT_BIT;
    } else { hardware.ports.input_2 &= 0b11111110_u8.rotate_left(P2_RIGHT_BIT as u32) }
}
