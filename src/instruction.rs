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
    MovWf(FileRegister, DestinationFlag),
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
    pub fn from(opcode: u16) -> Option<Self> {
        let category = InstructionCategory::from((opcode >> 12) as u8);

        match category {
            InstructionCategory::ByteOriented => {
                let file_register = FileRegister((opcode & 0b01111111) as u8);
                let destination_flag = DestinationFlag((opcode & 0b10000000) > 0);

                // TODO
            }
            InstructionCategory::BitOriented => {
                let file_register = FileRegister((opcode & 0b01111111) as u8);
                let bit_index = BitIndex(((opcode >> 7) & 0b00000111) as usize);

                // TODO
            }
            InstructionCategory::LiteralOriented => {
                let literal = Literal((opcode & 0b11111111) as u8);

                // TODO
            }
            InstructionCategory::AddressOriented => {
                let address = Address(opcode & 0b111_11111111);

                // TODO
            }
        }

        None
    }
}
