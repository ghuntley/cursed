# 🛠️ CURSED Development Tools Guide

Complete guide to the CURSED programming language development toolchain.

## 🎯 Overview

CURSED provides a comprehensive suite of development tools built for maximum productivity:

- **cursed-zig**: Main compiler and interpreter
- **cursed-lsp**: Language Server Protocol implementation  
- **cursed-stable**: Minimal stable compiler
- **cursed-perf**: Performance profiler and optimizer

## 🚀 Core Tools

### cursed-zig (Main Compiler)

The primary CURSED compiler and interpreter with full feature support.

#### Basic Usage
```bash
# Run CURSED program (interpreted)
cursed-zig program.csd

# Compile to native binary
cursed-zig program.csd --compile

# Interactive REPL
cursed-zig repl

# Type checking only
cursed-zig check program.csd
```

#### Advanced Options
```bash
# Debug output
cursed-zig program.csd --debug --verbose --tokens

# Optimization levels
cursed-zig program.csd --compile --optimize=3
cursed-zig program.csd --compile -O3

# Cross-compilation
cursed-zig program.csd --compile --target=aarch64-macos
cursed-zig program.csd --compile --target=x86_64-windows
cursed-zig program.csd --compile --target=wasm32

# LLVM IR generation
cursed-zig program.csd --emit-llvm

# Debug information
cursed-zig program.csd --compile --debug-info

# Performance optimization
cursed-zig program.csd --compile --enable-lto
cursed-zig program.csd --compile --enable-pgo
```

#### Built-in Commands
```bash
# Code formatting
cursed-zig format file.csd
cursed-zig format src/          # Directory
cursed-zig format --check src/  # Check formatting

# Code linting
cursed-zig lint file.csd
cursed-zig lint --format json src/
cursed-zig lint --fix src/      # Auto-fix issues
```

### cursed-lsp (Language Server)

Full Language Server Protocol implementation for IDE integration.

#### Starting the Server
```bash
# Stdio mode (for IDEs)
cursed-lsp --stdio

# TCP mode
cursed-lsp --tcp --port 9999

# Check server status
cursed-lsp --check
```

#### Features Provided
- **Hover Information**: Variable and function details
- **Code Completion**: Context-aware suggestions
- **Go to Definition**: Navigate to declarations
- **Error Diagnostics**: Real-time error reporting
- **Code Formatting**: Automatic code formatting
- **Rename Refactoring**: Symbol renaming
- **Find References**: Usage tracking

#### IDE Configuration

**VS Code**: Install the CURSED extension
```bash
code --install-extension cursed-lang.cursed-vscode
```

**Vim/Neovim**: Configure with nvim-lspconfig
```vim
lua << EOF
require'lspconfig'.cursed.setup{
  cmd = {"cursed-lsp", "--stdio"},
  filetypes = {"cursed"},
  root_dir = require'lspconfig'.util.root_pattern("CursedPackage.toml", ".git"),
}
EOF
```

**Any LSP-compatible editor**:
- Command: `cursed-lsp --stdio`
- File extensions: `.csd`
- Language ID: `cursed`

### cursed-stable (Minimal Compiler)

Lightweight compiler with core language features only.

#### Usage
```bash
# Fast compilation for CI/CD
cursed-stable program.csd --compile

# Minimal resource usage
cursed-stable program.csd --optimize=1
```

#### Use Cases
- **CI/CD pipelines**: Fast builds in constrained environments
- **Embedded development**: Minimal resource usage
- **Quick prototyping**: Rapid iteration without full features
- **Production builds**: When stability is critical

### cursed-perf (Performance Profiler)

Advanced performance analysis and optimization tool.

#### Profiling
```bash
# Profile execution
cursed-perf profile program.csd

# Memory profiling
cursed-perf memory program.csd

# Function-level profiling
cursed-perf functions program.csd

# Hot path analysis
cursed-perf hotpaths program.csd
```

#### Optimization
```bash
# Generate optimized build
cursed-perf optimize program.csd

# Profile-guided optimization
cursed-perf pgo-train program.csd input.txt
cursed-perf pgo-build program.csd
```

#### Performance Reports
```bash
# Benchmark suite
cursed-perf benchmark program.csd

# Performance regression detection
cursed-perf regression baseline.json current.csd

# Export metrics
cursed-perf metrics --format json program.csd
```

## 📦 Package Management

### Project Structure
```
my-project/
├── CursedPackage.toml    # Package configuration
├── src/
│   ├── main.csd          # Entry point
│   └── utils.csd         # Utilities
├── test/
│   └── test_main.csd     # Tests
├── docs/                 # Documentation
└── examples/             # Examples
```

### CursedPackage.toml
```toml
[package]
name = "my-project"
version = "1.0.0"
description = "My awesome CURSED project"
authors = ["Your Name <you@example.com>"]
license = "MIT"

[dependencies]
mathz = "1.0.0"
stringz = "1.0.0"
networkz = { version = "1.0.0", features = ["tls"] }

[dev-dependencies]
testz = "1.0.0"

[build]
entry_point = "src/main.csd"
optimize = true
target = "native"

[scripts]
test = "cursed-zig test/test_main.csd"
bench = "cursed-perf benchmark src/main.csd"
format = "cursed-zig format src/"
```

### Package Commands
```bash
# Create new project
cursed init my-project
cd my-project

# Add dependencies
cursed add mathz@1.0.0
cursed add networkz --features tls

# Build project
cursed build

# Run tests
cursed test

# Publish package
cursed publish
```

## 🔧 Development Workflow

### Daily Development
```bash
# Start development
cd my-project

# Edit code with IDE (VS Code, Vim, etc.)
code src/main.csd

# Quick test
cursed-zig src/main.csd

# Format code
cursed-zig format src/

# Lint code
cursed-zig lint src/

# Run tests
cursed-zig test/test_main.csd

# Build release
cursed-zig src/main.csd --compile --optimize=3
```

### CI/CD Pipeline
```yaml
# .github/workflows/ci.yml
name: CI
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Install CURSED
        run: curl -sSf https://install.cursedlang.org | sh
      - name: Format check
        run: cursed-zig format --check src/
      - name: Lint
        run: cursed-zig lint src/
      - name: Type check
        run: cursed-zig check src/main.csd
      - name: Test
        run: cursed-zig test/test_main.csd
      - name: Build
        run: cursed-zig src/main.csd --compile --optimize=3
```

### Cross-Platform Builds
```bash
# Build for all platforms
cursed-zig src/main.csd --compile --target=x86_64-linux
cursed-zig src/main.csd --compile --target=x86_64-macos
cursed-zig src/main.csd --compile --target=aarch64-macos
cursed-zig src/main.csd --compile --target=x86_64-windows
cursed-zig src/main.csd --compile --target=wasm32

# Automated cross-compilation
./scripts/build-all-platforms.sh
```

## 🐛 Debugging

### Debug Information
```bash
# Compile with debug info
cursed-zig program.csd --compile --debug-info

# Debug with GDB
gdb ./program
(gdb) break main
(gdb) run
(gdb) step
```

### Debug Output
```bash
# Verbose compilation
cursed-zig program.csd --debug --verbose

# Token stream analysis
cursed-zig program.csd --tokens

# LLVM IR inspection
cursed-zig program.csd --emit-llvm
cat program.ll
```

### Memory Debugging
```bash
# Memory leak detection
valgrind --leak-check=full cursed-zig program.csd

# Memory usage profiling
cursed-perf memory program.csd

# Address sanitizer
cursed-zig program.csd --compile --debug --address-sanitizer
```

## 📊 Performance Analysis

### Benchmarking
```bash
# Built-in benchmarks
cursed-perf benchmark program.csd

# Custom benchmarks
time cursed-zig program.csd
hyperfine 'cursed-zig program.csd'

# Compilation performance
time cursed-zig program.csd --compile
```

### Profiling
```bash
# Function profiling
cursed-perf profile program.csd

# Memory allocation profiling
cursed-perf memory program.csd

# Hot path analysis
cursed-perf hotpaths program.csd

# Performance regression testing
cursed-perf regression baseline.json program.csd
```

## 🏗️ Advanced Features

### Custom Optimizations
```bash
# Link-time optimization
cursed-zig program.csd --compile --enable-lto

# Profile-guided optimization
cursed-zig program.csd --profile-data=training.prof

# Target-specific optimizations
cursed-zig program.csd --compile --target-cpu=native
cursed-zig program.csd --compile --target-features=+avx2,+fma
```

### Static Analysis
```bash
# Comprehensive linting
cursed-zig lint --strict src/

# Security analysis
cursed-zig lint --security src/

# Performance analysis
cursed-zig lint --performance src/

# Custom lint rules
cursed-zig lint --config=.cursed-lint.toml src/
```

## 🔍 Troubleshooting

### Common Issues

#### Build Failures
```bash
# Clean build
rm -rf .cursed-cache/
cursed-zig program.csd --compile

# Verbose error output
cursed-zig program.csd --verbose

# Check system requirements
cursed-zig --version
```

#### Performance Issues
```bash
# Profile slow builds
cursed-perf profile-build program.csd

# Optimize for speed
cursed-zig program.csd --compile --optimize=3 --enable-lto

# Check for memory leaks
valgrind cursed-zig program.csd
```

#### IDE Integration Issues
```bash
# Restart language server
cursed-lsp --check
killall cursed-lsp

# Verify LSP configuration
cursed-lsp --stdio < /dev/null

# Update VS Code extension
code --install-extension cursed-lang.cursed-vscode --force
```

## 📚 Learning Resources

### Tool Documentation
- [Language Server Guide](docs/tools/lsp-guide.md)
- [Performance Optimization](docs/tools/performance.md)
- [Cross-Compilation](docs/tools/cross-compilation.md)
- [Debugging Guide](docs/tools/debugging.md)

### Video Tutorials
- [CURSED Tools Overview](https://youtube.com/watch?v=cursed-tools)
- [IDE Setup Guide](https://youtube.com/watch?v=cursed-ide-setup)
- [Performance Profiling](https://youtube.com/watch?v=cursed-profiling)

### Community Resources
- **Discord**: [discord.gg/cursed-lang](https://discord.gg/cursed-lang)
- **GitHub**: [github.com/ghuntley/cursed](https://github.com/ghuntley/cursed)
- **Documentation**: [docs.cursedlang.org](https://docs.cursedlang.org)

## 🎯 Quick Reference

### Essential Commands
```bash
# Run program
cursed-zig program.csd

# Compile to binary
cursed-zig program.csd --compile

# Start REPL
cursed-zig repl

# Format code
cursed-zig format src/

# Lint code
cursed-zig lint src/

# Start LSP
cursed-lsp --stdio

# Profile performance
cursed-perf profile program.csd
```

### Keyboard Shortcuts (VS Code)
- **Ctrl+Shift+P**: Command palette
- **F12**: Go to definition
- **Shift+F12**: Find all references
- **Ctrl+Space**: Code completion
- **F2**: Rename symbol
- **Ctrl+Shift+I**: Format document

This comprehensive tooling ecosystem makes CURSED development productive, efficient, and enjoyable! 🔥
