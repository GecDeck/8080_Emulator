use std::collections::HashMap;

mod instructions;
use instructions::INSTRUCTIONS;

pub fn disassemble(data: &[u8]) -> Vec<Operation> {
    let mut ops: Vec<Operation> = vec![];
    let instructions: HashMap<u8, (String, u8)> = get_instruction_set();

    let mut index: usize = 0;
    while index < data.len() {
        let op = get_operation(data, index, &instructions);
        index += op.op_bytes as usize;

        ops.push(op);
    };

    let mut address: u16 = 0;
    for op in &ops {
        match op.op_bytes {
            1 => println!("{:04x}   {:02x}          {}", address, op.op_code, op.instruction),
            2 => println!("{:04x}   {:02x} {:02x}       {}", address, op.op_code, op.data.0, op.instruction),
            3 => println!("{:04x}   {:02x} {:02x} {:02x}    {}", address, op.op_code, op.data.0, op.data.1, op.instruction),
            _ => panic!("Invalid number of bytes used for instruction"),
        }
        address += op.op_bytes as u16;
    }

    ops
}

fn get_instruction_set() -> HashMap<u8, (String, u8)> {
    let mut instruction_set: HashMap<u8, (String, u8)> = HashMap::new();

    for instruction_info in INSTRUCTIONS.lines() {
        // Line should look like this
        // 0x(hex op code) (operation name) (number of bytes used for operation)

        let (op_code_str, op): (&str, &str) = instruction_info.split_once(' ').expect("splitting op code from instruction");
        let op_code: u8 = u8::from_str_radix(&op_code_str[2..=3], 16).expect("converting hex string slice to byte");
        // Only using second half because the opcodes are written as 0x[8 bit code]

        let op_bytes: u8 = op.chars().last().expect("getting last char of op string which should be the number of bytes used in op")
            .to_digit(10).expect("converting digit into u8") as u8;
        // Getting number of bytes used by the operation

        let instruction = op.trim_end_matches(char::is_numeric).trim();
        // Trimming op_byte digit and whitespace off end

        instruction_set.insert(op_code, (String::from(instruction), op_bytes));
    }

    instruction_set
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

fn get_operation(data: &[u8], index: usize, instructions: &HashMap<u8, (String, u8)>) -> Operation {
    let op = match instructions.get(&data[index]) {
        // Searching dictionary by op code
        Some((instruction, op_bytes)) => match op_bytes {
            // Taking the correct number of bytes for the given instruction
            1 => Operation::new(instruction, data[index], *op_bytes, (0, 0)),
            2 => Operation::new(instruction, data[index], *op_bytes, (data[index+1], 0)),
            3 => Operation::new(instruction, data[index], *op_bytes, (data[index+2], data[index+1])),
            _ => panic!("There should never be an instruction with more than 3 bytes"),
        }
        None => {
            println!("No operation found for 0x{:02x}", data[index]);
            panic!("Every byte should coorespond to an instruction");
        },
    };

    op
}
