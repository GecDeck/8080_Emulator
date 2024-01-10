use std::collections::HashMap;

mod instructions;
use instructions::INSTRUCTIONS;

pub fn disassemble(data: &[u8]) -> Vec<Operation> {
    let mut ops: Vec<Operation> = vec![];

    let mut index: usize = 0;
    while index < data.len() {
        let op = get_operation(&data, index);
        index += op.op_bytes as usize;

        ops.push(op);
    };

    for op in &ops {
        match op.op_bytes {
            1 => println!("{:02x}           {}", op.op_code, op.instruction),
            2 => println!("{:02x} {:02x}        {}", op.op_code, op.data.0, op.instruction),
            3 => println!("{:02x} {:02x} {:02x}     {}", op.op_code, op.data.0, op.data.1, op.instruction),
            _ => panic!(),
        }
    }

    return ops;
}

fn get_instruction_set() -> HashMap<u8, (String, u8)> {
    let mut instruction_set: HashMap<u8, (String, u8)> = HashMap::new();

    for instruction in INSTRUCTIONS.lines() {
        // split at first whitespace
        // make first half the op code
        // get last digit of second half
        // make that the op bytes
        // get rest
        // make that the instruction
    }

    return instruction_set;
}

pub struct Operation {
    instruction: String,
    op_code: u8,
    // Hex code associated with instruction
    op_bytes: u8,
    // Number of bytes used in instruction should be 1-3
    data: (u8, u8),
    // Data used in instruction
    // TODO: Some way of handling instructions that use less than 3 bytes
}
impl Operation {
    fn new(instruction: &str, op_code: u8, op_bytes: u8, data: (u8, u8)) -> Self {
        Self {
            instruction: String::from(instruction),
            op_code,
            op_bytes,
            data,
        }
    }
}

fn get_operation(data: &[u8], index: usize) -> Operation {
    let op: Operation = match data[index] {
        0x00 => Operation::new("NOP", data[index], 1, (0, 0)),
        0x01 => Operation::new("LXI B", data[index], 3, (data[index+2], data[index+1])),
        0x02 => Operation::new("STAX B", data[index], 1, (0, 0)),
        0x03 => Operation::new("INX B", data[index], 1, (0, 0)),
        0x04 => Operation::new("INR B", data[index], 1, (0, 0)),
        0x05 => Operation::new("DCR B", data[index], 1, (0, 0)),
        0x06 => Operation::new("MVI B", data[index], 2, (data[index+1], 0)),
        0x07 => Operation::new("RLC", data[index], 1, (0, 0)),
        0xc3 => Operation::new("JMP", data[index], 3, (data[index+2], data[index+1])),
        0xf5 => Operation::new("PUSH PSW", data[index], 1, (0, 0)),
        _ => Operation::new("", data[index], 1, (0, 0)),
    };

    return op;
}
