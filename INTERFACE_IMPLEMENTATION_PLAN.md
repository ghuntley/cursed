# Interface Dynamic Dispatch Implementation Plan

## Overview

This document outlines the plan for implementing interfaces with dynamic dispatch in the Cursed programming language. The implementation follows the vtable-based approach commonly used in languages like Rust, C++, and Go.

## Architecture

### Interface Structure

- Each interface value consists of two pointers:
  1. Data pointer: Points to the actual object implementing the interface
  2. VTable pointer: Points to the vtable containing function pointers

- The VTable contains function pointers for each method in the interface

### Runtime Type Information

- Type information is stored for each interface implementation
- This allows runtime type checking and type assertions
- The TypeInfo struct contains type name, ID, and generic type arguments

### Generic Interface Support

- Interfaces can have type parameters
- Type parameter information is preserved in the interface structure
- Specialized implementations are created for each concrete type

## Implementation Phases

### Phase 1: Core VTable Structure

1. ✅ Define InterfaceStructure for storing interface information
2. ✅ Define VTable structure for method tables
3. ✅ Define VTableImpl for concrete implementations
4. ✅ Implement InterfaceManager to manage interfaces and implementations

### Phase 2: Dynamic Dispatch

1. ✅ Implement create_interface_value to create interface values
2. ✅ Implement call_interface_method for method dispatch
3. ✅ Add type checking for interface implementations
4. ✅ Add runtime type information

### Phase 3: Generic Interface Support

1. ✅ Add type parameter support to InterfaceStructure
2. ✅ Implement parsing for generic interface names
3. ✅ Support type arguments in interface types

### Phase 4: Integration with Type System

1. ⬜ Integrate with type checker for interface compatibility checking
2. ⬜ Add interface type conversion in code generator
3. ⬜ Support interface values in expressions

### Phase 5: Code Generation

1. ⬜ Generate vtables for interface implementations
2. ⬜ Generate interface values and casts
3. ⬜ Generate dynamic dispatch code

## Testing

1. ✅ Basic interface structure tests
2. ✅ VTable implementation tests
3. ✅ Dynamic dispatch tests
4. ✅ Generic interface tests
5. ⬜ End-to-end tests with actual code

## Next Steps

1. Fix the existing interface tests to work with the implementation
2. Complete the LlvmCodeGenerator integration for interfaces
3. Add support for interface type assertions
4. Add interface implementation verification in the type checker
5. Implement proper code generation for interface methods