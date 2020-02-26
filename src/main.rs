mod emulator;

use emulator::*;

fn main() {
    println!("[*] Starting PIC16F8X Emulator");
    /*let mut bus = DataBus::new();

    bus.set_pc(0xFFFF);
    assert_eq!(0b1111111111111, bus.get_pc());
    assert_eq!(bus.pcl, 0xFF);
    assert_eq!(bus.pclath, 0xFF);*/

    if let Ok(instr) = Instruction::from(0b00_0100_1000_1111) {
        println!("{:?}", instr)
    }
}
