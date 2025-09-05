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

## Key Concepts

*   **Gen Z Slang Keywords:** Core language constructs are represented by popular Gen Z slang terms (e.g., `slay` for function, `sus` for variable, `lowkey` for if).
*   **Simplicity and Expressiveness:** Aims for a clean syntax inspired by Go and Python, while incorporating unique slang-based keywords.
*   **Static Typing:** Uses a static type system with type inference.
*   **Compilation to LLVM:** Targets LLVM IR for performance and portability.
*   **Garbage Collection:** Manages memory automatically.

## Modules and Packages

CURSED uses a simple module system based on packages:

*   **Package Declaration:** Every source file (`.💀`) belongs to a package, declared at the top using `vibe PackageName;`.
*   **Exports:** Symbols (functions, variables, types) starting with an **uppercase** letter are exported (public) from their package. Symbols starting with a lowercase letter are private.
*   **Imports:** The `yeet` keyword imports other packages. `yeet "path/to/package";` imports the package found at `path/to/package.💀`. Standard library packages have predefined paths.
*   **Qualified Access:** Imported symbols are *always* accessed using the package name (or an import alias) as a qualifier, e.g., `packageName.ExportedSymbol`.
*   **Import Aliases:** An optional alias can be provided during import: `yeet aliasName "path/to/package";`. Access is then done via `aliasName.ExportedSymbol`.

## Goals

*   Create a fun, expressive, and modern programming language.
*   Explore the use of unconventional keywords in language design. 