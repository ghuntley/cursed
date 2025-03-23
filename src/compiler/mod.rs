mod bytecode;
mod symbol_table;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod property_tests;

pub use bytecode::{Bytecode, Instructions, Opcode};
pub use symbol_table::{Symbol, SymbolScope, SymbolTable};
use crate::ast::{self, Node, Program, Statement, Expression};
use crate::error::{Error, ErrorReporter, SourceLocation};
use crate::memory::gc::{Traceable, Visitor};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

/// The Compiler takes an AST and converts it into bytecode that can be executed by the VM
pub struct Compiler {
    instructions: Instructions,
    constants: Vec<Object>,
    symbol_table: Rc<RefCell<SymbolTable>>,
    scopes: Vec<CompilationScope>,
    scope_index: usize,
}

/// Object represents a runtime value
#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    Array(Vec<Object>),
    HashTable(HashMap<String, Object>),
    CompiledFunction(Rc<CompiledFunction>),
    Closure {
        function: Rc<CompiledFunction>,
        free_vars: Vec<Object>,
    },
    Null,
}

// Implementation of Traceable for Object to work with garbage collector
impl Traceable for Object {
    fn trace(&self, visitor: &mut dyn Visitor) {
        match self {
            Object::Array(arr) => {
                // Trace elements in array
                for obj in arr.iter() {
                    let ptr_addr = (obj as *const Object) as usize;
                    visitor.visit_ptr(ptr_addr, crate::memory::tagged::Tag::Object);
                }
            },
            Object::HashTable(map) => {
                // Trace values in hash table
                for (_, value) in map.iter() {
                    let ptr_addr = (value as *const Object) as usize;
                    visitor.visit_ptr(ptr_addr, crate::memory::tagged::Tag::Object);
                }
            },
            Object::Closure { function: _, free_vars } => {
                // Trace free variables
                for obj in free_vars.iter() {
                    let ptr_addr = (obj as *const Object) as usize;
                    visitor.visit_ptr(ptr_addr, crate::memory::tagged::Tag::Object);
                }
            },
            // Other object types don't contain references to trace
            _ => {}
        }
    }

    fn size(&self) -> usize {
        match self {
            Object::Integer(_) => std::mem::size_of::<i64>(),
            Object::Float(_) => std::mem::size_of::<f64>(),
            Object::Boolean(_) => std::mem::size_of::<bool>(),
            Object::String(s) => std::mem::size_of::<String>() + s.capacity(),
            Object::Array(arr) => {
                std::mem::size_of::<Vec<Object>>() + arr.capacity() * std::mem::size_of::<Object>()
            },
            Object::HashTable(map) => {
                let key_size = map.keys().map(|k| k.capacity()).sum::<usize>();
                let value_size = map.values().map(|v| v.size()).sum::<usize>();
                std::mem::size_of::<HashMap<String, Object>>() + key_size + value_size
            },
            Object::CompiledFunction(f) => {
                std::mem::size_of::<CompiledFunction>() + f.instructions.len()
            },
            Object::Closure { function, free_vars } => {
                let free_vars_size = free_vars.iter().map(|v| v.size()).sum::<usize>();
                std::mem::size_of::<Rc<CompiledFunction>>() + 
                std::mem::size_of::<Vec<Object>>() + 
                free_vars_size
            },
            Object::Null => 0,
        }
    }
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Integer(i) => write!(f, "{}", i),
            Object::Float(fl) => write!(f, "{}", fl),
            Object::Boolean(b) => write!(f, "{}", b),
            Object::String(s) => write!(f, "\"{}\"", s),
            Object::Array(arr) => {
                write!(f, "[")?;
                for (i, obj) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", obj)?;
                }
                write!(f, "]")
            }
            Object::HashTable(map) => {
                write!(f, "{{")?;
                let mut first = true;
                for (k, v) in map {
                    if !first {
                        write!(f, ", ")?;
                    }
                    write!(f, "\"{}\": {}", k, v)?;
                    first = false;
                }
                write!(f, "}}")
            }
            Object::CompiledFunction(func) => write!(f, "compiled function[{}]", func.instructions.len()),
            Object::Closure { function, free_vars } => write!(f, "closure[free_vars: {}]", free_vars.len()),
            Object::Null => write!(f, "cap"),
        }
    }
}

/// A compiled function
#[derive(Debug, Clone, PartialEq)]
pub struct CompiledFunction {
    /// The bytecode for the function
    pub instructions: Instructions,
    /// The number of local variables
    pub num_locals: usize,
    /// The number of parameters
    pub num_parameters: usize,
    /// Free variables for closures
    pub free_variables: Vec<Object>,
}

impl CompiledFunction {
    /// Create a new compiled function
    pub fn new(instructions: Instructions, num_locals: usize, num_parameters: usize) -> Self {
        Self {
            instructions,
            num_locals,
            num_parameters,
            free_variables: Vec::new(),
        }
    }
    
    /// Create a compiled function with free variables
    pub fn with_free_vars(
        instructions: Instructions, 
        num_locals: usize, 
        num_parameters: usize,
        free_variables: Vec<Object>
    ) -> Self {
        Self {
            instructions,
            num_locals,
            num_parameters,
            free_variables,
        }
    }
}

/// Compilation scope structure
#[derive(Default)]
struct CompilationScope {
    instructions: Instructions,
    last_instruction: Option<EmittedInstruction>,
    previous_instruction: Option<EmittedInstruction>,
}

/// Emitted instruction structure
#[derive(Clone, Debug)]
struct EmittedInstruction {
    opcode: Opcode,
    position: usize,
}

impl Compiler {
    /// Create a new compiler
    pub fn new() -> Self {
        let symbol_table = Rc::new(RefCell::new(SymbolTable::new()));
        
        Self {
            instructions: Vec::new(),
            constants: Vec::new(),
            symbol_table,
            scopes: vec![CompilationScope::new()],
            scope_index: 0,
        }
    }
    
    /// Create a new compiler with existing state
    pub fn new_with_state(symbol_table: SymbolTable, constants: Vec<Object>) -> Self {
        Self {
            instructions: Vec::new(),
            constants,
            symbol_table,
            scopes: vec![CompilationScope {
                instructions: Vec::new(),
                last_instruction: None,
                previous_instruction: None,
            }],
            scope_index: 0,
        }
    }

    /// Compile an AST program into bytecode
    pub fn compile(&mut self, program: Program) -> Result<Bytecode, Error> {
        for stmt in program.statements {
            self.compile_statement(stmt)?;
        }

        // We're compiling from the main scope
        let main_scope = &self.scopes[0];
        
        Ok(Bytecode {
            instructions: main_scope.instructions.clone(),
            constants: self.constants.clone(),
        })
    }

    /// Compile a statement
    fn compile_statement(&mut self, stmt: Box<dyn Statement>) -> Result<(), Error> {
        let location = SourceLocation::new(0, 0); // TODO: Add proper location tracking
        
        // Each statement type requires different compilation
        match stmt.token_literal().as_str() {
            "vibe" => {
                // Package declaration, skip for now
                Ok(())
            },
            "yeet" => {
                // Import declaration, skip for now
                Ok(())
            },
            "let" => {
                if let Some(let_stmt) = stmt.as_any().downcast_ref::<ast::LetStatement>() {
                    // Compile the value expression
                    if let Some(value) = &let_stmt.value {
                        self.compile_expression(value.as_ref())?;
                    } else {
                        // If no value provided, use null
                        self.emit(Opcode::Null, vec![]);
                    }
                    
                    // Define the variable in the symbol table
                    let symbol = self.symbol_table.borrow_mut().define(&let_stmt.name.value);
                    
                    // Emit the appropriate set instruction based on scope
                    if symbol.scope == symbol_table::SymbolScope::Global {
                        self.emit(Opcode::SetGlobal, vec![symbol.index]);
                    } else {
                        self.emit(Opcode::SetLocal, vec![symbol.index]);
                    }
                    
                    Ok(())
                } else {
                    Err(ErrorReporter::compilation_error(&format!(
                        "Expected LetStatement, got: {}", 
                        stmt.token_literal()
                    )))
                }
            },
            "return" => {
                if let Some(return_stmt) = stmt.as_any().downcast_ref::<ast::ReturnStatement>() {
                    // Compile the return value if present
                    if let Some(return_value) = &return_stmt.return_value {
                        self.compile_expression(return_value.as_ref())?;
                        self.emit(Opcode::ReturnValue, vec![]);
                    } else {
                        // Return null if no return value
                        self.emit(Opcode::Return, vec![]);
                    }
                    
                    Ok(())
                } else {
                    Err(ErrorReporter::compilation_error(&format!(
                        "Expected ReturnStatement, got: {}", 
                        stmt.token_literal()
                    )))
                }
            },
            "if" => {
                if let Some(if_stmt) = stmt.as_any().downcast_ref::<ast::IfStatement>() {
                    self.compile_if_statement(if_stmt)
                } else {
                    Err(ErrorReporter::compilation_error(&format!(
                        "Expected IfStatement, got: {}", 
                        stmt.token_literal()
                    )))
                }
            },
            "{" => {
                if let Some(block_stmt) = stmt.as_any().downcast_ref::<ast::BlockStatement>() {
                    self.compile_block_statement(block_stmt)
                } else {
                    Err(ErrorReporter::compilation_error(&format!(
                        "Expected BlockStatement, got: {}", 
                        stmt.token_literal()
                    )))
                }
            },
            _ => {
                // For expression statements
                if let Some(expr_stmt) = stmt.as_any().downcast_ref::<ast::ExpressionStatement>() {
                    if let Some(expr) = &expr_stmt.expression {
                        self.compile_expression(expr.as_ref())?;
                        // Pop the result if it's not used
                        self.emit(Opcode::Pop, vec![]);
                    }
                    Ok(())
                } else {
                    Err(ErrorReporter::compilation_error(&format!(
                        "Unknown statement type: {}", 
                        stmt.token_literal()
                    )))
                }
            }
        }
    }
    
    /// Compile an if statement
    fn compile_if_statement(&mut self, if_stmt: &ast::IfStatement) -> Result<(), Error> {
        // Compile the condition expression
        self.compile_expression(if_stmt.condition.as_ref())?;
        
        // Emit a jump-not-truthy with a placeholder position
        // We'll update this once we know where to jump to
        let jump_not_truthy_pos = self.emit(Opcode::JumpNotTruthy, vec![9999]);
        
        // Compile the consequence block
        self.compile_block_statement(&if_stmt.consequence)?;
        
        // If there's an else block, emit a jump over it after the consequence
        let mut jump_pos = 0;
        if if_stmt.alternative.is_some() {
            jump_pos = self.emit(Opcode::Jump, vec![9999]);
        }
        
        // Now we know where the jump_not_truthy should go
        let after_consequence_pos = self.current_instructions().len();
        self.change_operand(jump_not_truthy_pos, after_consequence_pos);
        
        // Compile the alternative block if it exists
        if let Some(alt) = &if_stmt.alternative {
            self.compile_block_statement(alt)?;
            
            // Now we know where the jump should go
            let after_alternative_pos = self.current_instructions().len();
            self.change_operand(jump_pos, after_alternative_pos);
        }
        
        Ok(())
    }
    
    /// Compile a block statement
    fn compile_block_statement(&mut self, block: &ast::BlockStatement) -> Result<(), Error> {
        for stmt in &block.statements {
            self.compile_statement(stmt.clone())?;
        }
        
        Ok(())
    }
    
    /// Change an operand at a position in the instructions
    fn change_operand(&mut self, op_pos: usize, operand: usize) {
        let op: Opcode = self.current_instructions()[op_pos].into();
        let new_instruction = bytecode::make(op, &[operand]);
        
        // Replace the instruction at op_pos with the new one
        let instructions = self.current_instructions_mut();
        for (i, byte) in new_instruction.iter().enumerate() {
            instructions[op_pos + i] = *byte;
        }
    }

    /// Compile an expression
    fn compile_expression(&mut self, expr: &dyn Expression) -> Result<(), Error> {
        let location = SourceLocation::new(0, 0); // TODO: Add proper location tracking
        
        // Handle different expression types
        if let Some(integer) = expr.as_any().downcast_ref::<ast::IntegerLiteral>() {
            let value = Object::Integer(integer.value);
            let const_index = self.add_constant(value);
            self.emit(Opcode::Constant, vec![const_index]);
            return Ok(());
        } 
        
        if let Some(float) = expr.as_any().downcast_ref::<ast::FloatLiteral>() {
            let value = Object::Float(float.value);
            let const_index = self.add_constant(value);
            self.emit(Opcode::Constant, vec![const_index]);
            return Ok(());
        } 
        
        if let Some(boolean) = expr.as_any().downcast_ref::<ast::BooleanLiteral>() {
            if boolean.value {
                self.emit(Opcode::True, vec![]);
            } else {
                self.emit(Opcode::False, vec![]);
            }
            return Ok(());
        } 
        
        if let Some(string) = expr.as_any().downcast_ref::<ast::StringLiteral>() {
            let value = Object::String(string.value.clone());
            let const_index = self.add_constant(value);
            self.emit(Opcode::Constant, vec![const_index]);
            return Ok(());
        }
        
        if let Some(ident) = expr.as_any().downcast_ref::<ast::Identifier>() {
            // Look up the identifier in the symbol table
            match self.symbol_table.borrow().resolve(&ident.value) {
                Some(symbol) => {
                    if symbol.scope.is_global() {
                        self.emit(Opcode::GetGlobal, vec![symbol.index]);
                    } else {
                        self.emit(Opcode::GetLocal, vec![symbol.index]);
                    }
                }
                None => {
                    return Err(ErrorReporter::compilation_error(&format!(
                        "Identifier not found: {}", 
                        ident.value
                    )));
                }
            }
            return Ok(());
        }
        
        // Handle prefix expressions
        if expr.is_prefix_expression() {
            if let Some((operator, right)) = expr.as_prefix_expression() {
                // Compile the right expression
                self.compile_expression(right)?;
                
                // Apply the operator
                match operator.as_str() {
                    "!" => self.emit(Opcode::Bang, vec![]),
                    "-" => self.emit(Opcode::Minus, vec![]),
                    _ => {
                        return Err(ErrorReporter::compilation_error(&format!(
                            "Unknown prefix operator: {}", 
                            operator
                        )));
                    }
                }
                
                return Ok(());
            }
        }
        
        // Handle infix expressions
        if expr.is_infix_expression() {
            if let Some((left, operator, right)) = expr.as_infix_expression() {
                // Special case for comparison operators
                // For < we flip the operands and use >
                if operator == "<" {
                    // Compile right then left
                    self.compile_expression(right)?;
                    self.compile_expression(left)?;
                    self.emit(Opcode::GreaterThan, vec![]);
                    return Ok(());
                }
                
                // For all other operators, compile left then right
                self.compile_expression(left)?;
                self.compile_expression(right)?;
                
                // Apply the operator
                match operator.as_str() {
                    "+" => self.emit(Opcode::Add, vec![]),
                    "-" => self.emit(Opcode::Sub, vec![]),
                    "*" => self.emit(Opcode::Mul, vec![]),
                    "/" => self.emit(Opcode::Div, vec![]),
                    ">" => self.emit(Opcode::GreaterThan, vec![]),
                    "==" => self.emit(Opcode::Equal, vec![]),
                    "!=" => self.emit(Opcode::NotEqual, vec![]),
                    _ => {
                        return Err(ErrorReporter::compilation_error(&format!(
                            "Unknown infix operator: {}", 
                            operator
                        )));
                    }
                }
                
                return Ok(());
            }
        }
        
        // Handle call expressions
        if expr.is_call_expression() {
            if let Some((function, arguments)) = expr.as_call_expression() {
                // Compile the function
                self.compile_expression(function)?;
                
                // Compile the arguments
                for arg in arguments {
                    self.compile_expression(arg)?;
                }
                
                // Emit the call instruction with the number of arguments
                self.emit(Opcode::Call, vec![arguments.len()]);
                
                return Ok(());
            }
        }
        
        // Handle index expressions
        if expr.is_index_expression() {
            if let Some((left, index)) = expr.as_index_expression() {
                // Compile the left expression (array/hash)
                self.compile_expression(left)?;
                
                // Compile the index expression
                self.compile_expression(index)?;
                
                // Emit the index instruction
                self.emit(Opcode::Index, vec![]);
                
                return Ok(());
            }
        }
        
        // Unknown expression type
        Err(ErrorReporter::compilation_error(&format!(
            "Unknown expression type: {}", 
            expr.token_literal()
        )))
    }

    /// Add a constant to the constants pool and return its index
    fn add_constant(&mut self, obj: Object) -> usize {
        self.constants.push(obj);
        self.constants.len() - 1
    }

    /// Emit bytecode instruction
    fn emit(&mut self, op: Opcode, operands: Vec<usize>) -> usize {
        let ins = bytecode::make(op, &operands);
        let pos = self.current_instructions().len();
        self.current_instructions_mut().extend(ins);
        
        self.set_last_instruction(op, pos);
        pos
    }

    /// Get the current instructions
    fn current_instructions(&self) -> &Instructions {
        &self.scopes[self.scope_index].instructions
    }

    /// Get mutable reference to current instructions
    fn current_instructions_mut(&mut self) -> &mut Instructions {
        &mut self.scopes[self.scope_index].instructions
    }

    /// Set the last instruction
    fn set_last_instruction(&mut self, op: Opcode, pos: usize) {
        let scope = &mut self.scopes[self.scope_index];
        let last = EmittedInstruction { opcode: op, position: pos };
        scope.previous_instruction = scope.last_instruction.take();
        scope.last_instruction = Some(last);
    }

    /// Enter a new scope
    fn enter_scope(&mut self) {
        let scope = CompilationScope::default();
        self.scopes.push(scope);
        self.scope_index += 1;
        
        // Create a new symbol table with the current one as its outer scope
        let outer = std::mem::replace(&mut self.symbol_table, Rc::new(RefCell::new(SymbolTable::new())));
        self.symbol_table = Rc::new(RefCell::new(SymbolTable::new_enclosed(outer.borrow().clone())));
    }
    
    /// Leave the current scope and return to the outer scope
    fn leave_scope(&mut self) -> Instructions {
        let instructions = self.current_instructions().clone();
        
        // Remove the current scope
        self.scopes.pop();
        self.scope_index -= 1;
        
        // Restore the outer symbol table
        if let Some(outer) = self.symbol_table.borrow_mut().take_outer() {
            self.symbol_table = Rc::new(RefCell::new(outer.clone()));
        }
        
        instructions
    }
    
    /// Provide a ByteCode object for testing
    #[cfg(test)]
    pub fn bytecode(&self) -> Bytecode {
        Bytecode {
            instructions: self.current_instructions().clone(),
            constants: self.constants.clone(),
        }
    }
}

// Compiler module for the CURSED language
// 
// This module contains the implementation of the compiler that transforms the AST
// into bytecode that can be executed by the VM.
//
// The compiler implements the following features:
// - Compilation of literal expressions (integers, floats, strings, booleans)
// - Compilation of prefix expressions (negation, boolean not)
// - Compilation of infix expressions (arithmetic, comparison)
// - Compilation of call expressions (function calls)
// - Compilation of index expressions (array/hash indexing)
//
// TODO: Remaining to implement:
// - Compilation of statements (let, return, if, block)
// - Support for let bindings and scoping
// - Support for function declarations and closures
// - Error reporting improvements 