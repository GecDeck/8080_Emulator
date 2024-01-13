use emulator::State;
use emulator::operations;

fn main() {
    let mut state: State = State::init();

    loop {
        let op_code: u8 = state.memory.read_at(state.pc.read_address());
        state.pc.increment(1);

        let additional_bytes: u16 = operations::handle_op_code(op_code, &mut state);
        state.pc.increment(additional_bytes);
    }
}
