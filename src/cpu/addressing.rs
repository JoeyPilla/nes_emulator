use crate::cpu::CPU;

#[derive(Debug)]
pub enum AddressingMode {
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Immediate,
    IndirectX,
    IndirectY,
    NoneAddressing,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
}

impl CPU {
    pub(crate) fn get_operand_address(&self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Absolute => self.mem_read_u16(self.program_counter),
            AddressingMode::AbsoluteX => self.absolute_register(self.register_x),
            AddressingMode::AbsoluteY => self.absolute_register(self.register_y),
            AddressingMode::Immediate => self.program_counter,
            AddressingMode::IndirectX => self.indirect_register_x(),
            AddressingMode::IndirectY => self.indirect_register_y(),
            AddressingMode::ZeroPage => self.mem_read(self.program_counter) as u16,
            AddressingMode::ZeroPageX => self.zero_page_register(self.register_x),
            AddressingMode::ZeroPageY => self.zero_page_register(self.register_y),
            AddressingMode::NoneAddressing => {
                panic!("mode {:?} is not supported", mode);
            }
        }
    }

    fn absolute_register(&self, register_value: u8) -> u16 {
        let base = self.mem_read_u16(self.program_counter);
        base.wrapping_add(register_value as u16)
    }

    fn indirect_register_x(&self) -> u16 {
        let base = self.mem_read(self.program_counter);
        let ptr = base.wrapping_add(self.register_x);

        u16::from_le_bytes([
            self.mem_read(ptr as u16),
            self.mem_read(ptr.wrapping_add(1) as u16),
        ])
    }

    fn indirect_register_y(&self) -> u16 {
        let base = self.mem_read(self.program_counter);

        let lo = self.mem_read(base as u16);
        let hi = self.mem_read((base as u8).wrapping_add(1) as u16);

        let deref = u16::from_le_bytes([lo, hi]);
        deref.wrapping_add(self.register_y as u16)
    }

    fn zero_page_register(&self, register_value: u8) -> u16 {
        let pos = self.mem_read(self.program_counter);
        pos.wrapping_add(register_value) as u16
    }
}
