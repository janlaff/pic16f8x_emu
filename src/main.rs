mod emulator;

use emulator::*;

fn main() {
    println!("[*] Starting PIC16F8X Emulator");

    let mut data_bus = DataBus::new();
    let mut rom_bus = RomBus::new();

    println!("[*] Disassembling ../SimTest01.bin");
    rom_bus.load_program(include_bytes!("../SimTest01.bin"), 0x0000);

    let (min, max) = rom_bus.get_rom_boundary();
    for i in (min..max + 2).step_by(2) {
        let result = rom_bus.read_instruction(i);
        if let Ok(instr) = result {
            println!("[{:04x}]: {:?}", i, instr);
        } else {
            println!("{}", result.err().unwrap());
        }
    }
}
