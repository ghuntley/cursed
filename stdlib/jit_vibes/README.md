# JIT Vibes Module

Pure CURSED implementation for Just-In-Time (JIT) compilation support. This module provides runtime code generation, compilation, and execution capabilities for the CURSED language.

## Overview

The `jit_vibes` module enables dynamic code generation and execution at runtime, supporting:
- Runtime code compilation
- LLVM IR generation
- Optimization level control
- Code validation and execution
- Performance benchmarking
- Memory-safe JIT operations

## Core Types

### JITContext
```cursed
sus JITContext = {
    code_buffer: tea,        # Source code buffer
    optimization_level: normie,  # Optimization level (0-3)
    target_arch: tea,        # Target architecture
    is_compiled: lit         # Compilation status
}
```

## Functions

### Context Management
- `create_jit_context() JITContext` - Create new JIT context
- `create_optimized_jit(optimization_level normie) JITContext` - Create optimized context
- `clear_jit(ctx *JITContext) lit` - Clear and reset context

### Code Management
- `add_code_to_jit(ctx *JITContext, code tea) lit` - Add CURSED code to buffer
- `validate_jit_code(code tea) lit` - Validate code syntax
- `set_jit_optimization(ctx *JITContext, level normie) lit` - Set optimization level

### Compilation
- `generate_llvm_ir(ctx *JITContext) tea` - Generate LLVM IR from code
- `compile_jit(ctx *JITContext) lit` - Compile code to native
- `execute_jit(ctx *JITContext) normie` - Execute compiled code

### Utilities
- `get_jit_stats(ctx *JITContext) tea` - Get compilation statistics
- `benchmark_jit_compilation(code tea, iterations normie) normie` - Benchmark compilation

## Usage Examples

### Basic JIT Compilation
```cursed
yeet "jit_vibes"

# Create JIT context
sus ctx := create_jit_context()

# Add CURSED code
add_code_to_jit(&ctx, "vibez.spill(\"Hello JIT!\")")

# Compile and execute
compile_jit(&ctx)
sus result := execute_jit(&ctx)
```

### Optimized Compilation
```cursed
# Create optimized context
sus opt_ctx := create_optimized_jit(3)

# Validate and add code
lowkey validate_jit_code("sus x := 42; damn x") {
    add_code_to_jit(&opt_ctx, "sus x := 42; damn x")
    compile_jit(&opt_ctx)
    sus value := execute_jit(&opt_ctx)
    vibez.spill("Result: " + tea(value))
}
```

### JIT Pipeline
```cursed
# Complete JIT workflow
sus pipeline_ctx := create_jit_context()
set_jit_optimization(&pipeline_ctx, 2)

# Add and validate code
sus code := "vibez.spill(\"JIT Pipeline\")"
lowkey validate_jit_code(code) {
    add_code_to_jit(&pipeline_ctx, code)
    
    # Generate IR and compile
    sus ir := generate_llvm_ir(&pipeline_ctx)
    compile_jit(&pipeline_ctx)
    
    # Get statistics and execute
    sus stats := get_jit_stats(&pipeline_ctx)
    vibez.spill(stats)
    execute_jit(&pipeline_ctx)
}
```

### Performance Benchmarking
```cursed
# Benchmark compilation performance
sus code := "sus x := 42; vibez.spill(x)"
sus avg_time := benchmark_jit_compilation(code, 10)
vibez.spill("Average compilation time: " + tea(avg_time) + " ns")
```

## Optimization Levels

- **Level 0**: No optimization (fastest compilation)
- **Level 1**: Basic optimizations
- **Level 2**: Standard optimizations (recommended)
- **Level 3**: Aggressive optimizations (slowest compilation, best performance)

## Architecture Support

Currently supports:
- x86_64 (default)
- ARM64 (planned)
- RISC-V (planned)

## Supported Code Patterns

The JIT compiler currently supports these CURSED patterns:
- Variable declarations (`sus x := 42`)
- Function calls (`vibez.spill("hello")`)
- Return statements (`damn value`)
- Basic expressions and arithmetic

## Error Handling

- Invalid optimization levels return `cap`
- Empty code buffers return empty strings/failure codes
- Compilation state is tracked to prevent invalid operations
- Code validation prevents syntax errors at runtime

## Performance Considerations

- Use appropriate optimization levels for your use case
- Validate code before adding to JIT buffer
- Clear contexts when done to free memory
- Benchmark critical compilation paths

## Memory Safety

- All operations are memory-safe within CURSED runtime
- No direct memory manipulation
- Automatic cleanup on context clearing
- Safe execution environment isolation

## Integration with LLVM

The module generates LLVM IR compatible with the CURSED compiler's backend:
- Standard LLVM IR format
- Compatible with existing optimization passes
- Native code generation support
- Debug information preservation

## Testing

Run comprehensive tests:
```bash
cargo run --bin cursed stdlib/jit_vibes/test_jit_vibes.csd
```

Both-mode verification:
```bash
cargo run --bin cursed stdlib/jit_vibes/test_jit_vibes.csd
cargo run --bin cursed -- compile stdlib/jit_vibes/test_jit_vibes.csd
./test_jit_vibes
```

## Implementation Status

✅ **Complete Features:**
- JIT context management
- Code buffer operations  
- LLVM IR generation
- Compilation state tracking
- Performance benchmarking
- Pure CURSED implementation (FFI-free)

🚧 **Future Enhancements:**
- Advanced optimization passes
- Multi-architecture support
- Debug information generation
- Runtime profiling integration
- Code caching mechanisms

## Contributing

When extending this module:
1. Maintain FFI-free pure CURSED implementation
2. Add comprehensive tests for new functions
3. Update documentation with examples
4. Follow established naming conventions
5. Ensure memory safety in all operations

## Dependencies

- `testz` - Testing framework
- Pure CURSED implementation (no external dependencies)
- Compatible with CURSED compiler LLVM backend
