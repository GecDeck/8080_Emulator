use raylib::prelude::*;

pub mod cpu;
pub mod hardware;

use cpu::Cpu;
use hardware::Hardware;

pub const WIDTH: i32 = 1920;
pub const HEIGHT: i32 = 1080;
const INVADERS_WIDTH: i32 = 224;
const INVADERS_HEIGHT: i32 = 256;

const TOP_COLOUR: Color = Color::RED;
const MID_COLOUR: Color = Color::WHITE;
const BOTTOM_COLOUR: Color = Color::GREEN;
const OFF_COLOUR: Color = Color::BLACK;

const DEBUG_TEXT_SIZE: i32 = 20;

pub fn update(raylib_handle: &mut raylib::RaylibHandle, hardware: &mut Hardware, cpu: &mut Cpu) -> u64 {
    // Handles updating the state of the emulator before rendering

    hardware::input::read_input(&raylib_handle, hardware, hardware::input::InputConfig::default());
    // Reads user input and changes the state of the hardware input ports

    let op_code: u8 = cpu.memory.read_at(cpu.pc.address);
    let op_code_location: u16 = cpu.pc.address;
    cpu.pc.address += 1;
    let additional_bytes: (u8, u8) = (cpu.memory.read_at(cpu.pc.address), cpu.memory.read_at(cpu.pc.address + 1));
    // Important to remember pc address is incremented before op code is handled
    //  when handling operations that read additional bytes, the first byte to be read will be
    //  at the pc address NOT pc address + 1

    let cycles: u8 = cpu::dispatcher::CLOCK_CYCLES[op_code as usize];

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
            println!("0x{:04x}: 0x{:02x} encountered error: {}", op_code_location, op_code, e);
            // panic!();
        },
        Ok(additional_bytes) => match additional_bytes {
            255 => panic!("HALT"),
            // Only halt should return 255
            _ => cpu.pc.address += additional_bytes,
        },
    }

    // println!("0x{:04x}: 0x{:02x}:   (0x{:02x}, 0x{:02x})", op_code_location, op_code, additional_bytes.0, additional_bytes.1);
    cycles as u64
}

pub fn render(raylib_handle: &mut raylib::RaylibHandle, thread: &raylib::RaylibThread, hardware: &Hardware, cpu: &Cpu) {
    // Renders things to the screen based on the state of the machine

    let mut draw_handle = raylib_handle.begin_drawing(thread);

    draw_handle.clear_background(OFF_COLOUR);

    // Debug Rendering
    draw_handle.draw_fps(0, 0);
    let input_1: String = format!("INPUT_1: 0b{:08b}", hardware.debug_input1());
    let input_2: String = format!("INPUT_2: 0b{:08b}", hardware.debug_input2());
    let stack_pointer: String = format!("SP:    0x{:04x}", cpu.debug_stack_pointer());
    let program_counter: String = format!("PC:  0x{:04x}", cpu.debug_program_counter());

    let debug_text: Vec<&str> = vec![&input_1, &input_2, &stack_pointer, &program_counter];
    for (i, text) in debug_text.iter().enumerate() {
        draw_handle.draw_text(text, 0, (1 + i as i32)*DEBUG_TEXT_SIZE, DEBUG_TEXT_SIZE, MID_COLOUR);
        // 1 + i to start the debug strings after the fps
    }
    // Draws each debug string in a column

    // Game Rendering
    let scale: i32 = HEIGHT / INVADERS_HEIGHT;
    // Scale Space Invaders so it fits vertically as close as possible
    //  Not a float so can't fit exactly

    let game_scaled_width: i32 = INVADERS_WIDTH * scale;
    let game_scaled_height: i32 = INVADERS_HEIGHT * scale;
    let game_x_offset: i32 = (WIDTH - game_scaled_width) / 2;
    let game_y_offset: i32 = (HEIGHT - game_scaled_height) / 2;
    draw_handle.draw_rectangle(game_x_offset, game_y_offset, INVADERS_WIDTH * scale, INVADERS_HEIGHT * scale, Color::BLUE);
    // Move the game to the middle of the screen and add a background behind it

    let vram: &[u8] = cpu.memory.read_vram();

    let mut i: usize = 0;
    for ix in 0..INVADERS_WIDTH {
        for iy in 0..(INVADERS_HEIGHT / 8) {
            let mut byte = vram[i];
            i += 1;

            for b in 0..8 {
                let x: i32 = (ix as i32) * scale;
                let y: i32 = (INVADERS_HEIGHT - ((iy * 8) as i32 + b)) * scale;

                if byte & 1 == 1 {
                    let colour: Color = match iy * 8 {
                        201..=219 => TOP_COLOUR,
                        0..=79 => BOTTOM_COLOUR,
                        _ => MID_COLOUR,
                    };
                    draw_handle.draw_rectangle(x + game_x_offset, y + game_y_offset, scale, scale, colour);
                }

                byte >>= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpu_diag() {
        // Load cpudiag
        // When an out instruction is called
        //  if port 0: Test is finished
        //  if port 1: Look at content of C register
        //      if c is 2: Print the content of the E register
        //      if c is 9: Print (DE)..(DE+1).. until (DE) == $

        let mut cpu: Cpu = Cpu::init();
        let cpu_diag: &[u8] = include_bytes!("../cpudiag");

        cpu.memory.load_rom(cpu_diag, 0x100);
        cpu.pc.address = 0x100;

        let start = std::time::Instant::now();
        loop {
            test_update(&mut cpu);

            let now = std::time::Instant::now();
            if now - start > std::time::Duration::from_secs(120) {
                panic!("CPU DIAG took over 2 minutes");
            }
        }
    }

    fn test_update(cpu: &mut Cpu) {

        let op_code: u8 = cpu.memory.read_at(cpu.pc.address);
        let op_code_location: u16 = cpu.pc.address;
        cpu.pc.address += 1;

        let result = match op_code {
            0xdb | 0xd3 => { // IN & OUT
                let port_byte: u8 = cpu.memory.read_at(cpu.pc.address);
                handle_out(&cpu, port_byte);

                Ok(1)
                // IN & OUT always read one additional byte
            },
            _ => cpu::dispatcher::handle_op_code(op_code, cpu)
        };

        match result {
            Err(e) => {
                println!("0x{:04x}: 0x{:02x} encountered error: {}", op_code_location, op_code, e);
            },
            Ok(additional_bytes) => match additional_bytes {
                255 => panic!("HALT"),
                _ => cpu.pc.address += additional_bytes,
            },
        }

    }

    fn handle_out(cpu: &Cpu, port_byte: u8) {
        match port_byte {
            0 => println!("Test Complete"),
            1 => os_syscall(cpu),
            _ => panic!("No other ports"),
        }
    }

    fn os_syscall(cpu: &Cpu) {
        match cpu.debug_c() {
            2 => println!("{}", cpu.debug_e()),
            9 => {
                let mut memory_address: u16 = (cpu.debug_d() as u16) << 8 | cpu.debug_e() as u16;
                let mut string_to_print: String = String::new();
                while cpu.memory.read_at(memory_address) != '$' as u8 {
                    string_to_print.push(cpu.memory.read_at(memory_address) as char);
                    memory_address += 1;
                }
            },
            _ => panic!("No syscalls other than 9 and 2"),
        }
    }
}
