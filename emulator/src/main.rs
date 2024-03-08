use std::env;

use emulator::cpu;
use emulator::cpu::Cpu;

fn main() -> Result<(), u8> {
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
        // Important to remember pc address is incremented before op code is handled
        //  when handling operations that read additional bytes, the first byte to be read will be
        //  at the pc address NOT pc address + 1

        println!("{:04x}    0x{:02x}    0x{:02x}    0x{:02x}", cpu.pc.address - 1, op_code, cpu.memory.read_at(cpu.pc.address), cpu.memory.read_at(cpu.pc.address + 1));
        let result = cpu::dispatcher::handle_op_code(op_code, &mut cpu);
        match result {
            Err(e) => {
                println!("0x{:02x} encountered error: {}", op_code, e);
                panic!();
            },
            Ok(additional_bytes) => match additional_bytes {
                255 => return Ok(()),
                // Only halt should return 255
                _ => cpu.pc.address += additional_bytes,
            },
        }
    }
}
