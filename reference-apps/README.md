# CURSED v1.0 Reference Applications

This directory contains 5 comprehensive reference applications demonstrating CURSED's capabilities:

## Applications

1. **CLI Tool** (`cli-tool/`) - File organization and search utility
2. **Web Server** (`web-server/`) - HTTP server with routing and middleware  
3. **Database App** (`database-app/`) - SQLite CRUD operations with connection pooling
4. **Crypto App** (`crypto-app/`) - File encryption/decryption with multiple algorithms
5. **Concurrent App** (`concurrent-app/`) - Producer-consumer with goroutines and channels

## Running Applications

Each application includes:
- Complete source code with comments
- Build instructions and dependencies
- Usage examples and documentation
- Cross-platform compilation support

```bash
# Build all reference apps
zig build

# Run specific application
cd reference-apps/cli-tool
../../zig-out/bin/cursed-zig main.csd --help

# Cross-platform test
cd reference-apps
./test-all-platforms.sh
```

## Features Demonstrated

- **Standard Library**: All 50+ modules showcased across applications
- **Concurrency**: Goroutines, channels, select operations
- **Error Handling**: Structured error management with yikes/fam/shook
- **Type System**: Generics, interfaces, pattern matching
- **Interop**: FFI with C libraries (SQLite, OpenSSL)
- **Performance**: Memory safety with zero-cost abstractions

These applications serve as production-ready examples and learning resources for CURSED development.
