# LLVM Code Generator Documentation

## Overview

The LLVM Code Generator is a core component of the CURSED programming language compiler. It translates the Abstract Syntax Tree (AST) representation of CURSED code into LLVM Intermediate Representation (IR), which can then be executed using Just-In-Time (JIT) compilation or compiled to native machine code.

## Modular Architecture

The LLVM code generator has been refactored into a modular structure, with each module handling a specific aspect of code generation:

- `context.rs`: Core `LlvmCodeGenerator` struct and initialization logic
- `types.rs`: Shared type definitions and type conversion utilities
- `errors.rs`: Error handling and reporting infrastructure
- `expression.rs`: Compilation of expressions (identifiers, operations, conditionals)
- `statement.rs`: Compilation of statements (loops, imports, breaks, continues)
- `struct_type.rs`: Handling of user-defined struct types
- `channel.rs`: Implementation of concurrent channels
- `pointer.rs`: Pointer operations and memory manipulation
- `string.rs`: String handling and text processing
- `stan.rs`: Goroutine (concurrency) implementation
- `array.rs`: Array operations and management
- `hash.rs`: Hash map operations
- `binary_compiler.rs`: AOT binary compiler implementation

## Core Components

### LlvmCodeGenerator

The `LlvmCodeGenerator` struct is the central component that manages code generation. It contains:

- LLVM context, module, and builder
- Symbol table for variable tracking
- Type registry for user-defined types
- Function registry
- Runtime function declarations

### Expression Compilation

The expression module handles various types of expressions:

- Identifier references
- Prefix expressions (unary operations)
- Infix expressions (binary operations)
- If-else expressions with type conversion
- Property access for structs
- Assignment operations

### Statement Compilation

The statement module processes language statements:

- Break and continue statements for loop control
- Import statements for module inclusion
- Later (defer) statements for cleanup operations
- Loop and control flow statements

### Control Flow

Control flow structures are managed by dedicated modules:

- Loop context tracking for nested loops
- Break/continue statement handling
- Switch/case statement implementation

### Type System

The type system components include:

- Generic type specialization
- Interface implementation
- Type inference for compatible types
- Monomorphization for generic code