use std::env;

use emulator::cpu;
use emulator::cpu::State;

fn main() {
    let mut state: State = State::init();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a file to disassemble");
    }

    let file_path: &str = &args[1];
    state.memory.load_rom(file_path);

    loop {
        let op_code: u8 = state.memory.read_at(state.pc.address);
        state.pc.address += 1;

        let additional_bytes: u16 = cpu::handle_op_code(op_code, &mut state);
        state.pc.address += additional_bytes;
    }
}
