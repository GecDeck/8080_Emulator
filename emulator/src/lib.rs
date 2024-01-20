pub mod operations;

#[derive(Clone, Copy)]
pub struct Register {
    pub value: u8,
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
pub struct State {
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
impl State {
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
}
