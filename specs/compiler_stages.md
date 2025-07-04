# CURSED Compiler Bootstrapping Process

This document outlines the bootstrapping process for building the CURSED compiler, following the self-hosting compiler approach.

## Overview of Stages

The CURSED compiler will be developed in four distinct stages, each building on the previous:

1. **Stage 0**: Bootstrap Environment Setup
2. **Stage 1**: Minimal Bootstrap Compiler
3. **Stage 2**: Full Compiler in CURSED
4. **Stage 3**: Self-Compiled Full Compiler

## Stage 0: Bootstrap Environment Setup

In this stage, we prepare the environment for the bootstrap compiler to work with.

### Objectives
- Select Rust as the implementation language for the bootstrap compiler
- Define the core CURSED language subset that will be implemented in Stage 1
- Create the project structure and build system
- Implement basic utilities and libraries needed for compiler development

### Deliverables
- Project structure with build system (using Cargo)
- CURSED language specification documents
- Lexer and parser for CURSED subset
- AST (Abstract Syntax Tree) representation
- Simple code generation framework

## Stage 1: Minimal Bootstrap Compiler

In this stage, we implement a minimal compiler in Rust that can compile a useful subset of CURSED to executable code.

### Objectives
- Implement a minimal but useful subset of CURSED language features
- Ensure the compiler can parse and generate code for this subset
- Implement enough features to allow writing the full compiler in CURSED
- Enable compilation of CURSED code to machine code or intermediate language

### Features to Implement
- Basic types (`lit`, `normie`, `tea`, etc.)
- Variable declarations (`sus`)
- Functions (`slay`) with parameters and return values
- Control structures (`lowkey`, `highkey`, `bestie`, `periodt`)
- Basic I/O operations
- Simple module system (`vibe`, `yeet`)
- Basic error handling

### Deliverables
- Minimal bootstrap compiler that can compile CURSED code
- Runtime library for CURSED programs
- Test suite for the bootstrap compiler
- Example programs written in the CURSED subset
- Documentation for the bootstrap compiler

## Stage 2: Full Compiler in CURSED

In this stage, we implement a more complete CURSED compiler written in the CURSED language itself, using only the subset implemented in Stage 1.

### Objectives
- Implement a full compiler for CURSED in the CURSED language
- Use only features available in the bootstrap compiler
- Add support for additional language features not in the bootstrap compiler
- Ensure the compiler can compile itself

### Additional Features to Implement
- Complete type system including generics
- Structs (`squad`) and interfaces (`collab`)
- Maps (`tea[K]V`) and slices
- Advanced control flow
- Error handling mechanisms
- Concurrency with goroutines (`stan`) and channels (`dm`)
- Standard library

### Deliverables
- Full compiler written in CURSED
- Expanded runtime library
- Comprehensive test suite
- Enhanced standard library
- Documentation for all language features

## Stage 3: Self-Compiled Full Compiler

In this stage, we use the Stage 2 compiler to compile itself, producing a fully self-hosted compiler.

### Objectives
- Compile the Stage 2 compiler using itself
- Verify correctness through extensive testing
- Optimize the compiler for performance
- Add additional tools like formatters, documentation generators, etc.

### Deliverables
- Self-compiled CURSED compiler
- Complete toolchain including formatter, linter, etc.
- Performance benchmarks
- Comprehensive documentation system
- Package manager for CURSED modules

## Compilation Pipeline

The CURSED compiler follows this compilation pipeline:

1. **Lexical Analysis**: Convert source code to token stream
2. **Parsing**: Generate Abstract Syntax Tree (AST) from tokens
3. **Semantic Analysis**: Type checking and semantic validation
4. **Intermediate Representation**: Convert AST to IR
5. **Optimization**: Apply various optimization passes
6. **Code Generation**: Generate target code (machine code or intermediate language)
7. **Linking**: Link with runtime libraries

## Testing Strategy

Each stage of the compiler will be thoroughly tested with:

1. **Unit Tests**: Test individual components
2. **Integration Tests**: Test interactions between components
3. **End-to-End Tests**: Test complete compilation pipeline
4. **Regression Tests**: Ensure bugs don't reappear
5. **Compliance Tests**: Verify language specification compliance
6. **Self-Hosting Tests**: Verify compiler can compile itself

## Timeline and Milestones

| Stage | Estimated Duration | Key Milestones |
|-------|-------------------|----------------|
| 0     | 2-4 weeks         | - Project setup complete<br>- Lexer and parser working<br>- Basic AST representation |
| 1     | 2-3 months        | - Minimal compiler working<br>- Can compile simple CURSED programs<br>- Basic runtime library |
| 2     | 3-6 months        | - Full compiler in CURSED<br>- All language features implemented<br>- Standard library available |
| 3     | 1-2 months        | - Self-compiled compiler<br>- Complete toolchain<br>- Documentation and ecosystem tools |

## Implementation Challenges

1. **Bootstrapping Gap**: Bridging the gap between Rust and CURSED
2. **Error Handling**: Creating meaningful error messages
3. **Performance**: Ensuring the compiler has acceptable performance
4. **Testing**: Verifying correctness throughout the process
5. **Language Evolution**: Managing changes to the language specification during development 