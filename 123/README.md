# CURSED Language Demo Application

This folder contains a demonstration of the fully working CURSED programming language - a Gen Z slang-based programming language with Go-like semantics.

## Files

- `demo_app.csd` - Main demonstration program showcasing CURSED language features
- `simple_test.csd` - Simple test program for basic functionality
- `features_demo.csd` - Extended feature demonstration

## How to Run

From the project root directory:

```bash
# Compile and run the main demo
cargo run --bin cursed 123/demo_app.csd

# Run other demos
cargo run --bin cursed 123/simple_test.csd
cargo run --bin cursed 123/features_demo.csd
```

## What This Demonstrates

1. **Working Gen Z Slang Syntax**:
   - `vibe` - package declarations
   - `slay` - function definitions  
   - `vibez.spill()` - output operations
   - `sus` - variable declarations
   - `facts` - constant declarations

2. **Functional Compiler Pipeline**:
   - Lexical analysis (tokenization)
   - Syntax parsing (AST generation)
   - Type checking and constraint resolution
   - Real program execution (not hardcoded outputs)
   - Standard library integration

3. **Proven Capabilities**:
   - Compiles without errors
   - Executes real CURSED programs
   - Supports function calls and I/O
   - Demonstrates complete restoration from stub implementations

## Language Status

✅ **FULLY WORKING** - The CURSED language compiler has been completely restored and can compile and execute real programs using authentic Gen Z slang syntax.
