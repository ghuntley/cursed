# Cross-Compilation Linker Script Selection for CURSED

The CURSED compiler now includes a comprehensive linker script selection system that automatically chooses appropriate linker configurations based on target platform and architecture. This system integrates with the target triple normalization to provide optimal cross-compilation support.

## Features

### Target Triple Normalization
- Automatic normalization of target triples for different platforms
- Support for user-friendly target names (e.g., `linux-arm64`, `windows-x64`, `macos-apple-silicon`)
- Conversion between different target triple formats (LLVM, Rust, GNU, Apple, Zig)

### Platform-Specific Linker Configurations
- **Linux x86_64/ARM64**: Standard system linker with GC optimizations
- **Windows MinGW/MSVC**: Static linking configuration for portable executables
- **macOS Intel/Apple Silicon**: Apple-specific linking optimizations
- **WebAssembly**: Memory layout and export configuration for web targets
- **Embedded ARM64**: Custom linker scripts with memory layout specifications

### ARM64 Specific Features
- Cortex-A53 CPU errata fixes for ARM64 Linux targets
- Apple Silicon optimization for macOS ARM64
- Custom embedded linker scripts for bare-metal ARM64

## Usage

### Basic Cross-Compilation

```bash
# Linux ARM64 cross-compilation
zig build -Dtarget=aarch64-linux

# Windows cross-compilation  
zig build -Dtarget=x86_64-windows

# macOS Apple Silicon cross-compilation
zig build -Dtarget=aarch64-macos

# WebAssembly compilation
zig build -Dtarget=wasm32-wasi

# Embedded ARM64 compilation
zig build -Dtarget=aarch64-freestanding
```

### Available Build Commands

```bash
# List all available linker configurations
zig build list-linker-configs

# Validate linker configuration for current target
zig build validate-linker

# Generate linker scripts for embedded targets
zig build generate-linker-scripts

# Build with verbose linker information
zig build --verbose
```

### Supported Target Triples

| Target Triple | Platform | Notes |
|---------------|----------|-------|
| `x86_64-unknown-linux-gnu` | Linux x86_64 | Native/cross compilation |
| `aarch64-unknown-linux-gnu` | Linux ARM64 | Cross-compilation with ARM64 optimizations |
| `x86_64-pc-windows-gnu` | Windows MinGW | Static linking for portability |
| `x86_64-pc-windows-msvc` | Windows MSVC | MSVC toolchain integration |
| `aarch64-pc-windows-gnu` | Windows ARM64 | ARM64 Windows support |
| `x86_64-apple-darwin` | macOS Intel | Intel Mac optimization |
| `aarch64-apple-darwin` | macOS Apple Silicon | Apple Silicon optimization |
| `wasm32-unknown-unknown` | WebAssembly | Standard WebAssembly |
| `wasm32-wasi` | WebAssembly WASI | WASI system interface |
| `aarch64-unknown-none` | Embedded ARM64 | Custom linker scripts |

## Platform-Specific Configurations

### Linux ARM64 (`aarch64-unknown-linux-gnu`)
- **Linker flags**: `--as-needed`, `--gc-sections`, ARM64 errata fixes
- **Libraries**: `c`, `m`, `pthread`
- **Features**: Cortex-A53 CPU bug workarounds

### Windows MinGW (`x86_64-pc-windows-gnu`)
- **Linker flags**: Static libgcc/libstdc++, stripped debug symbols
- **Libraries**: `mingw32`, `ws2_32`, `kernel32`
- **Features**: Portable static executables

### macOS Apple Silicon (`aarch64-apple-darwin`)
- **Linker flags**: Dead code stripping, symbol stripping
- **Libraries**: `System`
- **Features**: Apple-specific optimizations

### WebAssembly WASI (`wasm32-wasi`)
- **Memory layout**: 2MB initial, 32MB maximum
- **Features**: WASI system interface support
- **Optimization**: Size and performance optimized

### Embedded ARM64 (`aarch64-unknown-none`)
- **Custom linker script**: `linker_scripts/aarch64_embedded.ld`
- **Memory layout**: 2MB Flash @ 0x8000000, 256KB RAM @ 0x20000000
- **Features**: Bare-metal embedded system support

## Linker Script Directory Structure

```
cursed/
├── linker_scripts/
│   └── aarch64_embedded.ld    # Generated ARM64 embedded linker script
├── src-zig/
│   ├── linker_script_manager.zig       # Linker script management
│   └── target_triple_normalization.zig # Target triple normalization
└── build.zig                          # Build system integration
```

## Memory Layout for Embedded Targets

The embedded ARM64 linker script provides the following memory layout:

```
FLASH (rx)  : 0x8000000 - 0x8200000 (2MB)   - Code and constants
RAM (rwx)   : 0x20000000 - 0x20040000 (256KB) - Data, BSS, heap, stack

Memory Sections:
- .text     : Code and read-only data in FLASH
- .data     : Initialized data in RAM (loaded from FLASH)
- .bss      : Uninitialized data in RAM
- .heap     : 32KB heap space
- .stack    : 8KB stack space
```

## Integration with Existing Systems

The linker script selection system integrates with:

1. **Target Triple Normalization**: Automatic conversion of target specifications
2. **LLVM Backend**: Proper target configuration for code generation
3. **Cross-Compilation Manager**: Toolchain discovery and configuration
4. **Build System**: Zig build integration with proper dependency management

## Error Handling

The system provides comprehensive error handling:

- Validates target triple format and platform support
- Checks for existence of custom linker scripts
- Falls back to system default linkers when appropriate
- Provides detailed verbose output for debugging

## Examples

### Cross-Compiling for ARM64 Linux

```bash
# Build with ARM64-specific optimizations
zig build -Dtarget=aarch64-linux --verbose

# Output shows:
# Target triple: aarch64-unknown-linux-gnu
# Linker args: --fix-cortex-a53-843419, --fix-cortex-a53-835769
# Required libs: c, m, pthread
```

### Building for Embedded ARM64

```bash
# Generate embedded linker script first
zig build generate-linker-scripts

# Build for embedded target
zig build -Dtarget=aarch64-freestanding --verbose

# Output shows:
# Custom script: linker_scripts/aarch64_embedded.ld
# Memory layout: Flash=0x8000000+0x200000, RAM=0x20000000+0x40000
```

### Validating Cross-Compilation Setup

```bash
# Check available configurations
zig build list-linker-configs

# Validate current target
zig build validate-linker

# Comprehensive build validation
zig build validate
```

## Advanced Usage

### Custom Linker Scripts

To use custom linker scripts for specific targets:

1. Create the linker script in `linker_scripts/` directory
2. Update the `PlatformConfigs` in `linker_script_manager.zig`
3. Rebuild and test with the target

### Adding New Target Support

To add support for a new target platform:

1. Add target triple mapping in `target_triple_normalization.zig`
2. Add linker configuration in `linker_script_manager.zig`
3. Add target triple generation in `build.zig`
4. Test with the new target

## Troubleshooting

### Common Issues

1. **Linker script not found**: Run `zig build generate-linker-scripts`
2. **Cross-compilation hanging**: Use native builds only or install cross-toolchain
3. **Memory layout errors**: Check custom linker script syntax and memory regions
4. **Missing libraries**: Install target-specific development packages

### Debug Commands

```bash
# Verbose build output
zig build --verbose

# Linker configuration details
zig build validate-linker

# Available configurations
zig build list-linker-configs

# Test linker script manager
zig test src-zig/linker_script_manager.zig
```

## Performance Considerations

- Linker script selection adds minimal overhead to build process
- Target triple normalization is cached for performance
- Custom linker scripts are validated only once per build
- Cross-compilation toolchain discovery is optimized for common paths

## Security Considerations

- Linker scripts are validated for existence before use
- No arbitrary code execution in linker script generation
- Memory layouts use safe, well-tested configurations
- Static linking reduces runtime dependencies for better security
