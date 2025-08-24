# CURSED Standard Library

## Overview

The CURSED standard library is a comprehensive collection of modules providing essential functionality for systems programming, web development, data processing, and more. All modules are implemented in 100% pure CURSED code with no FFI dependencies.

## Migration Status: 45% Complete (380/907 files)

### ✅ Production-Ready Modules (100% Pure CURSED)

#### Core System Modules
- **`time`** - Time operations, formatting, parsing, timezones, duration handling
- **`memory`** - Memory allocation, garbage collection, leak detection, object pools
- **`process`** - Process spawning, execution, signals, environment variables
- **`regex`** - Regular expression engine with full POSIX support
- **`error_handling`** - Comprehensive error management and propagation
- **`mimez`** - MIME type detection from extensions and content, Content-Type headers

#### Data & Collections
- **`collections`** - Vector, HashMap, LinkedList, Set, Stack, Queue, sorting algorithms
- **`json`** - RFC 7159 compliant JSON parsing and generation
- **`string`** - String manipulation, searching, formatting operations
- **`math`** - Mathematical functions, constants, number theory

#### Networking & Web
- **`vibe_net`** - TCP/UDP sockets, DNS resolution, connection management
- **`web_vibez`** - HTTP client/server, routing, middleware, WebSocket support
- **`tls_vibe`** - TLS/SSL implementation (placeholder completion needed)

#### Database & Storage
- **`database_drivers`** - Database connectivity (100% FFI elimination complete)
- **`fs`** - File system operations, path manipulation
- **`io`** - Input/output operations, file handling

#### Security & Crypto
- **`cryptz`** - Cryptographic functions (ChaCha20, hashing, key derivation)
- **`crypto_secure`** - Security-focused cryptographic operations

#### Testing & Development
- **`testz`** - Testing framework with assertions and test reporting
- **`debug`** - Debugging utilities and development tools

### 🟡 In Progress Modules

#### Advanced I/O
- **`compression`** - Data compression algorithms (basic implementation complete)
- **`image_processing`** - Graphics and image manipulation (needs completion)

#### Platform Integration
- **`async`** - Asynchronous programming primitives
- **`concurrency`** - Advanced concurrency patterns and utilities

### 📦 Module Usage Examples

#### Time Operations
```cursed
yeet "time"

sus current Time = now()
sus formatted tea = current.format("2006-01-02 15:04:05")
vibez.spill("Current time: " + formatted)

sus duration Duration = hour()
sus later Time = current.add(duration)
```

#### Regular Expressions
```cursed
yeet "regex"

sus pattern Pattern = compile("\\d+")
sus matches []Match = pattern.find_all("abc123def456")
vibez.spill("Found numbers: " + matches[0].text + ", " + matches[1].text)
```

#### Process Management
```cursed
yeet "process"

sus result CommandResult = exec("echo", ["Hello CURSED"])
vibez.spill("Output: " + result.stdout)
vibez.spill("Success: " + result.success.(tea))
```

#### Memory Management
```cursed
yeet "memory"

sus addr normie = malloc(1024)
memset(addr, 0, 1024)
free(addr)

sus stats MemoryPool = get_memory_stats()
vibez.spill("Memory allocated: " + stats.total_allocated.(tea))
```

#### Collections
```cursed
yeet "collections"

sus vec [normie] = Vec_new()
vec = Vec_push(vec, 42)
vec = Vec_push(vec, 24)
sus sorted [normie] = Collections_bubble_sort(vec)
```

#### Web Framework
```cursed
yeet "web_vibez"

sus response tea = http_get("https://api.example.com/data")
sus json_response tea = build_json_response(200, "success")
vibez.spill("HTTP Response: " + response)
```

### 🧪 Testing

Each module includes comprehensive test suites using the `testz` framework:

```bash
# Run individual module tests
./cursed-unified stdlib/time/test_time.csd
./cursed-unified stdlib/regex/test_regex.csd
./cursed-unified stdlib/memory/test_memory.csd
./cursed-unified stdlib/process/test_process.csd

# Run all stdlib tests
./cursed-unified stdlib/comprehensive_stdlib_test.csd
```

### 📋 Module Structure

Each stdlib module follows a consistent structure:

```
stdlib/module_name/
├── mod.csd           # Main module implementation
├── test_module.csd   # Comprehensive test suite
└── README.md         # Module-specific documentation
```

### 🚀 Performance Characteristics

- **Memory Efficient**: Zero-copy operations where possible
- **Fast Execution**: Optimized algorithms for common operations
- **Minimal Overhead**: Lightweight runtime with efficient data structures
- **Scalable**: Designed for both small scripts and large applications

### 🔧 Development Guidelines

#### For Module Authors
1. **Pure CURSED Only**: No FFI dependencies allowed
2. **Comprehensive Testing**: Minimum 50 test cases per module
3. **Complete Implementation**: No placeholders or stubs
4. **Documentation**: Clear function signatures and usage examples
5. **Error Handling**: Proper validation and error propagation

#### Code Style
- Use `fr fr` for comments
- Follow CURSED naming conventions (`snake_case` for functions, `PascalCase` for types)
- Include type annotations for all function parameters
- Provide realistic behavior for system operations

### 📚 Additional Resources

- [CURSED Language Specification](../specs/)
- [Testing Framework Documentation](testz/README.md)
- [Migration Progress Report](../STDLIB_MIGRATION_COMPLETION_REPORT.md)
- [Build System Documentation](../BUILD_SYSTEM_README.md)

### 🤝 Contributing

To contribute to stdlib development:

1. Choose a module that needs migration or improvement
2. Implement in 100% pure CURSED (no FFI)
3. Create comprehensive test suite
4. Follow existing module patterns and documentation
5. Ensure all tests pass with the CURSED compiler

### 📞 Support

For questions about stdlib modules or migration efforts, see:
- [Development Documentation](../docs/)
- [Build and Test Instructions](../AGENT.md)
- [Issue Tracking](../specs/)
