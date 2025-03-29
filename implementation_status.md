# CURSED Implementation Status

This document tracks the implementation status of the CURSED language features and standard library based on the specifications found in the `/specs` directory.

## Overall Compiler/Runtime Status

*   **Target Backend:** The compiler targets LLVM IR as specified in `specs/target_llvm_ir.md`. All references to VM have been removed from the codebase in favor of exclusively using LLVM IR codegen.
*   **Compiler Stages (`specs/compiler_stages.md`):**
    *   Stage 0 (Rust Bootstrap Env): In Progress. Lexer, Parser, AST, and LLVM IR code generation components exist.
    *   Stage 1 (Minimal Bootstrap Compiler in Rust): Partially Implemented. Core syntax parsing exists, and LLVM codegen is underway, but semantic analysis, type checking, and runtime support for the full minimal subset are likely incomplete.
    *   Stage 2 (Full Compiler in CURSED): Not Started.
    *   Stage 3 (Self-Compilation): Not Started.

## Language Feature Status

### Lexical (`specs/lexical.md`)

*   **Keywords:** Most keywords are likely recognized by the lexer.
*   **Comments:**
    *   Line Comments (`fr fr`): Assumed Implemented.
    *   Block Comments (`no cap`/`on god`): **Likely Unimplemented**.
*   **Literals:** Basic literals (int, float, string, bool) likely handled. Octal, Hex, Binary integer formats need verification.

### Syntax/Grammar (`specs/grammar.md`)

*   **Declarations:**
    *   `vibe` (package): Parsing likely implemented, full semantics doubtful.
    *   `yeet` (import): Parsing likely implemented, full semantics doubtful.
    *   `sus` (var): Basic implementation likely exists.
    *   `slay` (func): Basic implementation likely exists.
    *   `facts` (const): **Likely Unimplemented**.
    *   `be_like` (type): **Likely Unimplemented**.
*   **Statements:**
    *   `lowkey`/`highkey` (if/else): Basic implementation likely exists.
    *   Assignments (`=`): Likely Implemented.
    *   Short Variable Declaration (`:=`): Likely Implemented.
    *   Expression Statements: Likely Implemented.
    *   `yolo` (return): Basic implementation likely exists.
    *   `vibe_check`/`mood`/`basic` (switch): **Likely Unimplemented**.
    *   `bestie` (for): Basic loop structure might exist, but `ForClause` and `RangeClause` (`flex`) are **Likely Unimplemented**.
    *   `periodt` (while): **Likely Unimplemented**.
    *   `ghosted` (break): **Likely Unimplemented**.
    *   `simp` (continue): **Likely Unimplemented**.
    *   `later` (defer): **Likely Unimplemented**.
*   **Expressions:** Basic arithmetic/logical operators likely implemented.

### Types (`specs/types.md`)

*   **Basic Types:**
    *   `normie` (int32): Likely Implemented.
    *   `lit` (bool): Likely Implemented. *Spec Mismatch: `specs/types.md` uses `lit`, `specs/target_llvm_ir.md` uses `bougie`. Needs consolidation.*
    *   `tea` (string): Basic implementation likely exists. Runtime details (GC) unclear.
    *   `smol`, `mid`, `thicc` (other integers): **Likely Unimplemented**.
    *   `snack`, `meal` (floats): **Likely Unimplemented** (or basic float support exists without specific types).
    *   `byte`, `rune`: **Likely Unimplemented**.
    *   `sip` (char) and its methods: **Likely Unimplemented**.
    *   `extra` (complex): **Likely Unimplemented**.
    *   `cap` (nil): Basic support likely exists.
*   **Composite Types:**
    *   Arrays (`[n]T`): **Likely Unimplemented**.
    *   Slices (`[]T`): **Likely Unimplemented** (or very basic support). `append`, `cap`, `len`, `make` builtins needed.
    *   Maps (`tea[K]V`): **Likely Unimplemented**. `make`, `cap`, `len` builtins needed.
    *   Structs (`squad`): **Likely Unimplemented**.
    *   Interfaces (`collab`): **Likely Unimplemented**.
    *   Pointers (`@T`): **Likely Unimplemented**. `new` builtin needed.
    *   Channels (`dm<T>`): **Likely Unimplemented**. `make`, `cap`, `len` builtins needed.
*   **Type System Features:**
    *   Type Declarations (`be_like`): **Likely Unimplemented**.
    *   Type Conversion: **Likely Unimplemented** (beyond basic numeric casts if floats exist).
    *   Type Inference (`:=`): Basic implementation likely exists.
    *   Type Assertions/Switches: **Likely Unimplemented**.
    *   Generics (`[T]`): **Likely Unimplemented**.

### Concurrency

*   Goroutines (`stan`): **Unimplemented**.
*   Channels (`dm`): **Unimplemented**.
*   Synchronization (`concurrenz` stdlib package): **Unimplemented**.

### Memory Management

*   Garbage Collection: Required for LLVM IR target. Implementation using LLVM's GC support features (`gcroot`, statepoints) is likely **Unimplemented**. Functions like `make` and `new` will require GC support.

## LLVM IR Code Generation

*   **Basic Structure**: LLVM IR code generator structure exists in `src/codegen/llvm.rs`.
*   **Core Language Features**: Code generation for basic expressions, control flow, and functions appears to be implemented or in progress.
*   **Runtime Support**: LLVM-compatible runtime support for garbage collection, complex types, and concurrency is likely **Unimplemented**.
*   **Memory Management**: Integration with LLVM's garbage collection is likely **Unimplemented**.
*   **JIT Execution**: JIT compilation for REPL may be **Unimplemented**.

## Standard Library Status (`specs/stdlib.md`)

The standard library packages appear largely **Unimplemented** based on the `src` directory structure.

*   `vibez` (fmt): **Unimplemented**.
*   `core` (builtin): Partially implemented? (`len`, maybe others depending on code generator). `append`, `cap`, `make`, `new`, `panic`, `recover` likely **Unimplemented/Incomplete**.
*   `dropz` (io): **Unimplemented**.
*   `vibe_life` (os): **Unimplemented**.
*   `stringz` (strings): **Unimplemented**.
*   `mathz` (math): **Unimplemented**.
*   `timez` (time): **Unimplemented**.
*   `concurrenz` (sync): **Unimplemented**.
*   `web_vibez` (net/http): **Unimplemented**.
*   `json_tea` (encoding/json): **Unimplemented**.

## Testing

*   While not explicitly listed as unimplemented features, the specs mention a testing strategy. The status of unit, integration, end-to-end, regression, compliance, and self-hosting tests is **Unknown**. Test authoring should proceed alongside feature implementation.
*   The LLVM codegen component has some unit tests.

---

**Next Steps:**

1.  Complete the LLVM IR codegen for the minimal language feature set.
2.  Implement runtime support for GC and complex types using LLVM features.
3.  Begin implementing missing basic types and composite types (`squad`, slices, maps).
4.  Start building the core standard library packages (`vibez`, `core`, `stringz`).
5.  Address missing control flow structures.
6.  Implement block comments.
7.  Develop tests concurrently with features.
8.  Add JIT execution capability for the REPL.
