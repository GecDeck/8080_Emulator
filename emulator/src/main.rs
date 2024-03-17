use std::env;

use emulator::cpu;
use emulator::cpu::Cpu;
use emulator::hardware;
use emulator::hardware::Hardware;

fn main() -> Result<(), u8> {
    let mut cpu: Cpu = Cpu::init();
    let mut hardware: Hardware = Hardware::init();
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

        // println!("{:04x}    0x{:02x}    0x{:02x}    0x{:02x}", cpu.pc.address - 1, op_code, cpu.memory.read_at(cpu.pc.address), cpu.memory.read_at(cpu.pc.address + 1));

        let result = match op_code {
            0xdb | 0xd3 => { // IN & OUT
                // IO is handled by the hardware module not the cpu
                // For IN operations handle_io returns the value read from the port
                let port_byte: u8 = cpu.memory.read_at(cpu.pc.address);
                match hardware::handle_io(op_code, &mut hardware, port_byte, cpu.a.value) {
                    Some(value) => cpu.a.value = value,
                    None => {},
                }
                Ok(1)
                // IN & OUT always read one additional byte
            },
            _ => cpu::dispatcher::handle_op_code(op_code, &mut cpu)
        };

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
