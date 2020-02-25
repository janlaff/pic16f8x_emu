pub struct DataBus {
    memory: [u8; 0xff],
}

impl DataBus {
    pub fn new() -> Self {
        Self { memory: [0; 0xff] }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
}
