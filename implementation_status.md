# CURSED Language Implementation Status

This document tracks the current implementation status of the CURSED programming language based on the `specs` and the code in the `src` directory.

## Current Implementation Status Overview

The CURSED language implementation is underway. Core components like the lexer, AST, symbol table, and parts of the parser and VM are implemented. A basic REPL exists. However, the compiler currently used is a stub, memory management is not fully implemented, and several language features and the standard library are pending.

## Component Status

| Component         | Status         | Notes                                                                                                | Testing         |
|-------------------|----------------|------------------------------------------------------------------------------------------------------|-----------------|
| **Project Setup** | ✅ Completed   | Basic structure, modules.                                                                            | N/A             |
| **Lexer**         | ✅ Completed   | Handles keywords, identifiers, literals (int, float, string, char), operators, punctuation, comments. | Property, Unit  |
| **AST**           | ✅ Completed   | Defines nodes for parsed language constructs (expressions, statements, types, etc.).                   | Unit            |
| **Parser**        | 🟡 In Progress | Parses: package, import, type (squad), interface (collab), method (`slay`), while (`periodt`), const (`facts`), return (`yolo`), if (`lowkey`/`highkey`), expressions. Needs: let (`sus`), for (`bestie`), switch (`vibe_check`), some expression types (struct instantiation, method calls). | Unit (Basic)    |
| **Symbol Table**  | ✅ Completed   | Handles symbol definition and resolution, including basic nested scopes.                               | Property, Unit  |
| **Compiler**      | 🔴 Stubbed     | `src/lib.rs` uses a stub compiler. `compiler_implementation.rs` exists but is not integrated. Needs full implementation according to specs. | Stub Tests Only |
| **Bytecode**      | 🔴 Stubbed     | Minimal definition in the stub compiler. Needs full instruction set implementation.                   | Needs Tests     |
| **VM**            | 🟡 In Progress | Core loop, stack, frames functional. Executes many opcodes (arithmetic, logic, globals, locals, array, hash, index, call, return, closure, builtins, type/interface/method definitions). Needs method call execution, instance creation/manipulation opcodes. | Unit (Good Core Coverage) |
| **Object System** | ✅ Completed   | `Object` enum defines runtime values (primitives, array, hash, functions, closures, types, instances, methods, etc.). `Traceable` implemented. | Needs Tests     |
| **Memory Mgmt/GC**| 🔴 Stubbed     | `memory/` contains stubs. Needs functional GC implementation.                                       | Needs Tests     |
| **Evaluator**     | 🔴 Stubbed     | Tree-walking interpreter stub exists but is not functional.                                            | Needs Tests     |
| **Error Handling**| ✅ Completed   | Basic `Error` enum and `ErrorReporter` implemented.                                                  | Needs Tests     |
| **REPL**          | ✅ Completed   | Basic REPL functionality implemented.                                                                | Needs Tests     |
| **Standard Lib**  | ❌ Not Started | Implementation pending based on `specs/stdlib.md`.                                                     | Needs Tests     |
| **Type Checker**  | ❌ Not Started | Implementation pending.                                                                              | Needs Tests     |

## Feature Status (Based on Grammar and Types)

*   **Literals:** ✅ Integer, Float, String, Boolean, Char, Null
*   **Identifiers:** ✅
*   **Keywords:** ✅ (Lexer handles them)
*   **Operators:**
    *   Arithmetic (`+`, `-`, `*`, `/`, `%`): ✅ (VM opcodes exist)
    *   Comparison (`==`, `!=`, `>`, `<`, `>=`, `<=`): ✅ (VM opcodes exist)
    *   Logical (`!`): ✅ (VM opcode exists)
    *   Assignment (`=`): ✅ (Parser, AST exist, Compiler needs work)
*   **Statements:**
    *   `vibe` (package): ✅ (Parser, AST, Stub Compiler)
    *   `yeet` (import): ✅ (Parser, AST, Stub Compiler)
    *   `sus` (let): ✅ (Parser, AST)
    *   `facts` (const): ✅ (Parser, AST)
    *   `yolo` (return): ✅ (Parser, AST)
    *   `lowkey`/`highkey` (if/else): ✅ (Parser, AST)
    *   `periodt` (while): ✅ (Parser, AST)
    *   `bestie` (for): ✅ (Parser, AST)
    *   `vibe_check`/`mood`

| `slay` (method)             | ✅ (Parser, AST) | ❓          | ❓        |                |
| `yolo` (return)             | ✅ (Parser, AST) | ❓          | ❓        |                |
| Expression Statement        | ✅ (Parser, AST) | ❓          | ❓        |                |
|                             |                  |             |           |                |
| **Expressions**             |                  |             |           |                |
|                             |                  |             |           |                |
| Integer Literals            | ✅ (Parser, AST) | ❓          | ❓        |                |
| String Literals             | ✅ (Parser, AST) | ❓          | ❓        |                |
| Boolean Literals (`based`/`cap`) | ✅ (Parser, AST) | ❓          | ❓        |                |
| Identifier Expressions      | ✅ (Parser, AST) | ❓          | ❓        |                |
| Prefix Expressions (`!`, `-`) | ✅ (Parser, AST) | ❓          | ❓        |                |
| Infix Expressions           | ✅ (Parser, AST) | ❓          | ❓        |                |
| Grouped Expressions (`()`)  | ✅ (Parser, AST) | ❓          | ❓        |                |
| Call Expressions (`ident()`) | ✅ (Parser, AST) | ❓          | ❓        |                |
| Index Expressions (`arr[idx]`) | ✅ (Parser, AST) | ❓          | ❓        |                |
| Assignment Expressions (`=`) | ✅ (Parser, AST) | ❓          | ❓        |                |
| Array Literals (`crew [...]`) | ✅ (Parser, AST) | ❓          | ❓        |                |
| Hash Literals (`tea {...}`)   | ✅ (Parser, AST) | ❓          | ❓        |                |
| Function Literals (`stan`)  | ✅ (Parser, AST) | ❓          | ❓        |                |
| Float Literals              | ✅ (Lexer)       | ❌          | ❌        | Not needed yet |
|                             |                  |             |           |                |
| **Other**                   |                  |             |           |                |