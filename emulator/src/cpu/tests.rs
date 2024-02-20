#[cfg(test)]
use super::*;
use super::dispatcher::handle_op_code;

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
    cpu.reset();
    cpu.pc.address = 0x0002;
    cpu.sp.address = 0x2400;

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
fn test_logical_operations() {
    let mut cpu: Cpu = Cpu::init();

    // AND
    assert_eq!(and(0b10101010, 0b01011010, &mut cpu.flags), 0b00001010);

    // XOR
    assert_eq!(xor(0b10101010, 0b10100000, &mut cpu.flags), 0b00001010);

    // OR
    assert_eq!(or(0b10101010, 0b00000101, &mut cpu.flags), 0b10101111);

    // NOT
    assert_eq!(not(0b10101010, &mut cpu.flags), 0b01010101);

    // Compare
    todo!();

    // Rotate
    todo!();
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

    assert_eq!(handle_op_code(0xc3, &mut cpu), 0);
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

    assert_eq!(handle_op_code(0xcd, &mut cpu), 0);
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
