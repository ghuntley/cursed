use crate::compiler::Object;
use std::fmt;

/// Bytecode representation
#[derive(Clone, Debug, PartialEq)]
pub struct Bytecode {
    pub instructions: Instructions,
    pub constants: Vec<Object>,
}

/// Bytecode Instructions
pub type Instructions = Vec<u8>;

/// Bytecode operation codes
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Opcode {
    Constant,   // 0x00: Load constant
    Add,        // 0x01: Add
    Sub,        // 0x02: Subtract
    Mul,        // 0x03: Multiply
    Div,        // 0x04: Divide
    True,       // 0x05: Push true
    False,      // 0x06: Push false
    Equal,      // 0x07: Equal
    NotEqual,   // 0x08: Not equal
    GreaterThan,// 0x09: Greater than
    Minus,      // 0x0A: Negate
    Bang,       // 0x0B: Logical NOT
    JumpNotTruthy,// 0x0C: Jump if not truthy
    Jump,       // 0x0D: Jump
    Null,       // 0x0E: Push null
    SetGlobal,  // 0x0F: Set global
    GetGlobal,  // 0x10: Get global
    Array,      // 0x11: Create array
    Hash,       // 0x12: Create hash
    Index,      // 0x13: Index operation
    Call,       // 0x14: Call function
    ReturnValue,// 0x15: Return with value
    Return,     // 0x16: Return without value
    SetLocal,   // 0x17: Set local
    GetLocal,   // 0x18: Get local
    GetBuiltin, // 0x19: Get builtin
    Closure,    // 0x1A: Create closure
    GetFree,    // 0x1B: Get free variable
    Pop,        // 0x1C: Pop from stack
}

impl From<u8> for Opcode {
    fn from(byte: u8) -> Self {
        match byte {
            0x00 => Opcode::Constant,
            0x01 => Opcode::Add,
            0x02 => Opcode::Sub,
            0x03 => Opcode::Mul,
            0x04 => Opcode::Div,
            0x05 => Opcode::True,
            0x06 => Opcode::False,
            0x07 => Opcode::Equal,
            0x08 => Opcode::NotEqual,
            0x09 => Opcode::GreaterThan,
            0x0A => Opcode::Minus,
            0x0B => Opcode::Bang,
            0x0C => Opcode::JumpNotTruthy,
            0x0D => Opcode::Jump,
            0x0E => Opcode::Null,
            0x0F => Opcode::SetGlobal,
            0x10 => Opcode::GetGlobal,
            0x11 => Opcode::Array,
            0x12 => Opcode::Hash,
            0x13 => Opcode::Index,
            0x14 => Opcode::Call,
            0x15 => Opcode::ReturnValue,
            0x16 => Opcode::Return,
            0x17 => Opcode::SetLocal,
            0x18 => Opcode::GetLocal,
            0x19 => Opcode::GetBuiltin,
            0x1A => Opcode::Closure,
            0x1B => Opcode::GetFree,
            0x1C => Opcode::Pop,
            _ => panic!("Unknown opcode: {}", byte),
        }
    }
}

impl From<Opcode> for u8 {
    fn from(op: Opcode) -> Self {
        match op {
            Opcode::Constant => 0x00,
            Opcode::Add => 0x01,
            Opcode::Sub => 0x02,
            Opcode::Mul => 0x03,
            Opcode::Div => 0x04,
            Opcode::True => 0x05,
            Opcode::False => 0x06,
            Opcode::Equal => 0x07,
            Opcode::NotEqual => 0x08,
            Opcode::GreaterThan => 0x09,
            Opcode::Minus => 0x0A,
            Opcode::Bang => 0x0B,
            Opcode::JumpNotTruthy => 0x0C,
            Opcode::Jump => 0x0D,
            Opcode::Null => 0x0E,
            Opcode::SetGlobal => 0x0F,
            Opcode::GetGlobal => 0x10,
            Opcode::Array => 0x11,
            Opcode::Hash => 0x12,
            Opcode::Index => 0x13,
            Opcode::Call => 0x14,
            Opcode::ReturnValue => 0x15,
            Opcode::Return => 0x16,
            Opcode::SetLocal => 0x17,
            Opcode::GetLocal => 0x18,
            Opcode::GetBuiltin => 0x19,
            Opcode::Closure => 0x1A,
            Opcode::GetFree => 0x1B,
            Opcode::Pop => 0x1C,
        }
    }
}

/// Bytecode definition
#[derive(Debug, Clone)]
pub struct Definition {
    pub name: &'static str,
    pub operand_widths: Vec<usize>,
}

/// Get the definition for an opcode
pub fn lookup(op: Opcode) -> Definition {
    match op {
        Opcode::Constant => Definition {
            name: "Constant",
            operand_widths: vec![2], // 2-byte operand
        },
        Opcode::Add => Definition {
            name: "Add",
            operand_widths: vec![],
        },
        Opcode::Sub => Definition {
            name: "Sub",
            operand_widths: vec![],
        },
        Opcode::Mul => Definition {
            name: "Mul",
            operand_widths: vec![],
        },
        Opcode::Div => Definition {
            name: "Div",
            operand_widths: vec![],
        },
        Opcode::True => Definition {
            name: "True",
            operand_widths: vec![],
        },
        Opcode::False => Definition {
            name: "False",
            operand_widths: vec![],
        },
        Opcode::Equal => Definition {
            name: "Equal",
            operand_widths: vec![],
        },
        Opcode::NotEqual => Definition {
            name: "NotEqual",
            operand_widths: vec![],
        },
        Opcode::GreaterThan => Definition {
            name: "GreaterThan",
            operand_widths: vec![],
        },
        Opcode::Minus => Definition {
            name: "Minus",
            operand_widths: vec![],
        },
        Opcode::Bang => Definition {
            name: "Bang",
            operand_widths: vec![],
        },
        Opcode::JumpNotTruthy => Definition {
            name: "JumpNotTruthy",
            operand_widths: vec![2], // 2-byte jump address
        },
        Opcode::Jump => Definition {
            name: "Jump",
            operand_widths: vec![2], // 2-byte jump address
        },
        Opcode::Null => Definition {
            name: "Null",
            operand_widths: vec![],
        },
        Opcode::SetGlobal => Definition {
            name: "SetGlobal",
            operand_widths: vec![2], // 2-byte symbol index
        },
        Opcode::GetGlobal => Definition {
            name: "GetGlobal",
            operand_widths: vec![2], // 2-byte symbol index
        },
        Opcode::Array => Definition {
            name: "Array",
            operand_widths: vec![2], // 2-byte element count
        },
        Opcode::Hash => Definition {
            name: "Hash",
            operand_widths: vec![2], // 2-byte element count (pairs)
        },
        Opcode::Index => Definition {
            name: "Index",
            operand_widths: vec![],
        },
        Opcode::Call => Definition {
            name: "Call",
            operand_widths: vec![1], // 1-byte arg count
        },
        Opcode::ReturnValue => Definition {
            name: "ReturnValue",
            operand_widths: vec![],
        },
        Opcode::Return => Definition {
            name: "Return",
            operand_widths: vec![],
        },
        Opcode::SetLocal => Definition {
            name: "SetLocal",
            operand_widths: vec![1], // 1-byte local index
        },
        Opcode::GetLocal => Definition {
            name: "GetLocal",
            operand_widths: vec![1], // 1-byte local index
        },
        Opcode::GetBuiltin => Definition {
            name: "GetBuiltin",
            operand_widths: vec![1], // 1-byte builtin index
        },
        Opcode::Closure => Definition {
            name: "Closure",
            operand_widths: vec![2, 1], // 2-byte constant index, 1-byte free vars count
        },
        Opcode::GetFree => Definition {
            name: "GetFree",
            operand_widths: vec![1], // 1-byte free var index
        },
        Opcode::Pop => Definition {
            name: "Pop",
            operand_widths: vec![],
        },
    }
}

/// Make a bytecode instruction
pub fn make(op: Opcode, operands: &[usize]) -> Instructions {
    let def = lookup(op);
    let instruction_len = 1 + def.operand_widths.iter().sum::<usize>();
    let mut instruction = vec![0; instruction_len];
    instruction[0] = op.into();

    let mut offset = 1;
    for (i, &operand) in operands.iter().enumerate() {
        let width = def.operand_widths[i];
        match width {
            1 => {
                instruction[offset] = operand as u8;
            }
            2 => {
                let bytes = (operand as u16).to_be_bytes();
                instruction[offset] = bytes[0];
                instruction[offset + 1] = bytes[1];
            }
            _ => panic!("Unsupported operand width: {}", width),
        }
        offset += width;
    }

    instruction
}

/// Read an operand from instructions
pub fn read_operand(def: &Definition, instructions: &[u8], offset: usize) -> (Vec<usize>, usize) {
    let mut operands = Vec::with_capacity(def.operand_widths.len());
    let mut read_offset = offset;

    for &width in &def.operand_widths {
        match width {
            1 => {
                operands.push(instructions[read_offset] as usize);
            }
            2 => {
                let value = u16::from_be_bytes([
                    instructions[read_offset],
                    instructions[read_offset + 1],
                ]) as usize;
                operands.push(value);
            }
            _ => panic!("Unsupported operand width: {}", width),
        }
        read_offset += width;
    }

    (operands, read_offset)
}

/// Format bytecode as a string for debugging
pub fn format_instructions(instructions: &Instructions) -> String {
    let mut result = String::new();
    let mut i = 0;

    while i < instructions.len() {
        let op: Opcode = instructions[i].into();
        let def = lookup(op);

        if i > 0 {
            result.push('\n');
        }

        let (operands, read) = read_operand(&def, instructions, i + 1);
        result.push_str(&format!("{:04} {}", i, def.name));

        for operand in operands {
            result.push_str(&format!(" {}", operand));
        }

        i = read;
    }

    result
}

impl Instructions {
    /// Create a new empty instructions vector
    pub fn new() -> Self {
        Vec::new()
    }
} 