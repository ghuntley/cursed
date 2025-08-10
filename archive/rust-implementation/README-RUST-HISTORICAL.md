# CURSED Rust Implementation (Historical Archive)

## ⚠️ ARCHIVED IMPLEMENTATION ⚠️

**This is a historical archive of the original Rust implementation of CURSED.**

- **Status**: Archived (August 2025)
- **Reason**: Successfully migrated to Zig with superior performance
- **Current Implementation**: See `/src-zig/` and use `zig build`

## Historical Context

This directory contains the complete Rust implementation of the CURSED programming language compiler and runtime. This implementation served as the foundation for CURSED development from initial conception through successful migration to Zig.

### Why This Implementation Was Retired

1. **Performance**: Zig implementation is 50-300x faster to build
2. **Memory**: Zero memory leaks vs occasional leaks in Rust version  
3. **Complexity**: 50+ dependencies reduced to ~5 system libraries
4. **Developer Experience**: Sub-second builds vs 10-30s builds
5. **Cross-Compilation**: Built-in Zig support vs complex Rust setup

### Historical Significance

This Rust implementation was crucial in:
- Establishing CURSED's syntax and semantics
- Validating the language design
- Proving the viability of the project
- Creating comprehensive test suites
- Building the initial standard library

## Original Build Instructions (Historical)

### Prerequisites
```bash
# Rust toolchain (archived version)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup install stable
rustup default stable

# LLVM development libraries
sudo apt install llvm-17-dev libllvm17 llvm-17-tools
```

### Building (Historical)
```bash
# Clone repository (historical state)
git clone https://github.com/ghuntley/cursed.git
cd cursed

# Build (slow but functional)
cargo build --release    # 10-30 seconds

# Run (historical)
./target/release/cursed program.csd
```

## Historical File Structure

```
src/                           # Main Rust source code
├── main.rs                   # Entry point
├── lib.rs                    # Core library
├── ast.rs                    # Abstract syntax tree
├── lexer/                    # Tokenization
├── parser/                   # Syntax analysis  
├── type_system/              # Type checking
├── codegen/                  # LLVM code generation
├── stdlib/                   # Standard library (Rust)
├── runtime/                  # Runtime system
├── concurrency/              # Async runtime (Tokio)
├── memory/                   # Memory management
├── error/                    # Error handling
├── cli/                      # Command-line interface
├── lsp/                      # Language server
├── tools/                    # Development tools
└── ...                       # 40+ other modules

Cargo.toml                     # Dependencies (50+ crates)
build.rs                       # Build configuration
rust-toolchain.toml           # Rust version specification
```

## Historical Dependencies

The Rust implementation required 50+ external dependencies:

### Core Dependencies
- `clap` - CLI argument parsing
- `serde` - Serialization 
- `tokio` - Async runtime
- `futures` - Async abstractions
- `crossbeam` - Concurrency primitives

### LLVM Integration  
- `llvm-sys` - LLVM system bindings
- `inkwell` - Safe LLVM wrapper

### Cryptography
- `ring`, `sha2`, `aes`, `ed25519-dalek`, etc.

### Platform-Specific
- `winapi` (Windows)
- `libc` (Unix-like systems)
- Various WASM bindings

## Migration Artifacts

See `/archive/migration-artifacts/` for:
- Performance comparisons
- Feature parity validation
- Migration lessons learned
- Technical decision rationale

## Code Archaeology

This archive serves as:
- **Historical Reference**: Understanding design evolution
- **Academic Study**: Language implementation techniques  
- **Backup**: Emergency reference if needed
- **Documentation**: Migration case study

## Git History Preservation

The complete git history of this implementation is preserved in the main repository. Key historical commits include:

- Initial Rust implementation
- Major feature additions
- Performance optimization attempts
- Final working state before migration

## Lessons Learned

### What Worked Well in Rust
1. **Type Safety**: Rust's type system caught many errors early
2. **Memory Safety**: Prevented most memory-related bugs
3. **Ecosystem**: Rich ecosystem for dependencies
4. **Documentation**: Excellent rustdoc integration

### Why Migration Was Beneficial
1. **Build Performance**: 50-300x improvement with Zig
2. **Simplicity**: Dramatically reduced complexity
3. **Control**: More direct control over compilation
4. **Dependencies**: Eliminated supply chain complexity

## For Developers Studying This Code

### Learning Objectives
- Language implementation in Rust
- LLVM integration patterns
- Async runtime usage
- Cross-platform compilation
- Memory management strategies

### Key Files to Study
- `src/parser/` - Recursive descent parsing
- `src/type_system/` - Type inference implementation
- `src/codegen/` - LLVM IR generation
- `src/runtime/` - Runtime system design
- `src/stdlib/` - Standard library architecture

## Migration Timeline

- **Phase 1**: Core language features (lexer, parser, basic codegen)
- **Phase 2**: Type system and advanced features
- **Phase 3**: Standard library migration
- **Phase 4**: Tools and development environment
- **Phase 5**: Performance optimization and validation

## Thank You, Rust

This Rust implementation served the CURSED project faithfully and made the eventual Zig implementation possible. The knowledge gained, patterns established, and tests created during the Rust phase were invaluable in creating the superior Zig implementation.

While this code is archived, it represents months of dedicated work and significant technical achievement in language implementation.

---

**Current CURSED Development**: Use `zig build` in the project root.  
**Historical Questions**: Refer to git history and migration documentation.  
**Archive Date**: August 2025
