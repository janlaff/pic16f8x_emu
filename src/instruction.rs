#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct DestinationFlag(bool);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct FileRegister(u8);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct BitIndex(usize);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Address(u16);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Literal(u8);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum InstructionCategory {
    ByteOriented,
    BitOriented,
    LiteralOriented,
    AddressOriented,
}

impl InstructionCategory {
    pub fn from(value: u8) -> Self {
        match value {
            0b00 => InstructionCategory::ByteOriented,
            0b01 => InstructionCategory::BitOriented,
            0b11 => InstructionCategory::LiteralOriented,
            0b10 => InstructionCategory::AddressOriented,
            _ => panic!("Invalid instruction category: {:02x}", value),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Instruction {
    AddWf(FileRegister, DestinationFlag),
    AndWf(FileRegister, DestinationFlag),
    ClrF(FileRegister),
    ClrW,
    ComF(FileRegister, DestinationFlag),
    DecF(FileRegister, DestinationFlag),
    DecFsz(FileRegister, DestinationFlag),
    IncF(FileRegister, DestinationFlag),
    IncFsz(FileRegister, DestinationFlag),
    IorWf(FileRegister, DestinationFlag),
    MovF(FileRegister, DestinationFlag),
    MovWf(FileRegister),
    Nop,
    RlF(FileRegister, DestinationFlag),
    RrF(FileRegister, DestinationFlag),
    SubWf(FileRegister, DestinationFlag),
    SwapWf(FileRegister, DestinationFlag),
    XorWf(FileRegister, DestinationFlag),
    BcF(FileRegister, BitIndex),
    BsF(FileRegister, BitIndex),
    BtFsc(FileRegister, BitIndex),
    BtFss(FileRegister, BitIndex),
    AddLw(Literal),
    AndLw(Literal),
    Call(Address),
    ClearWdt,
    Goto(Address),
    IorLw(Literal),
    MovLw(Literal),
    Return,
    Sleep,
    SubLw(Literal),
    XorLw(Literal),
}

impl Instruction {
    pub fn from(opcode: u16) -> Self {
        // Only accept properly formatted opcodes
        assert_eq!(opcode & 0xC000, 0);

        let category = InstructionCategory::from((opcode >> 12) as u8);

        match category {
            InstructionCategory::ByteOriented => {
                let file_register = FileRegister((opcode & 0b01111111) as u8);
                let destination_flag = DestinationFlag((opcode & 0b10000000) > 0);
                let selector = (opcode >> 8) as u8;

                match selector {
                    0b0111 => Instruction::AddWf(file_register, destination_flag),
                    0b0101 => Instruction::AndWf(file_register, destination_flag),
                    0b0001 => {
                        if destination_flag.0 {
                            Instruction::ClrF(file_register)
                        } else {
                            Instruction::ClrW
                        }
                    }
                    0b1001 => Instruction::ComF(file_register, destination_flag),
                    0b0011 => Instruction::DecF(file_register, destination_flag),
                    0b1011 => Instruction::DecFsz(file_register, destination_flag),
                    0b1010 => Instruction::IncF(file_register, destination_flag),
                    0b1111 => Instruction::IncFsz(file_register, destination_flag),
                    0b0100 => Instruction::IorWf(file_register, destination_flag),
                    0b1000 => Instruction::MovF(file_register, destination_flag),
                    0b0000 => {
                        if destination_flag.0 {
                            Instruction::MovWf(file_register)
                        } else {
                            Instruction::Nop
                        }
                    }
                    0b1101 => Instruction::RlF(file_register, destination_flag),
                    0b1100 => Instruction::RrF(file_register, destination_flag),
                    0b0010 => Instruction::SubWf(file_register, destination_flag),
                    0b1110 => Instruction::SwapWf(file_register, destination_flag),
                    0b0110 => Instruction::XorWf(file_register, destination_flag),
                    _ => panic!("Unknown opcode: {:04x}", opcode),
                }
            }
            InstructionCategory::BitOriented => {
                let file_register = FileRegister((opcode & 0b01111111) as u8);
                let bit_index = BitIndex(((opcode >> 7) & 0b00000111) as usize);
                let selector = (opcode >> 10) as u8;

                // TODO
                match selector {
                    _ => panic!("Unknown opcode: {:04x}", opcode),
                }
            }
            InstructionCategory::LiteralOriented => {
                let literal = Literal((opcode & 0b11111111) as u8);
                let selector = (opcode >> 8) as u8;

                // TODO
                match selector {
                    _ => panic!("Unknown opcode: {:04x}", opcode),
                }
            }
            InstructionCategory::AddressOriented => {
                let address = Address(opcode & 0b111_11111111);
                let selector = (opcode >> 11) as u8;

                // TODO
                match selector {
                    _ => panic!("Unknown opcode: {:04x}", opcode),
                }
            }
        }
    }
}
