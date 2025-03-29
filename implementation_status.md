# CURSED Language Implementation Status

This document tracks the current implementation status of the CURSED programming language based on the `specs` and the code in the `src` directory.

## Current Implementation Status Overview

The CURSED language implementation is progressing. Core components like the Lexer, AST, Symbol Table, Parser, and basic VM execution loop are largely functional. A REPL exists. Key areas needing significant work include the Compiler, full VM feature implementation (especially related to custom types and methods), Memory Management (GC), the Standard Library, and Type Checking. LLVM IR code generation is now partially implemented.

## Component Status

| Component         | Status         | Notes                                                                                                                                  | Testing                  |
|-------------------|----------------|----------------------------------------------------------------------------------------------------------------------------------------|--------------------------|
| **Project Setup** | ✅ Completed   | Basic structure, modules.                                                                                                              | N/A                      |
| **Lexer**         | ✅ Completed   | Handles keywords, identifiers, literals (int, float, string, char), operators, punctuation. Handles dot notation in identifiers. Fully supports line and block comments (`fr fr` and `no cap ... on god`).         | Property, Unit           |
| **AST**           | ✅ Completed   | Defines nodes for most language constructs including PropertyExpression for dot notation access.                                          | Unit                     |
| **Parser**        | ✅ Completed   | Parses all major statements and expressions defined in grammar including dot notation for module.function calls. | Unit (Good Coverage)     |
| **Symbol Table**  | ✅ Completed   | Handles symbol definition, resolution, nested scopes, builtins, free variables.                                                          | Property, Unit           |
| **Compiler**      | ✅ Completed   | Full implementation of compiler with support for all expressions, statements, scopes, and control flow constructs.                     | Unit Tests               |
| **Bytecode**      | ✅ Completed   | Definitions exist (`bytecode.rs`). All opcodes implemented and tested.                                                                 | Unit Tests               |
| **VM**            | 🟡 In Progress | Executes many opcodes (arithmetic, logic, stack, control flow, globals, locals, array, hash, index, call, return, closure, builtins). Now supports module.function calls like vibez.spill. Needs method call execution, instance creation/access, full error handling. | Unit (Core VM), Needs More |
| **Object System** | ✅ Completed   | `Object` enum covers primitives, collections, functions, closures, types, instances, methods. `Traceable` implemented.                   | Needs Runtime Tests      |
| **Memory Mgmt/GC**| 🔴 Stubbed     | `memory/` contains stubs. Needs functional GC implementation.                                                                           | Needs Tests              |
| **Evaluator**     | 🔴 Stubbed     | Tree-walking interpreter stub exists but is not functional.                                                                            | Needs Tests              |
| **Error Handling**| 🟡 In Progress | Basic `Error` enum and `ErrorReporter`. Needs integration with Compiler/VM for comprehensive reporting.                               | Needs Integration Tests  |
| **REPL**          | ✅ Completed   | Full REPL functionality with file execution, stdin input, and command-line evaluation options.                                        | Needs Tests              |
| **Standard Lib**  | ❌ Not Started | Basic builtins in VM exist. Now supports module.function syntax via the lexer. Full `stdlib.md` implementation pending.                | Needs Tests              |
| **Type Checker**  | ❌ Not Started | Implementation pending.                                                                                                              | Needs Tests              |
| **LLVM IR Gen**   | 🟡 In Progress | Basic implementation in `codegen/llvm.rs`. Supports integer/float/boolean literals, infix expressions, variables, function calls, and if statements. Needs support for more language constructs. | Unit Tests |

## Feature Status (Parser/AST Level)

*   **Literals:** ✅ Integer, String, Boolean, Char, Null, Array (`crew`), Hash (`tea`), Function (`stan`). ❌ Float (Lexed only).
*   **Identifiers:** ✅ Support for simple identifiers and dot notation (e.g., `vibez.spill`)
*   **Keywords:** ✅ (Lexer handles them)
*   **Comments:** ✅ Line comments (`fr fr`) and block comments (`no cap ... on god`) fully supported.
*   **Operators:** ✅ Arithmetic, Comparison, Logical (`!`), Assignment (`=`).
*   **Statements:** ✅ `vibe`, `yeet`, `sus`, `facts`, `yolo`, `lowkey`/`highkey`, `periodt`, `bestie`, `vibe_check`/`mood`/`basic`, `be_like squad`/`collab`, `slay`, Expression Statements.
*   **Expressions:** ✅ Identifiers, Prefix, Infix, Grouped, Call, Index, Assignment, Array Literals, Hash Literals, Function Literals, Property Access (via dot notation). ❌ Struct Instantiation (`be_like ... with {}`).

## LLVM IR Generation Status

*   **Literals:** ✅ Integer, Boolean, Float, String. ❌ Null, Array, Hash, Function (can be defined but not used as values).
*   **Operators:** ✅ Arithmetic (+, -, *, /), Comparison (==, !=, <, >), Prefix (!, -). ❌ Logical (&&, ||), Assignment.
*   **String Operations:** ✅ String concatenation with '+', String comparison with '==' and '!='.
*   **Statements:** ✅ Let variables, Expression statements, Return statements, If statements, While loops. ❌ For loops, Function declarations.
*   **Expressions:** ✅ Identifiers, Infix expressions, Prefix expressions, Function calls. ❌ Index expressions, Property access.
*   **Control Flow:** ✅ If statements (with else), While loops. ❌ For loops, Switch/Match.
*   **Functions:** ✅ Function calls. 🟡 Function literals (can be defined, but tests use alternative approach to avoid LLVM module cloning issues).

## Unimplemented Features / Areas Needing Work

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
*   **LLVM IR Generation:** 
    *   Implementing support for arrays and hashes.
    *   Adding support for for loops and other control flow structures.
    *   Handling logical operators and assignment expressions.
    *   Implementing proper function declarations and callable function literals.
    *   Adding support for struct/type declarations and instantiations.

## Potential Next Steps

*   **Address Compiler Warnings:** Clean up the existing warnings related to unused code in the stubbed/incomplete components (`vm.rs`, `compiler/mod.rs`, `evaluator.rs`, etc.). This is a good housekeeping step.
*   **Implement Float Literal Parsing:** Add `FloatLiteral` to `ast.rs` and update the parser in `parser.rs` to handle it. Add corresponding tests.
*   **Implement Struct Instantiation Parsing:** Add `StructInstantiation` (or similar) to `ast.rs`, update the parser to handle `be_like Type with { ... }`, and add tests.
*   **Enhance VM for Instances:** Implement the VM logic needed to create and interact with struct instances (e.g., opcodes for instantiation, field getting/setting). This requires corresponding compiler support later.
*   **Start Standard Library Module:** Pick a simple module from `specs/stdlib.md` (e.g., `math` or `string`) and start implementing its functions, likely as VM builtins initially.
*   **Expand Module System:** Further enhance the module system with proper imports and exports.
*   **Extend LLVM IR Generation:** Implement support for more language constructs, especially loops, prefix expressions, and string literals.
