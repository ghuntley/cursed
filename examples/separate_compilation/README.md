# CURSED Separate Compilation Example

This example demonstrates the separate compilation functionality of the CURSED programming language.

## Files

- `math_utils.💀` - Math utilities package with basic arithmetic operations
- `string_utils.💀` - String utilities package with string manipulation functions  
- `main.💀` - Main program that imports and uses both utility packages

## Package Structure

```
main
├── imports: mathutils, stringutils
│
mathutils (math_utils.💀)
├── exports: add, multiply, factorial, power
│
stringutils (string_utils.💀)  
├── exports: concat, length, toUpper, reverse, startsWith
```

## Compilation

### Using the CURSED Compiler

#### Single File Compilation (Traditional)
```bash
# This will detect separate compilation is needed and handle it automatically
cursed main.💀
```

#### Explicit Separate Compilation
```bash
# Compile and link all packages
cursed-compile link main.💀 math_utils.💀 string_utils.💀 -o demo_program

# Or compile packages individually
cursed-compile package math_utils.💀 -o ./build --emit-ir --emit-object
cursed-compile package string_utils.💀 -o ./build --emit-ir --emit-object
cursed-compile package main.💀 -o ./build --emit-ir --emit-object
```

### Analyzing Package Structure
```bash
# Analyze dependencies and structure
cursed-compile analyze main.💀 math_utils.💀 string_utils.💀
```

This will show:
- Package names and file locations
- Dependency relationships
- Exported functions
- Potential issues (missing dependencies, circular dependencies, etc.)

## Features Demonstrated

1. **Package Declarations**: Each file declares its package with `vibe packagename;`
2. **Import Statements**: Main imports dependencies with `yeet "packagename"`
3. **Symbol Export**: All public functions are automatically exported
4. **Cross-Package Calls**: Functions called using `packagename.function()` syntax
5. **Dependency Resolution**: Compiler automatically determines compilation order
6. **Module Linking**: Separate LLVM modules are linked into final executable

## Expected Output

When run, the program demonstrates:
- Basic math operations from the math utils package
- String manipulation from the string utils package  
- Cross-package integration (using string length in math operations)

## Build Artifacts

When using separate compilation, the following files are generated:

```
build/
├── mathutils.ll      # LLVM IR for math utils
├── mathutils.o       # Object file for math utils
├── stringutils.ll    # LLVM IR for string utils
├── stringutils.o     # Object file for string utils
├── main.ll           # LLVM IR for main
├── main.o            # Object file for main
└── demo_program      # Final linked executable
```

## Advantages of Separate Compilation

1. **Faster Incremental Builds**: Only changed packages need recompilation
2. **Modular Development**: Packages can be developed and tested independently
3. **Code Reuse**: Utility packages can be shared across multiple programs
4. **Parallel Compilation**: Independent packages can be compiled in parallel
5. **Better Organization**: Large programs can be split into logical modules
