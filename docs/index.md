# CURSED Programming Language Documentation

Welcome to the comprehensive documentation for the CURSED programming language - a modern, self-hosting systems programming language with advanced type safety and performance.

## Quick Start

```bash
# Install CURSED compiler
cargo build --release

# Run your first program
echo 'vibez.spill("Hello, CURSED!")' > hello.csd
cargo run --bin cursed hello.csd

# Compile to native executable
cargo run --bin cursed -- compile hello.csd
./hello
```

## 📚 Learning Path

### [Beginner Tutorial Series](tutorials/beginner/README.md)
- [Getting Started](tutorials/beginner/01-getting-started.md)
- [Basic Syntax](tutorials/beginner/02-basic-syntax.md)
- [Variables and Types](tutorials/beginner/03-variables-types.md)
- [Control Flow](tutorials/beginner/04-control-flow.md)
- [Functions](tutorials/beginner/05-functions.md)

### [Intermediate Tutorial Series](tutorials/intermediate/README.md)
- [Advanced Types](tutorials/intermediate/01-advanced-types.md)
- [Error Handling](tutorials/intermediate/02-error-handling.md)
- [Concurrency](tutorials/intermediate/03-concurrency.md)
- [Pattern Matching](tutorials/intermediate/04-pattern-matching.md)
- [Module System](tutorials/intermediate/05-module-system.md)

### [Advanced Tutorial Series](tutorials/advanced/README.md)
- [Generics and Constraints](tutorials/advanced/01-generics.md)
- [Interfaces and Traits](tutorials/advanced/02-interfaces.md)
- [Memory Management](tutorials/advanced/03-memory-management.md)
- [Compiler Optimization](tutorials/advanced/04-optimization.md)
- [Self-Hosting Development](tutorials/advanced/05-self-hosting.md)

## 🔧 Development Tools

### [Language Server Protocol (LSP)](LSP_SERVER.md)
Complete IDE integration with VS Code, Vim, and other editors.

### [Build System](build_system.md)
Comprehensive build tooling written in CURSED.

### [Package Manager](package_manager.md)
Dependency management and workspace support.

### [Testing Framework](testz.md)
Enterprise-grade testing with multiple specialized frameworks.

## 📖 Language Reference

### [Grammar and Syntax](grammar/README.md)
Complete language grammar specification.

### [Type System](types/README.md)
Comprehensive type system documentation.

### [Standard Library](stdlib/README.md)
Complete API reference for all 543+ stdlib modules.

### [Memory Management](memory/README.md)
Garbage collection and memory safety features.

### [Concurrency](concurrency/README.md)
Goroutines, channels, and async programming.

## 🚀 Migration Guides

### [From Other Languages](migration/README.md)
- [Python to CURSED](migration/python-to-cursed.md)
- [Go to CURSED](migration/go-to-cursed.md)
- [Rust to CURSED](migration/rust-to-cursed.md)
- [JavaScript to CURSED](migration/javascript-to-cursed.md)
- [C++ to CURSED](migration/cpp-to-cursed.md)

## 📝 Examples

### [Example Library](examples/README.md)
Comprehensive examples from simple programs to complex applications.

### [Cookbook](cookbook/README.md)
Common patterns and best practices.

### [Project Templates](templates/README.md)
Starter templates for different project types.

## 🛠️ Advanced Topics

### [Compiler Internals](compiler/README.md)
Deep dive into the CURSED compiler architecture.

### [LLVM Integration](llvm/README.md)
How CURSED integrates with LLVM for optimization.

### [Self-Hosting](self-hosting/README.md)
Complete guide to the self-hosting compiler.

### [Performance Optimization](performance/README.md)
Advanced optimization techniques and profiling.

## 🎯 Ecosystem

### [Third-Party Libraries](ecosystem/libraries.md)
Popular libraries and frameworks in the CURSED ecosystem.

### [Community Tools](ecosystem/tools.md)
Community-contributed development tools.

### [IDE Extensions](ecosystem/ide.md)
Editor and IDE support for CURSED.

## 📊 Benchmarks

### [Performance Comparisons](benchmarks/README.md)
Performance benchmarks against other languages.

### [Memory Usage](benchmarks/memory.md)
Memory efficiency analysis.

## 🎓 Resources

### [FAQ](faq.md)
Frequently asked questions and solutions.

### [Troubleshooting](troubleshooting.md)
Common issues and resolution guides.

### [Contributing](../CONTRIBUTING.md)
How to contribute to the CURSED project.

### [Changelog](../CHANGELOG.md)
Version history and release notes.

## 🏆 Production Ready

CURSED is production-ready with:
- ✅ 100% Self-hosting compiler
- ✅ 543+ Pure CURSED stdlib modules
- ✅ Complete LSP server integration
- ✅ Comprehensive testing framework
- ✅ Enterprise-grade tooling ecosystem
- ✅ Advanced optimization pipeline
- ✅ Memory safety and garbage collection
- ✅ Full concurrency support

---

*This documentation is automatically generated and continuously updated. For the latest information, visit the [CURSED GitHub repository](https://github.com/ghuntley/cursed).*
