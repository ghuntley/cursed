# Simple Math Calculator

A basic calculator app demonstrating how to use a single dependency in CURSED.

## Features

- Basic arithmetic operations (add, subtract, multiply, divide)
- Command-line interface
- Error handling for invalid inputs
- Uses the `math_utils` library for calculations

## Installation

```bash
# Clone this example
git clone https://github.com/cursed-lang/examples
cd examples/package_manager/simple_dependency

# Build the project
cursed-pkg build

# Run the calculator
cursed-pkg run -- add 5 3
```

## Usage

```bash
# Addition
./target/debug/calculator add 10 5
# Output: Result: 10 add 5 = 15

# Subtraction  
./target/debug/calculator subtract 10 3
# Output: Result: 10 subtract 3 = 7

# Multiplication
./target/debug/calculator multiply 4 6
# Output: Result: 4 multiply 6 = 24

# Division
./target/debug/calculator divide 15 3
# Output: Result: 15 divide 3 = 5
```

## Dependencies

This project uses one external dependency:

- **math_utils** (v1.0.0): Provides basic arithmetic functions with error handling

## Project Structure

```
simple_dependency/
├── CursedPackage.toml    # Package manifest with dependency
├── src/
│   └── main.csd         # Calculator implementation
├── README.md            # This file
└── target/              # Build artifacts (generated)
```

## Learning Objectives

This example teaches:

1. **Adding dependencies** to `CursedPackage.toml`
2. **Importing and using** external packages
3. **Building projects** with dependencies
4. **Command-line argument** handling
5. **Basic error handling** with Result types

## Package Manager Commands Used

```bash
# Initialize new project (if starting from scratch)
cursed-pkg new simple-math-app

# Add a dependency
cursed-pkg add math_utils@1.0.0

# Build the project
cursed-pkg build

# Run the binary
cursed-pkg run -- add 5 3

# Check dependencies
cursed-pkg list

# View dependency tree
cursed-pkg tree
```

## Next Steps

After mastering this example, try:

1. **complex_dependencies** - Multiple dependencies with features
2. **workspace_example** - Multi-package workspace
3. **performance_benchmark** - Many dependencies and optimization
