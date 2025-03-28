mod bytecode;
pub mod symbol_table;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod property_tests;

pub use bytecode::{Bytecode, Instructions, Opcode};
pub use symbol_table::{Symbol, SymbolScope, SymbolTable};
use crate::ast::{self, Program, Statement, Expression};
use crate::error::{Error, ErrorReporter, SourceLocation};
use crate::memory::gc::{Traceable, Visitor};
use crate::object::Object;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::prelude::VecExt;
use crate::prelude::RefCellSymbolTableExt;
use crate::prelude::SymbolScopeExt;

/// The Compiler takes an AST and converts it into bytecode that can be executed by the VM
pub struct Compiler {
    instructions: Instructions,
    constants: Vec<Object>,
    symbol_table: Rc<RefCell<SymbolTable>>,
    scopes: Vec<CompilationScope>,
    scope_index: usize,
    error_reporter: ErrorReporter,
}

/// A compiled function
#[derive(Debug, Clone, PartialEq)]
pub struct CompiledFunction {
    /// The bytecode for the function
    pub instructions: Vec<u8>,
    /// The number of local variables
    pub num_locals: u8,
    /// The number of parameters
    pub num_parameters: u8,
    /// Free variables for closures
    pub free_variables: Vec<Object>,
    /// Function name (optional)
    pub name: Option<String>,
}

impl CompiledFunction {
    /// Create a new compiled function
    ///
    /// # Arguments
    ///
    /// * `instructions` - The bytecode instructions for the function
    /// * `num_locals` - The number of local variables in the function
    /// * `num_parameters` - The number of parameters the function accepts
    ///
    /// # Returns
    ///
    /// A new CompiledFunction instance
    pub fn new(
        instructions: Vec<u8>,
        num_locals: u8,
        num_parameters: u8,
    ) -> Self {
        Self {
            instructions,
            num_locals,
            num_parameters,
            free_variables: Vec::new(),
            name: None,
        }
    }
    
    /// Create a compiled function with free variables for closures
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
            name: None,
        }
    }
    
    /// Create a compiled function with a name
    pub fn with_name(
        instructions: Instructions, 
        num_locals: usize, 
        num_parameters: usize,
        name: String
    ) -> Self {
        Self {
            instructions,
            num_locals,
            num_parameters,
            free_variables: Vec::new(),
            name: Some(name),
        }
    }
}

/// A compilation scope
#[derive(Debug, Clone)]
pub struct CompilationScope {
    /// The instructions for this scope
    pub instructions: Instructions,
    /// The last instruction emitted
    pub last_instruction: Instruction,
    /// The previous instruction emitted
    pub previous_instruction: Instruction,
}

impl Default for CompilationScope {
    fn default() -> Self {
        Self {
            instructions: Instructions::new(),
            last_instruction: Instruction::new(Opcode::Invalid, Vec::new()),
            previous_instruction: Instruction::new(Opcode::Invalid, Vec::new()),
        }
    }
}

impl CompilationScope {
    /// Create a new compilation scope
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            last_instruction: EmittedInstruction::default(),
            previous_instruction: EmittedInstruction::default(),
        }
    }
}

/// Emitted instruction structure
#[derive(Clone, Debug)]
struct EmittedInstruction {
    opcode: Opcode,
    position: usize,
}

/// An instruction in the bytecode
#[derive(Debug, Clone)]
pub struct Instruction {
    /// The opcode for this instruction
    pub opcode: Opcode,
    /// The operands for this instruction
    pub operands: Vec<u16>,
}

impl Instruction {
    /// Create a new instruction with the given opcode and operands
    pub fn new(opcode: Opcode, operands: Vec<u16>) -> Self {
        Self {
            opcode,
            operands,
        }
    }
}

impl EmittedInstruction {
    /// Create a new emitted instruction
    pub fn new(opcode: Opcode, position: usize) -> Self {
        Self {
            opcode,
            position,
        }
    }
}

impl Default for EmittedInstruction {
    fn default() -> Self {
        Self {
            opcode: Opcode::Invalid,
            position: 0,
        }
    }
}

impl Compiler {
    /// Create a new compiler
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            constants: Vec::new(),
            symbol_table: Rc::new(RefCell::new(SymbolTable::new())),
            scopes: vec![CompilationScope::new()],
            scope_index: 0,
            error_reporter: ErrorReporter::new(),
        }
    }
    
    /// Compile a program into bytecode
    ///
    /// # Arguments
    ///
    /// * `program` - The AST program to compile
    ///
    /// # Returns
    ///
    /// A result containing either the bytecode or an error
    pub fn compile_program(&mut self, program: &Program) -> Result<Bytecode, Error> {
        for statement in &program.statements {
            self.compile_statement(statement)?;
        }
        
        Ok(Bytecode {
            instructions: self.current_instructions().clone(),
            constants: self.constants.clone(),
        })
    }
    
    /// Compile a statement
    ///
    /// # Arguments
    ///
    /// * `statement` - The statement to compile
    ///
    /// # Returns
    ///
    /// A result indicating success or an error
    fn compile_statement(&mut self, statement: &dyn Statement) -> Result<(), Error> {
        // Stub implementation
        Ok(())
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
    
    /// Compile a program from an AST Program struct
    pub fn compile_program(&mut self, program: &Program) -> Result<Bytecode, Error> {
        for stmt in &program.statements {
            self.compile_statement(stmt.clone())?;
        }
        
        // We're compiling from the main scope
        let main_scope = &self.scopes[0];
        
        Ok(Bytecode {
            instructions: main_scope.instructions.clone(),
            constants: self.constants.clone(),
        })
    }

    /// Compile a statement
    pub fn compile_statement(&mut self, stmt: Box<dyn Statement>) -> Result<(), Error> {
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
            "be_like" => {
                if let Some(squad_stmt) = stmt.as_any().downcast_ref::<ast::SquadStatement>() {
                    // Create the struct type object
                    let mut fields = Vec::new();
                    for field in &squad_stmt.fields {
                        fields.push((
                            field.name.value.clone(),
                            field.type_name.value.clone()
                        ));
                    }
                    
                    let struct_obj = Object::Struct {
                        name: squad_stmt.name.value.clone(),
                        fields,
                    };
                    
                    // Add the struct to constants
                    let struct_idx = self.add_constant(struct_obj);
                    
                    // Emit class instruction
                    self.emit(Opcode::Class, vec![struct_idx]);
                    
                    // Define the struct in the symbol table
                    let symbol = self.symbol_table.borrow_mut().define(&squad_stmt.name.value);
                    
                    // Store the struct class in a global variable
                    if symbol.scope == symbol_table::SymbolScope::Global {
                        self.emit(Opcode::SetGlobal, vec![symbol.index]);
                    } else {
                        self.emit(Opcode::SetLocal, vec![symbol.index]);
                    }
                    
                    Ok(())
                } else {
                    Err(ErrorReporter::compilation_error(&format!(
                        "Expected SquadStatement, got: {}", 
                        stmt.token_literal()
                    )))
                }
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
    pub fn compile_if_statement(&mut self, if_stmt: &ast::IfStatement) -> Result<(), Error> {
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
    pub fn compile_block_statement(&mut self, block: &ast::BlockStatement) -> Result<(), Error> {
        for stmt in &block.statements {
            self.compile_statement(stmt.clone())?;
        }
        
        Ok(())
    }
    
    /// Change an operand at the given position
    pub fn change_operand(&mut self, op_pos: usize, operand: usize) {
        let op: Opcode = self.current_instructions()[op_pos].into();
        let new_instruction = bytecode::make(op, &[operand]);
        
        // Replace the instruction at op_pos with the new one
        let instructions = self.current_instructions_mut();
        for (i, byte) in new_instruction.iter().enumerate() {
            instructions[op_pos + i] = *byte;
        }
    }

    /// Compile an expression
    pub fn compile_expression(&mut self, expr: &dyn Expression) -> Result<(), Error> {
        let location = SourceLocation::new(0, 0); // TODO: Add proper location tracking
        
        // Check if this is a BeLikeExpression for struct instantiation
        if let Some(be_like_expr) = expr.as_any().downcast_ref::<ast::BeLikeExpression>() {
            // Resolve the struct name in the symbol table
            match self.symbol_table.resolve(&be_like_expr.struct_name.value) {
                Some(symbol) => {
                    // Load the struct definition
                    if symbol.scope.is_global() {
                        self.emit(Opcode::GetGlobal, vec![symbol.index]);
                    } else {
                        self.emit(Opcode::GetLocal, vec![symbol.index]);
                    }
                    
                    // Create a new instance
                    self.emit(Opcode::Instance, vec![]);
                    
                    // Set fields if provided
                    for (field_name, value_expr) in &be_like_expr.fields {
                        // Duplicate the instance on top of the stack 
                        // (we need it for each field operation)
                        self.emit(Opcode::Dup, vec![]);
                        
                        // Compile the value expression
                        self.compile_expression(value_expr.as_ref())?;
                        
                        // Get field name constant index or create it
                        let name_obj = Object::String(field_name.value.clone());
                        let name_idx = self.add_constant(name_obj);
                        
                        // Set the field
                        self.emit(Opcode::SetField, vec![name_idx]);
                        
                        // Pop the result (which is the instance)
                        self.emit(Opcode::Pop, vec![]);
                    }
                    
                    return Ok(());
                },
                None => {
                    return Err(ErrorReporter::compilation_error(&format!(
                        "Struct not found: {}", 
                        be_like_expr.struct_name.value
                    )));
                }
            }
        }
        
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
            match self.symbol_table.resolve(&ident.value) {
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

    /// Add a constant to the constants pool
    pub fn add_constant(&mut self, obj: Object) -> usize {
        self.constants.push(obj);
        self.constants.len() - 1
    }

    /// Emit an instruction
    pub fn emit(&mut self, op: Opcode, operands: Vec<usize>) -> usize {
        let pos = self.current_instructions().len();
        let operands_slice: &[usize] = operands.as_slice();
        let mut ins = bytecode::make(op, operands_slice);
        self.current_instructions_mut().extend(&ins);
        
        self.set_last_instruction(op, pos);
        
        pos
    }

    /// Get the current instructions
    pub fn current_instructions(&self) -> &Instructions {
        &self.scopes[self.scope_index].instructions
    }

    /// Get the current instructions as mutable
    pub fn current_instructions_mut(&mut self) -> &mut Instructions {
        &mut self.scopes[self.scope_index].instructions
    }

    /// Set the last instruction
    pub fn set_last_instruction(&mut self, op: Opcode, pos: usize) {
        let last = EmittedInstruction {
            opcode: op,
            position: pos,
        };
        
        let scope = &mut self.scopes[self.scope_index];
        scope.previous_instruction = scope.last_instruction.clone();
        scope.last_instruction = Instruction::new(op, Vec::new());
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

    /// Define built-in functions in the symbol table
    fn define_builtins(symbol_table: &mut SymbolTable) {
        // Define each built-in function with a unique index
        let builtins = [
            "len", "first", "last", "rest", "push", "puts", 
            "type", "print", "println", "array", "map", "range",
        ];
        
        for (i, builtin) in builtins.iter().enumerate() {
            symbol_table.define_builtin(i, builtin);
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