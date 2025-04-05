# Generics Implementation for CURSED

## Overview

This document explains the implementation of generics for the CURSED programming language.

## Components

### 1. Type Checker (src/core/type_checker.rs)

- Provides a type system with support for generic type parameters
- Defines the `Type` enum with variants for all CURSED types
- Includes `TypeParam` for representing generic type parameters
- Includes methods for type compatibility checking and environment management

### 2. Generic Instantiation (src/core/generic_instantiation.rs)

- Handles the conversion of generic types to concrete types
- Implements type parameter mapping and substitution
- Provides an API for creating concrete instances of generic code

### 3. LLVM Code Generation (src/codegen/llvm_generics.rs)

- Generates LLVM IR for generic type instantiations
- Maps CURSED types to LLVM types
- Handles monomorphization by creating specialized versions of generic functions
- Maintains a registry of instantiated generic functions

## Implementation Approach

### Parse Phase

1. The parser recognizes generic type parameters in square brackets
2. It builds AST nodes for generic structs, interfaces, and functions
3. Type parameters are stored in the respective AST nodes

### Type Checking Phase

1. The type checker validates generic type parameters and constraints
2. It ensures type safety when using generic types
3. It verifies that concrete type arguments satisfy any constraints

### Code Generation Phase

1. The compiler uses monomorphization to create specialized versions of generic code
2. For each generic function/type instantiation with specific type arguments:
   - Create a new name for the specialized version
   - Substitute type parameters with concrete types
   - Generate LLVM IR for the specialized version
   - Register the specialization in the instantiation registry

## Usage Examples

### Generic Struct

```
be_like Box[T] squad {
    value T
}

sus box_int = Box[normie]{value: 42}
sus box_string = Box[tea]{value: "hello"}
```

### Generic Function

```
slay identity[T](x T) T {
    yolo x
}

sus result = identity[normie](42)
sus str_result = identity[tea]("hello")
```

### Multiple Type Parameters

```
be_like Pair[A, B] squad {
    first A
    second B
}

sus pair = Pair[tea, normie]{first: "hello", second: 42}
```

### Higher-Order Functions with Generics

```
slay apply[A, B](f stan(A) B, x A) B {
    yolo f(x)
}

sus result = apply[normie, normie](add, 5, 3)
```

## Testing

The implementation includes comprehensive tests:

1. `tests/generic_type_checking_test.rs` - Unit tests for type checking of generic types
2. `tests/jit/generic_test.csd` - Integration test for JIT execution of generic code
3. `tests/generics_test.csd` - Example CURSED code using generics
4. `tests/generic_param_test.csd` - Tests for multiple type parameters

## Future Enhancements

1. Type constraints for generic parameters
2. Improved error messages for generic type errors
3. Performance optimizations for monomorphization
4. Separate compilation of generic code