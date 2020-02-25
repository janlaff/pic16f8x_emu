#[macro_use]
extern crate bitpat;

mod instruction;
use instruction::*;

fn main() {
    println!("{:?}", Instruction::from(0b00_0111_1111_1111));
}
