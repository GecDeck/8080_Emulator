use std::fs;

// HARDWARE

#[derive(Clone, Copy)]
pub struct Register {
    value: u8,
}
impl Register {
    pub fn new() -> Self {
        Self {
            value: 0x00
        }
    }
}
impl Default for Register {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy)]
pub struct AddressPointer {
    pub address: u16,
}
impl AddressPointer {
    pub fn at(address: u16) -> Self {
        Self {
            address,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Memory {
    held_memory: [u8; 0xffff],
    // 8080 should have 65536 addresses
    // 0x0000 -> 0x2000 should contain rom
    // 0x2001 -> 0x2400 is ram
    // 0x2401 -> 0x4000 is vram
    // 0x4000 -> 0xffff is a mirror
}
impl Memory {
    pub fn init() -> Self {
        Self {
            held_memory: [0x00; 0xffff],
        }
    }

    pub fn read_at(&self, addr: u16) -> u8 {
        self.held_memory[addr as usize]
    }

    pub fn write_at(&mut self, addr: u16, byte: u8) {
        self.held_memory[addr as usize] = byte;
    }

    pub fn load_rom(&mut self, file_path: &str) {
        // Loads a rom into memory
        let rom: Vec<u8> = match fs::read(file_path) {
            Ok(result) => result,
            Err(e) => panic!("{}", e),
        };

        for (address, byte) in rom.iter().enumerate() {
            assert!(address < 0x2000);
            // Rom should fit in the space of memory reserved for roms

            self.write_at(address as u16, *byte);
        }
    }
}

#[derive(Clone, Copy)]
pub struct Flags {
    // Flags are set after operations to indicate the results
    flags: u8,

    // The flags are in order:
    // Z: set if the result is zero
    // S: Set if the result is negative,
    // P: Set if the number of 1 bits in the result is even
    // CY: Set if addition resulted in a carry or subtraction in a borrow
    // AC: Used for binary coded decimal arithmetic
    // The last 3 bits should be unused
}
pub enum Flag {
    Z,
    S,
    P,
    CY,
    AC,
}
impl Flags {
    pub fn new() -> Self {
        Self {
            flags: 0x00,
        }
    }

    pub fn set_flag(&mut self, flag: Flag) {
        match flag {
            Flag::Z => self.flags |= 0b10000000,
            Flag::S => self.flags |= 0b01000000,
            Flag::P => self.flags |= 0b00100000,
            Flag::CY => self.flags |= 0b00010000,
            Flag::AC => self.flags |= 0b00001000,
        }

        assert_ne!(self.flags << 5, 0b11100000);
        // Asserts that none of the extra 3 bits are set
        // TODO: This might not be necessary
    }

    pub fn clear_flag(&mut self, flag: Flag) {
        match flag {
            Flag::Z => self.flags &= 0b01111111,
            Flag::S => self.flags &= 0b10111111,
            Flag::P => self.flags &= 0b11011111,
            Flag::CY => self.flags &= 0b11101111,
            Flag::AC => self.flags &= 0b11110111,
        }

        assert_ne!(self.flags << 5, 0b11100000);
    }

    pub fn check_flag(&mut self, flag: Flag) -> u8 {
        match flag {
            Flag::Z => if self.flags & 0b10000000 == 0b10000000 { 1 }
            else { 0 },
            Flag::S => if self.flags & 0b01000000 == 0b01000000 { 1 }
            else { 0 },
            Flag::P => if self.flags & 0b00100000 == 0b00100000 { 1 }
            else { 0 },
            Flag::CY => if self.flags & 0b00010000 == 0b00010000 { 1 }
            else { 0 },
            Flag::AC => if self.flags & 0b00001000 == 0b00001000 { 1 }
            else { 0 },
        }
    }

    pub fn clear_flags(&mut self) {
        self.flags = 0x00;
    }
}
impl Default for Flags {
    fn default() -> Self {
        Flags::new()
    }
}

#[derive(Clone, Copy)]
pub struct Cpu {
    a: Register,
    b: Register,
    c: Register,
    d: Register,
    e: Register,
    h: Register,
    l: Register,
    sp: AddressPointer,
    pub pc: AddressPointer,
    pub memory: Memory,
    flags: Flags,
}
impl Cpu {
    pub fn init() -> Self {
        Self {
            a: Register::default(),
            b: Register::default(),
            c: Register::default(),
            d: Register::default(),
            e: Register::default(),
            h: Register::default(),
            l: Register::default(),
            sp: AddressPointer::at(0x2400),
            // Stack pointer starts at end of ram and decrements on push
            pc: AddressPointer::at(0x0000),
            memory: Memory::init(),
            flags: Flags::default(),
        }
    }
}

// OPERATIONS

fn construct_address(h: Register, l: Register) -> u16 {
    // Creates an address from reading the value in H and L
    //  If H is 18 and L is d4 return 18d4
    // TODO: Ensure HL is the correct order

    (h.value as u16) << 8 | l.value as u16
}

fn add(reg_1: u8, reg_2: u8, flags: &mut Flags) -> u8 {
    // General add operation

    let result = reg_1 as u16 + reg_2 as u16;
    // Do math with i16 to capture carry and negatives without over or underflow
    set_flags_from_operation(result as i16, flags);

    (result & 0xff) as u8
    // & 0xff discards anything outside of 8 bits
}

fn adc(reg_1: u8, reg_2: u8, flags: &mut Flags) -> u8 {
    // ADD but also adds value from carry flag

    let carry: u8 = flags.check_flag(Flag::CY);
    let result: u16 = add(reg_1, reg_2, flags) as u16 + carry as u16;

    (result & 0xff) as u8
}

fn sub(reg_1: u8, reg_2: u8, flags: &mut Flags) -> u8 {
    // Basic subtraction operation

    let result = reg_1 as i16 - reg_2 as i16;
    set_flags_from_operation(result, flags);

    (result & 0xff).unsigned_abs() as u8
}

fn sbb(reg_1: u8, reg_2: u8, flags: &mut Flags) -> u8 {
    // SUB but also removes the value of the carry flag

    let carry: u8 = flags.check_flag(Flag::CY);
    let result: i16 = sub(reg_1, reg_2, flags) as i16 - carry as i16;

    (result & 0xff).unsigned_abs() as u8
}

fn set_flags_from_operation(result: i16, flags: &mut Flags) {
    // Sets flags based on the result of an arithmetic operation

    // Zero check
    if result == 0 { flags.set_flag(Flag::Z) }
    else { flags.clear_flag(Flag::Z) }

    // Negative Check
    if result < 0 { flags.set_flag(Flag::S) }
    else { flags.clear_flag(Flag::S) }

    // Parity Check
    if ((result & 0xff) as u8).count_ones() % 2 == 0 { flags.set_flag(Flag::P) }
    else { flags.clear_flag(Flag::P) }

    // Carry Check
    if result > u8::MAX as i16 { flags.set_flag(Flag::CY) }
    else { flags.clear_flag(Flag::CY) }

}

pub fn handle_op_code(op_code: u8, cpu: &mut Cpu) -> u16 {
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
        0x40 => cpu.b.value = cpu.b.value,
        0x41 => cpu.b.value = cpu.c.value,
        // Moves the value in C into B
        0x42 => cpu.b.value = cpu.d.value,
        0x43 => cpu.b.value = cpu.e.value,
        0x44 => cpu.b.value = cpu.h.value,
        0x45 => cpu.b.value = cpu.l.value,
        0x46 => cpu.b.value = cpu.memory.read_at( construct_address(cpu.h, cpu.l) ),
        // Moves the value in memory at the HL address into register B
        0x47 => cpu.b.value = cpu.a.value,
        0x48 => cpu.c.value = cpu.b.value,
        0x49 => cpu.c.value = cpu.c.value,
        0x4a => cpu.c.value = cpu.d.value,
        0x4b => cpu.c.value = cpu.e.value,
        0x4c => cpu.c.value = cpu.h.value,
        0x4d => cpu.c.value = cpu.l.value,
        0x4e => cpu.c.value = cpu.memory.read_at( construct_address(cpu.h, cpu.l) ),
        0x4f => cpu.c.value = cpu.a.value,
        0x50 => cpu.d.value = cpu.b.value,
        0x51 => cpu.d.value = cpu.c.value,
        0x52 => cpu.d.value = cpu.d.value,
        0x53 => cpu.d.value = cpu.e.value,
        0x54 => cpu.d.value = cpu.h.value,
        0x55 => cpu.d.value = cpu.l.value,
        0x56 => cpu.d.value = cpu.memory.read_at( construct_address(cpu.h, cpu.l) ),
        0x57 => cpu.d.value = cpu.a.value,
        0x58 => cpu.e.value = cpu.b.value,
        0x59 => cpu.e.value = cpu.c.value,
        0x5a => cpu.e.value = cpu.d.value,
        0x5b => cpu.e.value = cpu.e.value,
        0x5c => cpu.e.value = cpu.h.value,
        0x5d => cpu.e.value = cpu.l.value,
        0x5e => cpu.e.value = cpu.memory.read_at( construct_address(cpu.h, cpu.l) ),
        0x5f => cpu.e.value = cpu.a.value,
        0x60 => cpu.h.value = cpu.b.value,
        0x61 => cpu.h.value = cpu.c.value,
        0x62 => cpu.h.value = cpu.d.value,
        0x63 => cpu.h.value = cpu.e.value,
        0x64 => cpu.h.value = cpu.h.value,
        0x65 => cpu.h.value = cpu.l.value,
        0x66 => cpu.h.value = cpu.memory.read_at( construct_address(cpu.h, cpu.l) ),
        0x67 => cpu.h.value = cpu.a.value,
        0x68 => cpu.l.value = cpu.b.value,
        0x69 => cpu.l.value = cpu.c.value,
        0x6a => cpu.l.value = cpu.d.value,
        0x6b => cpu.l.value = cpu.e.value,
        0x6c => cpu.l.value = cpu.h.value,
        0x6d => cpu.l.value = cpu.l.value,
        0x6e => cpu.l.value = cpu.memory.read_at( construct_address(cpu.h, cpu.l) ),
        0x6f => cpu.l.value = cpu.a.value,
        0x70 => cpu.memory.write_at(construct_address(cpu.h, cpu.l), cpu.b.value),
        // Move the value in B into memory at the HL address
        0x71 => cpu.memory.write_at(construct_address(cpu.h, cpu.l), cpu.c.value),
        0x72 => cpu.memory.write_at(construct_address(cpu.h, cpu.l), cpu.d.value),
        0x73 => cpu.memory.write_at(construct_address(cpu.h, cpu.l), cpu.e.value),
        0x74 => cpu.memory.write_at(construct_address(cpu.h, cpu.l), cpu.h.value),
        0x75 => cpu.memory.write_at(construct_address(cpu.h, cpu.l), cpu.l.value),
        0x76 => panic!("HALT"),
        // TODO: should halt panic? Need to figure out what halt does
        0x77 => cpu.memory.write_at(construct_address(cpu.h, cpu.l), cpu.a.value),
        0x78 => cpu.a.value = cpu.b.value,
        0x79 => cpu.a.value = cpu.c.value,
        0x7a => cpu.a.value = cpu.d.value,
        0x7b => cpu.a.value = cpu.e.value,
        0x7c => cpu.a.value = cpu.h.value,
        0x7d => cpu.a.value = cpu.l.value,
        0x7e => cpu.a.value = cpu.memory.read_at( construct_address(cpu.h, cpu.l) ),
        0x7f => cpu.a.value = cpu.a.value,

        // ADD OPERATIONS
        0x80 => cpu.a.value = add(cpu.a.value, cpu.b.value, &mut cpu.flags),
        0x81 => cpu.a.value = add(cpu.a.value, cpu.c.value, &mut cpu.flags),
        0x82 => cpu.a.value = add(cpu.a.value, cpu.d.value, &mut cpu.flags),
        0x83 => cpu.a.value = add(cpu.a.value, cpu.e.value, &mut cpu.flags),
        0x84 => cpu.a.value = add(cpu.a.value, cpu.h.value, &mut cpu.flags),
        0x85 => cpu.a.value = add(cpu.a.value, cpu.l.value, &mut cpu.flags),
        0x86 => cpu.a.value = add(cpu.a.value, cpu.memory.read_at( construct_address(cpu.h, cpu.l) ), &mut cpu.flags),
        0x87 => cpu.a.value = add(cpu.a.value, cpu.a.value, &mut cpu.flags),
        // ADC
        0x88 => cpu.a.value = adc(cpu.a.value, cpu.b.value, &mut cpu.flags),
        0x89 => cpu.a.value = adc(cpu.a.value, cpu.c.value, &mut cpu.flags),
        0x8a => cpu.a.value = adc(cpu.a.value, cpu.d.value, &mut cpu.flags),
        0x8b => cpu.a.value = adc(cpu.a.value, cpu.e.value, &mut cpu.flags),
        0x8c => cpu.a.value = adc(cpu.a.value, cpu.h.value, &mut cpu.flags),
        0x8d => cpu.a.value = adc(cpu.a.value, cpu.l.value, &mut cpu.flags),
        0x8e => cpu.a.value = adc(cpu.a.value, cpu.memory.read_at( construct_address(cpu.h, cpu.l) ), &mut cpu.flags),
        0x8f => cpu.a.value = adc(cpu.a.value, cpu.a.value, &mut cpu.flags),

        // SUBTRACT OPERATIONS
        0x90 => cpu.a.value = sub(cpu.a.value, cpu.b.value, &mut cpu.flags),
        0x91 => cpu.a.value = sub(cpu.a.value, cpu.c.value, &mut cpu.flags),
        0x92 => cpu.a.value = sub(cpu.a.value, cpu.d.value, &mut cpu.flags),
        0x93 => cpu.a.value = sub(cpu.a.value, cpu.e.value, &mut cpu.flags),
        0x94 => cpu.a.value = sub(cpu.a.value, cpu.h.value, &mut cpu.flags),
        0x95 => cpu.a.value = sub(cpu.a.value, cpu.l.value, &mut cpu.flags),
        0x96 => cpu.a.value = sub(cpu.a.value, cpu.memory.read_at( construct_address(cpu.h, cpu.l) ), &mut cpu.flags),
        0x97 => cpu.a.value = sub(cpu.a.value, cpu.a.value, &mut cpu.flags),
        // SBB
        0x98 => cpu.a.value = sbb(cpu.a.value, cpu.b.value, &mut cpu.flags),
        0x99 => cpu.a.value = sbb(cpu.a.value, cpu.c.value, &mut cpu.flags),
        0x9a => cpu.a.value = sbb(cpu.a.value, cpu.d.value, &mut cpu.flags),
        0x9b => cpu.a.value = sbb(cpu.a.value, cpu.e.value, &mut cpu.flags),
        0x9c => cpu.a.value = sbb(cpu.a.value, cpu.h.value, &mut cpu.flags),
        0x9d => cpu.a.value = sbb(cpu.a.value, cpu.l.value, &mut cpu.flags),
        0x9e => cpu.a.value = sbb(cpu.a.value, cpu.memory.read_at( construct_address(cpu.h, cpu.l) ), &mut cpu.flags),
        0x9f => cpu.a.value = sbb(cpu.a.value, cpu.a.value, &mut cpu.flags),

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

    0
    // If an operation doesn't specify the number of additional bytes it read
    //  the function will return 0 additional bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_rw() {
        let mut test_mem: Memory = Memory::init();

        for i in 0..65535 {
            assert_eq!(test_mem.read_at(i), 0x00);

            test_mem.write_at(i, 0xff);
            assert_eq!(test_mem.read_at(i), 0xff);
        }
    }

    #[test]
    fn test_flags() {
        let mut flags: Flags = Flags::default();

        flags.set_flag(Flag::Z);
        assert_eq!(flags.flags, 0b10000000);
        assert_eq!(flags.check_flag(Flag::Z), 1);
        flags.clear_flag(Flag::Z);
        assert!(flags.flags == 0x00);

        flags.set_flag(Flag::S);
        assert_eq!(flags.flags, 0b01000000);
        flags.clear_flag(Flag::S);
        assert!(flags.flags == 0x00);

        flags.set_flag(Flag::P);
        assert_eq!(flags.flags, 0b00100000);
        flags.clear_flags();

        flags.set_flag(Flag::CY);
        assert_eq!(flags.flags, 0b00010000);
        flags.clear_flags();

        flags.set_flag(Flag::AC);
        assert_eq!(flags.flags, 0b00001000);
        flags.clear_flags();
    }

    #[test]
    fn test_hl_address() {
        let h: Register = Register { value: 0x18, };
        let l: Register = Register { value: 0xd4, };
        assert_eq!(construct_address(h, l), 0x18d4);
    }

    #[test]
    fn test_operation_flag_setting() {
        let mut flags: Flags = Flags::default();

        // No flags
        set_flags_from_operation(2, &mut flags);
        assert_eq!(flags.flags, 0x00);

        // Z flag setting
        set_flags_from_operation(0, &mut flags);
        assert_eq!(flags.flags, 0b10100000);
        // Zero has even 1 parity

        // S flag setting
        set_flags_from_operation(-2, &mut flags);
        assert_eq!(flags.flags, 0b01000000);

        // Parity flag setting
        set_flags_from_operation(3, &mut flags);
        assert_eq!(flags.flags, 0b00100000);
        set_flags_from_operation(2, &mut flags);
        assert_eq!(flags.flags, 0b00000000);

        // Carry test
        set_flags_from_operation(258, &mut flags);
        assert_eq!(flags.flags, 0b00010000);
    }

    #[test]
    fn test_arithmetic_operations() {
        let mut flags: Flags = Flags::default();

        // ADD
        assert_eq!(add(0, 2, &mut flags), 2);
        assert_eq!(add(0xff, 3, &mut flags), 2);

        // ADC
        flags.set_flag(Flag::CY);
        assert_eq!(adc(0, 2, &mut flags), 3);
        flags.set_flag(Flag::CY);
        assert_eq!(adc(0xff, 0, &mut flags), 0);

        // SUB
        assert_eq!(sub(9, 8, &mut flags), 1);
        assert_eq!(sub(0, 1, &mut flags), 255);

        // SBB
        flags.set_flag(Flag::CY);
        assert_eq!(sbb(10, 9, &mut flags), 0);
        flags.set_flag(Flag::CY);
        assert_eq!(sbb(0, 0, &mut flags), 255);
    }

    #[test]
    fn test_operation_handling() {
        let mut cpu: Cpu = Cpu::init();

        // MOV test C -> B
        cpu.c.value = 0xd4;
        handle_op_code(0x41, &mut cpu);
        assert_eq!(cpu.b.value, 0xd4);

        // MOV test C -> M
        cpu.h.value = 0x18;
        cpu.l.value = 0xd4;
        cpu.c.value = 0xff;

        handle_op_code(0x71, &mut cpu);
        assert_eq!(cpu.memory.read_at(construct_address(cpu.h, cpu.l)), 0xff);

        // MOV test M -> B
        handle_op_code(0x46, &mut cpu);
        assert_eq!(cpu.b.value, 0xff);

        // ADD test A + B -> A
        cpu.a.value = 0xf0;
        cpu.b.value = 0x0f;

        handle_op_code(0x80, &mut cpu);
        assert_eq!(cpu.a.value, 0xff);

        // ADC test A + M + CY -> A
        // Putting 0x02 in memory
        cpu.h.value = 0x18;
        cpu.l.value = 0xd4;
        cpu.memory.write_at(0x18d4, 0x02);

        cpu.flags.set_flag(Flag::CY);
        cpu.a.value = 0x02;

        handle_op_code(0x8e, &mut cpu);
        assert_eq!(cpu.a.value, 0x05);
        // A = 2, M = 2, CY = 1 ... = 5

        // SUB test A - M -> A
        // Putting 0xff into memory
        cpu.h.value = 0x18;
        cpu.l.value = 0xd4;
        cpu.memory.write_at(0x18d4, 0xff);

        cpu.a.value = 0xff;

        handle_op_code(0x96, &mut cpu);
        assert_eq!(cpu.a.value, 0x00);

        // SBB test A - C - CY -> A
        cpu.a.value = 0x09;
        cpu.c.value = 0x08;
        cpu.flags.set_flag(Flag::CY);

        handle_op_code(0x99, &mut cpu);
        assert_eq!(cpu.a.value, 0x00);
    }
}
