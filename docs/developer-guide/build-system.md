# Build System Developer Guide

## Architecture

The CURSED build system uses Zig as the underlying build tool, providing:

- **Incremental Compilation**: Sub-50ms rebuilds for single file changes
- **Parallel Builds**: Utilizes all available CPU cores
- **Memory-Safe**: Zero memory leaks confirmed with Valgrind
- **Cross-Platform**: Native support for all target architectures

## Build Process

### 1. Source Processing
```
*.csd files → Lexer → Tokens → Parser → AST → Type Checker → Typed AST
```

### 2. Code Generation
```
Typed AST → LLVM IR Generator → LLVM IR → LLVM Backend → Native Binary
```

### 3. Standard Library Integration
```
Module Resolution → Dependency Analysis → Link Standard Library → Final Binary
```

## Advanced Configuration

### Custom Build Scripts
```zig
// build.zig customization
const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    
    // Custom CURSED compiler build
    const cursed = b.addExecutable(.{
        .name = "cursed-zig",
        .root_source_file = .{ .path = "src-zig/main.zig" },
        .target = target,
        .optimize = optimize,
    });
    
    // Add custom flags
    cursed.linkLibC();
    cursed.addIncludePath(.{ .path = "/usr/include/llvm-16" });
    
    b.installArtifact(cursed);
}
```

### Performance Tuning
```bash
# Profile build times
time zig build

# Analyze build cache usage
du -sh zig-cache/

# Memory usage during builds
/usr/bin/time -v zig build
```

## Integration Points

### Standard Library Modules
- Automatic module discovery in `stdlib/` directory
- Dependency resolution and linking
- Circular dependency detection
- Dead code elimination

### LLVM Backend
- IR generation optimization
- Target-specific code generation
- Debug symbol generation
- Link-time optimization (LTO)

### Development Tools
- LSP server integration
- Formatter integration
- Linter integration
- Documentation generator

## Testing Integration

### Unit Tests
```bash
# Test individual components
zig test src-zig/lexer.zig
zig test src-zig/parser.zig

# Test with coverage
zig test --test-filter "lexer" src-zig/lexer.zig
```

### Integration Tests
```bash
# Full compiler pipeline test
./zig-out/bin/cursed-zig comprehensive_test.csd

# Memory safety validation
valgrind --leak-check=full ./zig-out/bin/cursed-zig test.csd
```

### Performance Tests
```bash
# Benchmark compilation speed
hyperfine 'zig build clean && zig build'

# Runtime performance testing
./zig-out/bin/cursed-zig --benchmark file.csd
```
