use std::fs;

// HARDWARE

const STACK_MIN: u16 = 0x2001;
// This should be where the minimum stack address is

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
    // S: Set if the result is negative, -- 1st bit
    // Z: set if the result is zero -- 2nd bit
    // AC: Used for binary coded decimal arithmetic -- 4th bit
    // P: Set if the number of 1 bits in the result is even -- 6th bit
    // CY: Set if addition resulted in a carry or subtraction in a borrow -- 8th bit
}
pub enum Flag {
    S,
    Z,
    AC,
    P,
    CY,
}
impl Flags {
    pub fn new() -> Self {
        Self {
            flags: 0x00,
        }
    }

    pub fn set_flag(&mut self, flag: Flag) {
        match flag {
            Flag::S => self.flags |= 0b10000000,
            Flag::Z => self.flags |= 0b01000000,
            Flag::AC => self.flags |= 0b00010000,
            Flag::P => self.flags |= 0b00000100,
            Flag::CY => self.flags |= 0b00000001,
        }

        assert_ne!(self.flags << 5, 0b11100000);
        // Asserts that none of the extra 3 bits are set
        // TODO: This might not be necessary
    }

    pub fn clear_flag(&mut self, flag: Flag) {
        match flag {
            Flag::S => self.flags &= 0b01111111,
            Flag::Z => self.flags &= 0b10111111,
            Flag::AC => self.flags &= 0b11101111,
            Flag::P => self.flags &= 0b11111011,
            Flag::CY => self.flags &= 0b11111110,
        }

        assert_ne!(self.flags << 5, 0b11100000);
    }

    pub fn check_flag(&mut self, flag: Flag) -> u8 {
        match flag {
            Flag::S => if self.flags & 0b10000000 == 0b10000000 { 1 }
            else { 0 },
            Flag::Z => if self.flags & 0b01000000 == 0b01000000 { 1 }
            else { 0 },
            Flag::AC => if self.flags & 0b00010000 == 0b00010000 { 1 }
            else { 0 },
            Flag::P => if self.flags & 0b00000100 == 0b00000100 { 1 }
            else { 0 },
            Flag::CY => if self.flags & 0b00000001 == 0b00000001 { 1 }
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

    pub fn reset(&mut self) {
        // Resets all the values of the cpu
        self.a = Register::default();
        self.b = Register::default();
        self.c = Register::default();
        self.d = Register::default();
        self.e = Register::default();
        self.h = Register::default();
        self.l = Register::default();
        self.sp = AddressPointer::at(0x2400);
        self.pc = AddressPointer::at(0x0000);
        self.memory = Memory::init();
        self.flags = Flags::default();
    }

    pub fn check_stack_overflow(&self) -> bool {
        // Checks if the stack has overflowed
        // The stack grows growns downwards on the 8080
        if self.sp.address < STACK_MIN {
            println!("STACK OVERFLOW");
            return true;
        }
        false
    }
}

// OPERATIONS

fn inx(reg_pair: u16) -> (u8, u8) {
    // Treats a pair of 8 bit registers as one 16 bit register and increments it
    // Returns the byte to be stored in the 1st register and 2nd register respectively

    let result: u32 = reg_pair as u32 + 1;
    // Using a u32 to avoid panic on overflow

    split_register_pair(result as u16)
}

fn dcx(reg_pair: u16) -> (u8, u8) {
    // Treats a pair of 8 bit registers as one 16 bit register and decrements it
    // Returns the byte to be stored in the 1st register and 2nd register respectively

    let result: i32 = reg_pair as i32 - 1;
    // Using a i32 to avoid panic on overflow or underflow

    split_register_pair((result & 0xffff) as u16)
}

fn inr(reg: u8, flags: &mut Flags) -> u8 {
    // Increments an 8 bit register
    // INR does not effect the carry flag

    let carry: u8 = flags.check_flag(Flag::CY);
    // Hold the status of the carry flag

    let result: u8 = add(reg, 1, flags);
    // Increment

    match carry {
        1 => flags.set_flag(Flag::CY),
        0 => flags.clear_flag(Flag::CY),
        _ => panic!("check_flag cannot return anything other then 0 or 1"),
    }
    // Resets the carry flag to what it was before since this operation should not effect it

    result
}

fn dcr(reg: u8, flags: &mut Flags) -> u8 {
    // Decrements an 8 bit register
    // DCR does not effect the carry flag

    let carry: u8 = flags.check_flag(Flag::CY);
    // Hold status of carry flag

    let result: u8 = sub(reg, 1, flags);
    // Decrement

    match carry {
        1 => flags.set_flag(Flag::CY),
        0 => flags.clear_flag(Flag::CY),
        _ => panic!("check_flag cannot return anything other than 0 or 1"),
    }
    // Resets carry flag

    result
}

fn dad(hl_pair: u16, reg_pair: u16, flags: &mut Flags) -> (u8, u8) {
    // Adds the value in register pair to the register pair HL
    // This operation only affects the carry flag

    let result: u32 = hl_pair as u32 + reg_pair as u32;
    // u32 to catch carry

    if result > u16::MAX as u32 { flags.set_flag(Flag::CY) }
    else { flags.clear_flag(Flag::CY) }
    // Setting carry flag
    // TODO: find a way to call set_flags_from_operation here

    split_register_pair(result as u16)
}

fn add(reg_1: u8, reg_2: u8, flags: &mut Flags) -> u8 {
    // General add operation

    let result: u16 = reg_1 as u16 + reg_2 as u16;
    // Do math with i16 to capture carry and negatives without over or underflow
    *flags = set_flags_from_operation(result as i16, *flags);

    result as u8
}

fn adc(reg_1: u8, reg_2: u8, flags: &mut Flags) -> u8 {
    // ADD but also adds value from carry flag

    let carry: u8 = flags.check_flag(Flag::CY);
    let result: u16 = add(reg_1, reg_2, flags) as u16 + carry as u16;

    result as u8
}

fn sub(reg_1: u8, reg_2: u8, flags: &mut Flags) -> u8 {
    // Basic subtraction operation

    let result = reg_1 as i16 - reg_2 as i16;
    *flags = set_flags_from_operation(result, *flags);

    (result & 0xff) as u8
    // Rust casting will cast i16 to a u16 first then to a u8
    //  This means -1 would become 1, but we want it to be 255
    //  So we and result with 0xff to skip straight to casting to a u8
    //  as u8 SHOULD then effectively do nothing to the actual value of result
}

fn sbb(reg_1: u8, reg_2: u8, flags: &mut Flags) -> u8 {
    // SUB but also removes the value of the carry flag

    let carry: u8 = flags.check_flag(Flag::CY);
    let result: i16 = sub(reg_1, reg_2, flags) as i16 - carry as i16;

    (result & 0xff) as u8
}

fn jmp(address_bytes: (u8, u8), condition: Option<bool>) -> Option<u16> {
    // Jumps to an address in memory, and optionally does so conditionaly
    // The condition will be whether a specific flag is set or not
    // TODO: should this modify the pc address directly???
    //  arithmetic operations modify the flags directly but I don't know if I actually like that

    if condition.is_none() | condition.is_some_and(|condition| condition == true) {
        // If there is no condition or the supplied condition is true do the following
        let address: u16 = pair_registers(address_bytes.1, address_bytes.0);
        // Little endian order
        // This is a horrible name for a function if i'm calling it here

        return Some(address);
    }

    None
}

fn call(
    address_bytes: (u8, u8),
    condition: Option<bool>,
    stack_pointer: &mut AddressPointer,
    memory: &mut Memory,
    return_adress: u16
    ) -> Option<u16> {
    // Pushes the return address to the stack then conditionally returns the address to jump to
    // The return address is the address of the next instruction

    let jmp_address: Option<u16> = jmp(address_bytes, condition);

    match jmp_address {
        Some(_) => {
            // Only add to stack if there is somewhere to jump to
            let return_adress_bytes: (u8, u8) = split_register_pair(return_adress);
            push((return_adress_bytes.1, return_adress_bytes.0), stack_pointer, memory);
            // Push return address to stack
            // 0xc3d4 will be pushed as 0xd4 0xc3
        }
        None => { }
    }

    jmp_address
}

fn ret(condition: Option<bool>, stack_pointer: &mut AddressPointer, memory: &mut Memory) -> Option<u16> {
    // Pops the return address from the stack and conditionally returns it

    if condition.is_none() | condition.is_some_and(|condition| condition == true) {
        // If there is no condition or the supplied condition is true do the following

        let return_adress_bytes: (u8, u8) = pop(stack_pointer, memory);
        // if the address 0xc3d4 was pushed this should return (0xd4, 0xc3)
        let return_adress: u16 = pair_registers(return_adress_bytes.1, return_adress_bytes.0);

        return Some(return_adress);
    }

    None
}

fn push(data_bytes: (u8, u8), stack_pointer: &mut AddressPointer, memory: &mut Memory) {
    // Puts some data onto the stack

    memory.write_at(stack_pointer.address, data_bytes.0);
    memory.write_at(stack_pointer.address - 1, data_bytes.1);
    // d4 c3 will go in as d4 c3

    stack_pointer.address -= 2;
    // stacl grows downwards
}

fn pop(stack_pointer: &mut AddressPointer, memory: &mut Memory) -> (u8, u8) {
    // Returns the data at the top of the stack

    let byte_1 = memory.read_at(stack_pointer.address + 2);
    let byte_2 = memory.read_at(stack_pointer.address + 1);
    // Find two bytes before stack pointer

    memory.write_at(stack_pointer.address + 1, 0x00);
    memory.write_at(stack_pointer.address + 2, 0x00);
    // Zeroes memory, probably not necessary but nice for cleanliness and debugging

    stack_pointer.address += 2;
    // stack shrinks upwards

    (byte_1, byte_2)
}

fn set_flags_from_operation(result: i16, flags: Flags) -> Flags {
    // Sets flags based on the result of an arithmetic operation
    let mut return_flags: Flags = flags;

    // Zero check
    if result == 0 { return_flags.set_flag(Flag::Z) }
    else { return_flags.clear_flag(Flag::Z) }

    // Negative Check
    if result < 0 { return_flags.set_flag(Flag::S) }
    else { return_flags.clear_flag(Flag::S) }

    // Parity Check
    if ((result & 0xff) as u8).count_ones() % 2 == 0 { return_flags.set_flag(Flag::P) }
    else { return_flags.clear_flag(Flag::P) }

    // Carry Check
    if result > u8::MAX as i16 { return_flags.set_flag(Flag::CY) }
    else { return_flags.clear_flag(Flag::CY) }

    return_flags
}

fn pair_registers(reg_1: u8, reg_2: u8) -> u16 {
    // Creates an address from reading the value in H and L
    //  If H is 18 and L is d4 return 18d4
    // TODO: Ensure HL is the correct order

    (reg_1 as u16) << 8 | reg_2 as u16
}

fn split_register_pair(reg_pair: u16) -> (u8, u8) {
    // Splits a u16 register pair into a tuple of u8s

    let byte_1: u8 = (reg_pair >> 8) as u8;
    let byte_2: u8 = (reg_pair & 0xff) as u8;

    (byte_1, byte_2)
}

pub fn handle_op_code(op_code: u8, cpu: &mut Cpu) -> u16 {
    // Reads an op_code and performs the cooresponding operation
    // Returns the number of additional bytes read for the operation

    match op_code {
        0x00 => {},
        // NOP
        0x01 => panic!("Operation unimplemented"),
        0x02 => panic!("Operation unimplemented"),
        0x03 => (cpu.b.value, cpu.c.value) = inx( pair_registers(cpu.b.value, cpu.c.value) ),
        0x04 => cpu.b.value = inr(cpu.b.value, &mut cpu.flags),
        0x05 => cpu.b.value = dcr(cpu.b.value, &mut cpu.flags),
        0x06 => panic!("Operation unimplemented"),
        0x07 => panic!("Operation unimplemented"),
        0x08 => panic!("Operation unimplemented"),
        0x09 => (cpu.h.value, cpu.l.value) = dad(
            pair_registers(cpu.h.value, cpu.l.value),
            pair_registers(cpu.b.value, cpu.c.value),
            &mut cpu.flags
            ),
        0x0a => panic!("Operation unimplemented"),
        0x0b => (cpu.b.value, cpu.c.value) = dcx( pair_registers(cpu.b.value, cpu.c.value) ),
        0x0c => cpu.c.value = inr(cpu.c.value, &mut cpu.flags),
        0x0d => cpu.c.value = dcr(cpu.c.value, &mut cpu.flags),
        0x0e => panic!("Operation unimplemented"),
        0x0f => panic!("Operation unimplemented"),
        0x10 => panic!("Operation unimplemented"),
        0x11 => panic!("Operation unimplemented"),
        0x12 => panic!("Operation unimplemented"),
        0x13 => (cpu.d.value, cpu.e.value) = inx( pair_registers(cpu.d.value, cpu.c.value) ),
        0x14 => cpu.d.value = inr(cpu.d.value, &mut cpu.flags),
        0x15 => cpu.d.value = dcr(cpu.d.value, &mut cpu.flags),
        0x16 => panic!("Operation unimplemented"),
        0x17 => panic!("Operation unimplemented"),
        0x18 => panic!("Operation unimplemented"),
        0x19 => (cpu.h.value, cpu.l.value) = dad(
            pair_registers(cpu.h.value, cpu.l.value),
            pair_registers(cpu.d.value, cpu.e.value),
            &mut cpu.flags
            ),
        0x1a => panic!("Operation unimplemented"),
        0x1b => (cpu.d.value, cpu.e.value) = dcx( pair_registers(cpu.d.value, cpu.e.value) ),
        0x1c => cpu.e.value = inr(cpu.e.value, &mut cpu.flags),
        0x1d => cpu.e.value = dcr(cpu.e.value, &mut cpu.flags),
        0x1e => panic!("Operation unimplemented"),
        0x1f => panic!("Operation unimplemented"),
        0x20 => panic!("Operation unimplemented"),
        0x21 => panic!("Operation unimplemented"),
        0x22 => panic!("Operation unimplemented"),
        0x23 => (cpu.h.value, cpu.l.value) = inx( pair_registers(cpu.h.value, cpu.l.value) ),
        0x24 => cpu.h.value = inr(cpu.h.value, &mut cpu.flags),
        0x25 => cpu.h.value = dcr(cpu.h.value, &mut cpu.flags),
        0x26 => panic!("Operation unimplemented"),
        0x27 => panic!("Operation unimplemented"),
        0x28 => panic!("Operation unimplemented"),
        0x29 => (cpu.h.value, cpu.l.value) = dad(
            pair_registers(cpu.h.value, cpu.l.value),
            pair_registers(cpu.h.value, cpu.l.value),
            &mut cpu.flags
            ),
        // This is documented as HL = HL + HI
        //  But I think it's supposed to just add HL to itself? I don't what what I means
        //  TODO: find out what I means
        0x2a => panic!("Operation unimplemented"),
        0x2b => (cpu.h.value, cpu.l.value) = dcx( pair_registers(cpu.h.value, cpu.l.value) ),
        0x2c => cpu.l.value = inr(cpu.l.value, &mut cpu.flags),
        0x2d => cpu.l.value = dcr(cpu.l.value, &mut cpu.flags),
        0x2e => panic!("Operation unimplemented"),
        0x2f => panic!("Operation unimplemented"),
        0x30 => panic!("Operation unimplemented"),
        0x31 => panic!("Operation unimplemented"),
        0x32 => panic!("Operation unimplemented"),
        0x33 => {
            let (sp_1, sp_2): (u8, u8) = split_register_pair(cpu.sp.address);
            let (byte_1, byte_2): (u8, u8) = inx( pair_registers(sp_1, sp_2) );
            cpu.sp.address = pair_registers(byte_1, byte_2);
        },
        0x34 => cpu.memory.write_at(
            pair_registers(cpu.h.value, cpu.l.value),
            inr(
                cpu.memory.read_at(
                    pair_registers(cpu.h.value, cpu.l.value)),
                    &mut cpu.flags)
            ),
        0x35 => cpu.memory.write_at(
            pair_registers(cpu.h.value, cpu.l.value), 
            dcr(
                cpu.memory.read_at(
                    pair_registers(cpu.h.value, cpu.l.value)),
                    &mut cpu.flags)
            ),
        0x36 => panic!("Operation unimplemented"),
        0x37 => panic!("Operation unimplemented"),
        0x38 => panic!("Operation unimplemented"),
        0x39 => (cpu.h.value, cpu.l.value) = dad(
            pair_registers(cpu.h.value, cpu.l.value),
            cpu.sp.address,
            &mut cpu.flags
            ),
        0x3a => panic!("Operation unimplemented"),
        0x3b => {
            let (sp_1, sp_2): (u8, u8) = split_register_pair(cpu.sp.address);
            let (byte_1, byte_2): (u8, u8) = dcx( pair_registers(sp_1, sp_2) );
            cpu.sp.address = pair_registers(byte_1, byte_2);
        },
        0x3c => cpu.a.value = inr(cpu.a.value, &mut cpu.flags),
        0x3d => cpu.a.value = dcr(cpu.a.value, &mut cpu.flags),
        0x3e => panic!("Operation unimplemented"),
        0x3f => panic!("Operation unimplemented"),

        // MOV OPERATIONS
        0x40 => cpu.b.value = cpu.b.value,
        0x41 => cpu.b.value = cpu.c.value,
        0x42 => cpu.b.value = cpu.d.value,
        0x43 => cpu.b.value = cpu.e.value,
        0x44 => cpu.b.value = cpu.h.value,
        0x45 => cpu.b.value = cpu.l.value,
        0x46 => cpu.b.value = cpu.memory.read_at( pair_registers(cpu.h.value, cpu.l.value) ),
        0x47 => cpu.b.value = cpu.a.value,
        0x48 => cpu.c.value = cpu.b.value,
        0x49 => cpu.c.value = cpu.c.value,
        0x4a => cpu.c.value = cpu.d.value,
        0x4b => cpu.c.value = cpu.e.value,
        0x4c => cpu.c.value = cpu.h.value,
        0x4d => cpu.c.value = cpu.l.value,
        0x4e => cpu.c.value = cpu.memory.read_at( pair_registers(cpu.h.value, cpu.l.value) ),
        0x4f => cpu.c.value = cpu.a.value,
        0x50 => cpu.d.value = cpu.b.value,
        0x51 => cpu.d.value = cpu.c.value,
        0x52 => cpu.d.value = cpu.d.value,
        0x53 => cpu.d.value = cpu.e.value,
        0x54 => cpu.d.value = cpu.h.value,
        0x55 => cpu.d.value = cpu.l.value,
        0x56 => cpu.d.value = cpu.memory.read_at( pair_registers(cpu.h.value, cpu.l.value) ),
        0x57 => cpu.d.value = cpu.a.value,
        0x58 => cpu.e.value = cpu.b.value,
        0x59 => cpu.e.value = cpu.c.value,
        0x5a => cpu.e.value = cpu.d.value,
        0x5b => cpu.e.value = cpu.e.value,
        0x5c => cpu.e.value = cpu.h.value,
        0x5d => cpu.e.value = cpu.l.value,
        0x5e => cpu.e.value = cpu.memory.read_at( pair_registers(cpu.h.value, cpu.l.value) ),
        0x5f => cpu.e.value = cpu.a.value,
        0x60 => cpu.h.value = cpu.b.value,
        0x61 => cpu.h.value = cpu.c.value,
        0x62 => cpu.h.value = cpu.d.value,
        0x63 => cpu.h.value = cpu.e.value,
        0x64 => cpu.h.value = cpu.h.value,
        0x65 => cpu.h.value = cpu.l.value,
        0x66 => cpu.h.value = cpu.memory.read_at( pair_registers(cpu.h.value, cpu.l.value) ),
        0x67 => cpu.h.value = cpu.a.value,
        0x68 => cpu.l.value = cpu.b.value,
        0x69 => cpu.l.value = cpu.c.value,
        0x6a => cpu.l.value = cpu.d.value,
        0x6b => cpu.l.value = cpu.e.value,
        0x6c => cpu.l.value = cpu.h.value,
        0x6d => cpu.l.value = cpu.l.value,
        0x6e => cpu.l.value = cpu.memory.read_at( pair_registers(cpu.h.value, cpu.l.value) ),
        0x6f => cpu.l.value = cpu.a.value,
        0x70 => cpu.memory.write_at(pair_registers(cpu.h.value, cpu.l.value), cpu.b.value),
        0x71 => cpu.memory.write_at(pair_registers(cpu.h.value, cpu.l.value), cpu.c.value),
        0x72 => cpu.memory.write_at(pair_registers(cpu.h.value, cpu.l.value), cpu.d.value),
        0x73 => cpu.memory.write_at(pair_registers(cpu.h.value, cpu.l.value), cpu.e.value),
        0x74 => cpu.memory.write_at(pair_registers(cpu.h.value, cpu.l.value), cpu.h.value),
        0x75 => cpu.memory.write_at(pair_registers(cpu.h.value, cpu.l.value), cpu.l.value),
        0x76 => panic!("HALT"),
        // TODO: should halt panic? Need to figure out what halt does
        0x77 => cpu.memory.write_at(pair_registers(cpu.h.value, cpu.l.value), cpu.a.value),
        0x78 => cpu.a.value = cpu.b.value,
        0x79 => cpu.a.value = cpu.c.value,
        0x7a => cpu.a.value = cpu.d.value,
        0x7b => cpu.a.value = cpu.e.value,
        0x7c => cpu.a.value = cpu.h.value,
        0x7d => cpu.a.value = cpu.l.value,
        0x7e => cpu.a.value = cpu.memory.read_at( pair_registers(cpu.h.value, cpu.l.value) ),
        0x7f => cpu.a.value = cpu.a.value,

        // ADD OPERATIONS
        0x80 => cpu.a.value = add(cpu.a.value, cpu.b.value, &mut cpu.flags),
        0x81 => cpu.a.value = add(cpu.a.value, cpu.c.value, &mut cpu.flags),
        0x82 => cpu.a.value = add(cpu.a.value, cpu.d.value, &mut cpu.flags),
        0x83 => cpu.a.value = add(cpu.a.value, cpu.e.value, &mut cpu.flags),
        0x84 => cpu.a.value = add(cpu.a.value, cpu.h.value, &mut cpu.flags),
        0x85 => cpu.a.value = add(cpu.a.value, cpu.l.value, &mut cpu.flags),
        0x86 => cpu.a.value = add(cpu.a.value, cpu.memory.read_at( pair_registers(cpu.h.value, cpu.l.value) ), &mut cpu.flags),
        0x87 => cpu.a.value = add(cpu.a.value, cpu.a.value, &mut cpu.flags),
        // ADC
        0x88 => cpu.a.value = adc(cpu.a.value, cpu.b.value, &mut cpu.flags),
        0x89 => cpu.a.value = adc(cpu.a.value, cpu.c.value, &mut cpu.flags),
        0x8a => cpu.a.value = adc(cpu.a.value, cpu.d.value, &mut cpu.flags),
        0x8b => cpu.a.value = adc(cpu.a.value, cpu.e.value, &mut cpu.flags),
        0x8c => cpu.a.value = adc(cpu.a.value, cpu.h.value, &mut cpu.flags),
        0x8d => cpu.a.value = adc(cpu.a.value, cpu.l.value, &mut cpu.flags),
        0x8e => cpu.a.value = adc(cpu.a.value, cpu.memory.read_at( pair_registers(cpu.h.value, cpu.l.value) ), &mut cpu.flags),
        0x8f => cpu.a.value = adc(cpu.a.value, cpu.a.value, &mut cpu.flags),

        // SUBTRACT OPERATIONS
        0x90 => cpu.a.value = sub(cpu.a.value, cpu.b.value, &mut cpu.flags),
        0x91 => cpu.a.value = sub(cpu.a.value, cpu.c.value, &mut cpu.flags),
        0x92 => cpu.a.value = sub(cpu.a.value, cpu.d.value, &mut cpu.flags),
        0x93 => cpu.a.value = sub(cpu.a.value, cpu.e.value, &mut cpu.flags),
        0x94 => cpu.a.value = sub(cpu.a.value, cpu.h.value, &mut cpu.flags),
        0x95 => cpu.a.value = sub(cpu.a.value, cpu.l.value, &mut cpu.flags),
        0x96 => cpu.a.value = sub(cpu.a.value, cpu.memory.read_at( pair_registers(cpu.h.value, cpu.l.value) ), &mut cpu.flags),
        0x97 => cpu.a.value = sub(cpu.a.value, cpu.a.value, &mut cpu.flags),
        // SBB
        0x98 => cpu.a.value = sbb(cpu.a.value, cpu.b.value, &mut cpu.flags),
        0x99 => cpu.a.value = sbb(cpu.a.value, cpu.c.value, &mut cpu.flags),
        0x9a => cpu.a.value = sbb(cpu.a.value, cpu.d.value, &mut cpu.flags),
        0x9b => cpu.a.value = sbb(cpu.a.value, cpu.e.value, &mut cpu.flags),
        0x9c => cpu.a.value = sbb(cpu.a.value, cpu.h.value, &mut cpu.flags),
        0x9d => cpu.a.value = sbb(cpu.a.value, cpu.l.value, &mut cpu.flags),
        0x9e => cpu.a.value = sbb(cpu.a.value, cpu.memory.read_at( pair_registers(cpu.h.value, cpu.l.value) ), &mut cpu.flags),
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
        0xc0 => { // RNZ
            let ret_address: Option<u16> = ret(
                Some(cpu.flags.check_flag(Flag::Z) == 0),
                &mut cpu.sp, &mut cpu.memory
                );
            match ret_address {
                Some(address) => cpu.pc.address = address,
                None => { return 0 },
            };
        },
        0xc1 => panic!("Operation unimplemented"),
        0xc2 => { // JNZ
            let jmp_address: Option<u16> = jmp(
                (cpu.memory.read_at(cpu.pc.address), cpu.memory.read_at(cpu.pc.address + 1)),
                Some(cpu.flags.check_flag(Flag::Z) == 0)
                );
            match jmp_address {
                Some(address) => cpu.pc.address = address,
                None => return 2,
            };
        },
        0xc3 => { // JMP
            let jmp_address: Option<u16> = jmp(
                (cpu.memory.read_at(cpu.pc.address), cpu.memory.read_at(cpu.pc.address + 1)),
                None
                );
            cpu.pc.address = jmp_address.expect("jmp with no condition should always return Some(address)");
            return 2;
        },
        0xc4 => { // CNZ
            let call_address: Option<u16> = call(
                (cpu.memory.read_at(cpu.pc.address), cpu.memory.read_at(cpu.pc.address + 1)),
                Some(cpu.flags.check_flag(Flag::Z) == 0),
                &mut cpu.sp, &mut cpu.memory,
                cpu.pc.address + 2
                );
            match call_address {
                Some(address) => cpu.pc.address = address,
                None => return 2,
            };
        },
        0xc5 => panic!("Operation unimplemented"),
        0xc6 => { // ADI
            cpu.a.value = add(cpu.a.value, cpu.memory.read_at(cpu.pc.address), &mut cpu.flags);
            return 1;
        },
        0xc7 => { // RST 0
            let call_address: Option<u16> = call(
                (0x00, 0x00),
                None,
                &mut cpu.sp, &mut cpu.memory,
                cpu.pc.address
                );
            cpu.pc.address = call_address.expect("call with no condition always returns an address");
        },
        0xc8 => { // RZ
            let ret_address: Option<u16> = ret(
                Some(cpu.flags.check_flag(Flag::Z) == 1),
                &mut cpu.sp, &mut cpu.memory
                );
            match ret_address {
                Some(address) => cpu.pc.address = address,
                None => { return 0 },
            };
        },
        0xc9 => { // RET
            let ret_address: Option<u16> = ret(
                None,
                &mut cpu.sp, &mut cpu.memory
                );
            cpu.pc.address = ret_address.expect("ret with no conditions always returns an address");
        },
        0xca => { // JZ
            let jmp_address: Option<u16> = jmp(
                (cpu.memory.read_at(cpu.pc.address), cpu.memory.read_at(cpu.pc.address + 1)),
                Some(cpu.flags.check_flag(Flag::Z) == 1)
                );
            match jmp_address {
                Some(address) => cpu.pc.address = address,
                None => return 2,
            };
        },
        0xcb => panic!("Operation unimplemented"),
        0xcc => { // CZ
            let call_address: Option<u16> = call(
                (cpu.memory.read_at(cpu.pc.address), cpu.memory.read_at(cpu.pc.address + 1)),
                Some(cpu.flags.check_flag(Flag::Z) == 1),
                &mut cpu.sp, &mut cpu.memory,
                cpu.pc.address + 2
                );
            match call_address {
                Some(address) => cpu.pc.address = address,
                None => return 2,
            };
        },
        0xcd => { // CALL
            let call_address: Option<u16> = call(
                (cpu.memory.read_at(cpu.pc.address), cpu.memory.read_at(cpu.pc.address + 1)),
                None,
                &mut cpu.sp, &mut cpu.memory,
                cpu.pc.address + 2
                );
            cpu.pc.address = call_address.expect("call with no condition always returns an address");
            return 2;
        },
        0xce => { // ACI
            cpu.a.value = adc(cpu.a.value, cpu.memory.read_at(cpu.pc.address), &mut cpu.flags);
            return 1;
        },
        0xcf => { // RST 1
            let call_address: Option<u16> = call(
                (0x08, 0x00),
                None,
                &mut cpu.sp, &mut cpu.memory,
                cpu.pc.address
                );
            cpu.pc.address = call_address.expect("call with no condition always returns an address");
        },
        0xd0 => { // RNC
            let ret_address: Option<u16> = ret(
                Some(cpu.flags.check_flag(Flag::CY) == 0),
                &mut cpu.sp, &mut cpu.memory
                );
            match ret_address {
                Some(address) => cpu.pc.address = address,
                None => { return 0 },
            };
        },
        0xd1 => panic!("Operation unimplemented"),
        0xd2 => { // JNC
            let jmp_address: Option<u16> = jmp(
                (cpu.memory.read_at(cpu.pc.address), cpu.memory.read_at(cpu.pc.address + 1)),
                Some(cpu.flags.check_flag(Flag::CY) == 0)
                );
            match jmp_address {
                Some(address) => cpu.pc.address = address,
                None => return 2,
            };
        },
        0xd3 => panic!("Operation unimplemented"),
        0xd4 => { // CNC
            let call_address: Option<u16> = call(
                (cpu.memory.read_at(cpu.pc.address), cpu.memory.read_at(cpu.pc.address + 1)),
                Some(cpu.flags.check_flag(Flag::CY) == 0),
                &mut cpu.sp, &mut cpu.memory,
                cpu.pc.address + 2
                );
            match call_address {
                Some(address) => cpu.pc.address = address,
                None => return 2,
            };
        },
        0xd5 => panic!("Operation unimplemented"),
        0xd6 => { // SUI
            cpu.a.value = sub(cpu.a.value, cpu.memory.read_at(cpu.pc.address), &mut cpu.flags);
            return 1;
        },
        0xd7 => { // RST 2
            let call_address: Option<u16> = call(
                (0x10, 0x00),
                None,
                &mut cpu.sp, &mut cpu.memory,
                cpu.pc.address
                );
            cpu.pc.address = call_address.expect("call with no condition always returns an address");
        },
        0xd8 => { // RC
            let ret_address: Option<u16> = ret(
                Some(cpu.flags.check_flag(Flag::CY) == 1),
                &mut cpu.sp, &mut cpu.memory
                );
            match ret_address {
                Some(address) => cpu.pc.address = address,
                None => { return 0 },
            };
        },
        0xd9 => panic!("Operation unimplemented"),
        0xda => { // JC
            let jmp_address: Option<u16> = jmp(
                (cpu.memory.read_at(cpu.pc.address), cpu.memory.read_at(cpu.pc.address + 1)),
                Some(cpu.flags.check_flag(Flag::CY) == 1)
                );
            match jmp_address {
                Some(address) => cpu.pc.address = address,
                None => return 2,
            };
        },
        0xdb => panic!("Operation unimplemented"),
        0xdc => { // CC
            let call_address: Option<u16> = call(
                (cpu.memory.read_at(cpu.pc.address), cpu.memory.read_at(cpu.pc.address + 1)),
                Some(cpu.flags.check_flag(Flag::CY) == 1),
                &mut cpu.sp, &mut cpu.memory,
                cpu.pc.address + 2
                );
            match call_address {
                Some(address) => cpu.pc.address = address,
                None => return 2,
            };
        },
        0xdd => panic!("Operation unimplemented"),
        0xde => { // SBI
            cpu.a.value = sbb(cpu.a.value, cpu.memory.read_at(cpu.pc.address), &mut cpu.flags);
            return 1;
        },
        0xdf => { // RST 3
            let call_address: Option<u16> = call(
                (0x18, 0x00),
                None,
                &mut cpu.sp, &mut cpu.memory,
                cpu.pc.address
                );
            cpu.pc.address = call_address.expect("call with no condition always returns an address");
        },
        0xe0 => { // RPO
            let ret_address: Option<u16> = ret(
                Some(cpu.flags.check_flag(Flag::P) == 0),
                &mut cpu.sp, &mut cpu.memory
                );
            match ret_address {
                Some(address) => cpu.pc.address = address,
                None => { return 0 },
            };
        },
        0xe1 => panic!("Operation unimplemented"),
        0xe2 => { // JPO
            let jmp_address: Option<u16> = jmp(
                (cpu.memory.read_at(cpu.pc.address), cpu.memory.read_at(cpu.pc.address + 1)),
                Some(cpu.flags.check_flag(Flag::P) == 0)
                );
            match jmp_address {
                Some(address) => cpu.pc.address = address,
                None => return 2,
            };
        },
        0xe3 => panic!("Operation unimplemented"),
        0xe4 => { // CPO
            let call_address: Option<u16> = call(
                (cpu.memory.read_at(cpu.pc.address), cpu.memory.read_at(cpu.pc.address + 1)),
                Some(cpu.flags.check_flag(Flag::P) == 0),
                &mut cpu.sp, &mut cpu.memory,
                cpu.pc.address + 2
                );
            match call_address {
                Some(address) => cpu.pc.address = address,
                None => return 2,
            };
        },
        0xe5 => panic!("Operation unimplemented"),
        0xe6 => panic!("Operation unimplemented"),
        0xe7 => { // RST 4
            let call_address: Option<u16> = call(
                (0x20, 0x00),
                None,
                &mut cpu.sp, &mut cpu.memory,
                cpu.pc.address
                );
            cpu.pc.address = call_address.expect("call with no condition always returns an address");
        },
        0xe8 => { // RPE
            let ret_address: Option<u16> = ret(
                Some(cpu.flags.check_flag(Flag::P) == 1),
                &mut cpu.sp, &mut cpu.memory
                );
            match ret_address {
                Some(address) => cpu.pc.address = address,
                None => { return 0 },
            };
        },
        0xe9 => { // PCHL
            let hi: u8 = cpu.h.value;
            let lo: u8 = cpu.l.value;
            cpu.pc.address = pair_registers(hi, lo);
        },
        0xea => { // JPE
            let jmp_address: Option<u16> = jmp(
                (cpu.memory.read_at(cpu.pc.address), cpu.memory.read_at(cpu.pc.address + 1)),
                Some(cpu.flags.check_flag(Flag::P) == 1)
                );
            match jmp_address {
                Some(address) => cpu.pc.address = address,
                None => return 2,
            };
        },
        0xeb => panic!("Operation unimplemented"),
        0xec => { // CPE
            let call_address: Option<u16> = call(
                (cpu.memory.read_at(cpu.pc.address), cpu.memory.read_at(cpu.pc.address + 1)),
                Some(cpu.flags.check_flag(Flag::P) == 1),
                &mut cpu.sp, &mut cpu.memory,
                cpu.pc.address + 2
                );
            match call_address {
                Some(address) => cpu.pc.address = address,
                None => return 2,
            };
        },
        0xed => panic!("Operation unimplemented"),
        0xee => panic!("Operation unimplemented"),
        0xef => { // RST 5
            let call_address: Option<u16> = call(
                (0x28, 0x00),
                None,
                &mut cpu.sp, &mut cpu.memory,
                cpu.pc.address
                );
            cpu.pc.address = call_address.expect("call with no condition always returns an address");
        },
        0xf0 => { // RP
            let ret_address: Option<u16> = ret(
                Some(cpu.flags.check_flag(Flag::S) == 0),
                &mut cpu.sp, &mut cpu.memory
                );
            match ret_address {
                Some(address) => cpu.pc.address = address,
                None => { return 0 },
            };
        },
        0xf1 => panic!("Operation unimplemented"),
        0xf2 => { // JP
            let jmp_address: Option<u16> = jmp(
                (cpu.memory.read_at(cpu.pc.address), cpu.memory.read_at(cpu.pc.address + 1)),
                Some(cpu.flags.check_flag(Flag::S) == 0)
                );
            match jmp_address {
                Some(address) => cpu.pc.address = address,
                None => return 2,
            };
        },
        0xf3 => panic!("Operation unimplemented"),
        0xf4 => { // CP
            let call_address: Option<u16> = call(
                (cpu.memory.read_at(cpu.pc.address), cpu.memory.read_at(cpu.pc.address + 1)),
                Some(cpu.flags.check_flag(Flag::S) == 0),
                &mut cpu.sp, &mut cpu.memory,
                cpu.pc.address + 2
                );
            match call_address {
                Some(address) => cpu.pc.address = address,
                None => return 2,
            };
        },
        0xf5 => panic!("Operation unimplemented"),
        0xf6 => panic!("Operation unimplemented"),
        0xf7 => { // RST 6
            let call_address: Option<u16> = call(
                (0x30, 0x00),
                None,
                &mut cpu.sp, &mut cpu.memory,
                cpu.pc.address
                );
            cpu.pc.address = call_address.expect("call with no condition always returns an address");
        },
        0xf8 => { // RM
            let ret_address: Option<u16> = ret(
                Some(cpu.flags.check_flag(Flag::S) == 1),
                &mut cpu.sp, &mut cpu.memory
                );
            match ret_address {
                Some(address) => cpu.pc.address = address,
                None => { return 0 },
            };
        },
        0xf9 => panic!("Operation unimplemented"),
        0xfa => { // JM
            let jmp_address: Option<u16> = jmp(
                (cpu.memory.read_at(cpu.pc.address), cpu.memory.read_at(cpu.pc.address + 1)),
                Some(cpu.flags.check_flag(Flag::S) == 1)
                );
            match jmp_address {
                Some(address) => cpu.pc.address = address,
                None => return 2,
            };
        },
        0xfb => panic!("Operation unimplemented"),
        0xfc => { // CM
            let call_address: Option<u16> = call(
                (cpu.memory.read_at(cpu.pc.address), cpu.memory.read_at(cpu.pc.address + 1)),
                Some(cpu.flags.check_flag(Flag::S) == 1),
                &mut cpu.sp, &mut cpu.memory,
                cpu.pc.address + 2
                );
            match call_address {
                Some(address) => cpu.pc.address = address,
                None => return 2,
            };
        },
        0xfd => panic!("Operation unimplemented"),
        0xfe => panic!("Operation unimplemented"),
        0xff => { // RST 7
            let call_address: Option<u16> = call(
                (0x38, 0x00),
                None,
                &mut cpu.sp, &mut cpu.memory,
                cpu.pc.address
                );
            cpu.pc.address = call_address.expect("call with no condition always returns an address");
        },
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

        for i in 0..0xffff {
            assert_eq!(test_mem.read_at(i), 0x00);

            test_mem.write_at(i, 0xff);
            assert_eq!(test_mem.read_at(i), 0xff);
        }
    }

    #[test]
    fn test_flags_set_clear() {
        let mut flags: Flags = Flags::default();

        flags.set_flag(Flag::Z);
        assert_eq!(flags.flags, 0b01000000);
        assert_eq!(flags.check_flag(Flag::Z), 1);
        flags.clear_flag(Flag::Z);
        assert!(flags.flags == 0x00);

        flags.set_flag(Flag::S);
        assert_eq!(flags.flags, 0b10000000);
        flags.clear_flag(Flag::S);
        assert!(flags.flags == 0x00);

        flags.set_flag(Flag::P);
        assert_eq!(flags.flags, 0b00000100);
        flags.clear_flags();

        flags.set_flag(Flag::CY);
        assert_eq!(flags.flags, 0b00000001);
        flags.clear_flags();

        flags.set_flag(Flag::AC);
        assert_eq!(flags.flags, 0b00010000);
        flags.clear_flags();
    }

    #[test]
    fn test_push_pop() {
        let mut sp: AddressPointer = AddressPointer::at(0x2400);
        let mut memory: Memory = Memory::init();

        // Push
        push((0xd4, 0xc3), &mut sp, &mut memory);
        assert_eq!(sp.address, 0x23fe);
        assert_eq!(memory.read_at(0x2400), 0xd4);
        assert_eq!(memory.read_at(0x23ff), 0xc3);

        // Pop
        assert_eq!(pop(&mut sp, &mut memory), (0xd4, 0xc3));
        assert_eq!(sp.address, 0x2400);
    }

    #[test]
    fn test_register_pairing() {
        let h: u8 = 0x18;
        let l: u8 = 0xd4;
        let hl: u16 = pair_registers(h, l);

        assert_eq!(hl, 0x18d4);
        assert_eq!(split_register_pair(hl), (h, l));
    }

    #[test]
    fn test_operation_flag_setting() {
        let mut flags: Flags = Flags::default();

        // No flags
        flags = set_flags_from_operation(2, flags);
        assert_eq!(flags.flags, 0x00);

        // Z flag setting
        flags = set_flags_from_operation(0, flags);
        assert_eq!(flags.flags, 0b01000100);
        // Zero has even 1 parity

        // S flag setting
        flags = set_flags_from_operation(-2, flags);
        assert_eq!(flags.flags, 0b10000000);

        // Parity flag setting
        flags = set_flags_from_operation(3, flags);
        assert_eq!(flags.flags, 0b00000100);
        flags = set_flags_from_operation(2, flags);
        assert_eq!(flags.flags, 0b00000000);

        // Carry test
        flags = set_flags_from_operation(258, flags);
        assert_eq!(flags.flags, 0b00000001);
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

        // INX
        assert_eq!(inx( pair_registers(2, 3) ), (2, 4));
        assert_eq!(inx( pair_registers(0xff, 0xff) ), (0x00, 0x00));

        // DCX
        assert_eq!((dcx( pair_registers(0xff, 0xff)) ), (0xff, 0xfe));
        assert_eq!((dcx( pair_registers(0x00, 0x00)) ), (0xff, 0xff));

        // INR
        assert_eq!(inr(0x02, &mut flags), 0x03);
        assert_eq!(flags.check_flag(Flag::P), 1);
        assert_eq!(inr(0xff, &mut flags), 0x00);

        // DCR
        flags.clear_flags();
        assert_eq!(dcr(0x01, &mut flags), 0x00);
        assert_eq!(flags.check_flag(Flag::Z), 1);
        assert_eq!(dcr(0x00, &mut flags), 0xff);
        assert_eq!(flags.check_flag(Flag::S), 1);
        assert_eq!(flags.check_flag(Flag::P), 1);

        // DAD
        flags.clear_flags();
        assert_eq!(dad(0xffff, 0x0001, &mut flags), (0x00, 0x00));
        assert_eq!(flags.check_flag(Flag::CY), 1);
        assert_eq!(dad(0x0002, 0x0001, &mut flags), (0x00, 0x03));
        assert_eq!(flags.check_flag(Flag::CY), 0);
        assert_eq!(flags.check_flag(Flag::P), 0);
        // This should never affect any flag other than the carry flag
    }

    #[test]
    fn test_branching_operations() {
        let mut cpu: Cpu = Cpu::init();

        // JMP
        assert_eq!(jmp((0xd4, 0xc3), None), Some(0xc3d4));

        // JNZ
        assert_eq!(jmp((0xd4, 0xc3), Some(cpu.flags.check_flag(Flag::Z) == 0)), Some(0xc3d4));
        cpu.flags.set_flag(Flag::Z);
        assert_eq!(jmp((0xd4, 0xc3), Some(cpu.flags.check_flag(Flag::Z) == 0)), None);

        // The rest should be identical so shouldn't require seperate testing

        // CALL & RET
        cpu.pc.address = 0x0002;

        assert_eq!(call((0xd4, 0xc3), None, &mut cpu.sp, &mut cpu.memory, cpu.pc.address), Some(0xc3d4));
        assert_eq!(ret(None, &mut cpu.sp, &mut cpu.memory), Some(0x0002));

        // CNZ & RNZ
        cpu.memory = Memory::init();
        cpu.pc.address = 0x0002;
        cpu.sp.address = 0x2400;
        cpu.flags.clear_flags();

        assert_eq!(call((0xd4, 0xc3), Some(cpu.flags.check_flag(Flag::Z) == 0), &mut cpu.sp, &mut cpu.memory, cpu.pc.address), Some(0xc3d4));
        assert_eq!(ret(Some(cpu.flags.check_flag(Flag::Z) == 0), &mut cpu.sp, &mut cpu.memory), Some(0x0002));

        cpu.flags.set_flag(Flag::Z);
        assert_eq!(call((0xd4, 0xc3), Some(cpu.flags.check_flag(Flag::Z) == 0), &mut cpu.sp, &mut cpu.memory, cpu.pc.address), None);
        assert_eq!(cpu.memory.read_at(0x2400), 0x00);
        assert_eq!(cpu.memory.read_at(0x23ff), 0x00);
        // Checking it didnt write a return address to the stack if it isn't jumping

        assert_eq!(ret(Some(cpu.flags.check_flag(Flag::Z) == 0), &mut cpu.sp, &mut cpu.memory), None);
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
        assert_eq!(cpu.memory.read_at(pair_registers(cpu.h.value, cpu.l.value)), 0xff);

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

        // INX test SP + 1
        cpu.sp.address = 0xc3d4;
        handle_op_code(0x33, &mut cpu);
        assert_eq!(cpu.sp.address, 0xc3d5);

        // DCX test SP - 1
        cpu.sp.address = 0xc3d5;
        handle_op_code(0x3b, &mut cpu);
        assert_eq!(cpu.sp.address, 0xc3d4);

        // INR test M + 1
        cpu.h.value = 0xc3;
        cpu.l.value = 0xd4;
        cpu.memory.write_at( pair_registers(cpu.h.value, cpu.l.value), 0x00);

        handle_op_code(0x34, &mut cpu);
        assert_eq!(cpu.memory.read_at( pair_registers(cpu.h.value, cpu.l.value) ), 0x01);

        // DCR M - 1
        cpu.h.value = 0xc3;
        cpu.l.value = 0xd4;
        cpu.memory.write_at( pair_registers(cpu.h.value, cpu.l.value), 0xff);

        handle_op_code(0x35, &mut cpu);
        assert_eq!(cpu.memory.read_at( pair_registers(cpu.h.value, cpu.l.value) ), 0xfe);

        // DAD HL + SP -> HL
        cpu.h.value = 0x01;
        cpu.l.value = 0x01;
        cpu.sp.address = 0x0101;

        handle_op_code(0x39, &mut cpu);
        assert_eq!((cpu.h.value, cpu.l.value), (0x02, 0x02));

        // JMP
        cpu.pc.address = 0x0005;
        // pc pointes to byte after op code when handling op codes
        cpu.memory.write_at(0x0005, 0xd4);
        cpu.memory.write_at(0x0006, 0xc3);

        assert_eq!(handle_op_code(0xc3, &mut cpu), 2);
        assert_eq!(cpu.pc.address, 0xc3d4);

        // JNZ
        cpu.pc.address = 0x0005;
        cpu.memory.write_at(0x0005, 0xd4);
        cpu.memory.write_at(0x0006, 0xc3);
        cpu.flags.clear_flags();

        handle_op_code(0xc2, &mut cpu);
        assert_eq!(cpu.pc.address, 0xc3d4);
        // Should jmp to c3d4 since Z flag is not set

        cpu.pc.address = 0x0005;
        cpu.memory.write_at(0x0005, 0xd4);
        cpu.memory.write_at(0x0006, 0xc3);
        cpu.flags.set_flag(Flag::Z);

        assert_eq!(handle_op_code(0xc2, &mut cpu), 2);
        // Should return 2 additional bytes if it doesn't jmp
        assert_eq!(cpu.pc.address, 0x0005);
        // Should not jmp to c3d4 since Z flag is set

        // CALL & RET
        cpu.reset();
        cpu.pc.address = 0x0005;
        cpu.memory.write_at(0x0005, 0xd4);
        cpu.memory.write_at(0x0006, 0xc3);

        assert_eq!(handle_op_code(0xcd, &mut cpu), 2);
        assert_eq!(cpu.pc.address, 0xc3d4);
        assert_eq!(cpu.sp.address, 0x23fe);
        // The stack pointer should be decremented 2

        assert_eq!(cpu.memory.read_at(0x2400), 0x07);
        assert_eq!(cpu.memory.read_at(0x23ff), 0x00);
        // The return address of the next instruction should be on the stack

        handle_op_code(0xc9, &mut cpu);
        assert_eq!(cpu.pc.address, 0x0007);
        assert_eq!(cpu.sp.address, 0x2400);
        // The stack pointer should be reincremented

        assert_eq!(cpu.memory.read_at(0x2400), 0x00);
        assert_eq!(cpu.memory.read_at(0x23ff), 0x00);
        // The return address should be removed from the stack

        // CNZ & RNZ
        cpu.reset();
        cpu.pc.address = 0x0005;
        cpu.memory.write_at(0x0005, 0xd4);
        cpu.memory.write_at(0x0006, 0xc3);

        cpu.flags.set_flag(Flag::Z);
        // Expect not to call
        assert_eq!(handle_op_code(0xc4, &mut cpu), 2);
        // Returns 2 additional bytes read if no call

        assert_eq!(cpu.pc.address, 0x0005);
        assert_eq!(cpu.sp.address, 0x2400);
        assert_eq!(cpu.memory.read_at(0x2400), 0x00);
        assert_eq!(cpu.memory.read_at(0x23ff), 0x00);
        // Nothing should change if no call

        cpu.flags.clear_flags();
        // Expect call
        assert_eq!(handle_op_code(0xc4, &mut cpu), 0);

        assert_eq!(cpu.pc.address, 0xc3d4);
        assert_eq!(cpu.sp.address, 0x23fe);
        assert_eq!(cpu.memory.read_at(0x2400), 0x07);
        assert_eq!(cpu.memory.read_at(0x23ff), 0x00);

        cpu.flags.set_flag(Flag::Z);
        // Expect to not return
        handle_op_code(0xc0, &mut cpu);

        assert_eq!(cpu.pc.address, 0xc3d4);
        assert_eq!(cpu.sp.address, 0x23fe);
        assert_eq!(cpu.memory.read_at(0x2400), 0x07);
        assert_eq!(cpu.memory.read_at(0x23ff), 0x00);
        // Nothing should change if not returning

        cpu.flags.clear_flags();
        // Expect to return
        handle_op_code(0xc0, &mut cpu);

        assert_eq!(cpu.pc.address, 0x0007);
        assert_eq!(cpu.sp.address, 0x2400);
        assert_eq!(cpu.memory.read_at(0x2400), 0x00);
        assert_eq!(cpu.memory.read_at(0x23ff), 0x00);

        // PCHL
        cpu.reset();
        cpu.pc.address = 0x0005;
        cpu.h.value = 0xc3;
        cpu.l.value = 0xd4;
        handle_op_code(0xe9, &mut cpu);

        assert_eq!(cpu.pc.address, 0xc3d4);
        // PCHL is a jmp not a call

        // RST 7
        cpu.reset();
        cpu.pc.address = 0x0005;

        cpu.pc.address += 1;
        handle_op_code(0xff, &mut cpu);

        assert_eq!(cpu.pc.address, 0x0038);
        assert_eq!(cpu.sp.address, 0x23fe);
        assert_eq!(cpu.memory.read_at(0x2400), 0x06);
        assert_eq!(cpu.memory.read_at(0x23ff), 0x00);
    }
}
