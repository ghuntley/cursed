# CURSED Build System Documentation

## Overview

The CURSED build system is based on Zig's build system, providing fast, reliable compilation with comprehensive standard library integration.

## Quick Start

```bash
# Build the CURSED compiler
zig build

# Run CURSED programs in interpreter mode
./zig-out/bin/cursed-zig file.csd

# Compile CURSED programs to native binaries
./zig-out/bin/cursed-zig --compile file.csd
```

## Build Targets

### Core Targets
- **cursed-zig**: Main CURSED interpreter and compiler
- **cursed-lsp**: Language Server Protocol implementation
- **cursed-fmt**: Code formatter
- **cursed-lint**: Code linter
- **cursed-doc**: Documentation generator

### Building Specific Targets
```bash
zig build cursed-zig      # Build only the main interpreter
zig build cursed-lsp      # Build only the LSP server
zig build cursed-fmt      # Build only the formatter
```

## Development Workflow

### Standard Development Commands
```bash
# Clean build (recommended for development)
zig build clean
zig build

# Run tests
zig test src-zig/lexer.zig
zig test src-zig/parser.zig
zig test src-zig/type_system_runtime.zig

# Memory safety validation
valgrind --leak-check=full ./zig-out/bin/cursed-zig test.csd
```

### Performance Optimization
```bash
# Debug build (default for development)
zig build -Doptimize=Debug

# Release build (for production)
zig build -Doptimize=ReleaseFast

# Size-optimized build
zig build -Doptimize=ReleaseSmall
```

## Cross-Compilation

```bash
# Linux x86_64
zig build -Dtarget=x86_64-linux

# macOS ARM64
zig build -Dtarget=aarch64-macos

# Windows x86_64
zig build -Dtarget=x86_64-windows
```

## Build Configuration

The build system is configured in `build.zig` with support for:

- **LLVM Integration**: Native code generation through LLVM
- **Standard Library**: Automatic stdlib module resolution
- **Memory Management**: Arena allocators and garbage collection
- **Concurrency**: Goroutine runtime integration
- **Cross-Platform**: Support for Linux, macOS, Windows, WebAssembly

## Troubleshooting

### Common Issues

1. **Build hangs on compilation**
   ```bash
   # Use debug builds to avoid LLVM optimization issues
   zig build -Doptimize=Debug
   ```

2. **Undefined symbol errors**
   ```bash
   # Clean rebuild often fixes linking issues
   rm -rf zig-cache/ zig-out/
   zig build
   ```

3. **Memory leaks detected**
   ```bash
   # Always validate memory safety
   valgrind --leak-check=full --error-exitcode=1 ./zig-out/bin/cursed-zig file.csd
   ```

## Integration with Standard Library

The build system automatically:
- Links standard library modules
- Resolves module dependencies
- Optimizes unused code removal
- Validates module compatibility

For more information, see the [Standard Library Documentation](stdlib/README.md).
