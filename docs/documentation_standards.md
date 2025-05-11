# Documentation Standards for Cursed Language

## Overview

This document outlines the standard format for code documentation in the Cursed Programming Language codebase. Consistent documentation is crucial for maintainability, especially for a language implementation with complex compiler components.

## Rust Doc Comments Format

### Public Functions and Methods

All public functions and methods (`pub fn`) MUST be documented with doc comments. The standard format is:

```rust
/// Brief one-line description of what the function does.
///
/// More detailed explanation if necessary. This can span multiple lines
/// and should explain the purpose, behavior, and any non-obvious aspects.
///
/// # Parameters
///
/// * `param1` - Description of the first parameter
/// * `param2` - Description of the second parameter
///
/// # Returns
///
/// Description of the return value, including possible errors for Result types
///
/// # Examples
///
/// ```
/// // Optional code example if applicable
/// ```
pub fn function_name(param1: Type1, param2: Type2) -> ReturnType {
    // Function implementation
}
```

### Public Structures and Enums

All public structures and enums must be documented with a description of their purpose and usage:

```rust
/// A structure that represents a compilation context for the LLVM backend.
///
/// This structure maintains the state necessary for code generation with LLVM,
/// including the LLVM context, module, builder, and variable scopes.
pub struct LlvmCodeGenerator<'ctx> {
    // Fields
}
```

### Public Traits

All public traits should document their purpose and contract:

```rust
/// A trait that defines the interface for type registry extension checking.
///
/// Implementors of this trait provide functionality to check relationships
/// between interfaces, allowing for verification of interface extensions
/// and implementation compatibility.
pub trait InterfaceTypeRegistryExtensionChecking {
    // Methods
}
```

## Code Implementation Annotations

### TODO Comments

Use TODO comments to mark areas that need future work, but provide specific details:

```rust
// TODO(issue-123): Implement proper error handling for circular references
```

### FIXME Comments

Use FIXME for known issues that must be addressed:

```rust
// FIXME: This approach can cause memory leaks when dealing with cyclic references
```

### Tracing and Instrumentation

As defined in AGENT.md, use the tracing crate for structured logging:

```rust
#[instrument(skip(large_field))]
pub fn process_data(input: &str, large_field: &Vec<u8>) {
    debug!("Processing data with length {}", input.len());
    // Implementation
}
```

## Test Documentation

Test functions should describe what they're testing and why it's important:

```rust
/// Tests that circular references in object graphs are properly handled by the GC.
///
/// This test creates a cycle with three objects pointing to each other in a circle
/// and verifies that all objects are correctly collected once they're no longer reachable
/// from the root set.
#[test]
fn test_circular_reference_collection() {
    // Test implementation
}
```

## Example File with Standardized Documentation

Below is an example of ideal documentation for a module:

```rust
//! Interface type assertion mechanism implementation.
//!
//! This module provides functionality for checking if a value implements a given interface,
//! allowing for safe downcasting from interface types to concrete types at runtime.
//! It builds on the interface registry system and provides efficient type checking.

use crate::error::Error;
use crate::core::types::Type;

/// A registry that manages interface type information.
///
/// The registry maintains a mapping of type IDs to type names and provides
/// methods for registering types and checking relationships between interfaces.
pub struct InterfaceTypeRegistry {
    // Fields
}

impl InterfaceTypeRegistry {
    /// Creates a new empty type registry.
    ///
    /// # Returns
    ///
    /// A new instance of `InterfaceTypeRegistry` with no registered types.
    pub fn new() -> Self {
        // Implementation
    }
    
    /// Checks if a type implements a given interface.
    ///
    /// # Parameters
    ///
    /// * `type_name` - The name of the concrete type to check
    /// * `interface_name` - The name of the interface to check against
    ///
    /// # Returns
    ///
    /// * `Ok(true)` - If the type implements the interface
    /// * `Ok(false)` - If the type does not implement the interface
    /// * `Err` - If there was an error during checking
    pub fn check_implements(&self, type_name: &str, interface_name: &str) -> Result<bool, Error> {
        // Implementation
    }
}
```

## Prioritizing Documentation

When documenting the codebase, prioritize in this order:

1. Public API interfaces, especially those used by other components
2. Complex algorithms and data structures
3. Error handling and edge cases
4. Extension points and customization hooks

## Documentation Review

Code reviews should explicitly check for documentation quality, verifying that:

- All public items are documented
- Documentation is clear and accurate
- Examples are included for complex functionality
- Non-obvious behavior and edge cases are explained

## Conclusion

Consistent documentation is a key aspect of the Cursed Language codebase. By following these standards, we ensure the project remains maintainable and accessible to new contributors.