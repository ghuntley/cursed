Study technical rules at .cursor/rules.mdc
Study requirements at cursed/specs
Study implementation at cursed/src
Study IMPLEMENTATION_STATUS.md
Continue implementation and keep IMPLEMENTATION_STATUS.md up-to-date.
Author property based tests for the compiler and run them after each change.


# CURSED Programming Language Development

## Project Overview
CURSED is an esoteric programming language that combines Go-like grammar with Generation Z slang for keywords and tokens. It is implemented as a self-compiling compiler written in Rust.

## Key Features
- **Go-like Grammar**: Follows Go syntax patterns but replaces keywords with Gen Z slang
- **Self-Compiling Compiler**: Designed to bootstrap itself through multiple stages
- **Rust Implementation**: Initial compiler implementation in Rust, later stages in CURSED itself

## Language Specifications
The language specifications have been created under `/specs` and include:
- Overview of language design and philosophy
- Lexical structure (tokens, keywords, comments)
- Type system (primitive and composite types)
- Grammar and syntax rules
- Compiler bootstrapping process plan
- Standard library documentation

## Example CURSED Code
```
vibe main

yeet "vibez"

slay main() {
    vibez.spill("Hello, World!")  fr fr This is a comment
    
    sus name tea = "bestie"
    vibez.spillf("Hey %s, what's good?", name)

    lowkey 1 < 2 {
        vibez.spill("This is based!")
    } highkey {
        vibez.spill("This is sus!")
    }
}
```

## Compiler Bootstrap Process
CURSED follows a four-stage bootstrapping process:
1. **Stage 0**: Bootstrap environment setup using Rust
2. **Stage 1**: Minimal bootstrap compiler in Rust
3. **Stage 2**: Full compiler written in CURSED
4. **Stage 3**: Self-compiled full compiler

## Project Structure
- `/src`: Compiler source code
- `/specs`: Language specifications and documentation
- `/examples`: Example CURSED programs
- `/.cursor`: Cursor AI rules for the project

## Implementation Rules
Based on the rust-hosted-langs/book repository, we're following these implementation principles:
- Using custom memory allocators for performance
- Creating safe abstractions over unsafe operations
- Implementing a bytecode virtual machine
- Following a multi-stage compilation pipeline
- Building with Make instead of direct Cargo commands

## Current Progress
Work has begun on implementing the compiler's basic structure, including:
- CLI command structure for running, checking, and REPL
- Error handling system
- Basic directory structure for compiler components (lexer, parser, AST, VM, etc.) 