use super::util;

// Bit constants for status register
pub const IRP: usize = 7;
pub const RP1: usize = 6;
pub const RP0: usize = 5;
pub const TO: usize = 4;
pub const PD: usize = 3;
pub const Z: usize = 2;
pub const DC: usize = 1;
pub const C: usize = 0;

pub struct DataBus {
    memory: [u8; 0x80],
    indirect: u8,
    pcl: u8,
    status: u8,
    fsr: u8,
    pclath: u8,
    intcon: u8,
    tmr0: u8,
    porta: u8,
    portb: u8,
    eedata: u8,
    eeadr: u8,
    option: u8,
    trisa: u8,
    trisb: u8,
    eecon1: u8,
    eecon2: u8,
}

impl DataBus {
    pub fn new() -> Self {
        Self {
            memory: [0; 0x80],
            indirect: 0,
            pcl: 0,
            status: 0,
            fsr: 0,
            pclath: 0,
            intcon: 0,
            tmr0: 0,
            porta: 0,
            portb: 0,
            eedata: 0,
            eeadr: 0,
            option: 0,
            trisa: 0,
            trisb: 0,
            eecon1: 0,
            eecon2: 0,
        }
    }

    pub fn read_byte(&mut self, address: u16) -> u8 {
        *self.map_address(address)
    }

    pub fn get_bit(&mut self, address: u16, bit: usize) -> bool {
        util::get_bit(*self.map_address(address), bit)
    }

    pub fn clear_bit(&mut self, address: u16, bit: usize) {
        util::clear_bit(self.map_address(address), bit);
    }

    pub fn set_bit(&mut self, address: u16, bit: usize) {
        util::set_bit(self.map_address(address), bit);
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        *self.map_address(address) = value;
    }

    fn map_address(&mut self, address: u16) -> &mut u8 {
        assert!(address < 0x80);

        if util::get_bit(self.status, RP0) {
            // Bank 1 is used
            match address {
                0x00 => &mut self.indirect,
                0x01 => &mut self.option,
                0x02 => &mut self.pcl,
                0x03 => &mut self.status,
                0x04 => &mut self.fsr,
                0x05 => &mut self.trisa,
                0x06 => &mut self.trisb,
                0x08 => &mut self.eecon1,
                0x09 => &mut self.eecon2,
                0x0a => &mut self.pclath,
                0x0b => &mut self.intcon,
                _ => &mut self.memory[address as usize],
            }
        } else {
            // Bank 0 is used
            match address {
                0x00 => &mut self.indirect,
                0x01 => &mut self.tmr0,
                0x02 => &mut self.pcl,
                0x03 => &mut self.status,
                0x04 => &mut self.fsr,
                0x05 => &mut self.porta,
                0x06 => &mut self.portb,
                0x08 => &mut self.eedata,
                0x09 => &mut self.eeadr,
                0x0a => &mut self.pclath,
                0x0b => &mut self.intcon,
                _ => &mut self.memory[address as usize],
            }
        }
    }
}
