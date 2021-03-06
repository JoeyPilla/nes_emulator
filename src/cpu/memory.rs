use crate::cpu::CPU;

impl CPU {
    pub(crate) fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub(crate) fn mem_read_u16(&self, pos: u16) -> u16 {
        u16::from_le_bytes([self.mem_read(pos), self.mem_read(pos + 1)])
    }

    pub(crate) fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let [first, second] = data.to_le_bytes();

        self.mem_write(pos, first);
        self.mem_write(pos + 1, second);
    }

    pub(crate) fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }
}
