mod emulator;

use emulator::*;

fn main() {
    println!("[*] Starting PIC16F8X Emulator");

    let mut data_bus = DataBus::new();
    let mut rom_bus = RomBus::new();
}
