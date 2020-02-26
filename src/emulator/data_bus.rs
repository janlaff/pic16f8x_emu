use super::bits::*;

pub struct DataBus {
    memory: [u8; 0x80],
    pub indirect: u8,
    pub pcl: u8,
    pub status: u8,
    pub fsr: u8,
    pub pclath: u8,
    pub intcon: u8,
    pub tmr0: u8,
    pub porta: u8,
    pub portb: u8,
    pub eedata: u8,
    pub eeadr: u8,
    pub option: u8,
    pub trisa: u8,
    pub trisb: u8,
    pub eecon1: u8,
    pub eecon2: u8,
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

    pub fn load_pc(&mut self, value: u16) {
        // When loading pc from GOTO or CALL instruction
        // The upper two bits are being ignored
        // -> only 11 bits from value are loaded
        self.pclath = (self.pclath & 0b11000) | ((value >> 8) as u8 & 0b00111);
        self.pcl = value as u8;
    }

    pub fn get_pc(&self) -> u16 {
        join_bytes(self.pclath & 0b11111, self.pcl)
    }

    pub fn set_pc(&mut self, value: u16) {
        self.pclath = ((value >> 8) & 0b11111) as u8;
        self.pcl = value as u8;
    }

    pub fn inc_pc(&mut self, amount: u16) {
        self.set_pc(self.get_pc().wrapping_add(amount));
    }

    pub fn read_byte(&mut self, address: u8) -> u8 {
        *self.map_address(address)
    }

    pub fn get_bit(&mut self, address: u8, bit: usize) -> bool {
        get_bit(*self.map_address(address), bit)
    }

    pub fn clear_bit(&mut self, address: u8, bit: usize) {
        clear_bit(self.map_address(address), bit);
    }

    pub fn set_bit(&mut self, address: u8, bit: usize) {
        set_bit(self.map_address(address), bit);
    }

    pub fn write_byte(&mut self, address: u8, value: u8) {
        *self.map_address(address) = value;
    }

    fn map_address(&mut self, address: u8) -> &mut u8 {
        assert!(address < 0x80);

        if get_bit(self.status, RP0) {
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
