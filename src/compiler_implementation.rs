// CURSED Compiler Implementation
// This file contains the enhanced compiler implementation as specified in the CURSED language specs.
// It is kept separate from the main codebase until all dependencies are ready for integration.

use crate::ast::{self, Program, Statement, Expression};
use crate::error::{Error, ErrorReporter, SourceLocation};
use crate::object::Object;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

// Re-export from symbol table
pub use crate::symbol::{Symbol, SymbolScope, SymbolTable};

/// Bytecode instruction type
pub type Instructions = Vec<u8>;

/// Bytecode operation codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    Invalid = 0x00,
    Constant = 0x01,   // Load constant
    Add = 0x02,        // Add
    Sub = 0x03,        // Subtract
    Mul = 0x04,        // Multiply
    Div = 0x05,        // Divide
    True = 0x06,       // Push true
    False = 0x07,      // Push false
    Equal = 0x08,      // Equal
    NotEqual = 0x09,   // Not equal
    GreaterThan = 0x0A,// Greater than
    Minus = 0x0B,      // Negate
    Bang = 0x0C,       // Logical NOT
    JumpNotTruthy = 0x0D,// Jump if not truthy
    Jump = 0x0E,       // Jump
    Null = 0x0F,       // Push null
    SetGlobal = 0x10,  // Set global
    GetGlobal = 0x11,  // Get global
    Array = 0x12,      // Create array
    Hash = 0x13,       // Create hash
    Index = 0x14,      // Index operation
    Call = 0x15,       // Call function
    ReturnValue = 0x16,// Return with value
    Return = 0x17,     // Return without value
    SetLocal = 0x18,   // Set local
    GetLocal = 0x19,   // Get local
    GetBuiltin = 0x1A, // Get builtin
    Closure = 0x1B,    // Create closure
    GetFree = 0x1C,    // Get free variable
    Pop = 0x1D,        // Pop from stack
    CurrentClosure = 0x1E, // Get current closure
    GreaterThanEqual = 0x1F, // Greater than or equal
    LessThan = 0x20,    // Less than
    LessThanEqual = 0x21,    // Less than or equal
    Modulo = 0x22,           // Modulo
    Dup = 0x23,              // Duplicate top stack value
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
            0x1E => Opcode::CurrentClosure,
            0x1F => Opcode::GreaterThanEqual,
            0x20 => Opcode::LessThan,
            0x21 => Opcode::LessThanEqual,
            0x22 => Opcode::Modulo,
            0x23 => Opcode::Dup,
            _ => Opcode::Invalid,
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
            operand_widths: vec![2],
        },
        Opcode::Jump => Definition {
            name: "Jump",
            operand_widths: vec![2],
        },
        Opcode::Null => Definition {
            name: "Null",
            operand_widths: vec![],
        },
        Opcode::SetGlobal => Definition {
            name: "SetGlobal",
            operand_widths: vec![2],
        },
        Opcode::GetGlobal => Definition {
            name: "GetGlobal",
            operand_widths: vec![2],
        },
        Opcode::SetLocal => Definition {
            name: "SetLocal",
            operand_widths: vec![1],
        },
        Opcode::GetLocal => Definition {
            name: "GetLocal",
            operand_widths: vec![1],
        },
        Opcode::GetBuiltin => Definition {
            name: "GetBuiltin",
            operand_widths: vec![1],
        },
        Opcode::GetFree => Definition {
            name: "GetFree",
            operand_widths: vec![1],
        },
        Opcode::CurrentClosure => Definition {
            name: "CurrentClosure",
            operand_widths: vec![],
        },
        Opcode::Array => Definition {
            name: "Array",
            operand_widths: vec![2],
        },
        Opcode::Hash => Definition {
            name: "Hash",
            operand_widths: vec![2],
        },
        Opcode::Index => Definition {
            name: "Index",
            operand_widths: vec![],
        },
        Opcode::Call => Definition {
            name: "Call",
            operand_widths: vec![1],
        },
        Opcode::ReturnValue => Definition {
            name: "ReturnValue",
            operand_widths: vec![],
        },
        Opcode::Return => Definition {
            name: "Return",
            operand_widths: vec![],
        },
        Opcode::Closure => Definition {
            name: "Closure",
            operand_widths: vec![2, 1], // constant index, free var count
        },
        Opcode::Pop => Definition {
            name: "Pop",
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
        _ => Definition {
            name: "Invalid",
            operand_widths: vec![],
        },
    }
}

/// Make bytecode from opcode and operands
pub fn make(op: Opcode, operands: &[usize]) -> Instructions {
    let def = lookup(op);
    let instruction_len = 1 + def.operand_widths.iter().sum::<usize>();
    let mut instruction = Vec::with_capacity(instruction_len);
    
    instruction.push(op as u8);
    
    for (i, width) in def.operand_widths.iter().enumerate() {
        match width {
            2 => {
                // The operand is 2 bytes (16 bits), so we need to get the big-endian representation
                let operand = operands[i];
                if operand > u16::MAX as usize {
                    panic!("Operand {} too large: {}", i, operand);
                }
                let bytes = (operand as u16).to_be_bytes();
                instruction.extend_from_slice(&bytes);
            },
            1 => {
                // The operand is 1 byte (8 bits)
                let operand = operands[i];
                if operand > u8::MAX as usize {
                    panic!("Operand {} too large: {}", i, operand);
                }
                instruction.push(operand as u8);
            },
            _ => panic!("Unsupported operand width: {}", width),
        }
    }
    
    instruction
}

/// Read operands from bytecode
pub fn read_operand(def: &Definition, instructions: &[u8], pos: usize) -> (Vec<usize>, usize) {
    let mut operands = Vec::with_capacity(def.operand_widths.len());
    let mut offset = pos + 1;
    
    for width in &def.operand_widths {
        match width {
            2 => {
                let operand = (instructions[offset] as usize) << 8 | (instructions[offset + 1] as usize);
                operands.push(operand);
                offset += 2;
            },
            1 => {
                let operand = instructions[offset] as usize;
                operands.push(operand);
                offset += 1;
            },
            _ => panic!("Unsupported operand width: {}", width),
        }
    }
    
    (operands, offset)
}

/// Bytecode is the output of the compiler
#[derive(Debug, Clone)]
pub struct Bytecode {
    pub instructions: Instructions,
    pub constants: Vec<Object>,
}

/// A compiled function
#[derive(Debug, Clone, PartialEq)]
pub struct CompiledFunction {
    pub instructions: Vec<u8>,
    pub num_locals: usize,
    pub num_parameters: usize,
    pub free_variables: Vec<Object>,
    pub name: Option<String>,
}

/// An emitted instruction with opcode and position
#[derive(Debug, Clone)]
struct EmittedInstruction {
    opcode: Opcode,
    position: usize,
}

impl Default for EmittedInstruction {
    fn default() -> Self {
        Self {
            opcode: Opcode::Invalid,
            position: 0,
        }
    }
}

/// An instruction in the bytecode
#[derive(Debug, Clone)]
pub struct Instruction {
    pub opcode: Opcode,
    pub operands: Vec<u16>,
}

impl Instruction {
    pub fn new(opcode: Opcode, operands: Vec<u16>) -> Self {
        Self { opcode, operands }
    }
}

/// A compilation scope
#[derive(Debug, Clone)]
pub struct CompilationScope {
    pub instructions: Instructions,
    pub last_instruction: Instruction,
    pub previous_instruction: Instruction,
}

impl Default for CompilationScope {
    fn default() -> Self {
        Self {
            instructions: Vec::new(),
            last_instruction: Instruction::new(Opcode::Invalid, Vec::new()),
            previous_instruction: Instruction::new(Opcode::Invalid, Vec::new()),
        }
    }
}

/// The CURSED compiler
pub struct Compiler {
    constants: Vec<Object>,
    symbol_table: Rc<RefCell<SymbolTable>>,
    scopes: Vec<CompilationScope>,
    scope_index: usize,
}

impl Compiler {
    /// Create a new compiler
    pub fn new() -> Self {
        let symbol_table = Rc::new(RefCell::new(SymbolTable::new()));
        
        Self {
            constants: Vec::new(),
            symbol_table,
            scopes: vec![CompilationScope::default()],
            scope_index: 0,
        }
    }
    
    /// Create a compiler with a symbol table
    pub fn with_state(symbol_table: SymbolTable, constants: Vec<Object>) -> Self {
        let mut compiler = Self::new();
        compiler.symbol_table = Rc::new(RefCell::new(symbol_table));
        compiler.constants = constants;
        compiler
    }
    
    /// Compile a program into bytecode
    pub fn compile(&mut self, program: &Program) -> Result<Bytecode, Error> {
        // Define built-in functions in the symbol table
        Self::define_builtins(&mut self.symbol_table.borrow_mut());
        
        // Compile each statement in the program
        for stmt in &program.statements {
            self.compile_statement(&**stmt)?;
        }
        
        Ok(self.bytecode())
    }
    
    /// Initialize the compiler with built-in functions
    fn define_builtins(symbol_table: &mut SymbolTable) {
        // Define built-in functions in the symbol table
        let builtins = vec![
            "len", "first", "last", "rest", "push", "puts", "print",
            "type", "input", "read_file", "write_file", "append_file",
            "delete_file", "exists", "is_dir", "list_dir", "make_dir",
        ];
        
        for (i, name) in builtins.iter().enumerate() {
            symbol_table.define_builtin(i, name);
        }
    }
    
    /// Compile a statement
    pub fn compile_statement(&mut self, stmt: &dyn Statement) -> Result<(), Error> {
        if let Some(expr_stmt) = stmt.as_any().downcast_ref::<ast::ExpressionStatement>() {
            // Compile expression statements
            if let Some(expr) = &expr_stmt.expression {
                self.compile_expression(&**expr)?;
                self.emit(Opcode::Pop, vec![]);
                Ok(())
            } else {
                // Empty expression statement
                Ok(())
            }
        } else if let Some(let_stmt) = stmt.as_any().downcast_ref::<ast::LetStatement>() {
            // Compile let statements (variable declarations)
            if let Some(value) = &let_stmt.value {
                // Compile the expression for the value
                self.compile_expression(&**value)?;
            } else {
                // If no value is provided, use null as default
                self.emit(Opcode::Null, vec![]);
            }

            // Define the symbol in the symbol table
            let symbol = self.symbol_table.borrow_mut().define(&let_stmt.name.value);

            // Emit the right opcode based on scope
            match symbol.scope {
                SymbolScope::Global => {
                    self.emit(Opcode::SetGlobal, vec![symbol.index]);
                }
                SymbolScope::Local => {
                    self.emit(Opcode::SetLocal, vec![symbol.index]);
                }
                _ => {
                    return Err(Error::from_str(
                        "Cannot define variable in this scope",
                        None,
                    ));
                }
            }
            Ok(())
        } else if let Some(return_stmt) = stmt.as_any().downcast_ref::<ast::ReturnStatement>() {
            // Compile return statements
            if let Some(return_value) = &return_stmt.return_value {
                self.compile_expression(&**return_value)?;
                self.emit(Opcode::ReturnValue, vec![]);
            } else {
                // Return without a value (return null)
                self.emit(Opcode::Return, vec![]);
            }
            Ok(())
        } else if let Some(if_stmt) = stmt.as_any().downcast_ref::<ast::IfStatement>() {
            // Compile if statements
            self.compile_if_statement(if_stmt)
        } else if let Some(block_stmt) = stmt.as_any().downcast_ref::<ast::BlockStatement>() {
            // Compile block statements
            self.compile_block_statement(block_stmt)
        } else {
            // Unknown statement type
            Err(Error::from_str(
                &format!("Unknown statement type: {}", stmt.string()),
                None,
            ))
        }
    }

    /// Compile an if statement
    pub fn compile_if_statement(&mut self, if_stmt: &ast::IfStatement) -> Result<(), Error> {
        // Compile the condition expression
        self.compile_expression(&*if_stmt.condition)?;
        
        // Emit a jump-if-not-truthy instruction.
        // We don't know the jump offset yet, so use a placeholder
        let jump_not_truthy_pos = self.emit(Opcode::JumpNotTruthy, vec![9999]);
        
        // Compile the consequence block
        self.compile_block_statement(&if_stmt.consequence)?;
        
        // If we have a consequence with no return, we need to jump over the alternative
        let jump_pos = self.emit(Opcode::Jump, vec![9999]);
        
        // Update the JumpNotTruthy instruction with the correct offset
        let after_consequence_pos = self.current_instructions().len();
        self.change_operand(jump_not_truthy_pos, after_consequence_pos);
        
        // If there's an alternative (else branch), compile it
        if let Some(alternative) = &if_stmt.alternative {
            self.compile_block_statement(alternative)?;
        } else {
            // If no alternative, emit a null value
            self.emit(Opcode::Null, vec![]);
        }
        
        // Update the Jump instruction with the correct offset
        let after_alternative_pos = self.current_instructions().len();
        self.change_operand(jump_pos, after_alternative_pos);
        
        Ok(())
    }
    
    /// Compile a block statement
    pub fn compile_block_statement(&mut self, block: &ast::BlockStatement) -> Result<(), Error> {
        for stmt in &block.statements {
            self.compile_statement(&**stmt)?;
        }
        Ok(())
    }
    
    /// Compile an expression
    pub fn compile_expression(&mut self, expr: &dyn Expression) -> Result<(), Error> {
        // Use dynamic dispatch to handle each expression type
        if let Some(integer_literal) = expr.as_any().downcast_ref::<ast::IntegerLiteral>() {
            // Compile integer literals
            let value = Object::Integer(integer_literal.value);
            let constant_index = self.add_constant(value);
            self.emit(Opcode::Constant, vec![constant_index]);
            Ok(())
        } else if let Some(float_literal) = expr.as_any().downcast_ref::<ast::FloatLiteral>() {
            // Compile float literals
            let value = Object::Float(float_literal.value);
            let constant_index = self.add_constant(value);
            self.emit(Opcode::Constant, vec![constant_index]);
            Ok(())
        } else if let Some(string_literal) = expr.as_any().downcast_ref::<ast::StringLiteral>() {
            // Compile string literals
            let value = Object::String(string_literal.value.clone());
            let constant_index = self.add_constant(value);
            self.emit(Opcode::Constant, vec![constant_index]);
            Ok(())
        } else if let Some(boolean_literal) = expr.as_any().downcast_ref::<ast::BooleanLiteral>() {
            // Compile boolean literals
            if boolean_literal.value {
                self.emit(Opcode::True, vec![]);
            } else {
                self.emit(Opcode::False, vec![]);
            }
            Ok(())
        } else if let Some(identifier) = expr.as_any().downcast_ref::<ast::Identifier>() {
            // Compile identifiers (variable references)
            let symbol = match self.symbol_table.borrow_mut().resolve(&identifier.value) {
                Some(symbol) => symbol,
                None => {
                    return Err(Error::from_str(
                        &format!("Undefined identifier: {}", identifier.value),
                        None,
                    ));
                }
            };

            match symbol.scope {
                SymbolScope::Global => {
                    self.emit(Opcode::GetGlobal, vec![symbol.index]);
                }
                SymbolScope::Local => {
                    self.emit(Opcode::GetLocal, vec![symbol.index]);
                }
                SymbolScope::Builtin => {
                    self.emit(Opcode::GetBuiltin, vec![symbol.index]);
                }
                SymbolScope::Free => {
                    self.emit(Opcode::GetFree, vec![symbol.index]);
                }
                SymbolScope::Function => {
                    self.emit(Opcode::CurrentClosure, vec![]);
                }
            }
            Ok(())
        } else if let Some(prefix_expr) = expr.as_any().downcast_ref::<ast::PrefixExpression>() {
            // Compile prefix expressions (like "!x" or "-y")
            self.compile_expression(&*prefix_expr.right)?;

            match prefix_expr.operator.as_str() {
                "!" => {
                    self.emit(Opcode::Bang, vec![]);
                }
                "-" => {
                    self.emit(Opcode::Minus, vec![]);
                }
                _ => {
                    return Err(Error::from_str(
                        &format!("Unknown prefix operator: {}", prefix_expr.operator),
                        None,
                    ));
                }
            }
            Ok(())
        } else if let Some(infix_expr) = expr.as_any().downcast_ref::<ast::InfixExpression>() {
            // For certain operators like < and >, we need to compile them in the correct order
            if infix_expr.operator == "<" {
                // For x < y, compile y then x then use GreaterThan (flipped order)
                self.compile_expression(&*infix_expr.right)?;
                self.compile_expression(&*infix_expr.left)?;
                self.emit(Opcode::GreaterThan, vec![]);
                return Ok(());
            } else if infix_expr.operator == "<=" {
                // For x <= y, compile y then x then use GreaterThanEqual (flipped order)
                self.compile_expression(&*infix_expr.right)?;
                self.compile_expression(&*infix_expr.left)?;
                self.emit(Opcode::GreaterThanEqual, vec![]);
                return Ok(());
            }

            // For normal order operators
            self.compile_expression(&*infix_expr.left)?;
            self.compile_expression(&*infix_expr.right)?;

            match infix_expr.operator.as_str() {
                "+" => {
                    self.emit(Opcode::Add, vec![]);
                }
                "-" => {
                    self.emit(Opcode::Sub, vec![]);
                }
                "*" => {
                    self.emit(Opcode::Mul, vec![]);
                }
                "/" => {
                    self.emit(Opcode::Div, vec![]);
                }
                "%" => {
                    self.emit(Opcode::Modulo, vec![]);
                }
                ">" => {
                    self.emit(Opcode::GreaterThan, vec![]);
                }
                ">=" => {
                    self.emit(Opcode::GreaterThanEqual, vec![]);
                }
                "==" => {
                    self.emit(Opcode::Equal, vec![]);
                }
                "!=" => {
                    self.emit(Opcode::NotEqual, vec![]);
                }
                _ => {
                    return Err(Error::from_str(
                        &format!("Unknown infix operator: {}", infix_expr.operator),
                        None,
                    ));
                }
            }
            Ok(())
        } else if let Some(index_expr) = expr.as_any().downcast_ref::<ast::IndexExpression>() {
            // Compile index expressions like array[index] or hash[key]
            self.compile_expression(&*index_expr.left)?;
            self.compile_expression(&*index_expr.index)?;
            self.emit(Opcode::Index, vec![]);
            Ok(())
        } else if let Some(call_expr) = expr.as_any().downcast_ref::<ast::CallExpression>() {
            // Compile function calls
            self.compile_expression(&*call_expr.function)?;
            
            // Compile each argument
            for arg in &call_expr.arguments {
                self.compile_expression(&**arg)?;
            }
            
            // Emit call instruction with argument count
            self.emit(Opcode::Call, vec![call_expr.arguments.len()]);
            Ok(())
        } else if let Some(function_literal) = expr.as_any().downcast_ref::<ast::FunctionLiteral>() {
            // Compile function literals (anonymous functions or function declarations)
            self.compile_function_literal(function_literal)
        } else if let Some(array_literal) = expr.as_any().downcast_ref::<ast::ArrayLiteral>() {
            // Compile array literals
            for element in &array_literal.elements {
                self.compile_expression(&**element)?;
            }
            
            // Emit array instruction with element count
            self.emit(Opcode::Array, vec![array_literal.elements.len()]);
            Ok(())
        } else if let Some(hash_literal) = expr.as_any().downcast_ref::<ast::HashLiteral>() {
            // Compile hash literals
            for (key, value) in &hash_literal.pairs {
                // Compile key and value in alternating order
                self.compile_expression(&**key)?;
                self.compile_expression(&**value)?;
            }
            
            // Emit hash instruction with pair count (key-value pairs)
            self.emit(Opcode::Hash, vec![hash_literal.pairs.len() * 2]);
            Ok(())
        } else {
            Err(Error::from_str(
                &format!("Unknown expression type: {}", expr.string()),
                None,
            ))
        }
    }
    
    /// Compile a function literal
    pub fn compile_function_literal(&mut self, func: &ast::FunctionLiteral) -> Result<(), Error> {
        // Enter a new scope for the function body
        self.enter_scope();
        
        // Define parameters in the function's scope
        for param in &func.parameters {
            self.symbol_table.borrow_mut().define(&param.value);
        }
        
        // Compile the function body
        self.compile_block_statement(&func.body)?;
        
        // If the function doesn't end with a return, add an implicit return
        if self.last_instruction_is(Opcode::Pop) {
            // Replace the last Pop with ReturnValue
            self.replace_last_pop_with_return();
        }
        
        // Ensure all functions end with a return
        if !self.last_instruction_is(Opcode::ReturnValue) && !self.last_instruction_is(Opcode::Return) {
            self.emit(Opcode::Return, vec![]);
        }
        
        // Get the local binding count and free variables
        let num_locals = self.symbol_table.borrow().num_definitions;
        let free_symbols = self.symbol_table.borrow().free_symbols.clone();
        let num_params = func.parameters.len();
        
        // Capture the compiled instructions and leave the function's scope
        let instructions = self.leave_scope();
        
        // Create a compiled function object
        let function_obj = Object::CompiledFunction {
            instructions,
            num_locals,
            num_parameters: num_params,
        };
        
        // Add the function to constants
        let fn_index = self.add_constant(function_obj);
        
        // If we have free variables (for closures), handle them
        if !free_symbols.is_empty() {
            // Push the free variables onto the stack
            for free_symbol in &free_symbols {
                match free_symbol.scope {
                    SymbolScope::Global => {
                        self.emit(Opcode::GetGlobal, vec![free_symbol.index]);
                    }
                    SymbolScope::Local => {
                        self.emit(Opcode::GetLocal, vec![free_symbol.index]);
                    }
                    SymbolScope::Free => {
                        self.emit(Opcode::GetFree, vec![free_symbol.index]);
                    }
                    _ => {
                        return Err(Error::from_str(
                            &format!("Invalid symbol scope for free variable: {:?}", free_symbol.scope),
                            None,
                        ));
                    }
                }
            }
            
            // Create a closure with the free variables
            self.emit(Opcode::Closure, vec![fn_index, free_symbols.len()]);
        } else {
            // No free variables, just emit the function
            self.emit(Opcode::Constant, vec![fn_index]);
        }
        
        Ok(())
    }
    
    /// Add a constant to the constants pool
    pub fn add_constant(&mut self, obj: Object) -> usize {
        self.constants.push(obj);
        self.constants.len() - 1
    }
    
    /// Emit an instruction
    pub fn emit(&mut self, op: Opcode, operands: Vec<usize>) -> usize {
        let pos = self.current_instructions().len();
        let operands_slice: &[usize] = operands.as_slice();
        let mut ins = make(op, operands_slice);
        self.current_instructions_mut().extend(&ins);
        
        // Update the last instruction info
        self.set_last_instruction(op, pos);
        
        pos
    }
    
    /// Set the last instruction
    pub fn set_last_instruction(&mut self, op: Opcode, pos: usize) {
        // Create emitted instruction
        let emitted = EmittedInstruction {
            opcode: op,
            position: pos,
        };
        
        // Get the operands from the bytecode
        let mut operands = Vec::new();
        if let Ok(def) = read_definition(op) {
            // Extract operands based on the definition
            let (extracted, _) = read_operand(&def, self.current_instructions(), pos);
            operands = extracted.into_iter().map(|o| o as u16).collect();
        }
        
        // Update the scope's instructions
        let scope = &mut self.scopes[self.scope_index];
        scope.previous_instruction = scope.last_instruction.clone();
        scope.last_instruction = Instruction {
            opcode: op,
            operands,
        };
    }
    
    /// Read definition for an opcode
    fn read_definition(op: Opcode) -> Result<Definition, String> {
        Ok(lookup(op))
    }
    
    /// Get the current instructions
    pub fn current_instructions(&self) -> &Instructions {
        &self.scopes[self.scope_index].instructions
    }
    
    /// Get a mutable reference to the current instructions
    pub fn current_instructions_mut(&mut self) -> &mut Instructions {
        &mut self.scopes[self.scope_index].instructions
    }
    
    /// Enter a new scope
    fn enter_scope(&mut self) {
        let scope = CompilationScope::default();
        self.scopes.push(scope);
        self.scope_index += 1;
        
        // Create a new symbol table with the current one as its outer scope
        let outer = Rc::clone(&self.symbol_table);
        self.symbol_table = Rc::new(RefCell::new(SymbolTable::with_outer(outer.borrow().clone())));
    }
    
    /// Leave the current scope and return to the outer scope
    fn leave_scope(&mut self) -> Instructions {
        let instructions = self.current_instructions().clone();
        
        // Remove the current scope
        self.scopes.pop();
        self.scope_index -= 1;
        
        // Restore the outer symbol table
        if let Some(outer) = self.symbol_table.borrow_mut().outer.clone() {
            self.symbol_table = Rc::new(RefCell::new(*outer));
        }
        
        instructions
    }
    
    /// Check if the last instruction is of the given opcode
    fn last_instruction_is(&self, op: Opcode) -> bool {
        let current_scope = &self.scopes[self.scope_index];
        if current_scope.last_instruction.opcode == Opcode::Invalid {
            return false;
        }
        current_scope.last_instruction.opcode == op
    }
    
    /// Replace the last Pop instruction with a ReturnValue
    fn replace_last_pop_with_return(&mut self) {
        let last_pos = {
            let current_scope = &self.scopes[self.scope_index];
            let position = match current_scope.last_instruction.opcode {
                Opcode::Pop => current_scope.last_instruction.operands.first().map_or(0, |&op| op as usize),
                _ => 0,
            };
            position
        };
        
        // Replace the opcode at last_pos with ReturnValue
        if last_pos < self.current_instructions().len() {
            self.current_instructions_mut()[last_pos] = Opcode::ReturnValue as u8;
        }
        
        // Update the last instruction
        let current_scope = &mut self.scopes[self.scope_index];
        current_scope.last_instruction.opcode = Opcode::ReturnValue;
    }
    
    /// Change an operand at the given position
    pub fn change_operand(&mut self, op_pos: usize, operand: usize) {
        let op: Opcode = self.current_instructions()[op_pos].into();
        let new_instruction = make(op, &[operand]);
        
        // Replace the instruction at op_pos with the new one
        let instructions = self.current_instructions_mut();
        for (i, byte) in new_instruction.iter().enumerate() {
            if op_pos + i < instructions.len() {
                instructions[op_pos + i] = *byte;
            }
        }
    }
    
    /// Get the bytecode from the current compiler state
    pub fn bytecode(&self) -> Bytecode {
        Bytecode {
            instructions: self.current_instructions().clone(),
            constants: self.constants.clone(),
        }
    }
} 