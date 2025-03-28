mod bytecode;
pub mod symbol_table;

#[cfg(test)]
mod tests;

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
    pub fn compile(&mut self, program: &Program) -> Result<Bytecode, Error> {
        // Define built-in functions in the symbol table
        Self::define_builtins(&mut self.symbol_table.borrow_mut());
        
        // Compile each statement in the program
        for stmt in &program.statements {
            self.compile_statement(&**stmt)?;
        }
        
        Ok(self.bytecode())
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
        } else if let Some(while_stmt) = stmt.as_any().downcast_ref::<ast::WhileStatement>() {
            // Compile while statements
            self.compile_while_statement(while_stmt)
        } else if let Some(for_stmt) = stmt.as_any().downcast_ref::<ast::ForStatement>() {
            // Compile for statements
            self.compile_for_statement(for_stmt)
        } else if let Some(switch_stmt) = stmt.as_any().downcast_ref::<ast::SwitchStatement>() {
            // Compile switch statements
            self.compile_switch_statement(switch_stmt)
        } else if let Some(package_stmt) = stmt.as_any().downcast_ref::<ast::PackageStatement>() {
            // Compile package declarations
            self.compile_package_statement(package_stmt)
        } else if let Some(import_stmt) = stmt.as_any().downcast_ref::<ast::ImportStatement>() {
            // Compile import statements
            self.compile_import_statement(import_stmt)
        } else if let Some(squad_stmt) = stmt.as_any().downcast_ref::<ast::SquadStatement>() {
            // Compile type (squad) declarations
            self.compile_squad_statement(squad_stmt)
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

    /// Compile a while statement (periodt in CURSED)
    pub fn compile_while_statement(&mut self, while_stmt: &ast::WhileStatement) -> Result<(), Error> {
        // Record the position of the start of the loop
        let loop_start_pos = self.current_instructions().len();
        
        // Compile the condition expression
        self.compile_expression(&*while_stmt.condition)?;
        
        // Emit a jump-if-not-truthy instruction to exit the loop
        // We don't know the jump offset yet, so use a placeholder
        let jump_not_truthy_pos = self.emit(Opcode::JumpNotTruthy, vec![9999]);
        
        // Compile the loop body
        self.compile_block_statement(&while_stmt.body)?;
        
        // Jump back to the start of the loop to check the condition again
        self.emit(Opcode::Jump, vec![loop_start_pos]);
        
        // Update the JumpNotTruthy instruction with the correct exit point
        let after_loop_pos = self.current_instructions().len();
        self.change_operand(jump_not_truthy_pos, after_loop_pos);
        
        Ok(())
    }

    /// Compile a for statement (bestie in CURSED)
    pub fn compile_for_statement(&mut self, for_stmt: &ast::ForStatement) -> Result<(), Error> {
        // If we have an initialization statement, compile it first
        if let Some(init) = &for_stmt.init {
            self.compile_statement(&**init)?;
        }
        
        // Record the position of the start of the loop (where condition check begins)
        let loop_start_pos = self.current_instructions().len();
        
        // If we have a condition expression, compile it and set up conditional jump
        let jump_not_truthy_pos = if let Some(condition) = &for_stmt.condition {
            self.compile_expression(&**condition)?;
            // Emit jump-if-not-truthy to exit loop if condition is false
            Some(self.emit(Opcode::JumpNotTruthy, vec![9999]))
        } else {
            // No condition means infinite loop, so no conditional jump needed
            None
        };
        
        // Compile the loop body
        self.compile_block_statement(&for_stmt.body)?;
        
        // If we have a post statement, compile it
        if let Some(post) = &for_stmt.post {
            self.compile_statement(&**post)?;
        }
        
        // Jump back to the start of the loop to check the condition again
        self.emit(Opcode::Jump, vec![loop_start_pos]);
        
        // If we have a jump_not_truthy instruction, update it with the correct exit point
        if let Some(jump_pos) = jump_not_truthy_pos {
            let after_loop_pos = self.current_instructions().len();
            self.change_operand(jump_pos, after_loop_pos);
        }
        
        Ok(())
    }

    /// Compile a switch statement (vibe_check in CURSED)
    pub fn compile_switch_statement(&mut self, switch_stmt: &ast::SwitchStatement) -> Result<(), Error> {
        // Compile the expression that we're switching on
        self.compile_expression(&*switch_stmt.value)?;
        
        // We'll store the positions of all jump instructions so we can update them later
        let mut jump_positions = Vec::new();
        
        // Popping the matched value at the end of the switch statement
        let mut pop_needed = true;
        
        // Compile each case
        for case in &switch_stmt.cases {
            // For each case value, we need to duplicate the switch value and compare it with the case value
            for expr in &case.expressions {
                // Duplicate the value we're switching on
                self.emit(Opcode::Dup, vec![]);
                
                // Compile the case expression
                self.compile_expression(&**expr)?;
                
                // Compare for equality
                self.emit(Opcode::Equal, vec![]);
                
                // Jump to the case if equal
                // We need to use a placeholder and update it later
                self.emit(Opcode::JumpNotTruthy, vec![9999]);
                
                // Update the last instruction with the position right after this block
                let case_is_equal_pos = self.current_instructions().len();
                
                // If the case matches, execute the body
                self.compile_block_statement(&case.body)?;
                
                // After executing the case, jump to the end of the switch
                let jump_end_pos = self.emit(Opcode::Jump, vec![9999]);
                jump_positions.push(jump_end_pos);
                
                // Update the JumpNotTruthy to jump to the next case comparison
                let next_case_pos = self.current_instructions().len();
                self.change_operand(case_is_equal_pos - 3, next_case_pos);
            }
        }
        
        // Compile the default case if it exists
        if let Some(default) = &switch_stmt.default {
            // Pop the switch value since we'll enter the default case
            self.emit(Opcode::Pop, vec![]);
            pop_needed = false;
            
            // Compile the default case body
            self.compile_block_statement(default)?;
        } else {
            // If no case matched and no default case exists, pop the switch value
            self.emit(Opcode::Pop, vec![]);
            pop_needed = false;
        }
        
        // End of switch statement
        let end_pos = self.current_instructions().len();
        
        // Update all the jump instructions to point to the end of the switch
        for pos in jump_positions {
            self.change_operand(pos, end_pos);
        }
        
        // If we haven't popped the switched value yet, do it now
        if pop_needed {
            self.emit(Opcode::Pop, vec![]);
        }
        
        Ok(())
    }

    /// Compile a package statement (vibe in CURSED)
    pub fn compile_package_statement(&mut self, package_stmt: &ast::PackageStatement) -> Result<(), Error> {
        // Store the package name in a symbol table or metadata
        // For now, we'll just define it as a global variable with value set to its name
        // In a more complete implementation, this would be used for module/namespace resolution
        
        // Create a string object with the package name
        let package_name = Object::String(package_stmt.name.value.clone());
        
        // Add the package name as a constant
        let constant_index = self.add_constant(package_name);
        
        // Emit instructions to load the package name
        self.emit(Opcode::Constant, vec![constant_index]);
        
        // Define a special global variable for the package
        let symbol = self.symbol_table.borrow_mut().define("__package__");
        self.emit(Opcode::SetGlobal, vec![symbol.index]);
        
        Ok(())
    }

    /// Compile an import statement (yeet in CURSED)
    pub fn compile_import_statement(&mut self, import_stmt: &ast::ImportStatement) -> Result<(), Error> {
        // Create a string object with the import path
        let import_path = Object::String(import_stmt.path.value.clone());
        
        // Add the import path as a constant
        let path_index = self.add_constant(import_path);
        
        // Emit instructions to load the import path
        self.emit(Opcode::Constant, vec![path_index]);
        
        // Determine the import name (either the alias or the last part of the path)
        let import_name = if let Some(alias) = &import_stmt.alias {
            alias.value.clone()
        } else {
            // Extract the module name from the path (last component)
            let path = &import_stmt.path.value;
            match path.rfind('/') {
                Some(pos) => path[pos + 1..].to_string(),
                None => path.clone(),
            }
        };
        
        // Store the import in a special global array of imports
        // First we need to define or get the __imports__ array
        let imports_symbol = self.symbol_table.borrow_mut().define("__imports__");
        
        // We'll emit instructions to create a tuple of (path, name) and add it to imports
        // In a real implementation, this would trigger loading the module
        
        // Create a string object with the import name
        let import_name_obj = Object::String(import_name.clone());
        let name_index = self.add_constant(import_name_obj);
        
        // Emit instructions to load the import name
        self.emit(Opcode::Constant, vec![name_index]);
        
        // Define a global variable for the imported module
        let module_symbol = self.symbol_table.borrow_mut().define(&import_name);
        self.emit(Opcode::SetGlobal, vec![module_symbol.index]);
        
        // Pop the import name from the stack (we just used it to set the global)
        self.emit(Opcode::Pop, vec![]);
        
        // Pop the import path from the stack (we're done with it)
        self.emit(Opcode::Pop, vec![]);
        
        Ok(())
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
        } else if let Some(assignment_expr) = expr.as_any().downcast_ref::<ast::AssignmentExpression>() {
            // Compile assignment expressions
            
            // Compile the right side expression first
            self.compile_expression(&*assignment_expr.value)?;
            
            // Get the symbol for the variable being assigned
            let symbol = match self.symbol_table.borrow_mut().resolve(&assignment_expr.name.value) {
                Some(symbol) => symbol,
                None => {
                    // If symbol doesn't exist, define it in the current scope
                    self.symbol_table.borrow_mut().define(&assignment_expr.name.value)
                }
            };
            
            // Emit the appropriate opcode based on scope
            match symbol.scope {
                SymbolScope::Global => {
                    self.emit(Opcode::SetGlobal, vec![symbol.index]);
                }
                SymbolScope::Local => {
                    self.emit(Opcode::SetLocal, vec![symbol.index]);
                }
                _ => {
                    return Err(Error::from_str(
                        &format!("Cannot assign to {:?}", symbol.scope),
                        None,
                    ));
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
        // Create emitted instruction
        let emitted = EmittedInstruction {
            opcode: op,
            position: pos,
        };
        
        // Get the operands from the bytecode
        let mut operands = Vec::new();
        if let Some(def) = get_definition(op) {
            // Extract operands based on the definition
            let (extracted, _) = bytecode::read_operand(&def, self.current_instructions(), pos);
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

    /// Get the definition for an opcode
    fn get_definition(op: Opcode) -> Option<bytecode::Definition> {
        Some(bytecode::lookup(op))
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
    
    /// Provide a ByteCode object from the current compiler state
    pub fn bytecode(&self) -> Bytecode {
        Bytecode {
            instructions: self.current_instructions().clone(),
            constants: self.constants.clone(),
        }
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

    /// Compile a function literal to bytecode
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
        let free_symbols = self.symbol_table.borrow().free_symbols().to_vec();
        let num_params = func.parameters.len();
        
        // Capture the compiled instructions and leave the function's scope
        let instructions = self.leave_scope();
        
        // Create a compiled function object
        let compiled_func = Object::CompiledFunction {
            instructions,
            num_locals,
            num_parameters: num_params,
        };
        
        // Add the function to constants
        let fn_index = self.add_constant(compiled_func);
        
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
            current_scope.last_instruction.position
        };
        
        // Replace the opcode at last_pos with ReturnValue
        self.current_instructions_mut()[last_pos] = Opcode::ReturnValue as u8;
        
        // Update the last instruction
        let current_scope = &mut self.scopes[self.scope_index];
        current_scope.last_instruction.opcode = Opcode::ReturnValue;
    }

    /// Compile a type declaration statement (be_like...squad in CURSED)
    pub fn compile_squad_statement(&mut self, squad_stmt: &ast::SquadStatement) -> Result<(), Error> {
        // Store the struct name as a constant
        let struct_name = Object::String(squad_stmt.name.value.clone());
        let name_index = self.add_constant(struct_name);
        
        // Create a map of field names to their types
        let mut field_map = Vec::new();
        
        for field in &squad_stmt.fields {
            let field_name = Object::String(field.name.value.clone());
            let field_type = Object::String(field.type_name.value.clone());
            
            let name_idx = self.add_constant(field_name);
            let type_idx = self.add_constant(field_type);
            
            field_map.push((name_idx, type_idx));
        }
        
        // Store the number of fields
        let num_fields = field_map.len();
        
        // Emit instruction to load the struct name
        self.emit(Opcode::Constant, vec![name_index]);
        
        // Emit instruction to define a struct type
        self.emit(Opcode::DefineType, vec![num_fields]);
        
        // Emit instructions for each field
        for (name_idx, type_idx) in field_map {
            self.emit(Opcode::Constant, vec![name_idx]);
            self.emit(Opcode::Constant, vec![type_idx]);
            self.emit(Opcode::DefineField, vec![]);
        }
        
        // Define the type in the symbol table
        let symbol = self.symbol_table.borrow_mut().define(&squad_stmt.name.value);
        self.emit(Opcode::SetGlobal, vec![symbol.index]);
        
        Ok(())
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