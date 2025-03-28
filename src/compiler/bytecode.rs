use crate::object::Object;
use std::fmt;

/// Bytecode structure containing compiled code
#[derive(Debug, Clone)]
pub struct Bytecode {
    pub instructions: Instructions,
    pub constants: Vec<crate::object::Object>,
}

/// Bytecode Instructions
pub type Instructions = Vec<u8>;

/// Bytecode operation codes
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Opcode {
    Invalid = 0x00,
    Constant,   // 0x01: Load constant
    Add,        // 0x02: Add
    Sub,        // 0x03: Subtract
    Mul,        // 0x04: Multiply
    Div,        // 0x05: Divide
    True,       // 0x06: Push true
    False,      // 0x07: Push false
    Equal,      // 0x08: Equal
    NotEqual,   // 0x09: Not equal
    GreaterThan,// 0x0A: Greater than
    Minus,      // 0x0B: Negate
    Bang,       // 0x0C: Logical NOT
    JumpNotTruthy,// 0x0D: Jump if not truthy
    Jump,       // 0x0E: Jump
    Null,       // 0x0F: Push null
    SetGlobal,  // 0x10: Set global
    GetGlobal,  // 0x11: Get global
    Array,      // 0x12: Create array
    Hash,       // 0x13: Create hash
    Index,      // 0x14: Index operation
    Call,       // 0x15: Call function
    ReturnValue,// 0x16: Return with value
    Return,     // 0x17: Return without value
    SetLocal,   // 0x18: Set local
    GetLocal,   // 0x19: Get local
    GetBuiltin, // 0x1A: Get builtin
    Closure,    // 0x1B: Create closure
    GetFree,    // 0x1C: Get free variable
    Pop,        // 0x1D: Pop from stack
    ForLoop,    // 0x1E: For loop
    Switch,     // 0x1F: Switch statement
    Case,       // 0x20: Case statement
    VariadicCall, // 0x21: Variadic function call
    Try,        // 0x22: Try block
    Catch,      // 0x23: Catch block
    CurrentClosure, // 0x24: Get current closure
    GetField,   // 0x25: Get field
    SetField,   // 0x26: Set field
    Method,     // 0x27: Method definition
    Class,      // 0x28: Class definition
    Instance,   // 0x29: Create instance
    InvokeMethod, // 0x2A: Invoke method
    InvokeSuper,  // 0x2B: Invoke super method
    Inherit,    // 0x2C: Inherit from superclass
    DefineMethod, // 0x2D: Define method
    LessThan,    // 0x2E: Less than
    GreaterThanEqual, // 0x2F: Greater than or equal
    LessThanEqual,    // 0x30: Less than or equal
    Modulo,           // 0x31: Modulo
    Dup,              // 0x32: Duplicate top stack value
}

impl From<u8> for Opcode {
    fn from(byte: u8) -> Self {
        match byte {
            0x00 => Opcode::Invalid,
            0x01 => Opcode::Constant,
            0x02 => Opcode::Add,
            0x03 => Opcode::Sub,
            0x04 => Opcode::Mul,
            0x05 => Opcode::Div,
            0x06 => Opcode::True,
            0x07 => Opcode::False,
            0x08 => Opcode::Equal,
            0x09 => Opcode::NotEqual,
            0x0A => Opcode::GreaterThan,
            0x0B => Opcode::Minus,
            0x0C => Opcode::Bang,
            0x0D => Opcode::JumpNotTruthy,
            0x0E => Opcode::Jump,
            0x0F => Opcode::Null,
            0x10 => Opcode::SetGlobal,
            0x11 => Opcode::GetGlobal,
            0x12 => Opcode::Array,
            0x13 => Opcode::Hash,
            0x14 => Opcode::Index,
            0x15 => Opcode::Call,
            0x16 => Opcode::ReturnValue,
            0x17 => Opcode::Return,
            0x18 => Opcode::SetLocal,
            0x19 => Opcode::GetLocal,
            0x1A => Opcode::GetBuiltin,
            0x1B => Opcode::Closure,
            0x1C => Opcode::GetFree,
            0x1D => Opcode::Pop,
            0x1E => Opcode::ForLoop,
            0x1F => Opcode::Switch,
            0x20 => Opcode::Case,
            0x21 => Opcode::VariadicCall,
            0x22 => Opcode::Try,
            0x23 => Opcode::Catch,
            0x24 => Opcode::CurrentClosure,
            0x25 => Opcode::GetField,
            0x26 => Opcode::SetField,
            0x27 => Opcode::Method,
            0x28 => Opcode::Class,
            0x29 => Opcode::Instance,
            0x2A => Opcode::InvokeMethod,
            0x2B => Opcode::InvokeSuper,
            0x2C => Opcode::Inherit,
            0x2D => Opcode::DefineMethod,
            0x2E => Opcode::LessThan,
            0x2F => Opcode::GreaterThanEqual,
            0x30 => Opcode::LessThanEqual,
            0x31 => Opcode::Modulo,
            0x32 => Opcode::Dup,
            _ => panic!("Unknown opcode: {}", byte),
        }
    }
}

impl From<Opcode> for u8 {
    fn from(op: Opcode) -> Self {
        match op {
            Opcode::Invalid => 0x00,
            Opcode::Constant => 0x01,
            Opcode::Add => 0x02,
            Opcode::Sub => 0x03,
            Opcode::Mul => 0x04,
            Opcode::Div => 0x05,
            Opcode::True => 0x06,
            Opcode::False => 0x07,
            Opcode::Equal => 0x08,
            Opcode::NotEqual => 0x09,
            Opcode::GreaterThan => 0x0A,
            Opcode::Minus => 0x0B,
            Opcode::Bang => 0x0C,
            Opcode::JumpNotTruthy => 0x0D,
            Opcode::Jump => 0x0E,
            Opcode::Null => 0x0F,
            Opcode::SetGlobal => 0x10,
            Opcode::GetGlobal => 0x11,
            Opcode::Array => 0x12,
            Opcode::Hash => 0x13,
            Opcode::Index => 0x14,
            Opcode::Call => 0x15,
            Opcode::ReturnValue => 0x16,
            Opcode::Return => 0x17,
            Opcode::SetLocal => 0x18,
            Opcode::GetLocal => 0x19,
            Opcode::GetBuiltin => 0x1A,
            Opcode::Closure => 0x1B,
            Opcode::GetFree => 0x1C,
            Opcode::Pop => 0x1D,
            Opcode::ForLoop => 0x1E,
            Opcode::Switch => 0x1F,
            Opcode::Case => 0x20,
            Opcode::VariadicCall => 0x21,
            Opcode::Try => 0x22,
            Opcode::Catch => 0x23,
            Opcode::CurrentClosure => 0x24,
            Opcode::GetField => 0x25,
            Opcode::SetField => 0x26,
            Opcode::Method => 0x27,
            Opcode::Class => 0x28,
            Opcode::Instance => 0x29,
            Opcode::InvokeMethod => 0x2A,
            Opcode::InvokeSuper => 0x2B,
            Opcode::Inherit => 0x2C,
            Opcode::DefineMethod => 0x2D,
            Opcode::LessThan => 0x2E,
            Opcode::GreaterThanEqual => 0x2F,
            Opcode::LessThanEqual => 0x30,
            Opcode::Modulo => 0x31,
            Opcode::Dup => 0x32,
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
        Opcode::ForLoop => Definition {
            name: "ForLoop",
            operand_widths: vec![2], // 2-byte loop start address
        },
        Opcode::Switch => Definition {
            name: "Switch",
            operand_widths: vec![2], // 2-byte switch value
        },
        Opcode::Case => Definition {
            name: "Case",
            operand_widths: vec![2], // 2-byte case value
        },
        Opcode::VariadicCall => Definition {
            name: "VariadicCall",
            operand_widths: vec![1], // 1-byte arg count
        },
        Opcode::Try => Definition {
            name: "Try",
            operand_widths: vec![],
        },
        Opcode::Catch => Definition {
            name: "Catch",
            operand_widths: vec![2], // 2-byte catch block address
        },
        Opcode::CurrentClosure => Definition {
            name: "CurrentClosure",
            operand_widths: vec![],
        },
        Opcode::GetField => Definition {
            name: "GetField",
            operand_widths: vec![2], // 2-byte field index
        },
        Opcode::SetField => Definition {
            name: "SetField",
            operand_widths: vec![2], // 2-byte field index
        },
        Opcode::Method => Definition {
            name: "Method",
            operand_widths: vec![2], // 2-byte method index
        },
        Opcode::Class => Definition {
            name: "Class",
            operand_widths: vec![2], // 2-byte class index
        },
        Opcode::Instance => Definition {
            name: "Instance",
            operand_widths: vec![2], // 2-byte instance index
        },
        Opcode::InvokeMethod => Definition {
            name: "InvokeMethod",
            operand_widths: vec![2], // 2-byte method index
        },
        Opcode::InvokeSuper => Definition {
            name: "InvokeSuper",
            operand_widths: vec![2], // 2-byte superclass index
        },
        Opcode::Inherit => Definition {
            name: "Inherit",
            operand_widths: vec![2], // 2-byte superclass index
        },
        Opcode::DefineMethod => Definition {
            name: "DefineMethod",
            operand_widths: vec![2], // 2-byte method index
        },
        Opcode::LessThan => Definition {
            name: "LessThan",
            operand_widths: vec![],
        },
        Opcode::GreaterThanEqual => Definition {
            name: "GreaterThanEqual",
            operand_widths: vec![],
        },
        Opcode::LessThanEqual => Definition {
            name: "LessThanEqual",
            operand_widths: vec![],
        },
        Opcode::Modulo => Definition {
            name: "Modulo",
            operand_widths: vec![],
        },
        Opcode::Dup => Definition {
            name: "Dup",
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

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    // Strategy to generate valid opcodes
    fn opcode_strategy() -> impl Strategy<Value = Opcode> {
        (0..=0x32u8).prop_map(|b| {
            // Safety: We ensure we don't go beyond the known opcodes
            if b <= 0x32 {
                Opcode::from(b)
            } else {
                Opcode::Invalid
            }
        })
    }

    // Strategy to generate valid bytecode instructions
    fn instruction_strategy() -> impl Strategy<Value = (Opcode, Vec<usize>)> {
        opcode_strategy().prop_flat_map(|op| {
            let def = lookup(op);
            let operand_strat = match def.operand_widths.len() {
                0 => proptest::collection::vec(0usize, 0..0).boxed(),
                1 => {
                    if def.operand_widths[0] == 1 {
                        proptest::collection::vec(0..=255usize, 1).boxed()
                    } else {
                        proptest::collection::vec(0..=65535usize, 1).boxed()
                    }
                }
                2 => {
                    let first = if def.operand_widths[0] == 1 {
                        0..=255usize
                    } else {
                        0..=65535usize
                    };
                    let second = if def.operand_widths[1] == 1 {
                        0..=255usize
                    } else {
                        0..=65535usize
                    };
                    (first, second).prop_map(|(a, b)| vec![a, b]).boxed()
                }
                _ => proptest::collection::vec(0usize, 0..0).boxed(),
            };
            operand_strat.prop_map(move |operands| (op, operands))
        })
    }

    proptest! {
        // Test that instruction encoding and decoding are inverses
        #[test]
        fn test_make_and_read_operands_are_inverse(
            (op, operands) in instruction_strategy()
        ) {
            let instruction = make(op, &operands);
            let def = lookup(op);
            
            // Skip if the definition doesn't match our generated operands
            if def.operand_widths.len() == operands.len() {
                let (decoded_operands, _) = read_operand(&def, &instruction, 0);
                prop_assert_eq!(operands, decoded_operands);
            }
        }

        // Test that opcode conversion is bijective
        #[test]
        fn test_opcode_conversion_bijective(op in opcode_strategy()) {
            let byte: u8 = op.into();
            let converted_back = Opcode::from(byte);
            prop_assert_eq!(op, converted_back);
        }

        // Test that all instructions have valid definitions
        #[test]
        fn test_all_opcodes_have_valid_definitions(op in opcode_strategy()) {
            let def = lookup(op);
            prop_assert!(!def.name.is_empty(), "Opcode {:?} has no name", op);
            
            // Definitions should have consistent operand widths
            for width in &def.operand_widths {
                prop_assert!(*width == 1 || *width == 2, 
                    "Opcode {:?} has invalid operand width {}", op, width);
            }
        }
        
        // Test that instruction lengths are correct
        #[test]
        fn test_instruction_length_is_correct(op in opcode_strategy(), operand in 0usize..65535usize) {
            let def = lookup(op);
            
            // Skip if the opcode doesn't take operands
            if !def.operand_widths.is_empty() {
                let instruction = make(op, &[operand]);
                let expected_len = 1 + def.operand_widths.iter().sum::<usize>();
                
                prop_assert_eq!(expected_len, instruction.len());
            }
        }
        
        // Test that multiple operands are encoded/decoded correctly
        #[test]
        fn test_multi_operand_encoding_decoding(operand1 in 0usize..65535usize, operand2 in 0usize..255usize) {
            // We'll test with opcodes that can take multiple operands (like Closure)
            let op = Opcode::Closure; // This opcode has 2 operands
            let def = lookup(op);
            
            if def.operand_widths.len() == 2 {
                let instruction = make(op, &[operand1, operand2]);
                let (operands, _) = read_operand(&def, &instruction, 0);
                
                prop_assert_eq!(operand1, operands[0]);
                prop_assert_eq!(operand2, operands[1]);
            }
        }
    }
    
    #[test]
    fn test_make_simple() {
        let instructions = make(Opcode::Constant, &[1]);
        assert_eq!(vec![Opcode::Constant as u8, 0, 1], instructions);
        
        let instructions = make(Opcode::Add, &[]);
        assert_eq!(vec![Opcode::Add as u8], instructions);
    }
    
    #[test]
    fn test_read_operand() {
        let instructions = vec![Opcode::Constant as u8, 0, 1];
        let def = lookup(Opcode::Constant);
        let (operands, bytes_read) = read_operand(&def, &instructions, 0);
        
        assert_eq!(vec![1], operands);
        assert_eq!(2, bytes_read);
    }
} 