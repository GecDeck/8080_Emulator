use crate::Flags;
use crate::State;
use crate::Register;
use crate::Flag;

fn construct_address(h: Register, l: Register) -> u16 {
    // Creates an address from reading the value in H and L
    //  If H is 18 and L is d4 return 18d4
    // TODO: Ensure HL is the correct order

    return (h.read() as u16) << 8 | l.read() as u16;
}

fn add_op(receiver: u8, sender: i16, flags: &mut Flags) -> u8 {
    // General add and subtract operation
    //  Specific implementation for individual add ops will be done in handle_op_code()
    // A substract operation should send negative values

    let result = receiver as i16 + sender;
    // Do math with i16 to capture carry and negatives without over or underflow

    // Zero check
    if result == 0 { flags.set_flag(Flag::Z) }
    else { flags.clear_flag(Flag::Z) }

    // Negative Check
    if result < 0 { flags.set_flag(Flag::S) }
    else { flags.clear_flag(Flag::S) }

    // Parity Check
    let overflowed: u8 = (result.abs() & 0xff) as u8;
    if overflowed.count_ones() % 2 == 0 { flags.set_flag(Flag::P) }
    else { flags.clear_flag(Flag::P) }

    // Carry Check
    if result > u8::MAX as i16 { flags.set_flag(Flag::CY) }
    else { flags.clear_flag(Flag::CY) }

    return overflowed;
    // & 0xff discards anything outside of 8 bits
}

pub fn handle_op_code(op_code: u8, state: &mut State) -> u16 {
    // Returns the number of additional bytes read for the operation

    match op_code {
        0x00 => {},
        // NOP
        0x01 => panic!("Operation unimplemented"),
        0x02 => panic!("Operation unimplemented"),
        0x03 => panic!("Operation unimplemented"),
        0x04 => panic!("Operation unimplemented"),
        0x05 => panic!("Operation unimplemented"),
        0x06 => panic!("Operation unimplemented"),
        0x07 => panic!("Operation unimplemented"),
        0x08 => panic!("Operation unimplemented"),
        0x09 => panic!("Operation unimplemented"),
        0x0a => panic!("Operation unimplemented"),
        0x0b => panic!("Operation unimplemented"),
        0x0c => panic!("Operation unimplemented"),
        0x0d => panic!("Operation unimplemented"),
        0x0e => panic!("Operation unimplemented"),
        0x0f => panic!("Operation unimplemented"),
        0x10 => panic!("Operation unimplemented"),
        0x11 => panic!("Operation unimplemented"),
        0x12 => panic!("Operation unimplemented"),
        0x13 => panic!("Operation unimplemented"),
        0x14 => panic!("Operation unimplemented"),
        0x15 => panic!("Operation unimplemented"),
        0x16 => panic!("Operation unimplemented"),
        0x17 => panic!("Operation unimplemented"),
        0x18 => panic!("Operation unimplemented"),
        0x19 => panic!("Operation unimplemented"),
        0x1a => panic!("Operation unimplemented"),
        0x1b => panic!("Operation unimplemented"),
        0x1c => panic!("Operation unimplemented"),
        0x1d => panic!("Operation unimplemented"),
        0x1e => panic!("Operation unimplemented"),
        0x1f => panic!("Operation unimplemented"),
        0x20 => panic!("Operation unimplemented"),
        0x21 => panic!("Operation unimplemented"),
        0x22 => panic!("Operation unimplemented"),
        0x23 => panic!("Operation unimplemented"),
        0x24 => panic!("Operation unimplemented"),
        0x25 => panic!("Operation unimplemented"),
        0x26 => panic!("Operation unimplemented"),
        0x27 => panic!("Operation unimplemented"),
        0x28 => panic!("Operation unimplemented"),
        0x29 => panic!("Operation unimplemented"),
        0x2a => panic!("Operation unimplemented"),
        0x2b => panic!("Operation unimplemented"),
        0x2c => panic!("Operation unimplemented"),
        0x2d => panic!("Operation unimplemented"),
        0x2e => panic!("Operation unimplemented"),
        0x2f => panic!("Operation unimplemented"),
        0x30 => panic!("Operation unimplemented"),
        0x31 => panic!("Operation unimplemented"),
        0x32 => panic!("Operation unimplemented"),
        0x33 => panic!("Operation unimplemented"),
        0x34 => panic!("Operation unimplemented"),
        0x35 => panic!("Operation unimplemented"),
        0x36 => panic!("Operation unimplemented"),
        0x37 => panic!("Operation unimplemented"),
        0x38 => panic!("Operation unimplemented"),
        0x39 => panic!("Operation unimplemented"),
        0x3a => panic!("Operation unimplemented"),
        0x3b => panic!("Operation unimplemented"),
        0x3c => panic!("Operation unimplemented"),
        0x3d => panic!("Operation unimplemented"),
        0x3e => panic!("Operation unimplemented"),
        0x3f => panic!("Operation unimplemented"),

        // MOV OPERATIONS
        0x40 => state.b.value = state.b.value,
        0x41 => state.b.value = state.c.value,
        // Moves the value in C into B
        0x42 => state.b.value = state.d.value,
        0x43 => state.b.value = state.e.value,
        0x44 => state.b.value = state.h.value,
        0x45 => state.b.value = state.l.value,
        0x46 => state.b.value = state.memory.read_at( construct_address(state.h, state.l) ),
        // Moves the value in memory at the HL address into register B
        0x47 => state.b.value = state.a.value,
        0x48 => state.c.value = state.b.value,
        0x49 => state.c.value = state.c.value,
        0x4a => state.c.value = state.d.value,
        0x4b => state.c.value = state.e.value,
        0x4c => state.c.value = state.h.value,
        0x4d => state.c.value = state.l.value,
        0x4e => state.c.value = state.memory.read_at( construct_address(state.h, state.l) ),
        0x4f => state.c.value = state.a.value,
        0x50 => state.d.value = state.b.value,
        0x51 => state.d.value = state.c.value,
        0x52 => state.d.value = state.d.value,
        0x53 => state.d.value = state.e.value,
        0x54 => state.d.value = state.h.value,
        0x55 => state.d.value = state.l.value,
        0x56 => state.d.value = state.memory.read_at( construct_address(state.h, state.l) ),
        0x57 => state.d.value = state.a.value,
        0x58 => state.e.value = state.b.value,
        0x59 => state.e.value = state.c.value,
        0x5a => state.e.value = state.d.value,
        0x5b => state.e.value = state.e.value,
        0x5c => state.e.value = state.h.value,
        0x5d => state.e.value = state.l.value,
        0x5e => state.e.value = state.memory.read_at( construct_address(state.h, state.l) ),
        0x5f => state.e.value = state.a.value,
        0x60 => state.h.value = state.b.value,
        0x61 => state.h.value = state.c.value,
        0x62 => state.h.value = state.d.value,
        0x63 => state.h.value = state.e.value,
        0x64 => state.h.value = state.h.value,
        0x65 => state.h.value = state.l.value,
        0x66 => state.h.value = state.memory.read_at( construct_address(state.h, state.l) ),
        0x67 => state.h.value = state.a.value,
        0x68 => state.l.value = state.b.value,
        0x69 => state.l.value = state.c.value,
        0x6a => state.l.value = state.d.value,
        0x6b => state.l.value = state.e.value,
        0x6c => state.l.value = state.h.value,
        0x6d => state.l.value = state.l.value,
        0x6e => state.l.value = state.memory.read_at( construct_address(state.h, state.l) ),
        0x6f => state.l.value = state.a.value,
        0x70 => state.memory.write_at(construct_address(state.h, state.l), state.b.value),
        // Move the value in B into memory at the HL address
        0x71 => state.memory.write_at(construct_address(state.h, state.l), state.c.value),
        0x72 => state.memory.write_at(construct_address(state.h, state.l), state.d.value),
        0x73 => state.memory.write_at(construct_address(state.h, state.l), state.e.value),
        0x74 => state.memory.write_at(construct_address(state.h, state.l), state.h.value),
        0x75 => state.memory.write_at(construct_address(state.h, state.l), state.l.value),
        0x76 => panic!("HALT"),
        // TODO: should halt panic? Need to figure out what halt does
        0x77 => state.memory.write_at(construct_address(state.h, state.l), state.a.value),
        0x78 => state.a.value = state.b.value,
        0x79 => state.a.value = state.c.value,
        0x7a => state.a.value = state.d.value,
        0x7b => state.a.value = state.e.value,
        0x7c => state.a.value = state.h.value,
        0x7d => state.a.value = state.l.value,
        0x7e => state.a.value = state.memory.read_at( construct_address(state.h, state.l) ),
        0x7f => state.a.value = state.a.value,

        // ADD OPERATIONS
        0x80 => state.a.value = add_op(state.a.value, state.b.value as i16, &mut state.flags),
        0x81 => state.a.value = add_op(state.a.value, state.c.value as i16, &mut state.flags),
        0x82 => state.a.value = add_op(state.a.value, state.d.value as i16, &mut state.flags),
        0x83 => state.a.value = add_op(state.a.value, state.e.value as i16, &mut state.flags),
        0x84 => state.a.value = add_op(state.a.value, state.h.value as i16, &mut state.flags),
        0x85 => state.a.value = add_op(state.a.value, state.l.value as i16, &mut state.flags),
        0x86 => state.a.value = add_op(
            state.a.value,
            state.memory.read_at( construct_address(state.h, state.l) ) as i16,
            &mut state.flags),
        0x87 => state.a.value = add_op(state.a.value, state.a.value as i16, &mut state.flags),
        0x88 => panic!("Operation unimplemented"),
        0x89 => panic!("Operation unimplemented"),
        0x8a => panic!("Operation unimplemented"),
        0x8b => panic!("Operation unimplemented"),
        0x8c => panic!("Operation unimplemented"),
        0x8d => panic!("Operation unimplemented"),
        0x8e => panic!("Operation unimplemented"),
        0x8f => panic!("Operation unimplemented"),

        // SUBTRACT OPERATIONS
        0x90 => state.a.value = add_op(state.a.value, -(state.b.value as i16), &mut state.flags),
        0x91 => state.a.value = add_op(state.a.value, -(state.c.value as i16), &mut state.flags),
        0x92 => state.a.value = add_op(state.a.value, -(state.d.value as i16), &mut state.flags),
        0x93 => state.a.value = add_op(state.a.value, -(state.e.value as i16), &mut state.flags),
        0x94 => state.a.value = add_op(state.a.value, -(state.h.value as i16), &mut state.flags),
        0x95 => state.a.value = add_op(state.a.value, -(state.l.value as i16), &mut state.flags),
        0x96 => state.a.value = add_op(
            state.a.value,
            -( state.memory.read_at( construct_address(state.h, state.l) ) as i16 ),
            &mut state.flags),
        0x97 => state.a.value = add_op(state.a.value, -(state.a.value as i16), &mut state.flags),
        0x98 => panic!("Operation unimplemented"),
        0x99 => panic!("Operation unimplemented"),
        0x9a => panic!("Operation unimplemented"),
        0x9b => panic!("Operation unimplemented"),
        0x9c => panic!("Operation unimplemented"),
        0x9d => panic!("Operation unimplemented"),
        0x9e => panic!("Operation unimplemented"),
        0x9f => panic!("Operation unimplemented"),

        0xa0 => panic!("Operation unimplemented"),
        0xa1 => panic!("Operation unimplemented"),
        0xa2 => panic!("Operation unimplemented"),
        0xa3 => panic!("Operation unimplemented"),
        0xa4 => panic!("Operation unimplemented"),
        0xa5 => panic!("Operation unimplemented"),
        0xa6 => panic!("Operation unimplemented"),
        0xa7 => panic!("Operation unimplemented"),
        0xa8 => panic!("Operation unimplemented"),
        0xa9 => panic!("Operation unimplemented"),
        0xaa => panic!("Operation unimplemented"),
        0xab => panic!("Operation unimplemented"),
        0xac => panic!("Operation unimplemented"),
        0xad => panic!("Operation unimplemented"),
        0xae => panic!("Operation unimplemented"),
        0xaf => panic!("Operation unimplemented"),
        0xb0 => panic!("Operation unimplemented"),
        0xb1 => panic!("Operation unimplemented"),
        0xb2 => panic!("Operation unimplemented"),
        0xb3 => panic!("Operation unimplemented"),
        0xb4 => panic!("Operation unimplemented"),
        0xb5 => panic!("Operation unimplemented"),
        0xb6 => panic!("Operation unimplemented"),
        0xb7 => panic!("Operation unimplemented"),
        0xb8 => panic!("Operation unimplemented"),
        0xb9 => panic!("Operation unimplemented"),
        0xba => panic!("Operation unimplemented"),
        0xbb => panic!("Operation unimplemented"),
        0xbc => panic!("Operation unimplemented"),
        0xbd => panic!("Operation unimplemented"),
        0xbe => panic!("Operation unimplemented"),
        0xbf => panic!("Operation unimplemented"),
        0xc0 => panic!("Operation unimplemented"),
        0xc1 => panic!("Operation unimplemented"),
        0xc2 => panic!("Operation unimplemented"),
        0xc3 => panic!("Operation unimplemented"),
        0xc4 => panic!("Operation unimplemented"),
        0xc5 => panic!("Operation unimplemented"),
        0xc6 => panic!("Operation unimplemented"),
        0xc7 => panic!("Operation unimplemented"),
        0xc8 => panic!("Operation unimplemented"),
        0xc9 => panic!("Operation unimplemented"),
        0xca => panic!("Operation unimplemented"),
        0xcb => panic!("Operation unimplemented"),
        0xcc => panic!("Operation unimplemented"),
        0xcd => panic!("Operation unimplemented"),
        0xce => panic!("Operation unimplemented"),
        0xcf => panic!("Operation unimplemented"),
        0xd0 => panic!("Operation unimplemented"),
        0xd1 => panic!("Operation unimplemented"),
        0xd2 => panic!("Operation unimplemented"),
        0xd3 => panic!("Operation unimplemented"),
        0xd4 => panic!("Operation unimplemented"),
        0xd5 => panic!("Operation unimplemented"),
        0xd6 => panic!("Operation unimplemented"),
        0xd7 => panic!("Operation unimplemented"),
        0xd8 => panic!("Operation unimplemented"),
        0xd9 => panic!("Operation unimplemented"),
        0xda => panic!("Operation unimplemented"),
        0xdb => panic!("Operation unimplemented"),
        0xdc => panic!("Operation unimplemented"),
        0xdd => panic!("Operation unimplemented"),
        0xde => panic!("Operation unimplemented"),
        0xdf => panic!("Operation unimplemented"),
        0xe0 => panic!("Operation unimplemented"),
        0xe1 => panic!("Operation unimplemented"),
        0xe2 => panic!("Operation unimplemented"),
        0xe3 => panic!("Operation unimplemented"),
        0xe4 => panic!("Operation unimplemented"),
        0xe5 => panic!("Operation unimplemented"),
        0xe6 => panic!("Operation unimplemented"),
        0xe7 => panic!("Operation unimplemented"),
        0xe8 => panic!("Operation unimplemented"),
        0xe9 => panic!("Operation unimplemented"),
        0xea => panic!("Operation unimplemented"),
        0xeb => panic!("Operation unimplemented"),
        0xec => panic!("Operation unimplemented"),
        0xed => panic!("Operation unimplemented"),
        0xee => panic!("Operation unimplemented"),
        0xef => panic!("Operation unimplemented"),
        0xf0 => panic!("Operation unimplemented"),
        0xf1 => panic!("Operation unimplemented"),
        0xf2 => panic!("Operation unimplemented"),
        0xf3 => panic!("Operation unimplemented"),
        0xf4 => panic!("Operation unimplemented"),
        0xf5 => panic!("Operation unimplemented"),
        0xf6 => panic!("Operation unimplemented"),
        0xf7 => panic!("Operation unimplemented"),
        0xf8 => panic!("Operation unimplemented"),
        0xf9 => panic!("Operation unimplemented"),
        0xfa => panic!("Operation unimplemented"),
        0xfb => panic!("Operation unimplemented"),
        0xfc => panic!("Operation unimplemented"),
        0xfd => panic!("Operation unimplemented"),
        0xfe => panic!("Operation unimplemented"),
        0xff => panic!("Operation unimplemented"),
    }

    return 0;
    // If an operation doesn't specify the number of additional bytes it read
    //  the function will return 0 additional bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hl_address() {
        let h: Register = Register { data: 0x18, };
        let l: Register = Register { data: 0xd4, };
        assert_eq!(construct_address(h, l), 0x18d4);
    }

    #[test]
    fn test_add() {
        let mut flags: Flags = Flags::new();

        // Basic addition
        assert_eq!(add_op(80, 8, &mut flags), 88);
        assert_eq!(flags.flags, 0x00);
        flags.clear_flags();

        // Z flag setting
        assert_eq!(add_op(0, 0, &mut flags), 0);
        assert_eq!(flags.flags, 0b10100000);
        // Zero has even 1 parity
        flags.clear_flags();

        // S flag setting and basic subtraction
        assert_eq!(add_op(10, -21, &mut flags), 11);
        // TODO: Check if this should return the absolute value or something like 245
        assert_eq!(flags.flags, 0b01000000);
        flags.clear_flags();

        // Parity flag setting
        add_op(1, 2, &mut flags);
        assert_eq!(flags.flags, 0b00100000);
        add_op(1, 1, &mut flags);
        assert_eq!(flags.flags, 0b00000000);

        // Carry test
        assert_eq!(add_op(u8::MAX, 3, &mut flags), 2);
        assert_eq!(flags.flags, 0b00010000);
        flags.clear_flags();
    }
}
