use super::addressing::AddressingMode;
use crate::cpu::CPU;

impl CPU {
    pub(crate) fn and(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        println!("{:#x} {:#x}", addr, value);

        self.register_a = self.register_a & value;

        println!("{:#x} {:#x}", self.register_a, value);

        self.update_zero_and_negative_flags(self.register_a)
    }

    pub(crate) fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x)
    }

    pub(crate) fn sta(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_a);
    }

    pub(crate) fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a)
    }

    pub(crate) fn inx(&mut self) {
        let (res, _overflow) = self.register_x.overflowing_add(1);
        self.register_x = res;
        self.update_zero_and_negative_flags(self.register_x)
    }

    pub(crate) fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.status = self.status | 0b0000_0010;
        } else {
            self.status = self.status & 0b1111_1101;
        }

        if result & 0b1000_0000 != 0 {
            self.status = self.status | 0b1000_0000;
        } else {
            self.status = self.status & 0b0111_1111;
        }
    }
}

#[cfg(test)]
mod test_and {
    use super::*;

    #[test]
    fn immediate() {
        let mut cpu = CPU::new();
        cpu.register_a = 0xFF;
        cpu.load(vec![0x29, 0xFF]);
        cpu.run();

        assert_eq!(cpu.register_a, 0xFF)
    }

    #[test]
    fn zero_page() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x55);
        cpu.register_a = 0xFF;

        cpu.load(vec![0x25, 0x10, 0x00]);
        cpu.run();

        assert_eq!(cpu.register_a, 0x55);
    }

    #[test]
    fn zero_page_x() {
        let mut cpu = CPU::new();

        cpu.mem_write(0x11, 0x55);
        cpu.register_x = 0x01;
        cpu.register_a = 0xFF;

        cpu.load(vec![0x35, 0x10, 0x00]);
        cpu.run();

        assert_eq!(cpu.register_a, 0x55);
    }

    #[test]
    fn absolute() {
        let mut cpu = CPU::new();
        cpu.register_a = 0xFF;

        cpu.mem_write(0x1234, 0x55);

        cpu.load(vec![0x2D, 0x34, 0x12, 0x00]);
        cpu.run();

        assert_eq!(cpu.register_a, 0x55);
    }

    #[test]
    fn absolute_x() {
        let mut cpu = CPU::new();

        cpu.mem_write(0x1234, 0x55);
        cpu.register_x = 0x04;
        cpu.register_a = 0xFF;

        cpu.load(vec![0x3D, 0x30, 0x12, 0x00]);
        cpu.run();

        assert_eq!(cpu.register_a, 0x55);
    }

    #[test]
    fn absolute_y() {
        let mut cpu = CPU::new();

        cpu.mem_write(0x1234, 0x55);
        cpu.register_y = 0x04;
        cpu.register_a = 0xFF;

        cpu.load(vec![0x39, 0x30, 0x12, 0x00]);
        cpu.run();

        assert_eq!(cpu.register_a, 0x55);
    }

    #[test]
    fn indirect_x() {
        let mut cpu = CPU::new();

        cpu.mem_write(0x31, 0x55);
        cpu.mem_write(0x55, 0x12);

        cpu.register_x = 0x01;
        cpu.register_a = 0xFF;

        cpu.load(vec![0x21, 0x30, 0x00]);
        cpu.run();

        assert_eq!(cpu.register_a, 0x12);
    }

    #[test]
    fn indirect_y() {
        let mut cpu = CPU::new();

        cpu.mem_write_u16(0x30, 0x1110);
        cpu.mem_write(0x1111, 0x55);
        cpu.register_y = 0x01;
        cpu.register_a = 0xFF;

        cpu.load(vec![0x31, 0x30, 0x00]);
        cpu.run();

        assert_eq!(cpu.register_a, 0x55);
    }
}

#[cfg(test)]
mod test_lda {
    use super::*;

    #[test]
    fn set_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn immediate() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0xA9, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0x10);
    }

    #[test]
    fn zero_page() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x55);

        cpu.load_and_run(vec![0xA5, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0x55);
    }

    #[test]
    fn zero_page_x() {
        let mut cpu = CPU::new();

        cpu.mem_write(0x11, 0x55);
        cpu.register_x = 0x01;

        cpu.load(vec![0xB5, 0x10, 0x00]);
        cpu.run();

        assert_eq!(cpu.register_a, 0x55);
    }

    #[test]
    fn absolute() {
        let mut cpu = CPU::new();

        cpu.mem_write(0x1234, 0x55);

        cpu.load(vec![0xAD, 0x34, 0x12, 0x00]);
        cpu.run();

        assert_eq!(cpu.register_a, 0x55);
    }

    #[test]
    fn absolute_x() {
        let mut cpu = CPU::new();

        cpu.mem_write(0x1234, 0x55);
        cpu.register_x = 0x04;

        cpu.load(vec![0xBD, 0x30, 0x12, 0x00]);
        cpu.run();

        assert_eq!(cpu.register_a, 0x55);
    }

    #[test]
    fn absolute_y() {
        let mut cpu = CPU::new();

        cpu.mem_write(0x1234, 0x55);
        cpu.register_y = 0x04;

        cpu.load(vec![0xB9, 0x30, 0x12, 0x00]);
        cpu.run();

        assert_eq!(cpu.register_a, 0x55);
    }

    #[test]
    fn indirect_x() {
        let mut cpu = CPU::new();

        cpu.mem_write(0x31, 0x55);
        cpu.mem_write(0x55, 0x12);

        cpu.register_x = 0x01;

        cpu.load(vec![0xA1, 0x30, 0x00]);
        cpu.run();

        assert_eq!(cpu.register_a, 0x12);
    }

    #[test]
    fn indirect_y() {
        let mut cpu = CPU::new();

        cpu.mem_write_u16(0x30, 0x1110);
        cpu.mem_write(0x1111, 0x55);
        cpu.register_y = 0x01;

        cpu.load(vec![0xB1, 0x30, 0x00]);
        cpu.run();

        assert_eq!(cpu.register_a, 0x55);
    }
}

#[cfg(test)]
mod test_tax {
    use super::*;

    #[test]
    fn move_a_to_x() {
        let mut cpu = CPU::new();

        cpu.register_a = 10;

        cpu.load(vec![0xaa, 0x00]);
        cpu.run();

        assert_eq!(cpu.register_x, 10)
    }
}

#[cfg(test)]
mod test_inx {
    use super::*;

    #[test]
    fn inx() {
        let mut cpu = CPU::new();
        cpu.register_x = 0x00;
        cpu.load(vec![0xE8, 0x00]);
        cpu.run();

        assert_eq!(cpu.register_x, 1)
    }

    #[test]
    fn inx_overflow() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xff;
        cpu.load(vec![0xE8, 0xE8, 0x00]);
        cpu.run();

        assert_eq!(cpu.register_x, 1)
    }
}

#[cfg(test)]
mod test_sta {
    use super::*;

    #[test]
    fn zero_page() {
        let mut cpu = CPU::new();

        cpu.register_a = 0x98;

        cpu.load_and_run(vec![0x85, 0x10, 0x00]);

        assert_eq!(cpu.mem_read(0x10), cpu.register_a);
    }

    #[test]
    fn zero_page_x() {
        let mut cpu = CPU::new();

        cpu.register_x = 0x01;
        cpu.register_a = 0x98;

        cpu.load(vec![0x95, 0x10, 0x00]);
        cpu.run();

        assert_eq!(cpu.mem_read(0x11), cpu.register_a);
    }

    #[test]
    fn absolute() {
        let mut cpu = CPU::new();

        cpu.register_a = 0x98;

        cpu.load(vec![0x8D, 0x34, 0x12, 0x00]);
        cpu.run();

        assert_eq!(cpu.mem_read(0x1234), cpu.register_a);
    }

    #[test]
    fn absolute_x() {
        let mut cpu = CPU::new();

        cpu.register_x = 0x04;
        cpu.register_a = 0x98;

        cpu.load(vec![0x9D, 0x30, 0x12, 0x00]);
        cpu.run();

        assert_eq!(cpu.mem_read(0x1234), cpu.register_a);
    }

    #[test]
    fn absolute_y() {
        let mut cpu = CPU::new();

        cpu.register_y = 0x04;
        cpu.register_a = 0x98;

        cpu.load(vec![0x99, 0x30, 0x12, 0x00]);
        cpu.run();

        assert_eq!(cpu.mem_read(0x1234), cpu.register_a);
    }

    #[test]
    fn indirect_x() {
        let mut cpu = CPU::new();

        cpu.mem_write(0x31, 0x55);
        cpu.mem_write(0x55, 0x12);

        cpu.register_x = 0x01;
        cpu.register_a = 0x98;

        cpu.load(vec![0x81, 0x30, 0x00]);
        cpu.run();

        assert_eq!(cpu.mem_read(0x55), cpu.register_a);
    }

    #[test]
    fn indirect_y() {
        let mut cpu = CPU::new();

        cpu.mem_write_u16(0x30, 0x1110);
        cpu.mem_write(0x1111, 0x55);
        cpu.register_y = 0x01;
        cpu.register_a = 0x98;

        cpu.load(vec![0x91, 0x30, 0x00]);
        cpu.run();

        assert_eq!(cpu.mem_read(0x1111), cpu.register_a);
    }
}
