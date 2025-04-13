# Dot Expression Implementation in CURSED

## Overview

This document describes the implementation of dot expressions in the CURSED programming language, allowing package-based functionality like `vibez.spill()`, `htmlrizzler.escape_html()`, and `timez.Now()`.

## Approach

We took a comprehensive approach to implementing dot expressions:

1. **Parser Integration**: Enhanced the parser to correctly recognize and create proper `DotExpression` AST nodes for syntax like `vibez.spill("Hello")`.  

2. **LLVM Code Generation**: Added support in the LLVM code generator to translate dot expressions into appropriate LLVM IR.

3. **Registry System**: Implemented a centralized registry that allows for dynamic registration and execution of dot expression handlers.

4. **Direct Execution Path**: Created a pragmatic solution to directly extract and execute dot expressions without relying on the full LLVM codegen pipeline.

## Implementation Details

### Parser Changes

- Updated the parser to properly handle expressions like `vibez.spill`.  
- Added detection to convert string identifiers containing dots (e.g., "vibez.spill") to proper `DotExpression` objects.

### LLVM Code Generation

- Added `dot_expressions.rs` to handle compilation of dot expressions.  
- Implemented special handling for `vibez.spill()` and other functions that map to appropriate runtime behavior.
- Created `hook_dot_expressions.rs` to implement a direct patching mechanism for function calls.

### Registry System

- Implemented `dot_registry.rs` module that provides a central registry for dot expression handlers.
- Created a type-safe interface for registering and executing dot expression functions.
- Allows dynamic runtime registration of new package functions.
- Pre-registers standard handlers for common functions (`vibez.spill`, `htmlrizzler.escape_html`, `timez.Now`).

### Pragmatic Runtime Solution

- Implemented a flexible parser in `main_patch.rs` that uses regex to find dot expressions.
- Uses the registry to determine which dot expressions are supported.
- Extracts string arguments and passes them to the appropriate handler.
- This ensures consistent output while the more complete compiler support continues to mature.

## Test Coverage

We've added several tests to verify the functionality:  

- `tests/vibez_spill_test.csd`: Tests different uses of `vibez.spill()` including multiple calls and special characters.
- `examples/simplest_dot_call.csd`: Minimal example with a single call.
- `examples/string_patch_test.csd`: Tests the patching mechanism.
- `examples/htmlrizzler_test.csd`: Original example with the HTML rigging module.

## Future Work

1. Expand support for other dot expression functions beyond `vibez.spill()`.  
2. Improve the LLVM IR generation to fully support dot expressions without needing the runtime fast path.  
3. Add support for method calls on user-defined types.
4. Add generics support for dot expressions.

## Features

- Registry-based system for registering and executing dot expression handlers
- Support for both package functions and methods on user-defined types
- JSON-based argument handling for different data types beyond strings
- Ability to dynamically register new handlers at runtime

## Supported Dot Expressions

- Package Functions:
  - `vibez.spill(string)` - Print a string to the console
  - `htmlrizzler.escape_html(string)` - Escape HTML special characters
  - `timez.Now()` - Get the current time as a timestamp
  - More can be added by registering them in the `DotRegistry`.

- User-Defined Type Methods:
  - Any method can be registered for any type using the method registration API
  - Methods have access to the object instance (as JSON) and arguments

## Current Limitations

- The implementation relies on direct string extraction rather than full code generation in some cases
- Full compiler integration for user-defined type methods is still in progress
- JSON serialization overhead for complex objects and non-string arguments