use super::opcodes;
use crate::cpu::CPU;
use std::collections::HashMap;

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: 0,
            program_counter: 0,
            memory: [0; 0xFFFF],
        }
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.status = 0;

        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    pub fn run(&mut self) {
        let ref opcodes: HashMap<u8, &'static opcodes::Opcode> = *opcodes::OPCODE_MAP;

        loop {
            let code = self.mem_read(self.program_counter);
            self.program_counter += 1;
            let current_pc = self.program_counter;

            let opcode = match opcodes.get(&code) {
                Some(opcode) => opcode,
                _ => panic!(),
            };

            match code {
                0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
                    self.lda(&opcode.mode);
                }
                0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => {
                    self.sta(&opcode.mode);
                }
                0xAA => self.tax(),
                0xE8 => self.inx(),
                0x00 => return,
                _ => todo!(),
            }

            if self.program_counter == current_pc {
                self.program_counter += (opcode.len - 1) as u16;
            }
        }
    }
}