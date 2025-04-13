# LLVM Code Generator for CURSED

This module implements the LLVM code generator for the CURSED language, translating AST nodes into LLVM IR.

## Migration to Standardized Implementation

We have recently migrated the code generator to a standardized architecture to improve:

1. **Maintainability**: Better organization with clear separation of concerns
2. **Extendability**: Easier to add new features through trait implementations
3. **Safety**: Improved error handling and null pointer safety
4. **Testing**: Comprehensive testing infrastructure

## Key Components

### Core Structure

- **context.rs**: Central `LlvmCodeGenerator` implementation
- **mod.rs**: Re-exports public types and traits
- **generator.rs**: Backward compatibility adapter

### Expression Handling

- **expression.rs**: Expression compilation entry point
- **basic_expressions.rs**: Literals and operators
- **pointer_ops.rs**: Pointer operations (standardized implementation)

### Statement Handling

- **statement.rs**: Statement compilation entry point
- **variables.rs**: Variable declarations and scopes
- **control_flow.rs**: If/while/for/switch statements

## Pointer Operations

One of the major improvements is the standardized pointer operations implementation, which includes:

- Safe pointer dereferencing with null checks
- Error reporting for pointer operations
- Support for store-through-pointer operations
- Consistent interface through the `PointerOperations` trait

## Testing

The code generator includes several test suites:

- Unit tests for individual components
- Integration tests for module interactions
- JIT execution tests for end-to-end verification

To run the tests:

```bash
cargo test --package cursed --test pointer_operations_test
cargo test --package cursed --test standardized_llvm_structure_test
cargo test --package cursed --test pointer_simplified_test
```

## Documentation

Several documentation files are provided:

- **ARCHITECTURE.md**: Overview of the module organization
- **SPECS.md**: Detailed specifications for the code generator
- **DEVELOPER.md**: Guide for developers working on the code generator

## Example Usage

```rust
use cursed::codegen::llvm::LlvmCodeGenerator;
use inkwell::context::Context;
use std::path::PathBuf;

// Create a code generator
let context = Context::create();
let file_path = PathBuf::from("my_file.csd");
let mut code_gen = LlvmCodeGenerator::new(&context, "my_module", file_path);

// Compile a program
code_gen.compile(&program)?;

// Get the LLVM module
let module = code_gen.module();
```

## Extending the Code Generator

To add support for a new language feature:

1. Create a new module (e.g., `my_feature.rs`)
2. Define a trait for your feature
3. Implement the trait for `LlvmCodeGenerator`
4. Update the dispatch method in the appropriate entry point

See **DEVELOPER.md** for detailed guidance on extending the code generator.

## Compatibility

The migration maintains backward compatibility through adapter modules and forwarding methods. Existing code that uses the code generator should continue to work without changes.

## Notes

For full details on the standardized approach, please refer to the **SPECS.md** document. 