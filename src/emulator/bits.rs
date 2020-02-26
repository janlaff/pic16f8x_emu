// Bit constants for status register
pub const IRP: usize = 7;
pub const RP1: usize = 6;
pub const RP0: usize = 5;
pub const TO: usize = 4;
pub const PD: usize = 3;
pub const Z: usize = 2;
pub const DC: usize = 1;
pub const C: usize = 0;

// Bit constants for porta register
pub const RA4: usize = 4;
pub const RA3: usize = 3;
pub const RA2: usize = 2;
pub const RA1: usize = 1;
pub const RA0: usize = 0;

// Bit constants for portb register
pub const RB7: usize = 7;
pub const RB6: usize = 6;
pub const RB5: usize = 5;
pub const RB4: usize = 4;
pub const RB3: usize = 3;
pub const RB2: usize = 2;
pub const RB1: usize = 1;
pub const RB0: usize = 0;

// Bit constants for intcon register
pub const GIE: usize = 7;
pub const EEIE: usize = 6;
pub const T0IE: usize = 5;
pub const INTE: usize = 4;
pub const RBIE: usize = 3;
pub const T0IF: usize = 2;
pub const INTF: usize = 1;
pub const RBIF: usize = 0;

// Bit constants for option register
pub const RBPU: usize = 7;
pub const INTEDG: usize = 6;
pub const T0CS: usize = 5;
pub const T0SE: usize = 4;
pub const PSA: usize = 3;
pub const PS2: usize = 2;
pub const PS1: usize = 1;
pub const PS0: usize = 0;

// Bit constants for eecon1 register
pub const EEIF: usize = 4;
pub const WRERR: usize = 3;
pub const WREN: usize = 2;
pub const WR: usize = 1;
pub const RD: usize = 0;

pub fn set_bit_enabled(value: &mut u8, bit: usize, enabled: bool) {
    if enabled {
        set_bit(value, bit);
    } else {
        clear_bit(value, bit);
    }
}

pub fn set_bit(value: &mut u8, bit: usize) {
    *value |= 1 << bit;
}

pub fn clear_bit(value: &mut u8, bit: usize) {
    *value &= !(1 << bit)
}

pub fn get_bit(value: u8, bit: usize) -> bool {
    value & (1 << bit) > 0
}

pub fn get_low_byte(reg: u16) -> u8 {
    (reg & 0xff) as u8
}

pub fn get_high_byte(reg: u16) -> u8 {
    (reg >> 8) as u8
}

pub fn set_low_byte(reg: &mut u16, val: u8) {
    *reg &= 0xff00;
    *reg |= val as u16;
}

pub fn set_high_byte(reg: &mut u16, val: u8) {
    *reg &= 0x00ff;
    *reg |= (val as u16) << 8
}

pub fn join_bytes(high: u8, low: u8) -> u16 {
    ((high as u16) << 8) | (low as u16)
}

pub fn get_enabled_bits(value: u8) -> u8 {
    let mut n = 0;
    for i in 0..8 {
        if (value >> i) & 0x1 == 0x1 {
            n += 1;
        }
    }
    n
}
