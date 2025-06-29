# CURSED Programming Language

[![Build Status](https://api.cirrus-ci.com/github/ghuntley/cursed.svg)](https://cirrus-ci.com/github/ghuntley/cursed)
[![Release](https://img.shields.io/github/v/release/ghuntley/cursed)](https://github.com/ghuntley/cursed/releases)
[![License](https://img.shields.io/github/license/ghuntley/cursed)](LICENSE)

A powerful, modern programming language with advanced features including:

## Features

- **Complete Language Ecosystem**: Compiler, package manager, LSP, debugger, and documentation tools
- **Advanced Cryptography**: Basic, advanced, and post-quantum cryptographic capabilities
- **Zero-Knowledge Proofs**: Built-in support for zk-SNARKs and other ZK protocols
- **Multi-Database Support**: SQLite, PostgreSQL, MySQL, Redis, MongoDB integration
- **LLVM Backend**: High-performance compilation with LLVM 17
- **Async Runtime**: Built on Tokio for high-concurrency applications
- **Memory Management**: Garbage collection with cycle detection
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
cursed program.csd

# Start the REPL
cursed-repl

# Build a project
cursed-build

# Run tests
cursed-test

# Start language server
cursed-lsp
```

## Building from Source

This project uses [devenv](https://devenv.sh/) for development environment management:

```bash
# Clone the repository
git clone https://github.com/ghuntley/cursed.git
cd cursed

# Enter development environment
devenv shell

# Build the project
cargo build --release

# Run tests
cargo test
```

## Tools Included

- **cursed**: Main CURSED language interpreter
- **cursed-doc**: Documentation generator
- **cursed-build**: Project build system
- **cursed-pkg**: Package manager
- **cursed-repl**: Interactive REPL
- **cursed-test**: Test runner
- **cursed-lsp**: Language Server Protocol implementation
- **cursed-debug**: Debugger
- **cursed-compile-fast**: Fast compiler

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
