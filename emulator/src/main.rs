use raylib::prelude::*;

use std::env;
use std::fs;

use emulator::cpu;
use emulator::cpu::Cpu;
use emulator::hardware;
use emulator::hardware::Hardware;

const WIDTH: i32 = 1920;
const HEIGHT: i32 = 1080;

const ON_COLOUR: Color = Color::WHITE;
const OFF_COLOUR: Color = Color::BLACK;

const DEBUG_TEXT_SIZE: i32 = 20;

fn main() -> Result<(), u8> {
    let (mut raylib_handle, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Space Invaders")
        .build();
    raylib_handle.set_target_fps(60);

    let mut cpu: Cpu = Cpu::init();
    let mut hardware: Hardware = Hardware::init();
    // Initialize Cpu

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a rom to emulate");
    }

    let file_path: &str = &args[1];
    let rom: Vec<u8> = match fs::read(file_path) {
        Ok(result) => result,
        Err(e) => panic!("{}", e),
    };
    cpu.memory.load_rom(&rom, 0);
    // Loads Rom into memory

    while !raylib_handle.window_should_close() {
        // Loading Render Loop @ 0x1a32:
        //  0x1a: LDAX D (A = (DE))
        //  0x77: MOV M, A ((HL) = A)
        //  0x23: INX H (HL + 1)
        //  0x13: INX D (DE + 1)
        //  0x05: DCR B (B - 1)
        //  0xc2: JNZ adr (if NZ PC = adr)
        // TODO: Need to find the other rendering loops
        //  after B hits 0 it moves onto another loop of some sort
        update(&mut raylib_handle, &mut hardware, &mut cpu);
        render(&mut raylib_handle, &thread, &hardware, &cpu);
    }

    Ok(())
}

fn update(raylib_handle: &mut raylib::RaylibHandle, hardware: &mut Hardware, cpu: &mut Cpu) {
    // Handles updating the state of the emulator before rendering

    hardware::input::read_input(&raylib_handle, hardware, hardware::input::InputConfig::default());
    // Reads user input and changes the state of the hardware input ports

    let op_code: u8 = cpu.memory.read_at(cpu.pc.address);
    cpu.pc.address += 1;
    // Important to remember pc address is incremented before op code is handled
    //  when handling operations that read additional bytes, the first byte to be read will be
    //  at the pc address NOT pc address + 1

    let result = match op_code {
        0xdb | 0xd3 => { // IN & OUT
            // IO is handled by the hardware module not the cpu
            // For IN operations handle_io returns the value read from the port
            let port_byte: u8 = cpu.memory.read_at(cpu.pc.address);
            match hardware::handle_io(op_code, hardware, port_byte, cpu.a.value) {
                Some(value) => cpu.a.value = value,
                None => {},
            }
            Ok(1)
            // IN & OUT always read one additional byte
        },
        _ => cpu::dispatcher::handle_op_code(op_code, cpu)
    };

    match result {
        Err(e) => {
            println!("0x{:02x} encountered error: {}", op_code, e);
            panic!();
        },
        Ok(additional_bytes) => match additional_bytes {
            255 => panic!("HALT"),
            // Only halt should return 255
            _ => cpu.pc.address += additional_bytes,
        },
    }
}

fn render(raylib_handle: &mut raylib::RaylibHandle, thread: &raylib::RaylibThread, hardware: &Hardware, cpu: &Cpu) {
    // Renders things to the screen based on the state of the machine

    let mut draw_handle = raylib_handle.begin_drawing(thread);

    draw_handle.clear_background(OFF_COLOUR);

    // Debug Rendering
    draw_handle.draw_fps(0, 400);
    // Input Debug
    let input_1: String = format!("INPUT_1: 0b{:08b}", hardware.debug_input1());
    draw_handle.draw_text(&input_1, 0, 400 + DEBUG_TEXT_SIZE, DEBUG_TEXT_SIZE, ON_COLOUR);
    let input_2: String = format!("INPUT_2: 0b{:08b}", hardware.debug_input2());
    draw_handle.draw_text(&input_2, 0, 400 + 2*DEBUG_TEXT_SIZE, DEBUG_TEXT_SIZE, ON_COLOUR);
    // CPU Debug
    let stack_pointer: String = format!("SP:    0x{:04x}", cpu.debug_stack_pointer());
    draw_handle.draw_text(&stack_pointer, 0, 400 + 3*DEBUG_TEXT_SIZE, DEBUG_TEXT_SIZE, ON_COLOUR);
    let program_counter: String = format!("PC:  0x{:04x}", cpu.debug_program_counter());
    draw_handle.draw_text(&program_counter, 0, 400 + 4*DEBUG_TEXT_SIZE, DEBUG_TEXT_SIZE, ON_COLOUR);
    let b_reg: String = format!("B_REGISTER:    0x{:02x}", cpu.debug_b());
    draw_handle.draw_text(&b_reg, 0, 400 + 5*DEBUG_TEXT_SIZE, DEBUG_TEXT_SIZE, ON_COLOUR);

    // Game Rendering
    let game_x_offset: i32 = 848;
    let game_y_offset: i32 = 412;
    draw_handle.draw_rectangle(game_x_offset as i32, game_y_offset as i32, 224, 256, Color::BLUE);

    let vram: &[u8] = cpu.memory.read_vram();

    let mut i: usize = 0;
    for ix in 0..224 {
        for iy in 0..(256 / 8) {
            let mut byte = vram[i];
            i += 1;

            for b in 0..8 {
                let x: i32 = ix as i32;
                let y: i32 = 256 - ((iy * 8) as i32 + b);

                if byte & 1 == 1 {
                    draw_handle.draw_pixel(x + game_x_offset, y + game_y_offset, ON_COLOUR);
                }

                byte >>= 1;
            }
        }
    }
}
