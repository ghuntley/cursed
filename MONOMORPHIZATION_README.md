# Monomorphization to LLVM IR Integration

## Overview
This implementation integrates the generic type monomorphization system with the LLVM code generator to support specialization of generic functions in the CURSED programming language.

## Components Implemented

1. **MonomorphizationManager** in `src/codegen/monomorphization.rs`
   - Tracks instantiated generic functions with their concrete type parameters
   - Generates unique names for monomorphized functions
   - Caches generated specializations to avoid duplicates
   - Coordinates the generation of specialized LLVM IR code

2. **LLVM Function Monomorphization** in `src/codegen/llvm/function_monomorphization.rs`
   - Extends the LlvmCodeGenerator with support for generic functions
   - Provides methods to generate code for generic function calls
   - Implements function specialization for concrete types

3. **Integration with LLVM Code Generator** 
   - Added MonomorphizationManager field to LlvmCodeGenerator
   - Modified code generation to handle generic function instantiations

## Name Mangling Scheme
The implementation uses a name mangling scheme for specialized functions:
```
{function_name}__{Type1}_{Type2}...{TypeN}
```

For example, a function `process<T, U>` specialized with types `Normie` and `Tea` would be named `process__Normie_Tea`.

## Testing
The implementation includes comprehensive tests:

1. **Basic Monomorphization Tests** in `tests/llvm_monomorphization_test.rs`
   - Tests the creation and functionality of the MonomorphizationManager
   - Verifies name generation and caching behavior
   - Tests integration with the LlvmCodeGenerator

2. **JIT Execution Tests** in `tests/jit_generics_test.rs`
   - Tests specializing generic functions with different types
   - Verifies multiple specializations of the same generic function
   - Tests more complex generic functions with multiple type parameters

## Future Work

1. **Complete Specialization**
   - Add full support for specializing all generic constructs
   - Implement interface monomorphization with vtable generation
   - Support generic struct specialization with field type substitution

2. **Performance Optimizations**
   - Add specialization request batching for better compilation performance
   - Implement lazy specialization for improved startup time
   - Add specialized code optimization passes

## Completed Features

1. **Memory Layout Optimization** ✅
   - Specialized types now have correct LLVM struct layouts
   - Pointer arithmetic is properly handled for specialized container types
   - GC metadata is generated for specialized types

## Usage Example

```rust
// Create an LLVM code generator
let context = Context::create();
let mut code_gen = LlvmCodeGenerator::new(&context, "module_name", file_path);

// Create or load a generic function AST
let generic_function = ...; 

// Specialize the function with concrete types
let specialized_name = code_gen.mono_manager.specialize_function(
    &mut code_gen,
    &generic_function,
    &[Type::Normie, Type::Tea], // Concrete type arguments
).expect("Specialization failed");

// Use the specialized function name for function calls
println!("Generated specialized function: {}", specialized_name);
```