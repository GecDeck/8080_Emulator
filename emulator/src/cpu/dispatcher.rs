use super::*;

pub const CLOCK_CYCLES: [u8; 0x100] = [
    4, 10, 7, 5, 5, 5, 7, 4, 4, 10, 7, 5, 5, 5, 7, 4, 4, 10, 7, 5, 5, 5, 7, 4, 4, 10, 7, 5, 5,
    5, 7, 4, 4, 10, 16, 5, 5, 5, 7, 4, 4, 10, 16, 5, 5, 5, 7, 4, 4, 10, 13, 5, 10, 10, 10, 4,
    4, 10, 13, 5, 5, 5, 7, 4, 5, 5, 5, 5, 5, 5, 7, 5, 5, 5, 5, 5, 5, 5, 7, 5, 5, 5, 5, 5, 5, 5,
    7, 5, 5, 5, 5, 5, 5, 5, 7, 5, 5, 5, 5, 5, 5, 5, 7, 5, 5, 5, 5, 5, 5, 5, 7, 5, 7, 7, 7, 7,
    7, 7, 7, 7, 5, 5, 5, 5, 5, 5, 7, 5, 4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4, 4, 4,
    4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4,
    4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4, 11, 10, 10, 10, 17, 11, 7, 11, 11, 10, 10,
    10, 10, 17, 7, 11, 11, 10, 10, 10, 17, 11, 7, 11, 11, 10, 10, 10, 10, 17, 7, 11, 11, 10,
    10, 18, 17, 11, 7, 11, 11, 5, 10, 5, 17, 17, 7, 11, 11, 10, 10, 4, 17, 11, 7, 11, 11, 5,
    10, 4, 17, 17, 7, 11,
];

pub fn handle_op_code(op_code: u8, cpu: &mut Cpu) -> Result<u16, &str> {
    // Reads an op_code and performs the cooresponding operation
    // Returns the number of additional bytes read for the operation

    match op_code {
        0x00 => {},
        // NOP
        0x01 => { // LXI B
            (cpu.b.value, cpu.c.value) = (cpu.memory.read_at(cpu.pc.address + 1), cpu.memory.read_at(cpu.pc.address));
            return Ok(2);
        },
        0x02 => cpu.memory.write_at(pair_registers(cpu.b.value, cpu.c.value), cpu.a.value),
        0x03 => (cpu.b.value, cpu.c.value) = inx( pair_registers(cpu.b.value, cpu.c.value) ),
        0x04 => cpu.b.value = inr(cpu.b.value, &mut cpu.flags),
        0x05 => cpu.b.value = dcr(cpu.b.value, &mut cpu.flags),
        0x06 => { // MVI B
            cpu.b.value = cpu.memory.read_at(cpu.pc.address);
            return Ok(1);
        },
        0x07 => cpu.a.value = rotate_left(cpu.a.value, false, &mut cpu.flags),
        0x08 => {},
        0x09 => (cpu.h.value, cpu.l.value) = dad(
            pair_registers(cpu.h.value, cpu.l.value),
            pair_registers(cpu.b.value, cpu.c.value),
            &mut cpu.flags
            ),
        0x0a => cpu.a.value = cpu.memory.read_at(pair_registers(cpu.b.value, cpu.c.value)),
        0x0b => (cpu.b.value, cpu.c.value) = dcx( pair_registers(cpu.b.value, cpu.c.value) ),
        0x0c => cpu.c.value = inr(cpu.c.value, &mut cpu.flags),
        0x0d => cpu.c.value = dcr(cpu.c.value, &mut cpu.flags),
        0x0e => { // MVI C
            cpu.c.value = cpu.memory.read_at(cpu.pc.address);
            return Ok(1);
        },
        0x0f => cpu.a.value = rotate_right(cpu.a.value, false, &mut cpu.flags),
        0x10 => {},
        0x11 => { // LXI D
            (cpu.d.value, cpu.e.value) = (cpu.memory.read_at(cpu.pc.address + 1), cpu.memory.read_at(cpu.pc.address));
            return Ok(2);
        },
        0x12 => cpu.memory.write_at(pair_registers(cpu.d.value, cpu.e.value), cpu.a.value),
        0x13 => (cpu.d.value, cpu.e.value) = inx( pair_registers(cpu.d.value, cpu.e.value) ),
        0x14 => cpu.d.value = inr(cpu.d.value, &mut cpu.flags),
        0x15 => cpu.d.value = dcr(cpu.d.value, &mut cpu.flags),
        0x16 => { // MVI D
            cpu.d.value = cpu.memory.read_at(cpu.pc.address);
            return Ok(1);
        },
        0x17 => cpu.a.value = rotate_left(cpu.a.value, true, &mut cpu.flags),
        0x18 => {},
        0x19 => (cpu.h.value, cpu.l.value) = dad(
            pair_registers(cpu.h.value, cpu.l.value),
            pair_registers(cpu.d.value, cpu.e.value),
            &mut cpu.flags
            ),
        0x1a => cpu.a.value = cpu.memory.read_at(pair_registers(cpu.d.value, cpu.e.value)),
        0x1b => (cpu.d.value, cpu.e.value) = dcx( pair_registers(cpu.d.value, cpu.e.value) ),
        0x1c => cpu.e.value = inr(cpu.e.value, &mut cpu.flags),
        0x1d => cpu.e.value = dcr(cpu.e.value, &mut cpu.flags),
        0x1e => { // MVI E
            cpu.e.value = cpu.memory.read_at(cpu.pc.address);
            return Ok(1);
        },
        0x1f => cpu.a.value = rotate_right(cpu.a.value, true, &mut cpu.flags),
        0x20 => {},
        0x21 => { // LXI H
            (cpu.h.value, cpu.l.value) = (cpu.memory.read_at(cpu.pc.address + 1), cpu.memory.read_at(cpu.pc.address));
            return Ok(2);
        },
        0x22 => { // SHLD
            let addr: u16 = pair_registers(
                cpu.memory.read_at(cpu.pc.address + 1), cpu.memory.read_at(cpu.pc.address)
                );
            cpu.memory.write_at(addr, cpu.l.value);
            cpu.memory.write_at(addr + 1, cpu.h.value);
            return Ok(2);
        },
        0x23 => (cpu.h.value, cpu.l.value) = inx( pair_registers(cpu.h.value, cpu.l.value) ),
        0x24 => cpu.h.value = inr(cpu.h.value, &mut cpu.flags),
        0x25 => cpu.h.value = dcr(cpu.h.value, &mut cpu.flags),
        0x26 => { // MVI H
            cpu.h.value = cpu.memory.read_at(cpu.pc.address);
            return Ok(1);
        },
        0x27 => cpu.a.value = daa(cpu.a.value, &mut cpu.flags),
        0x28 => {},
        0x29 => (cpu.h.value, cpu.l.value) = dad(
            pair_registers(cpu.h.value, cpu.l.value),
            pair_registers(cpu.h.value, cpu.l.value),
            &mut cpu.flags
            ),
        // This is documented as HL = HL + HI
        //  But I think it's supposed to just add HL to itself? I don't what what I means
        //  TODO: find out what I means
        0x2a => { // LHLD
            let addr: u16 = pair_registers(
                cpu.memory.read_at(cpu.pc.address + 1), cpu.memory.read_at(cpu.pc.address)
                );
            cpu.l.value = cpu.memory.read_at(addr);
            cpu.h.value = cpu.memory.read_at(addr + 1);
            return Ok(2);
        },
        0x2b => (cpu.h.value, cpu.l.value) = dcx( pair_registers(cpu.h.value, cpu.l.value) ),
        0x2c => cpu.l.value = inr(cpu.l.value, &mut cpu.flags),
        0x2d => cpu.l.value = dcr(cpu.l.value, &mut cpu.flags),
        0x2e => { // MVI L
            cpu.l.value = cpu.memory.read_at(cpu.pc.address);
            return Ok(1);
        },
        0x2f => cpu.a.value = !cpu.a.value,
        0x30 => {},
        0x31 => { // LXI SP
            cpu.sp.address = pair_registers(cpu.memory.read_at(cpu.pc.address + 1), cpu.memory.read_at(cpu.pc.address));
            return Ok(2);
        },
        0x32 => { // STA
            cpu.memory.write_at(
                pair_registers(
                    cpu.memory.read_at(cpu.pc.address + 1),
                    cpu.memory.read_at(cpu.pc.address)),
                cpu.a.value
                );
            return Ok(2);
        },
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
        0x36 => { // MVI M
            cpu.memory.write_at(
                pair_registers(cpu.h.value, cpu.l.value),
                cpu.memory.read_at(cpu.pc.address)
                );
            return Ok(1);
        },
        0x37 => cpu.flags.set_flag(Flag::CY),
        0x38 => {},
        0x39 => (cpu.h.value, cpu.l.value) = dad(
            pair_registers(cpu.h.value, cpu.l.value),
            cpu.sp.address,
            &mut cpu.flags
            ),
        0x3a => { // LDA
            cpu.a.value = cpu.memory.read_at(
                pair_registers(cpu.memory.read_at(cpu.pc.address + 1), cpu.memory.read_at(cpu.pc.address))
                );
            return Ok(2);
        },
        0x3b => {
            let (sp_1, sp_2): (u8, u8) = split_register_pair(cpu.sp.address);
            let (byte_1, byte_2): (u8, u8) = dcx( pair_registers(sp_1, sp_2) );
            cpu.sp.address = pair_registers(byte_1, byte_2);
        },
        0x3c => cpu.a.value = inr(cpu.a.value, &mut cpu.flags),
        0x3d => cpu.a.value = dcr(cpu.a.value, &mut cpu.flags),
        0x3e => { // MVI A
            cpu.a.value = cpu.memory.read_at(cpu.pc.address);
            return Ok(1);
        },
        0x3f => cpu.flags.clear_flag(Flag::CY),

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
        0x76 => return Ok(255),
        // Halt will return a unique u8 so main knows to exit
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

        // ANA
        0xa0 => cpu.a.value = and(cpu.a.value, cpu.b.value, &mut cpu.flags),
        0xa1 => cpu.a.value = and(cpu.a.value, cpu.c.value, &mut cpu.flags),
        0xa2 => cpu.a.value = and(cpu.a.value, cpu.d.value, &mut cpu.flags),
        0xa3 => cpu.a.value = and(cpu.a.value, cpu.e.value, &mut cpu.flags),
        0xa4 => cpu.a.value = and(cpu.a.value, cpu.h.value, &mut cpu.flags),
        0xa5 => cpu.a.value = and(cpu.a.value, cpu.l.value, &mut cpu.flags),
        0xa6 => cpu.a.value = and(cpu.a.value, cpu.memory.read_at( pair_registers(cpu.h.value, cpu.l.value) ), &mut cpu.flags),
        0xa7 => cpu.a.value = and(cpu.a.value, cpu.a.value, &mut cpu.flags),

        // XRA
        0xa8 => cpu.a.value = xor(cpu.a.value, cpu.b.value, &mut cpu.flags),
        0xa9 => cpu.a.value = xor(cpu.a.value, cpu.c.value, &mut cpu.flags),
        0xaa => cpu.a.value = xor(cpu.a.value, cpu.d.value, &mut cpu.flags),
        0xab => cpu.a.value = xor(cpu.a.value, cpu.e.value, &mut cpu.flags),
        0xac => cpu.a.value = xor(cpu.a.value, cpu.h.value, &mut cpu.flags),
        0xad => cpu.a.value = xor(cpu.a.value, cpu.l.value, &mut cpu.flags),
        0xae => cpu.a.value = xor(cpu.a.value, cpu.memory.read_at( pair_registers(cpu.h.value, cpu.l.value) ), &mut cpu.flags),
        0xaf => cpu.a.value = xor(cpu.a.value, cpu.a.value, &mut cpu.flags),

        // ORA
        0xb0 => cpu.a.value = or(cpu.a.value, cpu.b.value, &mut cpu.flags),
        0xb1 => cpu.a.value = or(cpu.a.value, cpu.c.value, &mut cpu.flags),
        0xb2 => cpu.a.value = or(cpu.a.value, cpu.d.value, &mut cpu.flags),
        0xb3 => cpu.a.value = or(cpu.a.value, cpu.e.value, &mut cpu.flags),
        0xb4 => cpu.a.value = or(cpu.a.value, cpu.h.value, &mut cpu.flags),
        0xb5 => cpu.a.value = or(cpu.a.value, cpu.l.value, &mut cpu.flags),
        0xb6 => cpu.a.value = or(cpu.a.value, cpu.memory.read_at( pair_registers(cpu.h.value, cpu.l.value) ), &mut cpu.flags),
        0xb7 => cpu.a.value = or(cpu.a.value, cpu.a.value, &mut cpu.flags),

        // CMP
        0xb8 => cmp(cpu.a.value, cpu.b.value, &mut cpu.flags),
        0xb9 => cmp(cpu.a.value, cpu.c.value, &mut cpu.flags),
        0xba => cmp(cpu.a.value, cpu.d.value, &mut cpu.flags),
        0xbb => cmp(cpu.a.value, cpu.e.value, &mut cpu.flags),
        0xbc => cmp(cpu.a.value, cpu.h.value, &mut cpu.flags),
        0xbd => cmp(cpu.a.value, cpu.l.value, &mut cpu.flags),
        0xbe => cmp(cpu.a.value, cpu.memory.read_at( pair_registers(cpu.h.value, cpu.l.value) ), &mut cpu.flags),
        0xbf => cmp(cpu.a.value, cpu.a.value, &mut cpu.flags),

        0xc0 => { // RNZ
            let ret_address: Option<u16> = ret(
                Some(cpu.flags.check_flag(Flag::Z) == 0),
                &mut cpu.sp, &mut cpu.memory
                );
            match ret_address {
                Some(address) => cpu.pc.address = address,
                None => { return Ok(0) },
            };
        },
        0xc1 => (cpu.b.value, cpu.c.value) = pop(&mut cpu.sp, &mut cpu.memory),
        0xc2 => { // JNZ
            let jmp_address: Option<u16> = jmp(
                (cpu.memory.read_at(cpu.pc.address), cpu.memory.read_at(cpu.pc.address + 1)),
                Some(cpu.flags.check_flag(Flag::Z) == 0)
                );
            match jmp_address {
                Some(address) => cpu.pc.address = address,
                None => return Ok(2),
            };
        },
        0xc3 => { // JMP
            let jmp_address: Option<u16> = jmp(
                (cpu.memory.read_at(cpu.pc.address), cpu.memory.read_at(cpu.pc.address + 1)),
                None
                );
            cpu.pc.address = jmp_address.expect("jmp with no condition should always return Some(address)");
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
                None => return Ok(2),
            };
        },
        0xc5 => push((cpu.b.value, cpu.c.value), &mut cpu.sp, &mut cpu.memory),
        0xc6 => { // ADI
            cpu.a.value = add(cpu.a.value, cpu.memory.read_at(cpu.pc.address), &mut cpu.flags);
            return Ok(1);
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
                None => { return Ok(0) },
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
                None => return Ok(2),
            };
        },
        0xcb => {},
        0xcc => { // CZ
            let call_address: Option<u16> = call(
                (cpu.memory.read_at(cpu.pc.address), cpu.memory.read_at(cpu.pc.address + 1)),
                Some(cpu.flags.check_flag(Flag::Z) == 1),
                &mut cpu.sp, &mut cpu.memory,
                cpu.pc.address + 2
                );
            match call_address {
                Some(address) => cpu.pc.address = address,
                None => return Ok(2),
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
        },
        0xce => { // ACI
            cpu.a.value = adc(cpu.a.value, cpu.memory.read_at(cpu.pc.address), &mut cpu.flags);
            return Ok(1);
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
                None => { return Ok(0) },
            };
        },
        0xd1 => (cpu.d.value, cpu.e.value) = pop(&mut cpu.sp, &mut cpu.memory),
        0xd2 => { // JNC
            let jmp_address: Option<u16> = jmp(
                (cpu.memory.read_at(cpu.pc.address), cpu.memory.read_at(cpu.pc.address + 1)),
                Some(cpu.flags.check_flag(Flag::CY) == 0)
                );
            match jmp_address {
                Some(address) => cpu.pc.address = address,
                None => return Ok(2),
            };
        },
        0xd3 => { // OUT
            // This opcode and the opcode for IN will not be handled here
            panic!("OUT should have been handled by the hardware module");
        },
        0xd4 => { // CNC
            let call_address: Option<u16> = call(
                (cpu.memory.read_at(cpu.pc.address), cpu.memory.read_at(cpu.pc.address + 1)),
                Some(cpu.flags.check_flag(Flag::CY) == 0),
                &mut cpu.sp, &mut cpu.memory,
                cpu.pc.address + 2
                );
            match call_address {
                Some(address) => cpu.pc.address = address,
                None => return Ok(2),
            };
        },
        0xd5 => push((cpu.d.value, cpu.e.value), &mut cpu.sp, &mut cpu.memory),
        0xd6 => { // SUI
            cpu.a.value = sub(cpu.a.value, cpu.memory.read_at(cpu.pc.address), &mut cpu.flags);
            return Ok(1);
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
                None => { return Ok(0) },
            };
        },
        0xd9 => {},
        0xda => { // JC
            let jmp_address: Option<u16> = jmp(
                (cpu.memory.read_at(cpu.pc.address), cpu.memory.read_at(cpu.pc.address + 1)),
                Some(cpu.flags.check_flag(Flag::CY) == 1)
                );
            match jmp_address {
                Some(address) => cpu.pc.address = address,
                None => return Ok(2),
            };
        },
        0xdb => { // IN
            // This opcode and the opcode for OUT will not be handled here
            panic!("IN should have been handled by the hardware module");
        },
        0xdc => { // CC
            let call_address: Option<u16> = call(
                (cpu.memory.read_at(cpu.pc.address), cpu.memory.read_at(cpu.pc.address + 1)),
                Some(cpu.flags.check_flag(Flag::CY) == 1),
                &mut cpu.sp, &mut cpu.memory,
                cpu.pc.address + 2
                );
            match call_address {
                Some(address) => cpu.pc.address = address,
                None => return Ok(2),
            };
        },
        0xdd => {},
        0xde => { // SBI
            cpu.a.value = sbb(cpu.a.value, cpu.memory.read_at(cpu.pc.address), &mut cpu.flags);
            return Ok(1);
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
                None => { return Ok(0) },
            };
        },
        0xe1 => (cpu.h.value, cpu.l.value) = pop(&mut cpu.sp, &mut cpu.memory),
        0xe2 => { // JPO
            let jmp_address: Option<u16> = jmp(
                (cpu.memory.read_at(cpu.pc.address), cpu.memory.read_at(cpu.pc.address + 1)),
                Some(cpu.flags.check_flag(Flag::P) == 0)
                );
            match jmp_address {
                Some(address) => cpu.pc.address = address,
                None => return Ok(2),
            };
        },
        0xe3 => { //XTHL
            let (h, l): (u8, u8) = pop(&mut cpu.sp, &mut cpu.memory);
            push((cpu.h.value, cpu.l.value), &mut cpu.sp, &mut cpu.memory);
            (cpu.h.value, cpu.l.value) = (h, l);
        },
        0xe4 => { // CPO
            let call_address: Option<u16> = call(
                (cpu.memory.read_at(cpu.pc.address), cpu.memory.read_at(cpu.pc.address + 1)),
                Some(cpu.flags.check_flag(Flag::P) == 0),
                &mut cpu.sp, &mut cpu.memory,
                cpu.pc.address + 2
                );
            match call_address {
                Some(address) => cpu.pc.address = address,
                None => return Ok(2),
            };
        },
        0xe5 => push((cpu.h.value, cpu.l.value), &mut cpu.sp, &mut cpu.memory),
        0xe6 => { // ANI
            cpu.a.value = and(cpu.a.value, cpu.memory.read_at(cpu.pc.address), &mut cpu.flags);
            return Ok(1);
        },
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
                None => { return Ok(0) },
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
                None => return Ok(2),
            };
        },
        0xeb => { // XCHG
            (cpu.h.value, cpu.d.value) = swap_registers(cpu.h.value, cpu.d.value);
            (cpu.l.value, cpu.e.value) = swap_registers(cpu.l.value, cpu.e.value);
        },
        0xec => { // CPE
            let call_address: Option<u16> = call(
                (cpu.memory.read_at(cpu.pc.address), cpu.memory.read_at(cpu.pc.address + 1)),
                Some(cpu.flags.check_flag(Flag::P) == 1),
                &mut cpu.sp, &mut cpu.memory,
                cpu.pc.address + 2
                );
            match call_address {
                Some(address) => cpu.pc.address = address,
                None => return Ok(2),
            };
        },
        0xed => {},
        0xee => { // XRI
            cpu.a.value = xor(cpu.a.value, cpu.memory.read_at(cpu.pc.address), &mut cpu.flags);
            return Ok(1);
        },
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
                None => { return Ok(0) },
            };
        },
        0xf1 => (cpu.a.value, cpu.flags.flags) = pop(&mut cpu.sp, &mut cpu.memory),
        0xf2 => { // JP
            let jmp_address: Option<u16> = jmp(
                (cpu.memory.read_at(cpu.pc.address), cpu.memory.read_at(cpu.pc.address + 1)),
                Some(cpu.flags.check_flag(Flag::S) == 0)
                );
            match jmp_address {
                Some(address) => cpu.pc.address = address,
                None => return Ok(2),
            };
        },
        0xf3 => cpu.interrupt_enabled = false,
        0xf4 => { // CP
            let call_address: Option<u16> = call(
                (cpu.memory.read_at(cpu.pc.address), cpu.memory.read_at(cpu.pc.address + 1)),
                Some(cpu.flags.check_flag(Flag::S) == 0),
                &mut cpu.sp, &mut cpu.memory,
                cpu.pc.address + 2
                );
            match call_address {
                Some(address) => cpu.pc.address = address,
                None => return Ok(2),
            };
        },
        0xf5 => push((cpu.a.value, cpu.flags.flags), &mut cpu.sp, &mut cpu.memory),
        0xf6 => { // ORI
            cpu.a.value = or(cpu.a.value, cpu.memory.read_at(cpu.pc.address), &mut cpu.flags);
            return Ok(1);
        },
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
                None => { return Ok(0) },
            };
        },
        0xf9 => cpu.sp.address = pair_registers(cpu.h.value, cpu.l.value),
        0xfa => { // JM
            let jmp_address: Option<u16> = jmp(
                (cpu.memory.read_at(cpu.pc.address), cpu.memory.read_at(cpu.pc.address + 1)),
                Some(cpu.flags.check_flag(Flag::S) == 1)
                );
            match jmp_address {
                Some(address) => cpu.pc.address = address,
                None => return Ok(2),
            };
        },
        0xfb => cpu.interrupt_enabled = true,
        0xfc => { // CM
            let call_address: Option<u16> = call(
                (cpu.memory.read_at(cpu.pc.address), cpu.memory.read_at(cpu.pc.address + 1)),
                Some(cpu.flags.check_flag(Flag::S) == 1),
                &mut cpu.sp, &mut cpu.memory,
                cpu.pc.address + 2
                );
            match call_address {
                Some(address) => cpu.pc.address = address,
                None => return Ok(2),
            };
        },
        0xfd => {},
        0xfe => { // CPI
            cmp(cpu.a.value, cpu.memory.read_at(cpu.pc.address), &mut cpu.flags);
            return Ok(1);
        },
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

    Ok(0)
    // If an operation doesn't specify the number of additional bytes it read
    //  the function will return 0 additional bytes
}
