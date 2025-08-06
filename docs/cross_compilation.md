# CURSED Cross-Compilation Guide

This guide covers comprehensive cross-compilation support in the CURSED compiler, enabling you to build CURSED programs for multiple target platforms from a single development environment.

## Table of Contents

1. [Overview](#overview)
2. [Supported Platforms](#supported-platforms)
3. [Quick Start](#quick-start)
4. [CLI Usage](#cli-usage)
5. [Target Platform Details](#target-platform-details)
6. [Backend Compatibility](#backend-compatibility)
7. [Linking Options](#linking-options)
8. [Examples](#examples)
9. [Troubleshooting](#troubleshooting)
10. [Advanced Usage](#advanced-usage)

## Overview

CURSED supports cross-compilation to multiple target platforms, allowing you to:

- Build native executables for different operating systems and architectures
- Compile WebAssembly modules for browser deployment
- Generate platform-specific optimized binaries
- Support both static and dynamic linking modes
- Integrate with existing CI/CD pipelines for multi-platform releases

## Supported Platforms

| Target | Description | Architecture | LLVM Support | Status |
|--------|-------------|--------------|--------------|---------|
| `native` | Host platform (default) | Host | ✅ | ✅ Stable |
| `linux-x64` | Linux x86_64 | x86_64 | ✅ | ✅ Stable |
| `linux-arm64` | Linux ARM64 | aarch64 | ✅ | ✅ Stable |
| `macos-x64` | macOS x86_64 | x86_64 | ✅ | ✅ Stable |
| `macos-arm64` | macOS Apple Silicon | aarch64 | ✅ | ✅ Stable |
| `windows-x64` | Windows x86_64 | x86_64 | ✅ | ✅ Stable |
| `wasm32` | WebAssembly | wasm32 | ⚠️ Limited | ✅ Stable |

## Quick Start

### 1. Basic Cross-Compilation

```bash
# Compile for Linux x64
cursed compile hello.csd --target linux-x64

# Compile for WebAssembly
cursed compile hello.csd --target wasm32

# Compile for macOS ARM64 with static linking
cursed compile hello.csd --target macos-arm64 --linking static
```

### 2. Using the Build System

```bash
# Cross-compile all supported targets
zig build cross-compile

# Test cross-compilation setup
./test_cross_compilation.sh
```

## CLI Usage

### Command Syntax

```bash
cursed compile <source.csd> [OPTIONS]
```

### Cross-Compilation Options

| Option | Short | Description | Example |
|--------|-------|-------------|---------|
| `--target <TARGET>` | `-t` | Target platform | `-t linux-x64` |
| `--linking <MODE>` | `-l` | Linking mode | `-l static` |
| `--backend <BACKEND>` | `-b` | Compilation backend | `-b llvm` |
| `--output <FILE>` | `-o` | Output filename | `-o myapp` |
| `--optimize <LEVEL>` | `-O` | Optimization level (0-3) | `-O2` |
| `--verbose` | | Verbose output | |

### Examples

```bash
# Cross-compile with all options
cursed compile app.csd \
  --target linux-arm64 \
  --backend llvm \
  --linking static \
  --optimize 3 \
  --output myapp \
  --verbose

# Short form
cursed compile app.csd -t windows-x64 -b llvm -l static -O2 -o myapp.exe
```

## Target Platform Details

### Linux (linux-x64, linux-arm64)

**Features:**
- Full LLVM backend support
- Dynamic and static linking
- Standard C library integration
- POSIX system calls

**Requirements:**
- Cross-compilation toolchain (automatically detected)
- Target system headers (for advanced features)

```bash
# Linux x64 compilation
cursed compile app.csd --target linux-x64 --linking dynamic

# Linux ARM64 with optimization
cursed compile app.csd --target linux-arm64 -O3 --linking static
```

### macOS (macos-x64, macos-arm64)

**Features:**
- Apple Silicon and Intel support
- Framework linking support
- Code signing compatibility
- Optimized for Apple platforms

**Requirements:**
- Xcode command line tools (for advanced features)
- Apple Developer certificates (for code signing)

```bash
# macOS Intel compilation
cursed compile app.csd --target macos-x64

# macOS Apple Silicon with frameworks
cursed compile app.csd --target macos-arm64 --linking dynamic
```

### Windows (windows-x64)

**Features:**
- Native Windows executable generation
- Visual Studio toolchain integration
- Windows API access
- PE format output

**Requirements:**
- Windows SDK (for advanced Windows features)
- Visual Studio Build Tools (recommended)

```bash
# Windows compilation
cursed compile app.csd --target windows-x64 --output myapp.exe

# Windows with static linking
cursed compile app.csd --target windows-x64 --linking static
```

### WebAssembly (wasm32)

**Features:**
- Browser-compatible WASM modules
- Node.js compatibility
- Memory-safe execution
- Portable binary format

**Limitations:**
- No file system access
- Limited threading support
- No network access (browser sandbox)

```bash
# WebAssembly compilation
cursed compile app.csd --target wasm32 --output app.wasm

# WebAssembly with optimization
cursed compile app.csd --target wasm32 -O3 --output optimized.wasm
```

## Backend Compatibility

| Backend | Native | Linux | macOS | Windows | WebAssembly |
|---------|--------|-------|-------|---------|-------------|
| `llvm` | ✅ | ✅ | ✅ | ✅ | ⚠️ Limited |
| `c` | ✅ | ✅ | ✅ | ✅ | ❌ |
| `wasm` | ❌ | ❌ | ❌ | ❌ | ✅ |
| `script` | ✅ (interpret only) | ✅ (interpret only) | ✅ (interpret only) | ✅ (interpret only) | ❌ |

### Backend Selection Guidelines

- **LLVM**: Best for most cross-compilation scenarios
- **C**: Good for custom toolchain integration
- **WASM**: Required for WebAssembly targets
- **Script**: Interpretation only, not for compilation

## Linking Options

### Dynamic Linking (default)

**Advantages:**
- Smaller binary size
- Shared library updates
- Memory efficiency

**Disadvantages:**
- Runtime dependencies
- Potential compatibility issues

```bash
cursed compile app.csd --target linux-x64 --linking dynamic
```

### Static Linking

**Advantages:**
- Self-contained binaries
- No runtime dependencies
- Easier deployment

**Disadvantages:**
- Larger binary size
- Security update complexity

```bash
cursed compile app.csd --target linux-x64 --linking static
```

## Examples

### Complete Cross-Compilation Workflow

```bash
# 1. Create CURSED application
cat > multiplatform_app.csd << 'EOF'
yeet "vibez"

slay main() {
    vibez.spill("Hello from CURSED!")
    vibez.spill("Cross-platform application")
    damn 0
}
EOF

# 2. Compile for all major platforms
cursed compile multiplatform_app.csd --target linux-x64 -o linux_app
cursed compile multiplatform_app.csd --target macos-arm64 -o macos_app
cursed compile multiplatform_app.csd --target windows-x64 -o windows_app.exe
cursed compile multiplatform_app.csd --target wasm32 -o web_app.wasm

# 3. Verify outputs
ls -la *_app*
file *_app*
```

### Automated CI/CD Integration

```yaml
# GitHub Actions example
name: Cross-Platform Build
on: [push, pull_request]

jobs:
  cross-compile:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        target: [linux-x64, linux-arm64, macos-x64, macos-arm64, windows-x64, wasm32]
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Setup CURSED
      run: |
        zig build
        
    - name: Cross-compile for ${{ matrix.target }}
      run: |
        ./zig-out/bin/cursed compile app.csd \
          --target ${{ matrix.target }} \
          --optimize 3 \
          --linking static \
          --output app-${{ matrix.target }}
    
    - name: Upload artifacts
      uses: actions/upload-artifact@v3
      with:
        name: cursed-app-${{ matrix.target }}
        path: app-${{ matrix.target }}*
```

### Docker Multi-Architecture Builds

```dockerfile
# Multi-stage Dockerfile for cross-compilation
FROM ghuntley/cursed:latest AS builder

WORKDIR /app
COPY . .

# Build for multiple architectures
RUN cursed compile app.csd --target linux-x64 --linking static -o app-linux-x64
RUN cursed compile app.csd --target linux-arm64 --linking static -o app-linux-arm64

# Runtime stage
FROM scratch AS linux-x64
COPY --from=builder /app/app-linux-x64 /app
ENTRYPOINT ["/app"]

FROM scratch AS linux-arm64  
COPY --from=builder /app/app-linux-arm64 /app
ENTRYPOINT ["/app"]
```

## Troubleshooting

### Common Issues

#### 1. Target Not Found

```
Error: Unknown target 'linux-x64'. Valid targets: ...
```

**Solution:** Use `cursed --help` to see supported targets.

#### 2. Cross-Compilation Toolchain Missing

```
Error: Cross-compilation toolchain not found
```

**Solution:** Install development tools:

```bash
# Ubuntu/Debian
sudo apt-get install build-essential

# macOS
xcode-select --install

# Windows
# Install Visual Studio Build Tools
```

#### 3. WebAssembly Compilation Issues

```
Error: LLVM backend may not fully support WebAssembly target
```

**Solution:** Use WASM backend for WebAssembly:

```bash
cursed compile app.csd --target wasm32 --backend wasm
```

#### 4. Static Linking Failures

```
Error: Static linking failed - library not found
```

**Solution:** Install static libraries:

```bash
# Ubuntu/Debian
sudo apt-get install libc6-dev-i386  # for 32-bit targets
sudo apt-get install gcc-multilib    # for multi-arch

# Or use dynamic linking
cursed compile app.csd --target linux-x64 --linking dynamic
```

### Debugging Cross-Compilation

Enable verbose output for detailed information:

```bash
cursed compile app.csd --target linux-x64 --verbose
```

Test cross-compilation setup:

```bash
./test_cross_compilation.sh --verbose
```

## Advanced Usage

### Custom Target Architectures

For advanced users, you can specify custom Zig targets:

```bash
# Direct Zig target specification (build system level)
zig build -Dtarget=riscv64-linux
```

### Performance Optimization

```bash
# Maximum optimization for production
cursed compile app.csd \
  --target linux-x64 \
  --optimize 3 \
  --linking static \
  --lto

# Debug builds with symbols
cursed compile app.csd \
  --target linux-x64 \
  --optimize 0 \
  --debug-info \
  --preserve-debug-info
```

### Cross-Compilation Testing

```bash
# Test all supported targets
./test_cross_compilation.sh

# Test specific target with verbose output
VERBOSE=true ./test_cross_compilation.sh

# Integration with build system
zig build cross-test
```

### Memory and Binary Size Analysis

```bash
# Analyze binary size across platforms
cursed compile app.csd --target linux-x64 -o app-linux
cursed compile app.csd --target windows-x64 -o app-windows.exe
cursed compile app.csd --target wasm32 -o app.wasm

# Compare sizes
ls -lh app-* app.wasm

# Analyze with external tools
strip app-linux                    # Remove debug symbols
wasm-opt -O3 app.wasm -o app-opt.wasm  # Optimize WASM
```

---

## Resources

- [CURSED Language Documentation](../README.md)
- [Build System Guide](build_system.md)
- [Deployment Guide](deployment.md)
- [Performance Optimization](performance.md)

For issues or questions about cross-compilation, please visit the [CURSED GitHub repository](https://github.com/ghuntley/cursed).
