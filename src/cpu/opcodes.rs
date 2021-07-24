use super::addressing::AddressingMode;
use std::collections::HashMap;

pub struct Opcode {
    pub code: u8,
    pub name: &'static str,
    pub cycles: u8,
    pub len: u8,
    pub mode: AddressingMode,
}

impl Opcode {
    pub fn new(code: u8, name: &'static str, len: u8, cycles: u8, mode: AddressingMode) -> Self {
        Opcode {
            code,
            name,
            cycles,
            len,
            mode,
        }
    }
}

lazy_static! {
    pub static ref CPU_OP_CODES: Vec<Opcode> = vec![
        Opcode::new(0x29, "AND", 2, 2, AddressingMode::Immediate),
        Opcode::new(0x25, "AND", 2, 3, AddressingMode::ZeroPage),
        Opcode::new(0x35, "AND", 2, 4, AddressingMode::ZeroPageX),
        Opcode::new(0x2D, "AND", 3, 4, AddressingMode::Absolute),
        Opcode::new(0x3D, "AND", 3, 4, AddressingMode::AbsoluteX),
        Opcode::new(0x39, "AND", 3, 4, AddressingMode::AbsoluteY),
        Opcode::new(0x21, "AND", 2, 6, AddressingMode::IndirectX),
        Opcode::new(0x31, "AND", 2, 5, AddressingMode::IndirectY),
        Opcode::new(0x90, "BCC", 2, 2, AddressingMode::NoneAddressing),
        Opcode::new(0x00, "BRK", 1, 7, AddressingMode::NoneAddressing),
        Opcode::new(0xAA, "TAX", 1, 2, AddressingMode::NoneAddressing),
        Opcode::new(0xE8, "INX", 1, 2, AddressingMode::NoneAddressing),
        Opcode::new(0xA9, "LDA", 2, 2, AddressingMode::Immediate),
        Opcode::new(0xA5, "LDA", 2, 3, AddressingMode::ZeroPage),
        Opcode::new(0xB5, "LDA", 2, 4, AddressingMode::ZeroPageX),
        Opcode::new(0xAD, "LDA", 3, 4, AddressingMode::Absolute),
        Opcode::new(0xBD, "LDA", 3, 4, AddressingMode::AbsoluteX),
        Opcode::new(0xB9, "LDA", 3, 4, AddressingMode::AbsoluteY),
        Opcode::new(0xA1, "LDA", 2, 6, AddressingMode::IndirectX),
        Opcode::new(0xB1, "LDA", 2, 5, AddressingMode::IndirectY),
        Opcode::new(0x85, "STA", 2, 3, AddressingMode::ZeroPage),
        Opcode::new(0x95, "STA", 2, 4, AddressingMode::ZeroPageX),
        Opcode::new(0x8D, "STA", 3, 4, AddressingMode::Absolute),
        Opcode::new(0x9D, "STA", 3, 5, AddressingMode::AbsoluteX),
        Opcode::new(0x99, "STA", 3, 5, AddressingMode::AbsoluteY),
        Opcode::new(0x81, "STA", 2, 6, AddressingMode::IndirectX),
        Opcode::new(0x91, "STA", 2, 6, AddressingMode::IndirectY),
    ];
    pub static ref OPCODE_MAP: HashMap<u8, &'static Opcode> = {
        let mut map = HashMap::new();

        for op in &*CPU_OP_CODES {
            map.insert(op.code, op);
        }

        map
    };
}
