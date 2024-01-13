pub mod operations;

pub struct Register {
    data: u8,
}
impl Register {
    pub fn new() -> Self {
        Self {
            data: 0x00
        }
    }

    pub fn read(&self) -> u8 {
        return self.data;
    }

    pub fn write(&mut self, byte: u8) {
        self.data = byte;
    }
}

pub struct AddressPointer {
    address: u16,
}
impl AddressPointer {
    pub fn new() -> Self {
        Self {
            address: 0x00,
        }
    }

    pub fn read_address(&self) -> u16 {
        return self.address;
    }

    pub fn increment(&mut self, steps: u16) {
        self.address += steps;
    }
}

pub struct Memory {
    held_memory: [u8; 65536],
    // 8080 should have 65536 addresses
}
impl Memory {
    pub fn init() -> Self {
        Self {
            held_memory: [0x00; 65536],
        }
    }

    pub fn read_at(&self, addr: u16) -> u8 {
        return self.held_memory[addr as usize];
    }

    pub fn write_at(&mut self, addr: u16, byte: u8) {
        self.held_memory[addr as usize] = byte;
    }
}

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

    pub fn clear_flags(&mut self) {
        self.flags = 0x00;
    }
}

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
            a: Register::new(),
            b: Register::new(),
            c: Register::new(),
            d: Register::new(),
            e: Register::new(),
            h: Register::new(),
            l: Register::new(),
            sp: AddressPointer::new(),
            pc: AddressPointer::new(),
            memory: Memory::init(),
            flags: Flags::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_rw() {
        let mut test_reg: Register = Register::new();

        assert_eq!(test_reg.read(), 0x00);

        test_reg.write(0xff);
        assert_eq!(test_reg.read(), 0xff);
    }

    #[test]
    fn test_memory_rw() {
        let mut test_mem: Memory = Memory::init();

        for i in 0..=65535 {
            assert_eq!(test_mem.read_at(i), 0x00);

            test_mem.write_at(i, 0xff);
            assert_eq!(test_mem.read_at(i), 0xff);
        }
    }

    #[test]
    fn test_set_flags() {
        let mut flags: Flags = Flags::new();

        flags.set_flag(Flag::Z);
        assert_eq!(flags.flags, 0b10000000);
        flags.clear_flags();

        flags.set_flag(Flag::S);
        assert_eq!(flags.flags, 0b01000000);
        flags.clear_flags();

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
