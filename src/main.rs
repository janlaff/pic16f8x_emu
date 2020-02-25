mod instruction;
use instruction::*;

fn main() {
    println!("{:?}", Instruction::from(0b00_0111_0111_1111));
}
