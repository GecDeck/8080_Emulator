#[cfg(test)]
use super::*;

#[test]
fn test_shift() {
    let mut hardware: Hardware = Hardware::init();

    write_port(0xff, Port::SHFTDATA, &mut hardware);
    assert_eq!(hardware.shift_register, 0xff00);
    write_port(0xee, Port::SHFTDATA, &mut hardware);
    assert_eq!(hardware.shift_register, 0xeeff);
    write_port(0xaa, Port::SHFTDATA, &mut hardware);
    assert_eq!(hardware.shift_register, 0xaaee);

    hardware.shift_register = 0b0001111111100000;
    hardware.ports.shift_amount = 0b0000_0011;
    // Offset 3
    assert_eq!(read_port(Port::SHFTIN, &mut hardware), 0b11111111);
}

#[test]
fn test_handle_io() {
    let mut hardware: Hardware = Hardware::init();

    // SHFTDATA
    handle_io(0xd3, &mut hardware, 4, 0b11100000);
    handle_io(0xd3, &mut hardware, 4, 0b00011111);
    assert_eq!(hardware.shift_register, 0b0001111111100000);

    // SHFTIN
    hardware.reset();
    hardware.shift_register = 0b0001111111100000;
    hardware.ports.shift_amount = 0b0000_0011;

    assert_eq!(handle_io(0xdb, &mut hardware, 3, 0x00), Some(0xff));

    // INPUT
    // TODO: write this
    // How do I test this well? This is why you write your tests before the code

    // SOUND
    // TODO: write this
}
