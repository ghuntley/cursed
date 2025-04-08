# AST Module Structure

This directory contains the Abstract Syntax Tree (AST) for the CURSED programming language, organized into logical modules:

- **mod.rs**: Core traits and re-exports
- **base.rs**: Program structure and basic AST elements
- **expressions.rs**: Expression nodes (literals, operators, etc)
- **statements.rs**: Statement nodes (let, return, blocks, etc)
- **control_flow.rs**: Control flow statements (if, while, for, switch)
- **declarations.rs**: Type declarations and definitions (struct, interface)
- **pointer.rs**: Pointer-related types and expressions

## Migration Plan

A re-export module at `src/ast.rs` maintains backward compatibility with existing code. Once all dependent code has been updated to use the new module structure, the re-export file can be removed.

## Importing Guidelines

When importing AST types in new code, use the following guidelines:

```rust
// Prefer specific imports when using just a few types
use crate::ast::expressions::{Identifier, StringLiteral};
use crate::ast::statements::LetStatement;

// Use wildcards for modules when using many types from one module
use crate::ast::expressions::*;

// When you need the core traits
use crate::ast::{Node, Expression, Statement};
```