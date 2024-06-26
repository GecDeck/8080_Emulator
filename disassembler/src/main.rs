use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a file to disassemble");
    }

    let file_path: &str = &args[1];
    let data: Vec<u8> = match fs::read(file_path) {
        Ok(result) => result,
        Err(e) => panic!("{}", e),
    };

    disassembler::disassemble(&data);
}
