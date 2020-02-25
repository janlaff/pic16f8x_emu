// Mapped SFR for both banks
pub const INDIRECT: u8 = 0x00;
pub const PCL: u8 = 0x02;
pub const STATUS: u8 = 0x03;
pub const FSR: u8 = 0x04;
pub const PCLATH: u8 = 0x0a;
pub const INTCON: u8 = 0x0b;
// SFR for bank 0
pub const TMR0: u8 = 0x01;
pub const PORTA: u8 = 0x05;
pub const PORTB: u8 = 0x06;
pub const EEDATA: u8 = 0x08;
pub const EEADR: u8 = 0x09;
pub const SRAM_MIN: u8 = 0x0c;
pub const SRAM_MAX: u8 = 0x4f;
// SFR for bank 1
pub const OPTION: u8 = 0x81;
pub const TRISA: u8 = 0x85;
pub const TRISB: u8 = 0x86;
pub const EECON1: u8 = 0x88;
pub const EECON2: u8 = 0x89;
// Bit constants for status register
pub const IRP: u8 = 7;
pub const RP1: u8 = 6;
pub const RP0: u8 = 5;
pub const TO: u8 = 4;
pub const PD: u8 = 3;
pub const Z: u8 = 2;
pub const DC: u8 = 1;
pub const C: u8 = 0;

pub struct DataBus {
    memory: [u8; 0xff],
}

impl DataBus {
    pub fn new() -> Self {
        Self { memory: [0; 0xff] }
    }

    pub fn read_byte(&self, address: u8) -> u8 {
        // TODO
        0
    }

    pub fn read_byte_direct(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn write_byte_direct(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
}
