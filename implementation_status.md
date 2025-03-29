# CURSED Language Implementation Status

This document tracks the current implementation status of the CURSED programming language based on the `specs` and the code in the `src` directory.

## Current Implementation Status Overview

The CURSED language implementation is progressing. Core components like the Lexer, AST, Symbol Table, Parser, and basic VM execution loop are largely functional. A REPL exists. Key areas needing significant work include the Compiler, full VM feature implementation (especially related to custom types and methods), Memory Management (GC), the Standard Library, and Type Checking.

## Component Status

| Component         | Status         | Notes                                                                                                                                  | Testing                  |
|-------------------|----------------|----------------------------------------------------------------------------------------------------------------------------------------|--------------------------|
| **Project Setup** | ✅ Completed   | Basic structure, modules.                                                                                                              | N/A                      |
| **Lexer**         | ✅ Completed   | Handles keywords, identifiers, literals (int, float, string, char), operators, punctuation. Needs comment handling verification.         | Property, Unit           |
| **AST**           | ✅ Completed   | Defines nodes for most language constructs. Needs FloatLiteral node, struct instantiation node.                                          | Unit                     |
| **Parser**        | ✅ Completed   | Parses all major statements and expressions defined in grammar. Needs FloatLiteral parsing, struct instantiation (`be_like ... with {}`). | Unit (Good Coverage)     |
| **Symbol Table**  | ✅ Completed   | Handles symbol definition, resolution, nested scopes, builtins, free variables.                                                          | Property, Unit           |
| **Compiler**      | 🔴 Stubbed     | `src/compiler/mod.rs` is a basic stub. `compiler_implementation.rs` is WIP, not integrated. Needs full implementation.                 | Stub Tests Only          |
| **Bytecode**      | 🟡 In Progress | Definitions exist (`bytecode.rs`). Needs validation against full compiler. Some newer opcodes may need refinement.                       | Needs Compiler Integration |
| **VM**            | 🟡 In Progress | Executes many opcodes (arithmetic, logic, stack, control flow, globals, locals, array, hash, index, call, return, closure, builtins). Needs method call execution, instance creation/access, full error handling. | Unit (Core VM), Needs More |
| **Object System** | ✅ Completed   | `Object` enum covers primitives, collections, functions, closures, types, instances, methods. `Traceable` implemented.                   | Needs Runtime Tests      |
| **Memory Mgmt/GC**| 🔴 Stubbed     | `memory/` contains stubs. Needs functional GC implementation.                                                                           | Needs Tests              |
| **Evaluator**     | 🔴 Stubbed     | Tree-walking interpreter stub exists but is not functional.                                                                            | Needs Tests              |
| **Error Handling**| 🟡 In Progress | Basic `Error` enum and `ErrorReporter`. Needs integration with Compiler/VM for comprehensive reporting.                               | Needs Integration Tests  |
| **REPL**          | ✅ Completed   | Basic REPL functionality implemented.                                                                                                | Needs Tests              |
| **Standard Lib**  | ❌ Not Started | Basic builtins in VM exist. Full `stdlib.md` implementation pending.                                                                   | Needs Tests              |
| **Type Checker**  | ❌ Not Started | Implementation pending.                                                                                                              | Needs Tests              |

## Feature Status (Parser/AST Level)

*   **Literals:** ✅ Integer, String, Boolean, Char, Null, Array (`crew`), Hash (`tea`), Function (`stan`). ❌ Float (Lexed only).
*   **Identifiers:** ✅
*   **Keywords:** ✅ (Lexer handles them)
*   **Operators:** ✅ Arithmetic, Comparison, Logical (`!`), Assignment (`=`).
*   **Statements:** ✅ `vibe`, `yeet`, `sus`, `facts`, `yolo`, `lowkey`/`highkey`, `periodt`, `bestie`, `vibe_check`/`mood`/`basic`, `be_like squad`/`collab`, `slay`, Expression Statements.
*   **Expressions:** ✅ Identifiers, Prefix, Infix, Grouped, Call, Index, Assignment, Array Literals, Hash Literals, Function Literals. ❌ Struct Instantiation (`be_like ... with {}`).

## Unimplemented Features / Areas Needing Work

*   **Compiler Implementation:** The biggest gap. Needs full implementation and integration, replacing the current stub. This includes:
    *   Compiling all expression types.
    *   Compiling all statement types (control flow, variable declarations, type/interface/method declarations).
    *   Correct handling of scopes (locals, globals, free variables).
    *   Function and closure compilation.
    *   Generating correct bytecode for all features.
*   **VM Enhancements:**
    *   Implementing opcodes and logic for struct instance creation and field access/modification.
    *   Implementing method call execution (`Opcode::Method`).
    *   Robust error handling and stack traces during execution.
    *   Testing and potential refinement of `Opcode::VariadicCall`.
*   **Memory Management:** Implementing a working Garbage Collector.
*   **Standard Library:** Implementing modules and functions defined in `specs/stdlib.md`.
*   **Type System & Type Checker:** Implementing static type checking.
*   **Parser Additions:**
    *   Parsing Float Literals into a distinct AST node.
    *   Parsing struct instantiation expressions (`be_like MyStruct with { field: value }`).
*   **Testing:** Expanding test coverage, especially for Compiler, VM runtime behavior, GC, and Standard Library.

## Potential Next Steps

*   **Address Compiler Warnings:** Clean up the existing warnings related to unused code in the stubbed/incomplete components (`vm.rs`, `compiler/mod.rs`, `evaluator.rs`, etc.). This is a good housekeeping step.
*   **Implement Float Literal Parsing:** Add `FloatLiteral` to `ast.rs` and update the parser in `parser.rs` to handle it. Add corresponding tests.
*   **Implement Struct Instantiation Parsing:** Add `StructInstantiation` (or similar) to `ast.rs`, update the parser to handle `be_like Type with { ... }`, and add tests.
*   **Begin Compiler Integration:** Start replacing the stub compiler (`compiler/mod.rs`) with the logic from `compiler_implementation.rs`, focusing on one feature area at a time (e.g., basic expressions, variable handling). This is a major task.
*   **Enhance VM for Instances:** Implement the VM logic needed to create and interact with struct instances (e.g., opcodes for instantiation, field getting/setting). This requires corresponding compiler support later.
*   **Start Standard Library Module:** Pick a simple module from `specs/stdlib.md` (e.g., `math` or `string`) and start implementing its functions, likely as VM builtins initially.
