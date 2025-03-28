# CURSED Compiler Specification

## 1. Overview

The CURSED compiler translates the Abstract Syntax Tree (AST) representation of CURSED programs into bytecode that can be executed by the CURSED virtual machine. This document specifies the design, architecture, and implementation details of the compiler according to the language specifications.

## 2. Compiler Architecture

The compiler follows a single-pass compilation approach, traversing the AST once to generate bytecode. The main components of the compiler are:

### 2.1 Bytecode Generation
- **Instructions**: Sequences of bytes representing operations and their operands
- **Constants Pool**: A collection of constant values used in the program
- **Opcodes**: Enumeration of all supported bytecode operations

### 2.2 Symbol Management
- **Symbol Table**: Tracks identifiers and their scopes
- **Scope Management**: Handles global, local, free, and function scopes

### 2.3 Compilation Process
- **Statement Compilation**: Translates statements into bytecode operations
- **Expression Compilation**: Converts expressions into bytecode sequences
- **Control Flow**: Implements jumping and conditional execution

## 3. Bytecode Format

### 3.1 Instruction Format
Each instruction consists of:
- An opcode (1 byte)
- Zero or more operands (varying width)

### 3.2 Operand Encoding
- 1-byte operands: For small-range values (0-255)
- 2-byte operands: For larger ranges (0-65535), stored in big-endian format

### 3.3 Bytecode Structure
The bytecode output consists of:
- Instruction sequence
- Constants pool (objects referenced by index)

## 4. Opcode Specification

### 4.1 Constants and Literals
- `Constant (0x01)`: Load constant from pool [2-byte operand: index]
- `True (0x06)`: Push boolean true
- `False (0x07)`: Push boolean false
- `Null (0x0F)`: Push null value

### 4.2 Arithmetic Operations
- `Add (0x02)`: Addition
- `Sub (0x03)`: Subtraction
- `Mul (0x04)`: Multiplication
- `Div (0x05)`: Division
- `Modulo (0x22)`: Modulo operation

### 4.3 Comparison Operations
- `Equal (0x08)`: Equality check
- `NotEqual (0x09)`: Inequality check
- `GreaterThan (0x0A)`: Greater than comparison
- `GreaterThanEqual (0x1F)`: Greater than or equal comparison
- `LessThan (0x20)`: Less than comparison
- `LessThanEqual (0x21)`: Less than or equal comparison

### 4.4 Unary Operations
- `Minus (0x0B)`: Numeric negation
- `Bang (0x0C)`: Logical NOT
- `Dup (0x23)`: Duplicate the top value on the stack

### 4.5 Variable Operations
- `GetGlobal (0x11)`: Get global variable [2-byte operand: index]
- `SetGlobal (0x10)`: Set global variable [2-byte operand: index]
- `GetLocal (0x19)`: Get local variable [1-byte operand: index]
- `SetLocal (0x18)`: Set local variable [1-byte operand: index]
- `GetBuiltin (0x1A)`: Get builtin function [1-byte operand: index]
- `GetFree (0x1C)`: Get free variable (closure) [1-byte operand: index]

### 4.6 Control Flow
- `Jump (0x0E)`: Unconditional jump [2-byte operand: offset]
- `JumpNotTruthy (0x0D)`: Conditional jump [2-byte operand: offset]
- `Pop (0x1D)`: Pop value from stack

### 4.7 Function Operations
- `Call (0x15)`: Call function [1-byte operand: number of arguments]
- `ReturnValue (0x16)`: Return with a value
- `Return (0x17)`: Return without a value (null)
- `Closure (0x1B)`: Create closure [2-byte operand: constant index, 1-byte operand: free var count]
- `CurrentClosure (0x1E)`: Get current function's closure

### 4.8 Data Structure Operations
- `Array (0x12)`: Create array [2-byte operand: number of elements]
- `Hash (0x13)`: Create hash map [2-byte operand: number of key-value pairs * 2]
- `Index (0x14)`: Index operation for arrays and hashes

## 5. Symbol Table Implementation

### 5.1 Symbol Types
- **Global**: Variables at the top level scope
- **Local**: Variables in a local scope (function/block)
- **Free**: Variables captured from outer scopes
- **Function**: Self-reference within a function
- **Builtin**: Pre-defined functions

### 5.2 Symbol Structure
- **Name**: Identifier string
- **Scope**: One of the symbol types
- **Index**: Position in the storage location

### 5.3 Scope Management
- Nested scopes are implemented as linked symbol tables
- The outermost scope contains globals
- Inner scopes reference outer scopes through a chain

## 6. Compilation Strategies

### 6.1 Control Flow
- **If Statements**: Conditional jumps with JumpNotTruthy
- **Block Statements**: Sequential compilation of statements

### 6.2 Variable Declarations
- **Let Statements**: Define symbol and emit Set instructions

### 6.3 Functions
- **Function Literals**: Create new scope, compile body, capture free variables
- **Closures**: Track and capture variables from outer scopes

### 6.4 Expressions
- **Infix Expressions**: Compile both sides, emit operation
- **Prefix Expressions**: Compile operand, emit operation
- **Literals**: Add to constants and emit load instruction

### 6.5 Data Structures
- **Arrays**: Compile elements, emit Array instruction
- **Hash Maps**: Compile key-value pairs, emit Hash instruction

## 7. Optimization Techniques

### 7.1 Constant Folding
- Evaluate constant expressions at compile time

### 7.2 Jump Optimization
- Adjust jump targets to minimize code size

### 7.3 Stack Usage Optimization
- Minimize stack operations when possible

## 8. Error Handling

### 8.1 Compile-Time Errors
- Undefined variables
- Invalid operators
- Type mismatches (where possible at compile time)
- Syntax errors (caught by parser)

### 8.2 Error Reporting
- Source location information
- Descriptive error messages

## 9. Built-in Function Integration

### 9.1 Built-in Definition
- Built-ins are registered in the symbol table at initialization
- Each built-in has a unique index

### 9.2 Available Built-ins
- `len`: Get length of arrays and strings
- `first`, `last`, `rest`: Array operations
- `push`: Add to arrays
- `puts`, `print`: Output functions
- `type`: Get type of value
- File operations: `read_file`, `write_file`, `append_file`, `delete_file`
- Directory operations: `exists`, `is_dir`, `list_dir`, `make_dir`
- `input`: Get user input

## 10. Implementation Notes

### 10.1 Compiler State
- Constants pool
- Symbol table
- Instruction buffer
- Scope management

### 10.2 Special Cases
- Less-than operators are implemented by swapping operands and using greater-than
- Infix comparison operators have specific compilation strategies

### 10.3 Function Compilation
- Parameters are defined in the function's scope
- Implicit returns are added when needed
- Free variables are captured for closures

## 11. Future Enhancements

### 11.1 Planned Features
- Loop optimization
- More aggressive constant folding
- Tail call optimization
- Type checking and inference

### 11.2 Performance Improvements
- Instruction compaction
- Register-based bytecode
- Just-in-time compilation

### 11.3 Advanced Features
- Module system
- Import/export system
- Advanced type system

## 12. Example Compilation

### 12.1 Sample Program
```
let x = 10;
let y = 20;
let add = fn(a, b) { return a + b; };
let result = add(x, y);
```

### 12.2 Compiled Bytecode
```
0000 CONSTANT 0    ; 10
0003 SET_GLOBAL 0  ; x
0006 CONSTANT 1    ; 20
0009 SET_GLOBAL 1  ; y
0012 CONSTANT 2    ; compiled function
0015 SET_GLOBAL 2  ; add
0018 GET_GLOBAL 2  ; add
0021 GET_GLOBAL 0  ; x
0024 GET_GLOBAL 1  ; y
0027 CALL 2        ; call with 2 args
0029 SET_GLOBAL 3  ; result
0032 POP
```

### 12.3 Constants Pool
```
[0] = 10
[1] = 20
[2] = CompiledFunction { instructions: [GET_LOCAL 0, GET_LOCAL 1, ADD, RETURN_VALUE], num_locals: 2, num_parameters: 2 }
```

## 13. Conclusion

The CURSED compiler is designed to efficiently translate the high-level language constructs into bytecode that can be executed by the virtual machine. It follows a straightforward single-pass approach while still supporting advanced features like closures and lexical scoping.

This specification serves as a guide for both implementers and users of the CURSED language, providing insights into how the language's semantics are translated into executable code. 