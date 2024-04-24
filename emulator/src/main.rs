use std::env;
use std::fs;

use emulator::cpu;
use emulator::cpu::Cpu;
use emulator::hardware::Hardware;

fn main() -> Result<(), u8> {
    let (mut raylib_handle, thread) = raylib::init()
        .size(emulator::WIDTH, emulator::HEIGHT)
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

    // for i in 0x03be..0x03c1 {
    //     println!("0x{:04x}: 0x{:02x}", i, cpu.memory.read_at(i));
    // }

    while !raylib_handle.window_should_close() {
        // Locked to 60 frames per second
        // Interrupts twice per frame; Once in the middle, and once at the end
        // There are a total of 33 000 cycles in every half frame
        let mut frame_cycles: u64 = 0;
        let cycle_max: u64 = 33_000;

        while frame_cycles < cycle_max / 2 {
            frame_cycles += emulator::update(&mut raylib_handle, &mut hardware, &mut cpu);
        }
        cpu::generate_interrupt(0xcf, &mut cpu);
        // Call mid screen interrupt

        while frame_cycles < cycle_max {
            frame_cycles += emulator::update(&mut raylib_handle, &mut hardware, &mut cpu);
        }
        cpu::generate_interrupt(0xd7, &mut cpu);
        // Call full screen interrupt

        emulator::render(&mut raylib_handle, &thread, &hardware, &cpu);
        // Render frame
    }

    Ok(())
}
