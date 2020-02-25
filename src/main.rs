mod data_bus;
mod instruction;

use data_bus::*;
use instruction::*;

fn main() {
    println!("{:?}", Instruction::from(0b00_0111_0111_1111));
    println!("{:?}", Instruction::from(0b01_0000_0111_1111));
    println!("{:?}", Instruction::from(0b11_1001_1000_0000));
    println!("{:?}", Instruction::from(0b10_0111_1111_1111));
    println!("{:?}", Instruction::from(0b00_0000_0000_1001));
}
