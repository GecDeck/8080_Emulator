mod tests;
pub mod input;

#[derive(Debug, Clone, Copy)]
enum Port {
    INP1,
    INP2,
    SHFTIN,
    SHFTAMNT,
    SOUND1,
    SHFTDATA,
    SOUND2,
    WATCHDOG,
}

#[derive(Debug, Clone, Copy)]
struct Ports {
    input_1: u8,
    // Bit  0: Coin, 0 when coin inserted
    //      1: P2 Start
    //      2: P1 Start
    //      3: Always 1
    //      4: P1 Shoot
    //      5: P1 Left
    //      6: P1 Right
    //      7: Not Connected
    input_2: u8,
    // Bit  0,1: Lives (0: 3 Lives, 1: 4, 2: 5, 3: 6)
    //      2: Tilt Button?
    //      3: Bonus life at score (0: 1500, 1: 1000)
    //      4: P2 Shoot
    //      5: P2 Left
    //      6: P2 Right
    //      7: Coin info toggle (0: On, 1: Off)
    shift_result: u8,
    shift_amount: u8,
    // Offset from the left that will be read when reading shift_result
    // First 3 bits are the offset
    // Offset of 2 will start reading from the 3rd bit
    sound_1: u8,
    shift_data: u8,
    // Data to write to shift register
    //  Every write adds this byte to the high byte of the shift register
    //  And moves the previous high byte to the low byte
    sound_2: u8,
    watchdog: u8,
    // When text is written to the screen this is the ascii value of each letter written
}
impl Ports {
    fn new() -> Self {
        Self {
            input_1: 0x00,
            input_2: 0x00,
            shift_result: 0x00,
            shift_amount: 0x00,
            sound_1: 0x00,
            shift_data: 0x00,
            sound_2: 0x00,
            watchdog: 0x00,
        }
    }
}
impl Default for Ports {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Hardware {
    shift_register: u16,
    ports: Ports,
}
impl Hardware {
    pub fn init() -> Self {
        Self {
            shift_register: 0x0000,
            ports: Ports::default(),
        }
    }

    pub fn reset(&mut self) {
        // Resets all the values of the cpu
        *self = Hardware::default();
    }

    pub fn debug_input1(&self) -> u8 {
        self.ports.input_1
    }
    pub fn debug_input2(&self) -> u8 {
        self.ports.input_2
    }
}
impl Default for Hardware {
    fn default() -> Self {
        Self::init()
    }
}

pub fn handle_io(op_code: u8, hardware: &mut Hardware, port_byte: u8, reg_a: u8) -> Option<u8> {
    match op_code {
        0xd3 => { // OUT
            let port: Port = match port_byte {
                2 => Port::SHFTAMNT,
                3 => Port::SOUND1,
                4 => Port::SHFTDATA,
                5 => Port::SOUND2,
                6 => Port::WATCHDOG,
                _ => panic!("OUT should only ever have an additional byte between 2 and 6"),
            };

            write_port(reg_a, port, hardware);
            return None;
        },
        0xdb => { // IN
            let port: Port = match port_byte {
                0 => panic!("INP0 port is not used by space invaders"),
                1 => Port::INP1,
                2 => Port::INP2,
                3 => Port::SHFTIN,
                _ => panic!("IN should only ever have an additional byte between 0 and 3"),
            };

            return Some(read_port(port, hardware));
        },
        _ => panic!("All other op_codes should be handled by the cpu module"),
    }
}

fn write_port(write_value: u8, port: Port, hardware: &mut Hardware) {
    match port {
        Port::SHFTAMNT => hardware.ports.shift_amount = write_value,
        Port::SOUND1 => hardware.ports.sound_1 = write_value,
        Port::SHFTDATA => hardware.shift_register = ((write_value as u16) << 8) | (hardware.shift_register >> 8),
        Port::SOUND2 => hardware.ports.sound_2 = write_value,
        Port::WATCHDOG => hardware.ports.watchdog = write_value,
        _ => panic!("Can only write to write ports"),
    }
}

fn read_port(port: Port, hardware: &mut Hardware) -> u8 {
    match port {
        Port::INP1 => return hardware.ports.input_1,
        Port::INP2 => return hardware.ports.input_2,
        Port::SHFTIN => {
            let left_offset = hardware.ports.shift_amount >> 5;
            // Only get bits 0-2 for offset
            let right_offset = 8 - left_offset;
            // we read 8 bit which leaves over right_offset of bits not read

            return (hardware.shift_register >> right_offset) as u8;
        },
        _ => panic!("Can only read from read ports"),
    }
}
