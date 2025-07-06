## Development Commands

```bash
# Build compiler
cargo build

# Run tests (all 318 tests pass)
cargo test

# Compile CURSED program to native executable
cargo run --bin cursed -- compile program.csd

# Execute CURSED program (JIT)
cargo run --bin cursed program.csd

# Run with clean environment
./build-with-fixed-env.sh

# Test specific integration (with proper ignores for JIT)
cargo test jit_integration_tests -- --ignored

# Quick build check
cargo check

# Run specific test file
cargo test test_name

# Build and run comprehensive demo
cargo run --bin cursed -- comprehensive_demo.csd

# Test array/slice parsing
cargo test array_parsing_tests

# Test for-in loops
cargo test for_in_tests

# Test native compilation with mixed types
cargo run --bin cursed -- compile program.csd
./program  # Run the compiled executable

# LLVM Compilation Verification
# Always test both modes when verifying LLVM compilation:
# 1. Test interpretation: cargo run --bin cursed program.csd
# 2. Test native compilation: cargo run --bin cursed -- compile program.csd
# 3. Ensure llc is available in PATH for native compilation
# 4. Test generated executables run correctly
# 5. Check LLVM IR register numbering consistency if issues arise
```

## Development Environment

Uses https://devenv.sh to provide development dependencies. These are defined in  devenv.nix. You will need to run "direnv allow" after making a change then restart bash sessions.

## Project Structure

```
src/
├── main.rs                 # CLI entry point
├── lexer/                  # Tokenization
├── parser/                 # AST generation
├── semantic/               # Type checking
├── codegen/                # LLVM IR generation
├── runtime/                # Runtime system
├── stdlib/                 # Standard library
├── optimization/           # Compiler optimizations
├── debug/                  # Debug information
└── linter/                 # Code quality analysis
```

## Language Features

### Tuples
- Basic syntax: `(1, "hello", based)`
- Access elements: `tuple.0`, `tuple.1`, `tuple.2`
- Destructuring: `(a, b, c) = tuple`
- Tests: `tests/tuple_tests.rs` (all 14 tests pass)
- Status: Works in interpretation mode, may have LLVM codegen issues

### Boolean Literals
- Specification-compliant syntax: `based` (true) and `sus` (false)
- Boolean type: `lit` 
- Usage: `sus flag lit = based` or `sus flag lit = sus`
- Tests: Boolean literals work correctly in both variable declarations and expressions
- Status: Fully compliant with language specification

### Mixed Arithmetic Operations
- Mixed Integer-Float arithmetic fully supported in interpretation mode
- All arithmetic (+, -, *, /) and comparison (<, >, ==, etc.) operations work
- Automatic type promotion: integers converted to floats for mixed operations
- Example: `5 * 3.14` works correctly in interpretation mode
- Status: Works perfectly in interpretation mode, compilation mode works for non-constant expressions

### Character Type
- Character type: `sip` (single character values)
- Character literals: `'a'`, `'Z'`, `'1'`, `'@'`
- Escape sequences: `'\n'`, `'\t'`, `'\\'`, `'\''`
- Variable declaration: `sus ch sip = 'x'`
- String interpolation and comparison operations supported
- Status: Fully functional in both interpretation and compilation modes

### Array/Slice Types
- Array/slice type parsing is fully implemented
- Support for both fixed arrays and dynamic slices
- Type syntax: `[type]` for arrays, `[type; size]` for fixed arrays
- Status: Fully functional in both interpretation and compilation modes

### Array Indexing
- Basic syntax: `array[index]`, `numbers[0]`, `data[variable]`
- Examples: `sus first drip = numbers[0]`, `sus value drip = data[i]`
- Support for nested arrays: `matrix[row][col]`
- Tests: `test_array_indexing_test.csd` (comprehensive array access tests)
- Status: Fully functional in both interpretation and compilation modes

### For-in Loops
- For-in loop syntax is implemented and working at top level
- Support for iteration over arrays and other collections
- Status: Fully functional, working correctly in both modes

### Native Compilation
- LLVM-based native compilation fully functional
- Mixed-type printf support for vibez.spill() with strings, integers, booleans
- Automatic type inference for printf format strings (%s, %d, %f)
- Boolean to integer conversion for printf compatibility
- Status: Fully functional for core CURSED programs

## Known Issues

### JIT Execution Environment
- JIT integration tests require `#[ignore = "Requires LLVM environment setup"]` to prevent segfaults
- LLVM initialization can cause SIGSEGV in test environments
- Native compilation works perfectly via `cursed compile`
- JIT infrastructure is preserved for future activation when LLVM issues are resolved

### LLVM Type Inference
- LLVM codegen now properly handles mixed-type expressions in vibez.spill()
- String variables correctly identified as i8* pointer types
- Integer and boolean types properly converted for printf calls
- Status: Fixed in v6.2.0 - native compilation works for mixed types

