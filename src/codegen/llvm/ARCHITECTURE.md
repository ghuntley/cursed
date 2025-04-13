# LLVM Code Generator Architecture

This document provides an overview of the LLVM code generator architecture for the CURSED language compiler.

## Architecture Diagram

```
┌─────────────────────────────────────────┐
│                  mod.rs                 │
│           (Re-exports & Entry)          │
└───────────────┬─────────────────────────┘
                │
                ▼
┌─────────────────────────────────────────┐
│              context.rs                 │
│     (LlvmCodeGenerator definition)      │
└───────────────┬─────────────────────────┘
                │
                ├─────────────┬───────────────┬───────────────┬───────────────┐
                │             │               │               │               │
                ▼             ▼               ▼               ▼               ▼
┌──────────────────┐  ┌──────────────┐  ┌───────────────┐ ┌──────────────┐ ┌──────────────┐
│ pointer_ops.rs   │  │ expression.rs│  │statement.rs   │ │variables.rs  │ │ basic_expr.rs│
│(Pointer handling)│  │(Expressions) │  │ (Statements)  │ │ (Variables)  │ │ (Literals)   │
└──────────────────┘  └──────────────┘  └───────────────┘ └──────────────┘ └──────────────┘
        │                   │                 │                 │                │
        │                   │                 │                 │                │
        ▼                   ▼                 ▼                 ▼                ▼
┌──────────────────┐  ┌──────────────┐  ┌───────────────┐ ┌──────────────┐ ┌──────────────┐
│  pointer.rs      │  │function_mono.│  │control_flow.rs│ │struct_mono.rs│ │intrinsics.rs │
│(Legacy adapter)  │  │(Generic funcs)│  │ (If/while/for)│ │(Generic types)│ │(Stdlib impl) │
└──────────────────┘  └──────────────┘  └───────────────┘ └──────────────┘ └──────────────┘
```

## Module Responsibilities

### Core Modules

- **mod.rs**: Entry point, re-exports public types and traits
- **context.rs**: Defines the `LlvmCodeGenerator` struct and core methods
- **generator.rs**: Backward compatibility adapter for context.rs

### Expression Handling

- **expression.rs**: General expression compilation
- **basic_expressions.rs**: Literals and arithmetic expressions
- **pointer_ops.rs**: Pointer operations (address-of, dereference)

### Statement Handling

- **statement.rs**: General statement compilation
- **control_flow.rs**: If, while, for, switch statements
- **break_continue.rs**: Break and continue statements

### Type System

- **variables.rs**: Variable declaration and scope management
- **struct_monomorphization.rs**: Generic struct specialization
- **function_monomorphization.rs**: Generic function specialization

### Runtime Support

- **intrinsics.rs**: Standard library intrinsic functions
- **concurrency.rs**: Goroutines and channels

## Data Flow

1. The AST is passed to `LlvmCodeGenerator.compile()`
2. The program is traversed, dispatching to specialized handlers:
   - Statements → `StatementCompilation` trait
   - Expressions → `ExpressionCompilation` trait
   - Pointers → `PointerOperations` trait
   - Variables → `VariableHandling` trait
3. Each handler generates LLVM IR using the LLVM builder
4. The resulting LLVM module can be:
   - JIT-compiled and executed
   - Output as LLVM IR text
   - Compiled to native code

## Extension Pattern

To add new functionality:

1. Create a new module (e.g., `my_feature.rs`)
2. Define a trait for the functionality:
   ```rust
   pub trait MyFeatureOperations<'ctx> {
       fn compile_my_feature(&mut self, ...) -> Result<..., Error>;
   }
   ```
3. Implement the trait for `LlvmCodeGenerator`
4. Re-export the trait in `mod.rs`

This architecture ensures separation of concerns while maintaining a cohesive API. 