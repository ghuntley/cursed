# CURSED Language Overview

CURSED is an esoteric programming language designed to combine the simplicity and efficiency of Go with the contemporary linguistic flair of Generation Z slang. This document provides a high-level overview of the language's design philosophy and core principles.

## Design Philosophy

CURSED follows these core principles:

1. **Syntactic Familiarity with a Twist**: The language follows Go-like grammar and structure but replaces traditional programming keywords with Gen Z slang.

2. **Self-Hosting**: The compiler is designed to be self-hosting through a multi-stage bootstrapping process, starting with a foundation in Rust.

3. **Pragmatic Absurdity**: While the language is esoteric by design, it aims to be fully functional and capable of real-world programming tasks.

4. **Compilation Efficiency**: The compiler should produce efficient code while maintaining readability of the source.

## Language Characteristics

- **Static Typing**: CURSED is statically typed with type inference
- **Garbage Collected**: Automatic memory management
- **Concurrency Support**: Built-in primitives for concurrent programming
- **Modularity**: Package-based code organization
- **Expression-Based**: Most constructs are expressions that yield values

## Target Audience

CURSED is primarily designed for:

1. Programming language enthusiasts
2. Compiler design students and researchers
3. Developers looking for a unique programming experience
4. Anyone interested in exploring the intersection of language and programming

## Implementation Strategy

The CURSED compiler will be implemented following the bootstrap compiler approach:

1. Stage 0: Development of a minimal compiler in Rust
2. Stage 1: Self-compilation of the bootstrap compiler
3. Stage 2: Development of a full CURSED compiler in CURSED
4. Stage 3: Self-compilation of the full compiler

Each stage will expand the language's feature set until it reaches its complete specification. 