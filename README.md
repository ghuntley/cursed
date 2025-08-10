# CURSED Programming Language

[![Build Status](https://api.cirrus-ci.com/github/ghuntley/cursed.svg)](https://cirrus-ci.com/github/ghuntley/cursed)
[![Release](https://img.shields.io/github/v/release/ghuntley/cursed)](https://github.com/ghuntley/cursed/releases)
[![License](https://img.shields.io/github/license/ghuntley/cursed)](LICENSE)

A powerful, modern programming language implemented in Zig with advanced features including:

> **📦 Implementation Migration**: CURSED has been successfully migrated from Rust to Zig, achieving 50-300x faster build times and zero memory leaks. The historical Rust implementation is preserved in `/archive/rust-implementation/` for reference.

## Features

- **Complete Language Ecosystem**: Compiler, package manager, LSP, debugger, and documentation tools
- **Advanced Cryptography**: Basic, advanced, and post-quantum cryptographic capabilities
- **Zero-Knowledge Proofs**: Built-in support for zk-SNARKs and other ZK protocols
- **Multi-Database Support**: SQLite, PostgreSQL, MySQL, Redis, MongoDB integration
- **LLVM Backend**: High-performance compilation with LLVM integration and optimizations  
- **Advanced Concurrency**: Custom goroutines, channels, and select operations
- **Memory Management**: Zero-leak memory management with arena allocators and GC
- **Cross-Platform**: Supports Linux, macOS, and Windows

## Installation

Download the latest release from the [releases page](https://github.com/ghuntley/cursed/releases).

### Linux/macOS
```bash
# Download and extract
curl -L https://github.com/ghuntley/cursed/releases/latest/download/cursed-linux-x86_64.tar.gz | tar -xz
# or for macOS
curl -L https://github.com/ghuntley/cursed/releases/latest/download/cursed-macos-x86_64.tar.gz | tar -xz

# Make executable and add to PATH
chmod +x cursed cursed-*
sudo mv cursed* /usr/local/bin/
```

## Usage

```bash
# Run a CURSED program
./zig-out/bin/cursed-zig program.csd

# Start the REPL
./zig-out/bin/cursed-zig --repl

# Compile to binary
./zig-out/bin/cursed-zig --compile program.csd

# Run tests with built-in framework
./zig-out/bin/cursed-zig test.csd

# Start language server
./zig-out/bin/cursed-lsp
```

## Building from Source

This project uses [devenv](https://devenv.sh/) for development environment management and Zig for compilation:

```bash
# Clone the repository
git clone https://github.com/ghuntley/cursed.git
cd cursed

# Enter development environment (provides Zig toolchain)
devenv shell

# Build the project (fast: 0.1-0.2s builds)
zig build

# Run tests
zig build test

# Run the compiler
./zig-out/bin/cursed-zig program.csd
```

## Tools Included

- **cursed-zig**: Main CURSED language compiler and interpreter (built with Zig)
- **cursed-stable**: Minimal stable compiler for core language features
- **cursed-lsp**: Language Server Protocol implementation
- **Built-in REPL**: Interactive development environment (`--repl` flag)
- **Built-in Compiler**: Native code generation (`--compile` flag)
- **Built-in Formatter**: Code formatting (`format` command)
- **Built-in Package Manager**: Dependency management (`pkg` command)
- **Built-in Documentation**: Doc generation (`doc` command)
- **Built-in Testing**: Test framework with testz stdlib module

## CI/CD

This project uses [Cirrus CI](https://cirrus-ci.com/) for continuous integration and automated releases:

- **Linux builds**: 8 CPU cores, 16GB RAM, 100GB disk
- **macOS builds**: 4 CPU cores, 8GB RAM
- **Automated releases**: Created automatically for tagged commits
- **Cross-platform testing**: Ensures compatibility across platforms

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

See [LICENSE](LICENSE) file for details.
