# Range Clause Testing Plan

## Overview
This document outlines the test plan for the enhanced range clause implementation in the LLVM code generator. The goal is to ensure comprehensive test coverage for all range clause variants and use cases.

## Test Categories

### 1. Numeric Range Tests

#### Basic Range Iteration
- Simple `for i := range end` loops (e.g., `range 10`)
- Verify sum of values matches expected result

#### Start-End Range Iteration
- `for i := range start, end` loops (e.g., `range 2, 8`)
- Verify correct bounds handling

#### Step-Based Range Iteration
- `for i := range start, end, step` loops (e.g., `range 1, 10, 2`)
- Test with positive step values

#### Negative Step Ranges
- Test ranges with negative step values (e.g., `range 10, 1, -2`)
- Verify correct decrementing behavior

#### Empty Ranges
- Test ranges that produce no iterations (e.g., when start >= end with positive step)
- Verify loop body is not executed

#### Edge Cases
- Test with very large ranges (close to integer limits)
- Test with step values approaching 0
- Test with negative start and end values

### 2. Container Iteration Tests

#### Array Iteration
- Basic array iteration (`for elem := range array`)
- Check sum of elements matches expected result

#### Slice Iteration
- Iterating over slice values
- Verifying all elements are accessed in order

#### String Iteration
- Iterating over characters in a string
- Handling UTF-8 characters correctly

#### Mixed Type Arrays
- Arrays containing elements of mixed compatible types
- Verifying type coercion happens correctly

### 3. Map Iteration Tests

#### Basic Map Iteration
- Key-only iteration (`for key := range map`)
- Verify all keys are accessed

#### Key-Value Iteration
- Key-value pair iteration (`for key, value := range map`)
- Verify correct key-value mapping

#### Nested Map Iteration
- Maps containing maps or complex values
- Handling deeply nested structures

### 4. Control Flow Tests

#### Break Statement
- Testing break statements within range loops
- Verify early exit behavior

#### Continue Statement
- Testing continue statements to skip iterations
- Verify loop continues correctly after skipping

#### Nested Loops
- Multiple range loops nested within each other
- Verify correct scoping and variable handling

#### Conditionals Within Loops
- If/else statements within range loops
- Complex control flow patterns

### 5. Error Handling Tests

#### Invalid Range Parameters
- Non-integer range parameters
- Invalid step values (zero or incorrect types)

#### Container Type Errors
- Attempting to iterate over non-iterable types
- Passing incorrect argument types

#### Runtime Errors
- Handling array bounds checking
- Out-of-memory conditions

## Test Matrix

Each test category should be tested with:
- JIT execution
- AOT compilation
- Debug mode vs. Release mode
- Different optimization levels

## Implementation Strategy

1. Implement basic numeric range tests first
2. Extend to container iteration tests
3. Add map iteration tests
4. Implement control flow and error handling tests
5. Cross-validate against existing implementation

## Success Criteria

- All tests pass with the enhanced implementation
- Tests match expected output with the original implementation
- No regressions in existing test cases
- Edge cases are properly handled
- Error cases produce appropriate error messages