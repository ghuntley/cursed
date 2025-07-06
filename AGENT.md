## Development Commands

```bash
# Build compiler
cargo build

# Run tests
cargo test

# Compile CURSED program
./target/debug/cursed --compile program.csd

# Execute CURSED program (JIT)
./target/debug/cursed program.csd

# Run with clean environment
./build-with-fixed-env.sh
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

