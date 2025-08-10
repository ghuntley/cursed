# CURSED Programming Language Documentation

Welcome to the comprehensive documentation for the CURSED programming language compiler and runtime system.

## Documentation Structure

### 📚 User Documentation
- **[Language Guide](user-guide/)** - Complete syntax and features reference
- **[Getting Started](user-guide/getting-started.md)** - Quick start tutorial
- **[Standard Library](user-guide/stdlib/)** - Comprehensive stdlib documentation
- **[Examples](examples/)** - Real-world code examples

### 🔧 Developer Documentation
- **[Architecture Overview](developer-guide/architecture.md)** - System design and components
- **[Build System](developer-guide/build-system.md)** - Build and deployment guide
- **[API Reference](api/)** - Complete API documentation
- **[Contributing](developer-guide/contributing.md)** - Development guidelines

### 🚀 Production Deployment
- **[Installation Guide](deployment/installation.md)** - Production setup
- **[Performance Guide](deployment/performance.md)** - Optimization and benchmarks
- **[Security Guide](deployment/security.md)** - Security best practices
- **[Monitoring](deployment/monitoring.md)** - Observability and debugging

### 📖 Migration & Support
- **[Migration from Rust](migration/from-rust.md)** - Transition guide
- **[Troubleshooting](support/troubleshooting.md)** - Common issues and solutions
- **[FAQ](support/faq.md)** - Frequently asked questions
- **[Release Notes](CHANGELOG.md)** - Version history and changes

## Quick Reference

### Essential Commands
```bash
# Build the compiler
zig build

# Run CURSED programs
./zig-out/bin/cursed-zig program.csd

# Compile to native binary
./zig-out/bin/cursed-zig --compile program.csd

# Memory safety validation
valgrind ./zig-out/bin/cursed-zig program.csd
```

### Language Quick Start
```cursed
# Variables and output
sus message tea = "Hello, CURSED!"
vibez.spill(message)

# Functions
slay greet(name tea) tea {
    damn "Hello, " + name + "!"
}

# Modules
yeet "mathz"
sus result drip = abs_normie(-42)
```

## System Status

**Current Version**: v1.0-alpha  
**Production Readiness**: ~95% complete  
**Memory Safety**: Zero leaks confirmed  
**Performance**: 0.1-0.2s build times  

For support, visit our [GitHub repository](https://github.com/ghuntley/cursed) or check the [troubleshooting guide](support/troubleshooting.md).
