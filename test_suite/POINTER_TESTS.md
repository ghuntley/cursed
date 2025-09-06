# CURSED Pointer Syntax Test Suite

This directory contains comprehensive tests for the new `ඞ` (Among Us) pointer syntax in CURSED.

## Test Categories

### Memory Tests (`test_programs/memory/`)
- **01_basic_pointers.cursed**: Basic pointer declaration, assignment, and dereferencing
- **02_pointer_arithmetic_simple.cursed**: Simplified array pointer access
- **03_nested_pointers.cursed**: Double and triple pointer indirection

### Function Tests (`test_programs/functions/`)
- **01_pointer_parameters.cursed**: Passing pointers to functions as parameters
- **02_pointer_return_values.cursed**: Functions that return pointers

### Complex Tests (`test_programs/complex/`)  
- **01_linked_list_pointers.cursed**: Advanced data structure usage with pointers

### Validation Tests (`test_programs/validation/`)
- **01_pointer_type_validation.cursed**: Type safety and null pointer handling

### Comprehensive Tests (`test_programs/comprehensive/`)
- **01_comprehensive_pointers.cursed**: All major pointer operations in one test

### Edge Cases (`test_programs/edge_cases/`)
- **01_pointer_edge_cases.cursed**: Boundary conditions and edge cases

### Error Tests (`test_programs/errors/`)
- **01_pointer_syntax_errors.cursed**: Validates new `ඞ` syntax works
- **02_old_syntax_rejection.cursed**: Ensures old `@` syntax is properly rejected

### Performance Tests (`test_programs/performance/`)
- **01_pointer_intensive.cursed**: Heavy pointer usage scenarios

### Regression Tests (`test_programs/regression/`)
- **01_pointer_regression.cursed**: Ensures pointers don't break existing features

## Running Tests

### Quick Test Suite
```bash
cd test_suite
./test_pointer_syntax_fixed.sh
```

### Individual Test
```bash
# Interpreter mode
./zig-out/bin/cursed-compiler --interpret test_suite/test_programs/memory/01_basic_pointers.cursed

# Compiled mode  
./zig-out/bin/cursed-compiler --compile test_suite/test_programs/memory/01_basic_pointers.cursed -o test
./test
```

## Test Coverage

The test suite covers:

✅ **Basic Operations**
- Pointer declaration with `ඞ` syntax
- Address-of operations (`ඞvariable`)  
- Dereference operations (`*pointer`)
- Null pointer handling

✅ **Type Safety**
- Different pointer types (`ඞnormie`, `ඞtea`, `ඞmeal`)
- Type validation and compatibility

✅ **Advanced Usage**
- Nested pointers (`ඞඞnormie`, `ඞඞඞnormie`)
- Function parameters and return values
- Struct member pointers

✅ **Error Handling**
- Old `@` syntax rejection
- Proper lexer error messages

✅ **Integration**
- Compatibility with existing CURSED features
- Standard library integration (`vibez`, `mathz`)

## Expected Results

All tests should produce identical output in both interpreter and compiled modes, demonstrating the robustness of the new pointer syntax implementation.

## Among Us Integration 🔥

These tests validate that CURSED is now the world's first programming language to use the Among Us character (`ඞ`) for pointer operations, making it both technically sound and culturally relevant!

```cursed
sus ptr ඞnormie = ඞvalue  // Sus but it works! 
```

**No cap, this syntax is based! 🎮**
