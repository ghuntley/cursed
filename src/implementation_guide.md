# CURSED Compiler Implementation Guide

This guide provides practical steps for implementing the CURSED compiler according to the specifications.

## Overview

The CURSED compiler translates the Abstract Syntax Tree (AST) into bytecode that can be executed by the Virtual Machine. This implementation guide will walk through each component of the compiler and provide best practices for a robust implementation.

## Implementation Roadmap

1. Basic compilation infrastructure
2. Support for literals and expressions
3. Variable support (global, local)
4. Control flow (if statements)
5. Functions and closures
6. Advanced data structures (arrays, hashes)
7. Error handling and optimization

## 1. Basic Infrastructure

### Bytecode Representation

Start by implementing the fundamental building blocks:

```rust
// Define instruction type
pub type Instructions = Vec<u8>;

// Define opcodes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    // Constants and simple values
    Constant = 0x01,   // Load constant from pool
    True = 0x06,       // Push true
    False = 0x07,      // Push false
    Null = 0x0F,       // Push null
    
    // Basic stack operations
    Pop = 0x1D,        // Pop top value from stack
    
    // Add more opcodes as needed...
}

// Define bytecode output structure
#[derive(Debug, Clone)]
pub struct Bytecode {
    pub instructions: Instructions,
    pub constants: Vec<Object>,
}
```

### Opcode Definitions

Create a mapping between opcodes and their definitions:

```rust
pub struct Definition {
    pub name: &'static str,
    pub operand_widths: Vec<usize>,
}

pub fn lookup(op: Opcode) -> Definition {
    match op {
        Opcode::Constant => Definition {
            name: "Constant",
            operand_widths: vec![2], // 2-byte operand
        },
        Opcode::Pop => Definition {
            name: "Pop",
            operand_widths: vec![],  // No operands
        },
        // Define other opcodes...
    }
}
```

### Basic Instruction Generation

Implement functions to generate bytecode from opcodes and operands:

```rust
pub fn make(op: Opcode, operands: &[usize]) -> Instructions {
    let def = lookup(op);
    let instruction_len = 1 + def.operand_widths.iter().sum::<usize>();
    let mut instruction = Vec::with_capacity(instruction_len);
    
    instruction.push(op as u8);
    
    for (i, width) in def.operand_widths.iter().enumerate() {
        match width {
            2 => {
                let operand = operands[i];
                if operand > u16::MAX as usize {
                    panic!("Operand too large");
                }
                let bytes = (operand as u16).to_be_bytes();
                instruction.extend_from_slice(&bytes);
            },
            1 => {
                let operand = operands[i];
                if operand > u8::MAX as usize {
                    panic!("Operand too large");
                }
                instruction.push(operand as u8);
            },
            _ => panic!("Unsupported operand width"),
        }
    }
    
    instruction
}
```

### Initial Compiler Structure

Create the basic compiler structure:

```rust
pub struct Compiler {
    constants: Vec<Object>,
    instructions: Instructions,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            constants: Vec::new(),
            instructions: Vec::new(),
        }
    }
    
    pub fn bytecode(&self) -> Bytecode {
        Bytecode {
            instructions: self.instructions.clone(),
            constants: self.constants.clone(),
        }
    }
    
    // Helper method for adding constants
    pub fn add_constant(&mut self, obj: Object) -> usize {
        self.constants.push(obj);
        self.constants.len() - 1
    }
    
    // Helper method for emitting instructions
    pub fn emit(&mut self, op: Opcode, operands: Vec<usize>) -> usize {
        let pos = self.instructions.len();
        let operands_slice: &[usize] = operands.as_slice();
        let ins = make(op, operands_slice);
        self.instructions.extend(&ins);
        pos
    }
}
```

## 2. Literals and Expressions

### Implement Literal Compilation

Add methods to compile basic literals:

```rust
impl Compiler {
    pub fn compile_program(&mut self, program: &Program) -> Result<(), Error> {
        for stmt in &program.statements {
            self.compile_statement(stmt)?;
        }
        Ok(())
    }
    
    fn compile_statement(&mut self, stmt: &dyn Statement) -> Result<(), Error> {
        if let Some(expr_stmt) = stmt.as_any().downcast_ref::<ast::ExpressionStatement>() {
            if let Some(expr) = &expr_stmt.expression {
                self.compile_expression(&**expr)?;
                // Pop the result since expression statements don't produce values
                self.emit(Opcode::Pop, vec![]);
            }
            Ok(())
        } else {
            Err(Error::from_str("Unknown statement type", None))
        }
    }
    
    fn compile_expression(&mut self, expr: &dyn Expression) -> Result<(), Error> {
        if let Some(integer_literal) = expr.as_any().downcast_ref::<ast::IntegerLiteral>() {
            let value = Object::Integer(integer_literal.value);
            let constant_index = self.add_constant(value);
            self.emit(Opcode::Constant, vec![constant_index]);
            Ok(())
        } else if let Some(boolean_literal) = expr.as_any().downcast_ref::<ast::BooleanLiteral>() {
            if boolean_literal.value {
                self.emit(Opcode::True, vec![]);
            } else {
                self.emit(Opcode::False, vec![]);
            }
            Ok(())
        } else if let Some(string_literal) = expr.as_any().downcast_ref::<ast::StringLiteral>() {
            let value = Object::String(string_literal.value.clone());
            let constant_index = self.add_constant(value);
            self.emit(Opcode::Constant, vec![constant_index]);
            Ok(())
        } else {
            Err(Error::from_str("Unknown expression type", None))
        }
    }
}
```

### Add Support for Prefix Expressions

Implement compilation for prefix expressions like `-x` and `!x`:

```rust
impl Compiler {
    // Add to compile_expression method
    fn compile_expression(&mut self, expr: &dyn Expression) -> Result<(), Error> {
        // Existing literal handling...
        
        if let Some(prefix_expr) = expr.as_any().downcast_ref::<ast::PrefixExpression>() {
            self.compile_expression(&*prefix_expr.right)?;
            
            match prefix_expr.operator.as_str() {
                "!" => {
                    self.emit(Opcode::Bang, vec![]);
                },
                "-" => {
                    self.emit(Opcode::Minus, vec![]);
                },
                _ => {
                    return Err(Error::from_str(
                        &format!("Unknown prefix operator: {}", prefix_expr.operator),
                        None,
                    ));
                }
            }
            Ok(())
        } else {
            // Other expression types...
        }
    }
}
```

### Add Support for Infix Expressions

Implement compilation for infix expressions like `a + b` and `x == y`:

```rust
impl Compiler {
    // Add to compile_expression method
    fn compile_expression(&mut self, expr: &dyn Expression) -> Result<(), Error> {
        // Existing expression handling...
        
        if let Some(infix_expr) = expr.as_any().downcast_ref::<ast::InfixExpression>() {
            // Special case for < and <= operators
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
            
            // Standard order for other operators
            self.compile_expression(&*infix_expr.left)?;
            self.compile_expression(&*infix_expr.right)?;
            
            match infix_expr.operator.as_str() {
                "+" => { self.emit(Opcode::Add, vec![]); },
                "-" => { self.emit(Opcode::Sub, vec![]); },
                "*" => { self.emit(Opcode::Mul, vec![]); },
                "/" => { self.emit(Opcode::Div, vec![]); },
                "%" => { self.emit(Opcode::Modulo, vec![]); },
                ">" => { self.emit(Opcode::GreaterThan, vec![]); },
                ">=" => { self.emit(Opcode::GreaterThanEqual, vec![]); },
                "==" => { self.emit(Opcode::Equal, vec![]); },
                "!=" => { self.emit(Opcode::NotEqual, vec![]); },
                _ => {
                    return Err(Error::from_str(
                        &format!("Unknown infix operator: {}", infix_expr.operator),
                        None,
                    ));
                }
            }
            Ok(())
        } else {
            // Other expression types...
        }
    }
}
```

## 3. Variables Support

### Symbol Table

Implement a symbol table to track variables:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolScope {
    Global,
    Local,
    Builtin,
    Free,
    Function,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub scope: SymbolScope,
    pub index: usize,
}

pub struct SymbolTable {
    pub store: HashMap<String, Symbol>,
    pub outer: Option<Box<SymbolTable>>,
    pub num_definitions: usize,
    pub free_symbols: Vec<Symbol>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
            outer: None,
            num_definitions: 0,
            free_symbols: Vec::new(),
        }
    }
    
    pub fn with_outer(outer: SymbolTable) -> Self {
        Self {
            store: HashMap::new(),
            outer: Some(Box::new(outer)),
            num_definitions: 0,
            free_symbols: Vec::new(),
        }
    }
    
    pub fn define(&mut self, name: &str) -> Symbol {
        let symbol = Symbol {
            name: name.to_string(),
            scope: SymbolScope::Global, // Default to global, will be updated
            index: self.num_definitions,
        };
        
        self.store.insert(name.to_string(), symbol.clone());
        self.num_definitions += 1;
        
        symbol
    }
    
    pub fn resolve(&mut self, name: &str) -> Option<Symbol> {
        if let Some(symbol) = self.store.get(name) {
            return Some(symbol.clone());
        }
        
        if let Some(outer) = &mut self.outer {
            if let Some(symbol) = outer.resolve(name) {
                // Check if this is a free variable
                if symbol.scope != SymbolScope::Global && symbol.scope != SymbolScope::Builtin {
                    // Define as a free variable in this scope
                    return Some(self.define_free(symbol));
                }
                return Some(symbol);
            }
        }
        
        None
    }
    
    // Define a free variable (for closures)
    pub fn define_free(&mut self, original: Symbol) -> Symbol {
        let free_symbol = Symbol {
            name: original.name.clone(),
            scope: SymbolScope::Free,
            index: self.free_symbols.len(),
        };
        
        self.free_symbols.push(original);
        self.store.insert(free_symbol.name.clone(), free_symbol.clone());
        
        free_symbol
    }
    
    // Define builtin functions
    pub fn define_builtin(&mut self, index: usize, name: &str) -> Symbol {
        let symbol = Symbol {
            name: name.to_string(),
            scope: SymbolScope::Builtin,
            index,
        };
        
        self.store.insert(name.to_string(), symbol.clone());
        
        symbol
    }
}
```

### Enhance Compiler with Symbol Table

Modify the compiler to use the symbol table:

```rust
pub struct Compiler {
    constants: Vec<Object>,
    symbol_table: Rc<RefCell<SymbolTable>>,
    scopes: Vec<CompilationScope>, // For tracking different compilation scopes
    scope_index: usize,
}

impl Compiler {
    pub fn new() -> Self {
        let symbol_table = Rc::new(RefCell::new(SymbolTable::new()));
        
        Self {
            constants: Vec::new(),
            symbol_table,
            scopes: vec![CompilationScope::default()],
            scope_index: 0,
        }
    }
    
    // Implement let statements
    fn compile_statement(&mut self, stmt: &dyn Statement) -> Result<(), Error> {
        // Existing statement handling...
        
        if let Some(let_stmt) = stmt.as_any().downcast_ref::<ast::LetStatement>() {
            // Compile the value expression
            if let Some(value) = &let_stmt.value {
                self.compile_expression(&**value)?;
            } else {
                // If no value is provided, use null as default
                self.emit(Opcode::Null, vec![]);
            }
            
            // Define the symbol
            let symbol = self.symbol_table.borrow_mut().define(&let_stmt.name.value);
            
            // Emit the correct set instruction based on scope
            match symbol.scope {
                SymbolScope::Global => {
                    self.emit(Opcode::SetGlobal, vec![symbol.index]);
                },
                SymbolScope::Local => {
                    self.emit(Opcode::SetLocal, vec![symbol.index]);
                },
                _ => {
                    return Err(Error::from_str(
                        "Cannot define variable in this scope",
                        None,
                    ));
                }
            }
            
            Ok(())
        } else {
            // Other statement types...
        }
    }
    
    // Add identifier compilation for variable access
    fn compile_expression(&mut self, expr: &dyn Expression) -> Result<(), Error> {
        // Existing expression handling...
        
        if let Some(identifier) = expr.as_any().downcast_ref::<ast::Identifier>() {
            let symbol = match self.symbol_table.borrow_mut().resolve(&identifier.value) {
                Some(symbol) => symbol,
                None => {
                    return Err(Error::from_str(
                        &format!("Undefined variable: {}", identifier.value),
                        None,
                    ));
                }
            };
            
            // Emit the correct get instruction based on scope
            match symbol.scope {
                SymbolScope::Global => {
                    self.emit(Opcode::GetGlobal, vec![symbol.index]);
                },
                SymbolScope::Local => {
                    self.emit(Opcode::GetLocal, vec![symbol.index]);
                },
                SymbolScope::Builtin => {
                    self.emit(Opcode::GetBuiltin, vec![symbol.index]);
                },
                SymbolScope::Free => {
                    self.emit(Opcode::GetFree, vec![symbol.index]);
                },
                SymbolScope::Function => {
                    self.emit(Opcode::CurrentClosure, vec![]);
                }
            }
            
            Ok(())
        } else {
            // Other expression types...
        }
    }
}
```

## 4. Control Flow

### Implement If Statements

```rust
impl Compiler {
    // Add to compile_statement method
    fn compile_statement(&mut self, stmt: &dyn Statement) -> Result<(), Error> {
        // Existing statement handling...
        
        if let Some(if_stmt) = stmt.as_any().downcast_ref::<ast::IfStatement>() {
            // Compile the condition expression
            self.compile_expression(&*if_stmt.condition)?;
            
            // Emit a jump-if-not-truthy instruction with a placeholder offset
            let jump_not_truthy_pos = self.emit(Opcode::JumpNotTruthy, vec![9999]);
            
            // Compile the consequence block
            self.compile_block_statement(&if_stmt.consequence)?;
            
            // If the consequence doesn't end with a return, we need to jump over the alternative
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
        } else {
            // Other statement types...
        }
    }
    
    // Helper for compiling block statements
    fn compile_block_statement(&mut self, block: &ast::BlockStatement) -> Result<(), Error> {
        for stmt in &block.statements {
            self.compile_statement(&**stmt)?;
        }
        Ok(())
    }
    
    // Helper for changing the operand of an existing instruction
    fn change_operand(&mut self, op_pos: usize, operand: usize) {
        let op: Opcode = self.current_instructions()[op_pos].into();
        let new_instruction = make(op, &[operand]);
        
        let instructions = self.current_instructions_mut();
        for (i, byte) in new_instruction.iter().enumerate() {
            if op_pos + i < instructions.len() {
                instructions[op_pos + i] = *byte;
            }
        }
    }
}
```

## 5. Functions and Closures

### Implement Function Literals

```rust
// Add compilation scope structure
#[derive(Debug, Clone)]
pub struct CompilationScope {
    pub instructions: Instructions,
    pub last_instruction: EmittedInstruction,
    pub previous_instruction: EmittedInstruction,
}

impl Default for CompilationScope {
    fn default() -> Self {
        Self {
            instructions: Vec::new(),
            last_instruction: EmittedInstruction::default(),
            previous_instruction: EmittedInstruction::default(),
        }
    }
}

impl Compiler {
    // Implement function literal compilation
    fn compile_expression(&mut self, expr: &dyn Expression) -> Result<(), Error> {
        // Existing expression handling...
        
        if let Some(function_literal) = expr.as_any().downcast_ref::<ast::FunctionLiteral>() {
            // Enter a new scope for the function body
            self.enter_scope();
            
            // Define parameters in the function's scope
            for param in &function_literal.parameters {
                self.symbol_table.borrow_mut().define(&param.value);
            }
            
            // Compile the function body
            self.compile_block_statement(&function_literal.body)?;
            
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
            let num_params = function_literal.parameters.len();
            
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
                        },
                        SymbolScope::Local => {
                            self.emit(Opcode::GetLocal, vec![free_symbol.index]);
                        },
                        SymbolScope::Free => {
                            self.emit(Opcode::GetFree, vec![free_symbol.index]);
                        },
                        _ => {
                            return Err(Error::from_str(
                                &format!("Invalid scope for free variable: {:?}", free_symbol.scope),
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
        } else {
            // Other expression types...
        }
    }
    
    // Add return statement support
    fn compile_statement(&mut self, stmt: &dyn Statement) -> Result<(), Error> {
        // Existing statement handling...
        
        if let Some(return_stmt) = stmt.as_any().downcast_ref::<ast::ReturnStatement>() {
            if let Some(return_value) = &return_stmt.return_value {
                self.compile_expression(&**return_value)?;
                self.emit(Opcode::ReturnValue, vec![]);
            } else {
                // Return without a value (null)
                self.emit(Opcode::Return, vec![]);
            }
            Ok(())
        } else {
            // Other statement types...
        }
    }
    
    // Add function call support
    fn compile_expression(&mut self, expr: &dyn Expression) -> Result<(), Error> {
        // Existing expression handling...
        
        if let Some(call_expr) = expr.as_any().downcast_ref::<ast::CallExpression>() {
            // Compile the function expression
            self.compile_expression(&*call_expr.function)?;
            
            // Compile each argument
            for arg in &call_expr.arguments {
                self.compile_expression(&**arg)?;
            }
            
            // Emit call instruction with argument count
            self.emit(Opcode::Call, vec![call_expr.arguments.len()]);
            Ok(())
        } else {
            // Other expression types...
        }
    }
    
    // Helper methods for scope management
    fn enter_scope(&mut self) {
        let scope = CompilationScope::default();
        self.scopes.push(scope);
        self.scope_index += 1;
        
        // Create a new symbol table with the current one as its outer scope
        let outer = Rc::clone(&self.symbol_table);
        self.symbol_table = Rc::new(RefCell::new(SymbolTable::with_outer(outer.borrow().clone())));
    }
    
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
}
```

## 6. Advanced Data Structures

### Implement Arrays

```rust
impl Compiler {
    // Add to compile_expression method
    fn compile_expression(&mut self, expr: &dyn Expression) -> Result<(), Error> {
        // Existing expression handling...
        
        if let Some(array_literal) = expr.as_any().downcast_ref::<ast::ArrayLiteral>() {
            // Compile each array element
            for element in &array_literal.elements {
                self.compile_expression(&**element)?;
            }
            
            // Emit array instruction with element count
            self.emit(Opcode::Array, vec![array_literal.elements.len()]);
            Ok(())
        } else {
            // Other expression types...
        }
    }
}
```

### Implement Hash Maps

```rust
impl Compiler {
    // Add to compile_expression method
    fn compile_expression(&mut self, expr: &dyn Expression) -> Result<(), Error> {
        // Existing expression handling...
        
        if let Some(hash_literal) = expr.as_any().downcast_ref::<ast::HashLiteral>() {
            // Compile key-value pairs in alternating order
            for (key, value) in &hash_literal.pairs {
                self.compile_expression(&**key)?;
                self.compile_expression(&**value)?;
            }
            
            // Emit hash instruction with pair count (key-value pairs * 2)
            self.emit(Opcode::Hash, vec![hash_literal.pairs.len() * 2]);
            Ok(())
        } else {
            // Other expression types...
        }
    }
    
    // Add index expression support (for array[index] and hash[key])
    fn compile_expression(&mut self, expr: &dyn Expression) -> Result<(), Error> {
        // Existing expression handling...
        
        if let Some(index_expr) = expr.as_any().downcast_ref::<ast::IndexExpression>() {
            // Compile the left expression (array or hash)
            self.compile_expression(&*index_expr.left)?;
            
            // Compile the index expression
            self.compile_expression(&*index_expr.index)?;
            
            // Emit index operation
            self.emit(Opcode::Index, vec![]);
            Ok(())
        } else {
            // Other expression types...
        }
    }
}
```

## 7. Error Handling and Optimization

### Error Handling

Enhance error reporting:

```rust
#[derive(Debug, Clone)]
pub struct CompilerError {
    pub message: String,
    pub location: Option<SourceLocation>,
}

impl Compiler {
    // Enhance errors with source location
    fn error(&self, message: &str, location: Option<SourceLocation>) -> CompilerError {
        CompilerError {
            message: message.to_string(),
            location,
        }
    }
    
    // Use this for error reporting in all compilation methods
    fn compile_expression(&mut self, expr: &dyn Expression) -> Result<(), CompilerError> {
        // Example error handling
        if let Some(identifier) = expr.as_any().downcast_ref::<ast::Identifier>() {
            let symbol = match self.symbol_table.borrow_mut().resolve(&identifier.value) {
                Some(symbol) => symbol,
                None => {
                    return Err(self.error(
                        &format!("Undefined variable: {}", identifier.value),
                        identifier.token.location,
                    ));
                }
            };
            
            // Continue with normal compilation...
        }
    }
}
```

### Basic Optimizations

Implement constant folding for simple cases:

```rust
impl Compiler {
    // Add constant folding optimization
    fn optimize_constant_folding(&mut self, expr: &dyn Expression) -> Option<Object> {
        // Handle infix expressions with constant operands
        if let Some(infix_expr) = expr.as_any().downcast_ref::<ast::InfixExpression>() {
            // Try to get constant values for both sides
            let left_const = self.expr_as_constant(&*infix_expr.left);
            let right_const = self.expr_as_constant(&*infix_expr.right);
            
            // If both sides are constants, fold the operation
            if let (Some(left), Some(right)) = (left_const, right_const) {
                match infix_expr.operator.as_str() {
                    "+" => self.fold_add(left, right),
                    "-" => self.fold_sub(left, right),
                    "*" => self.fold_mul(left, right),
                    "/" => self.fold_div(left, right),
                    // Add more operations...
                    _ => None,
                }
            } else {
                None
            }
        } else {
            None
        }
    }
    
    // Helper to extract constant value from expression
    fn expr_as_constant(&self, expr: &dyn Expression) -> Option<Object> {
        if let Some(int_lit) = expr.as_any().downcast_ref::<ast::IntegerLiteral>() {
            Some(Object::Integer(int_lit.value))
        } else if let Some(bool_lit) = expr.as_any().downcast_ref::<ast::BooleanLiteral>() {
            Some(Object::Boolean(bool_lit.value))
        } else if let Some(string_lit) = expr.as_any().downcast_ref::<ast::StringLiteral>() {
            Some(Object::String(string_lit.value.clone()))
        } else {
            None
        }
    }
    
    // Folding operations
    fn fold_add(&self, left: Object, right: Object) -> Option<Object> {
        match (left, right) {
            (Object::Integer(l), Object::Integer(r)) => Some(Object::Integer(l + r)),
            (Object::String(l), Object::String(r)) => Some(Object::String(l + &r)),
            _ => None,
        }
    }
    
    // Add more folding methods for other operations...
}
```

## 8. Putting It All Together

Here's a complete example of using the compiler to compile and execute a CURSED program:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Source code
    let input = r#"
        let x = 10;
        let y = 20;
        let add = fn(a, b) { return a + b; };
        let result = add(x, y);
        result;
    "#;
    
    // Parse the source code
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program()?;
    
    // Check for parser errors
    if !parser.errors().is_empty() {
        for error in parser.errors() {
            eprintln!("Parser error: {}", error);
        }
        return Err("Failed to parse program".into());
    }
    
    // Compile the program
    let mut compiler = Compiler::new();
    compiler.compile(&program)?;
    let bytecode = compiler.bytecode();
    
    // Execute the bytecode
    let mut vm = VirtualMachine::new(bytecode);
    let result = vm.run()?;
    
    // Print the result
    println!("Result: {}", result);
    
    Ok(())
}
```

## 9. Best Practices

### Organization

- Separate the compiler code into modules based on functionality
- Keep opcodes and bytecode definitions in their own module
- Maintain clear distinction between compilation phases

### Testing

- Unit test each compilation component separately
- Integration test end-to-end compilation and execution
- Create test cases for edge cases and error conditions

### Documentation

- Document opcode and bytecode formats
- Provide examples of compiled code
- Explain optimization strategies

### Performance

- Profile compilation time for large programs
- Optimize symbol table lookups
- Cache frequently accessed values

## 10. Conclusion

By following this implementation guide, you should be able to build a working CURSED compiler that translates AST nodes into bytecode for the virtual machine. The key aspects to focus on are:

1. Strong bytecode representation
2. Proper symbol table management
3. Robust scope handling
4. Clear error reporting
5. Optimization techniques

With these components in place, your compiler will be ready to tackle even complex CURSED programs. 