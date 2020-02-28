use super::bits::*;

#[derive(Copy, Clone)]
pub struct SFRBank {
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

impl SFRBank {
    pub fn new() -> Self {
        Self {
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
}

pub struct DataBus {
    pub memory: [u8; 0x80],
    pub sfr_bank: SFRBank,
}

impl DataBus {
    pub fn new() -> Self {
        Self {
            memory: [0; 0x80],
            sfr_bank: SFRBank::new(),
        }
    }

    pub fn load_pc(&mut self, value: u16) {
        // When loading pc from GOTO or CALL instruction
        // The upper two bits are being ignored
        // -> only 11 bits from value are loaded
        self.sfr_bank.pclath = (self.sfr_bank.pclath & 0b11000) | ((value >> 8) as u8 & 0b00111);
        self.sfr_bank.pcl = value as u8;
    }

    pub fn get_pc(&self) -> u16 {
        join_bytes(self.sfr_bank.pclath & 0b11111, self.sfr_bank.pcl)
    }

    pub fn set_pc(&mut self, value: u16) {
        // TODO: check if this is event needed
        self.sfr_bank.pclath = ((value >> 8) & 0b11111) as u8;
        self.sfr_bank.pcl = value as u8;
    }

    pub fn inc_pc(&mut self, amount: u16) {
        self.set_pc(self.get_pc().wrapping_add(amount));
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

    pub fn read_byte(&mut self, address: u8) -> u8 {
        let value = *self.map_address(address);
        debug!("Reading {:02x} from {:02x}", value, address);
        value
    }

    pub fn write_byte(&mut self, address: u8, value: u8) {
        let real_addr = self.map_address(address);
        debug!("Writing {:02x} to {:02x}", value, address);
        *real_addr = value;
    }

    fn map_address(&mut self, address: u8) -> &mut u8 {
        assert!((address as usize) < self.memory.len());

        if get_bit(self.sfr_bank.status, RP0) {
            // Bank 1 is used
            match address {
                0x00 => &mut self.sfr_bank.indirect,
                0x01 => &mut self.sfr_bank.option,
                0x02 => &mut self.sfr_bank.pcl,
                0x03 => &mut self.sfr_bank.status,
                0x04 => &mut self.sfr_bank.fsr,
                0x05 => &mut self.sfr_bank.trisa,
                0x06 => &mut self.sfr_bank.trisb,
                0x08 => &mut self.sfr_bank.eecon1,
                0x09 => &mut self.sfr_bank.eecon2,
                0x0a => &mut self.sfr_bank.pclath,
                0x0b => &mut self.sfr_bank.intcon,
                _ => &mut self.memory[address as usize],
            }
        } else {
            // Bank 0 is used
            match address {
                0x00 => &mut self.sfr_bank.indirect,
                0x01 => &mut self.sfr_bank.tmr0,
                0x02 => &mut self.sfr_bank.pcl,
                0x03 => &mut self.sfr_bank.status,
                0x04 => &mut self.sfr_bank.fsr,
                0x05 => &mut self.sfr_bank.porta,
                0x06 => &mut self.sfr_bank.portb,
                0x08 => &mut self.sfr_bank.eedata,
                0x09 => &mut self.sfr_bank.eeadr,
                0x0a => &mut self.sfr_bank.pclath,
                0x0b => &mut self.sfr_bank.intcon,
                _ => &mut self.memory[address as usize],
            }
        }
    }
}
