#[macro_use]
extern crate log;

extern crate simple_logger;

mod emulator;
use emulator::*;

fn main() {
    simple_logger::init().unwrap();
    info!("Starting PIC16F8X emulator");

    let mut cpu = CPU::new();
    cpu.rom_bus
        .load_program(include_bytes!("../SimTest01.bin"), 0);
    let (min, max) = cpu.rom_bus.get_rom_boundary();

    while cpu.data_bus.get_pc() <= max {
        cpu.step();
    }
}
