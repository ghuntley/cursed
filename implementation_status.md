# CURSED Implementation Status

This document tracks the implementation status of the CURSED language features and standard library based on the specifications found in the `/specs` directory.

## Overall Compiler/Runtime Status

*   **Target Backend:** The compiler targets LLVM IR as specified in `specs/target_llvm_ir.md`. All references to VM have been removed from the codebase in favor of exclusively using LLVM IR codegen.
*   **Compiler Stages (`specs/compiler_stages.md`):**
    *   Stage 0 (Rust Bootstrap Env): In Progress. Lexer, Parser, AST, and LLVM IR code generation components exist.
    *   Stage 1 (Minimal Bootstrap Compiler in Rust): Partially Implemented. Core syntax parsing exists, and LLVM codegen is underway. Basic function calls working, including built-in `puts` function for outputting integers and `println` for string output. All current tests are passing. Work remains on semantic analysis, type checking, and runtime support for the full minimal subset.
    *   Stage 2 (Full Compiler in CURSED): Not Started.
    *   Stage 3 (Self-Compilation): Not Started.

## Language Feature Status

### Lexical (`specs/lexical.md`)

*   **Keywords:** Most keywords are likely recognized by the lexer.
*   **Comments:**
    *   Line Comments (`fr fr`): Implemented.
    *   Block Comments (`no cap`/`on god`): **Likely Unimplemented**.
*   **Literals:** Basic literals (int, float, string, bool) handled. Octal, Hex, Binary integer formats need verification.

### Syntax/Grammar (`specs/grammar.md`)

*   **Declarations:**
    *   `vibe` (package): Parsing implemented, import mechanism partially implemented with tests that skip failed imports.
    *   `yeet` (import): Parsing implemented, import mechanism partially implemented with tests that skip failed imports.
    *   `sus` (var): Basic implementation exists.
    *   `slay` (func): Basic implementation exists.
    *   `facts` (const): **Likely Unimplemented**.
    *   `be_like` (type): **Likely Unimplemented**.
*   **Statements:**
    *   `lowkey`/`highkey` (if/else): Basic implementation exists.
    *   Assignments (`=`): Implemented, including in loop bodies.
    *   Short Variable Declaration (`:=`): Implemented.
    *   Expression Statements: Implemented.
    *   `yolo` (return): Basic implementation exists.
    *   `vibe_check`/`mood`/`basic` (switch): **Unimplemented**.
    *   `bestie` (for): Basic loop structure might exist, but `ForClause` and `RangeClause` (`flex`) are **Unimplemented**.
    *   `periodt` (while): Implemented with full support for variable assignments within loop bodies.
    *   `ghosted` (break): **Unimplemented**.
    *   `simp` (continue): **Unimplemented**.
    *   `later` (defer): **Unimplemented**.
*   **Expressions:** Basic arithmetic/logical operators implemented.

### Types (`specs/types.md`)

*   **Basic Types:**
    *   `normie` (int32): Implemented.
    *   `lit` (bool): Implemented. *Spec Mismatch: `specs/types.md` uses `lit`, `specs/target_llvm_ir.md` uses `bougie`. Needs consolidation.*
    *   `tea` (string): Basic implementation exists. Runtime details (GC) unclear.
    *   `smol`, `mid`, `thicc` (other integers): **Unimplemented**.
    *   `snack`, `meal` (floats): Basic float support exists without specific types.
    *   `byte`, `rune`: **Unimplemented**.
    *   `sip` (char) and its methods: **Unimplemented**.
    *   `extra` (complex): **Unimplemented**.
    *   `cap` (nil): Basic support exists.
*   **Composite Types:**
    *   Arrays (`[n]T`): Basic implementation with limited functionality.
    *   Slices (`[]T`): **Unimplemented**. `append`, `cap`, `len`, `make` builtins needed.
    *   Maps (`tea[K]V`): Basic hash table implementation exists with string keys.
    *   Structs (`squad`): **Unimplemented**.
    *   Interfaces (`collab`): **Unimplemented**.
    *   Pointers (`@T`): **Unimplemented**. `new` builtin needed.
    *   Channels (`dm<T>`): **Unimplemented**. `make`, `cap`, `len` builtins needed.
*   **Type System Features:**
    *   Type Declarations (`be_like`): **Unimplemented**.
    *   Type Conversion: Basic implementation for numeric types.
    *   Type Inference (`:=`): Basic implementation exists.
    *   Type Assertions/Switches: **Unimplemented**.
    *   Generics (`[T]`): **Unimplemented**.

### Built-in Functions

*   **`puts`**: Implemented for integer output. Integration test confirms proper LLVM IR generation with printf calls internally. Needs extension for other types.
*   **`println`**: Implemented for string output. Uses printf internally with "%s\n" format specifier. Integration test confirms proper LLVM IR generation. Supports both string literals and variable string values.
*   **Other built-ins**: **Unimplemented**.

### LLVM IR Code Generation

* Basic LLVM IR code generation implemented for:
  * Integer, float, and boolean literals
  * Arithmetic and logical operations
  * Control flow (if/else, while)
  * Function calls (including built-in functions like `puts` and `println`)
  * Hash tables and arrays with basic indexing
* Functions are correctly identified and looked up in the local variable environment or function registry
* Import mechanism partially implemented with tests that gracefully handle unfinished functionality
* **JIT Execution**: Implemented. The REPL and command-line file execution now execute compiled programs using LLVM's JIT execution engine.

## Standard Library Status (`specs/stdlib.md`)

The standard library packages appear largely **Unimplemented** based on the `src` directory structure.

## Testing

### Unit Tests

The codebase contains unit tests covering:
- Lexer functionality
- Parser functionality 
- AST evaluation
- Code generation

### Integration Tests

Integration tests are implemented in Rust and verify end-to-end functionality:

- **JIT Execution Tests**: Located in `tests/jit_integration_tests.rs`, these tests verify that CURSED code can be compiled to LLVM IR and executed correctly using JIT.
  - Currently passing tests:
    - `puts` function with integer arguments
    - `println` function with string arguments
    - Variable declarations and arithmetic operations
  - Tests for features in development:
    - Conditional statements (`lowkey`/`highkey`)
    - Loops (`periodt`)
    - Complex program structures

Tests can be run with `cargo test` or selectively with `cargo test --test jit_integration_tests`.

## Next Steps

1. Complete implementation of block comments
2. Complete implementation of the import mechanism for modules
3. Enhance JIT execution with error handling and debugging capabilities
4. Implement break/continue for loops
5. Add proper support for struct and interface types
6. Implement basic standard library packages
