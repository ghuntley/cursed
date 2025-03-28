# CURSED Compiler Implementation Reference

This document describes the implementation of the compiler for the CURSED programming language, based on the specifications in the `specs` directory. The compiler translates AST (Abstract Syntax Tree) nodes into bytecode that can be executed by the CURSED virtual machine.

## Compiler Structure

The compiler consists of several key components:

1. **Bytecode Definitions**: Opcodes and their operand specifications
2. **Symbol Table**: Tracks variables and their scopes
3. **Compilation Scope**: Manages the current compilation context
4. **Core Compiler Logic**: Methods to compile expressions, statements, and programs

## Compiler API

### Main Compiler Methods

```rust
// Create a new compiler instance
pub fn new() -> Self

// Compile a program into bytecode
pub fn compile(&mut self, program: &Program) -> Result<Bytecode, Error>

// Compile a statement
pub fn compile_statement(&mut self, stmt: &dyn Statement) -> Result<(), Error>

// Compile an expression
pub fn compile_expression(&mut self, expr: &dyn Expression) -> Result<(), Error>

// Compile a function literal
pub fn compile_function_literal(&mut self, func: &ast::FunctionLiteral) -> Result<(), Error>

// Get the bytecode from the current compiler state
pub fn bytecode(&self) -> Bytecode
```

### Expression Compilation

The `compile_expression` method handles different expression types:

- **Integer Literals**: Adds the value to constants and emits a Constant opcode
- **Float Literals**: Adds the value to constants and emits a Constant opcode
- **String Literals**: Adds the value to constants and emits a Constant opcode
- **Boolean Literals**: Emits True or False opcodes
- **Identifier Expressions**: Resolves the symbol and emits GetGlobal, GetLocal, GetBuiltin, etc.
- **Prefix Expressions**: Compiles the right expression and then emits Bang or Minus opcodes
- **Infix Expressions**: Compiles both expressions and emits the appropriate operation opcode
- **If Expressions**: Compiles condition, consequence, and alternative with appropriate jumps
- **Function Literals**: Enters a new scope, compiles parameters and body, and creates a closure
- **Call Expressions**: Compiles function and arguments, then emits a Call opcode
- **Array Literals**: Compiles each element and emits an Array opcode
- **Hash Literals**: Compiles each key-value pair and emits a Hash opcode
- **Index Expressions**: Compiles the left expression and index, then emits an Index opcode

### Statement Compilation

The `compile_statement` method handles different statement types:

- **Expression Statements**: Compiles the expression and emits Pop to discard the value
- **Let Statements**: Compiles the value expression, defines the symbol, and emits SetGlobal/SetLocal
- **Return Statements**: Compiles the return value and emits ReturnValue or Return
- **Block Statements**: Compiles each statement in the block
- **If Statements**: Compiles the condition, consequence, and alternative with jumps

### Scope Management

The compiler maintains a stack of compilation scopes to handle nested blocks and functions:

```rust
// Enter a new compilation scope
fn enter_scope(&mut self)

// Leave the current scope and return to the outer scope
fn leave_scope(&mut self) -> Instructions
```

### Symbol Table

The compiler uses a symbol table to track variables across different scopes:

```rust
// Define a symbol in the current scope
fn define(&mut self, name: &str) -> Symbol

// Resolve a symbol by name
fn resolve(&mut self, name: &str) -> Option<Symbol>

// Define a builtin function
fn define_builtin(&mut self, index: usize, name: &str) -> Symbol
```

### Bytecode Generation

The compiler emits bytecode using these methods:

```rust
// Add a constant to the constants pool
pub fn add_constant(&mut self, obj: Object) -> usize

// Emit an instruction with operands
pub fn emit(&mut self, op: Opcode, operands: Vec<usize>) -> usize

// Change an operand at the given position
pub fn change_operand(&mut self, op_pos: usize, operand: usize)
```

## Bytecode Representation

The bytecode consists of instructions and constants:

```rust
pub struct Bytecode {
    pub instructions: Instructions,
    pub constants: Vec<Object>,
}
```

### Opcodes

Here are the key opcodes supported by the compiler:

- **Constant**: Load a constant from the constant pool
- **Add, Sub, Mul, Div**: Arithmetic operations
- **Equal, NotEqual, GreaterThan, LessThan**: Comparison operations
- **True, False, Null**: Push literal values
- **Bang, Minus**: Unary operations
- **SetGlobal, GetGlobal**: Access global variables
- **SetLocal, GetLocal**: Access local variables
- **Jump, JumpNotTruthy**: Control flow
- **Call**: Function calls
- **ReturnValue, Return**: Function returns
- **Array, Hash**: Create composite data structures
- **Index**: Access elements of arrays or hashes
- **Closure, GetFree**: Handle closures
- **Pop**: Discard a value from the stack

## Implementation Notes

1. **Scoping**: The compiler creates a new scope for each function and block that introduces new variables.
2. **Closures**: The compiler captures free variables and creates closures for functions that reference variables from outer scopes.
3. **Optimizations**: Some simple optimizations are performed, such as replacing Pop with ReturnValue for function returns.
4. **Error Handling**: Compilation errors include information about where they occurred.

## Example: Compiling a Let Statement

Here's how the compiler handles a `let` statement:

1. Compile the value expression, leaving the result on the virtual stack
2. Define the variable name in the symbol table, getting a symbol with scope and index
3. Emit a SetGlobal or SetLocal instruction with the symbol index as the operand

```rust
if let Some(let_stmt) = stmt.as_any().downcast_ref::<ast::LetStatement>() {
    // Compile the value expression
    if let Some(value) = &let_stmt.value {
        self.compile_expression(&**value)?;
    } else {
        // If no value provided, use null
        self.emit(Opcode::Null, vec![]);
    }
    
    // Define the variable in the symbol table
    let symbol = self.symbol_table.borrow_mut().define(&let_stmt.name.value);
    
    // Emit the appropriate set instruction based on scope
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
}
```

## Next Steps for Implementation

1. Support for loops and switch statements
2. Type checking and validation
3. Advanced optimizations
4. Method resolution and object-oriented features
5. Module system and imports 