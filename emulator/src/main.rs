use std::env;

use emulator::cpu;
use emulator::cpu::Cpu;

fn main() {
    let mut cpu: Cpu = Cpu::init();
    // Initialize Cpu

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a file to disassemble");
    }

    let file_path: &str = &args[1];
    cpu.memory.load_rom(file_path);
    // Loads Rom into memory

    loop {
        let op_code: u8 = cpu.memory.read_at(cpu.pc.address);
        cpu.pc.address += 1;

        let additional_bytes: u16 = cpu::handle_op_code(op_code, &mut cpu);
        cpu.pc.address += additional_bytes;
    }
}
