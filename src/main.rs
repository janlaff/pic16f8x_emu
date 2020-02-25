mod emulator;

use emulator::*;

fn main() {
    for i in 0u16..0b100_0000_0000_0000 {
        if let Ok(instruction) = Instruction::from(i) {
            println!("{:04x}: {:?}", i, instruction);
        }
    }
}
