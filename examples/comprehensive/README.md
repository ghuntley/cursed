# CURSED Language Comprehensive Examples

This directory contains complete, working examples that demonstrate all major features of the CURSED programming language. These examples showcase why CURSED is a complete, functional programming language despite its unique Gen Z slang syntax.

## Directory Structure

### 📚 Language Features (`language_features/`)
Core language constructs and syntax demonstrations:
- **Basic Syntax** - Variables, functions, control flow
- **Gen Z Slang Keywords** - All the iconic CURSED keywords in action
- **Type System** - Structs, interfaces, generics, type assertions
- **Concurrency** - Goroutines (`stan`), channels, synchronization
- **Error Handling** - Error propagation with `?` operator
- **Memory Management** - Garbage collection and memory safety

### 🏗️ Standard Library Modules (`stdlib_modules/`)
Comprehensive examples for each stdlib module:
- **Math Operations** - Basic, advanced, statistics, trigonometry
- **String Manipulation** - All string operations and utilities
- **File System** - File I/O, directory operations, permissions
- **Networking** - HTTP clients/servers, WebSockets, protocols
- **Database Operations** - SQLite, PostgreSQL, MongoDB integrations
- **Cryptography** - Complete crypto ecosystem with 10+ modules
- **Process Management** - System processes, IPC, monitoring
- **Time Handling** - Date/time operations, formatting, timezones
- **Collections** - Data structures, iterators, algorithms
- **Testing Framework** - Unit testing, benchmarks, assertions

### 🚀 Real-World Applications (`real_world_applications/`)
Complete applications demonstrating practical CURSED usage:
- **Web Server** - Full-featured HTTP server with routing and middleware
- **CLI Tool** - Command-line application with argument parsing
- **Microservice** - RESTful API with database integration
- **Chat Application** - Real-time messaging with WebSockets
- **File Processor** - Batch file processing with parallel execution
- **Crypto Wallet** - Cryptocurrency operations with security
- **Monitoring System** - System monitoring with metrics and alerts
- **Template Engine** - Dynamic content generation
- **Package Manager** - Package installation and dependency management

## Key Features Demonstrated

### 🎯 Gen Z Slang Syntax
```cursed
// Variable declarations
sus x = 42;              // Mutable variable
facts PI = 3.14159;      // Constant

// Function definitions  
slay function calculate_vibe(x: i32, y: i32) -> i32 {
    periodt x + y;  // Return statement
}

// Control flow
lowkey (x > 0) {
    spill("Positive vibes only!");
} highkey {
    spill("Not the vibe...");
}

// Loops with yield points
bestie (sus i = 0; i < 10; i++) {
    process_item(i);
    yolo;  // Yield point for goroutine scheduling
}

// Goroutine spawning
stan background_task();  // Like "go" but more iconic

// Error handling
facts result = risky_operation()?;  // Error propagation
```

### 🏛️ Production-Ready Features
- **Complete Type System** - Structs, interfaces, generics, type safety
- **Memory Safety** - Garbage collection with cycle detection
- **Concurrency** - Real goroutine runtime with work-stealing scheduler
- **Comprehensive Standard Library** - 30+ modules covering all needs
- **Error Handling** - Robust error propagation and recovery
- **Package Management** - Full dependency resolution and distribution
- **Testing Framework** - Unit tests, benchmarks, property testing
- **Documentation Generation** - Automatic API documentation
- **Build System** - Incremental compilation with optimization
- **Cross-Platform** - Linux, macOS, Windows support

### 🔧 Advanced Capabilities
- **LLVM Backend** - Optimized native code generation
- **JIT Compilation** - Runtime code optimization
- **Foreign Function Interface** - C library integration
- **WebAssembly Target** - Browser and edge deployment
- **Cryptographic Security** - Enterprise-grade crypto suite
- **Database Integration** - SQL and NoSQL database support
- **Web Framework** - Full-stack web development
- **Process Management** - System-level process control
- **Template System** - Multiple template format support

## Running the Examples

Each example directory contains:
- **Source code** (`.csd` files) - Complete implementations
- **Documentation** - Detailed explanations and API docs
- **Tests** - Unit tests and integration tests
- **Build configuration** - Compilation and optimization settings

### Prerequisites
```bash
# Ensure CURSED compiler is installed and in PATH
which cursed

# Build the CURSED standard library
make stdlib

# Run tests to verify setup
make test
```

### Example Execution
```bash
# Language features
cd examples/comprehensive/language_features/basic_syntax
cursed run main.csd

# Standard library modules  
cd examples/comprehensive/stdlib_modules/math_operations
cursed run math_demo.csd

# Real-world applications
cd examples/comprehensive/real_world_applications/web_server
cursed build --release
./web_server --port 8080
```

### Documentation Generation
```bash
# Generate documentation for examples
cursed doc examples/comprehensive/ \
    --output docs/examples \
    --format html \
    --format markdown \
    --include-examples \
    --include-source

# Serve documentation locally
cursed doc --serve 8080 --open
```

## Testing Examples

All examples include comprehensive tests:

```bash
# Test all examples
make test-examples

# Test specific category
make test-examples-stdlib
make test-examples-realworld

# Run with coverage
make test-examples-coverage
```

## Performance Benchmarks

Many examples include performance benchmarks:

```bash
# Run benchmarks
make bench-examples

# Generate performance reports
cursed bench examples/comprehensive/ --report perf_report.html
```

## Educational Value

These examples serve as:
- **Learning Materials** - Step-by-step introduction to CURSED
- **Reference Implementation** - Best practices and patterns
- **Performance Baselines** - Optimized code examples
- **Testing Validation** - Proof that CURSED works in practice
- **Documentation Source** - Live examples for API documentation

## Contributing Examples

When adding new examples:
1. Follow the established directory structure
2. Include comprehensive documentation
3. Add unit tests and integration tests
4. Ensure cross-platform compatibility
5. Include performance benchmarks where relevant
6. Update this README with new features demonstrated

## Why These Examples Matter

These comprehensive examples prove that CURSED is not just a novelty language with funny syntax - it's a complete, production-ready programming language that can handle:

- **Enterprise Applications** - Complex business logic and data processing
- **System Programming** - Low-level operations and performance-critical code
- **Web Development** - Full-stack applications with modern features
- **Distributed Systems** - Microservices and concurrent applications
- **Cryptographic Applications** - Security-critical implementations
- **Scientific Computing** - Mathematical operations and data analysis

The Gen Z slang syntax makes programming more engaging and memorable while maintaining all the power and safety of modern programming languages.

---

*Generated by CURSED Documentation System*
*Showing that serious programming can also be fun! 💅✨*
