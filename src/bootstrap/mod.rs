/// CURSED Bootstrap System
/// 
/// This module provides the bootstrap verification infrastructure for CURSED's
/// self-hosting capability. The bootstrap process enables CURSED to compile itself
/// through multiple stages, proving the language's completeness and self-sufficiency.
/// 
/// ## Bootstrap Stages
/// 
/// - **Stage 1**: Rust-based CURSED compiler (bootstrap compiler)
/// - **Stage 2**: CURSED-based compiler (written in CURSED, compiled by Stage 1)
/// - **Stage 3+**: Iterative self-compilation for convergence testing
/// 
/// ## Stage 2 Self-Hosting Compiler
/// 
/// The Stage 2 compiler is a complete CURSED compiler implementation written
/// entirely in CURSED syntax, located in `src/bootstrap/stage2/`:
/// 
/// - **`main.csd`** - Main compiler entry point and CLI
/// - **`lexer.csd`** - Lexical analysis (source → tokens)
/// - **`parser.csd`** - Recursive descent parser (tokens → AST)
/// - **`type_checker.csd`** - Semantic analysis and type validation
/// - **`codegen.csd`** - LLVM IR code generation
/// - **`error.csd`** - Comprehensive error handling system
/// - **`test_simple.csd`** - Test programs for validation
/// 
/// This represents over 3,500 lines of CURSED code implementing a complete
/// compilation pipeline with Gen Z slang syntax.

pub mod self_compilation_verification;

pub use self_compilation_verification::*;
