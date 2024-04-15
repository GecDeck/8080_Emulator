use self::dispatcher::handle_op_code;

mod tests;
pub mod dispatcher;

const STACK_MIN: u16 = 0x2001;
// This should be where the minimum stack address is

// CPU HARDWARE

#[derive(Clone, Copy)]
pub struct Register {
    pub value: u8,
    // Value is public so it can be accessed from main
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

    pub fn read_vram(&self) -> &[u8] {
        &self.held_memory[0x2400..=0x3fff]
    }

    pub fn read_at(&self, addr: u16) -> u8 {
        self.held_memory[addr as usize]
    }

    pub fn write_at(&mut self, addr: u16, byte: u8) {
        self.held_memory[addr as usize] = byte;
    }

    pub fn load_rom(&mut self, rom: &[u8], offset: u16) {
        // Loads a rom into memory

        for (address, byte) in rom.iter().enumerate() {
            assert!(address < 0x2000);
            // Rom should fit in the space of memory reserved for roms

            self.write_at(address as u16 + offset, *byte);
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
    pub a: Register,
    // A is public so it can be accessed from main
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
    interrupt_enabled: bool,
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
            interrupt_enabled: true,
        }
    }

    pub fn reset(&mut self) {
        // Resets all the values of the cpu
        *self = Cpu::init();
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

    // Being used for debug info
    pub fn debug_stack_pointer(&self) -> u16 {
        self.sp.address
    }
    pub fn debug_program_counter(&self) -> u16 {
        self.pc.address
    }

    // Being used for CPU DIAG tests
    pub fn debug_c(&self) -> u8 {
        self.c.value
    }
    pub fn debug_d(&self) -> u8 {
        self.d.value
    }
    pub fn debug_e(&self) -> u8 {
        self.e.value
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

    memory.write_at(stack_pointer.address - 1, data_bytes.0);
    memory.write_at(stack_pointer.address - 2, data_bytes.1);
    // d4 c3 will go in as d4 c3

    stack_pointer.address -= 2;
    // stack grows downwards
}

fn pop(stack_pointer: &mut AddressPointer, memory: &mut Memory) -> (u8, u8) {
    // Returns the data at the top of the stack

    let byte_1 = memory.read_at(stack_pointer.address + 1);
    let byte_2 = memory.read_at(stack_pointer.address);
    // Find two bytes before stack pointer

    memory.write_at(stack_pointer.address + 1, 0x00);
    memory.write_at(stack_pointer.address, 0x00);
    // Zeroes memory, probably not necessary but nice for cleanliness and debugging

    stack_pointer.address += 2;
    // stack shrinks upwards

    (byte_1, byte_2)
}

fn and(reg_1: u8, reg_2: u8, flags: &mut Flags) -> u8 {
    // &s two registers together, sets flags based on the result, then returns the result

    let result: u8 = reg_1 & reg_2;
    *flags = set_flags_from_operation(result as i16, *flags);
    if result == 0b10000000 { flags.set_flag(Flag::S) }
    // This is just how the cpu works I think

    result
}

fn xor(reg_1: u8, reg_2: u8, flags: &mut Flags) -> u8 {
    // ^s two registers together, sets flags based on the result, then returns the result

    let result: u8 = reg_1 ^ reg_2;
    *flags = set_flags_from_operation(result as i16, *flags);
    if result == 0b10000000 { flags.set_flag(Flag::S) }

    result
}

fn or(reg_1: u8, reg_2: u8, flags: &mut Flags) -> u8 {
    // |s two registers together, sets flags based on the result, then returns the result

    let result: u8 = reg_1 | reg_2;
    *flags = set_flags_from_operation(result as i16, *flags);
    if result == 0b10000000 { flags.set_flag(Flag::S) }

    result
}

fn cmp(reg_1: u8, reg_2: u8, flags: &mut Flags) {
    // Gets the difference of two registers, and sets flags based on the result
    //  The result is discarded

    let result: i16 = reg_1 as i16 - reg_2 as i16;

    *flags = set_flags_from_operation(result, *flags);

    match result {
        i16::MIN..=-1 => flags.set_flag(Flag::CY),
        0 => flags.set_flag(Flag::Z),
        1..=i16::MAX => flags.clear_flag(Flag::CY),
    }
    // Carry works differently here so it will be calculated after
}

fn rotate_right(reg: u8, through_carry: bool, flags: &mut Flags) -> u8 {
    // Sets the carry bit from bit 0, rotates right, optionally rotates through the carry bit
    //  returns result

    let mut result: u8 = reg >> 1;
    if !through_carry { result |= reg << 7 }
    else { result |= flags.check_flag(Flag::CY) << 7 }

    match reg & 0b00000001 {
        0b00000001 => flags.set_flag(Flag::CY),
        0b00000000 => flags.clear_flag(Flag::CY),
        _ => panic!("No other possible results for & 0b00000001"),
    }
    // The carry flag is set to the low bit of the register

    result
}

fn rotate_left(reg: u8, through_carry: bool, flags: &mut Flags) -> u8 {
    // Sets the carry bit from bit 7, rotates left, optionally rotates through the carry bit
    //  returns result

    let mut result: u8 = reg << 1;
    if !through_carry { result |= reg >> 7 }
    else { result |= flags.check_flag(Flag::CY) }

    match reg & 0b10000000 {
        0b10000000 => flags.set_flag(Flag::CY),
        0b00000000 => flags.clear_flag(Flag::CY),
        _ => panic!("No other possible results for & 0b10000000"),
    }
    // The carry bit is set to the high bit of the register

    result
}

fn set_flags_from_operation(result: i16, flags: Flags) -> Flags {
    // Sets flags based on the result of an arithmetic operation
    let mut return_flags: Flags = flags;
    return_flags.clear_flags();

    // Zero check
    if result == 0 { return_flags.set_flag(Flag::Z) }

    // Negative Check
    if result < 0 { return_flags.set_flag(Flag::S) }

    // Parity Check
    if ((result & 0xff) as u8).count_ones() % 2 == 0 { return_flags.set_flag(Flag::P) }

    // Carry Check
    if result > u8::MAX as i16 { return_flags.set_flag(Flag::CY) }

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

fn swap_registers(reg_1: u8, reg_2: u8) -> (u8, u8) {
    (reg_2, reg_1)
}

pub fn generate_interrupt(op_code: u8, cpu: &mut Cpu) {
    if cpu.interrupt_enabled {
        let _ = handle_op_code(op_code, cpu);
    }
}
