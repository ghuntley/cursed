# CURSED JIT Compilation System

## Overview

The CURSED programming language includes a comprehensive Just-In-Time (JIT) compilation system that enables dynamic code compilation and execution. This system is essential for the bootstrap self-compilation process and provides enhanced performance for interactive development in the REPL.

## Architecture

The JIT system consists of several interconnected components:

### Core Components

1. **JIT Engine (`CursedJitEngine`)**: The core compilation engine that wraps LLVM's ExecutionEngine with CURSED-specific functionality.

2. **JIT Compilation Interface (`JitCompilationInterface`)**: High-level interface that bridges CURSED AST compilation with JIT execution, including hot path detection and optimization.

3. **Hot Path Detector**: Monitors function execution patterns to identify frequently-used code paths for aggressive optimization.

4. **REPL Integration**: Seamless integration with the CURSED REPL for interactive development and testing.

## Features

### 🔧 Core JIT Engine Features

- **Function Compilation**: Compile LLVM IR to machine code in memory
- **Execution Engine**: Execute compiled functions dynamically
- **Function Caching**: Cache compiled functions for improved performance
- **Memory Management**: Automatic memory management for JIT-compiled code
- **Configuration Management**: Flexible configuration options for different use cases

### ⚡ Advanced Compilation Features

- **Hot Path Detection**: Automatically identify frequently-executed code paths
- **Dynamic Recompilation**: Recompile hot paths with aggressive optimizations
- **Incremental Compilation**: Support for incremental code compilation in REPL
- **Background Compilation**: Non-blocking optimization of hot paths
- **Performance Monitoring**: Comprehensive performance statistics and profiling

### 🎮 REPL Integration

- **Interactive Compilation**: Compile and execute code snippets in real-time
- **Function Management**: Define, cache, and execute functions interactively
- **Performance Analysis**: Built-in performance monitoring and reporting
- **Command Interface**: Rich command system for JIT management

### 📊 Optimization System

- **Multiple Optimization Levels**: None, Default, Aggressive optimization levels
- **Target-Specific Optimization**: CPU-specific optimizations (native, SSE, AVX)
- **Profile-Guided Optimization**: Use execution profiles to guide optimizations
- **Hot Path Optimization**: Specialized optimization for frequently-used code

## Usage

### Basic JIT Engine Usage

```rust
use cursed::codegen::llvm::{CursedJitEngine, create_optimized_jit_engine};
use inkwell::context::Context;

// Create LLVM context
let context = Context::create();

// Create optimized JIT engine
let mut engine = create_optimized_jit_engine(&context)?;

// Compile a function
engine.compile_function("test_func", llvm_ir_code)?;

// Execute the function
let result = engine.execute_function("test_func")?;
```

### JIT Compilation Interface

```rust
use cursed::codegen::llvm::{create_optimized_jit_interface};
use cursed::codegen::LlvmCodeGenerator;

// Create JIT interface
let context = Context::create();
let mut interface = create_optimized_jit_interface(&context)?;

// Execute REPL code directly
let result = interface.execute_repl_code("facts x = 42;")?;

// Compile and cache functions
interface.compile_and_cache_function("my_func", "slay test() { facts result = 100; }")?;

// Execute cached functions
let result = interface.execute_function("my_func")?;
```

### REPL Integration

```rust
use cursed::repl::ReplEvaluator;

let mut evaluator = ReplEvaluator::new()?;

// Initialize JIT support
evaluator.initialize_codegen()?;

// Check JIT availability
if evaluator.has_jit_support() {
    // Use JIT for evaluation
    let result = evaluator.evaluate_with_jit("facts x = 42;")?;
    
    // Compile functions
    evaluator.compile_function("test", "slay test() { facts x = 1; }")?;
    
    // Execute functions
    let result = evaluator.execute_jit_function("test")?;
}
```

## Configuration Options

### JIT Engine Configuration

```rust
use cursed::codegen::llvm::JitEngineConfig;
use inkwell::OptimizationLevel;

let config = JitEngineConfig {
    optimization_level: OptimizationLevel::Aggressive,
    enable_function_cache: true,
    enable_performance_monitoring: true,
    max_cached_functions: 5000,
    enable_debug_info: false,
    target_cpu: Some("native".to_string()),
    target_features: vec!["avx2".to_string(), "sse4.2".to_string()],
};
```

### JIT Compilation Configuration

```rust
use cursed::codegen::llvm::JitCompilationConfig;

let config = JitCompilationConfig {
    hot_path_threshold: 100,
    compilation_timeout: Duration::from_secs(30),
    enable_dynamic_recompilation: true,
    enable_background_compilation: true,
    hot_path_optimization_level: OptimizationLevel::Aggressive,
    regular_optimization_level: OptimizationLevel::Default,
    max_parallel_compilations: 4,
    enable_pgo: true,
};
```

## REPL Commands

The JIT system integrates with the CURSED REPL through several commands:

### `:jit` - JIT System Management

```bash
:jit                    # Show JIT status
:jit status             # Show detailed JIT status
:jit report             # Generate performance report
:jit functions          # List compiled functions
```

### `:optimize` - Hot Path Optimization

```bash
:optimize               # Trigger hot path optimization
```

### `:profile` - Function Profiling

```bash
:profile <function> [iterations]    # Profile function execution
```

## Performance Monitoring

The JIT system provides comprehensive performance monitoring:

### Engine Statistics

- **Functions Compiled**: Total number of functions compiled
- **Functions Executed**: Total number of function executions
- **Compilation Time**: Total and average compilation times
- **Execution Time**: Total and average execution times
- **Cache Efficiency**: Cache hit/miss ratios
- **Memory Usage**: Memory consumption by compiled functions

### Hot Path Analysis

- **Execution Counts**: Number of executions per function
- **Hot Path Detection**: Identification of frequently-used code
- **Optimization Events**: Hot path optimization statistics
- **Performance Improvements**: Quantified performance gains

### Function Profiling

- **Execution Time Profiling**: Detailed timing analysis
- **Performance Comparison**: Before/after optimization comparison
- **Bottleneck Identification**: Identify performance bottlenecks

## Integration with Bootstrap System

The JIT compilation system is designed to support the CURSED bootstrap self-compilation process:

### Self-Compilation Support

1. **Dynamic Function Compilation**: Compile CURSED functions to machine code during bootstrap
2. **Incremental Building**: Support incremental compilation of changed modules
3. **Performance Optimization**: Optimize frequently-used compiler functions
4. **Memory Efficiency**: Manage memory usage during large compilation processes

### Bootstrap Workflow Integration

```rust
// Bootstrap compiler can use JIT for self-compilation
let mut jit_interface = create_optimized_jit_interface(&context)?;

// Compile compiler functions with JIT
for (name, source) in compiler_functions {
    jit_interface.compile_function(name, source)?;
}

// Execute compilation steps using JIT
let compilation_result = jit_interface.execute_function("compile_module")?;
```

## Testing

The JIT system includes comprehensive testing:

### Test Categories

1. **Engine Core Tests**: Basic JIT engine functionality
2. **Compilation Interface Tests**: High-level interface testing
3. **REPL Integration Tests**: Interactive development testing
4. **Performance Tests**: Optimization and performance validation
5. **Stress Tests**: Concurrent and high-load testing
6. **Memory Safety Tests**: Memory management validation

### Running Tests

```bash
# Run all JIT tests
make jit-test

# Run quick tests (skip stress tests)
make jit-test-quick

# Run specific test categories
make jit-test-engine        # Core engine tests
make jit-test-repl          # REPL integration tests

# Generate test reports
make jit-test-report        # Comprehensive test report
make jit-test-coverage      # Coverage analysis

# Run with linking fix (for Nix environments)
make jit-test-linking-fix
```

### Test Infrastructure

The testing infrastructure includes:

- **Comprehensive Test Suite**: 40+ individual test cases
- **Automated Test Runner**: Shell script for complete test execution
- **Performance Benchmarking**: Quantified performance validation
- **Memory Safety Validation**: Thorough memory management testing
- **Concurrent Testing**: Multi-threaded execution validation
- **Error Scenario Testing**: Comprehensive error handling validation

## Memory Management

The JIT system implements sophisticated memory management:

### Function Caching

- **LRU Eviction**: Least Recently Used eviction policy
- **Configurable Limits**: Maximum cached function limits
- **Memory Monitoring**: Track memory usage per function
- **Cache Statistics**: Hit/miss ratios and efficiency metrics

### Memory Safety

- **Automatic Cleanup**: Automatic memory deallocation
- **Leak Prevention**: Comprehensive leak detection and prevention
- **Stack Management**: Safe stack allocation and deallocation
- **Thread Safety**: Thread-safe memory operations

## Error Handling

The JIT system provides robust error handling:

### Error Categories

- **Compilation Errors**: LLVM compilation failures
- **Execution Errors**: Runtime execution failures
- **Memory Errors**: Memory allocation/deallocation errors
- **Configuration Errors**: Invalid configuration parameters
- **Timeout Errors**: Compilation or execution timeouts

### Error Recovery

- **Graceful Degradation**: Fall back to interpretation on JIT failure
- **Error Context**: Detailed error information with context
- **Recovery Mechanisms**: Automatic recovery from transient failures
- **User Feedback**: Clear error messages for developers

## Future Enhancements

Planned improvements to the JIT system:

### Advanced Optimization

- **Adaptive Optimization**: Machine learning-guided optimization
- **Cross-Function Optimization**: Whole-program optimization
- **Speculative Optimization**: Optimistic optimization with deoptimization
- **Profile-Guided Optimization**: Enhanced PGO support

### Enhanced REPL Integration

- **Live Code Editing**: Real-time code modification and recompilation
- **Debugging Support**: Integration with debugger for JIT code
- **Visual Performance Analysis**: Graphical performance monitoring
- **Interactive Optimization**: User-guided optimization hints

### Production Features

- **Serialization Support**: Save/load compiled functions
- **Distribution Support**: Distribute compiled functions across nodes
- **Security Features**: Code signature verification and sandboxing
- **Resource Management**: Advanced resource usage controls

## Conclusion

The CURSED JIT compilation system provides a powerful foundation for dynamic code compilation and execution. Its integration with the REPL system enables interactive development, while its optimization capabilities ensure excellent performance for production use. The comprehensive testing infrastructure and robust error handling make it suitable for the critical role it plays in the bootstrap self-compilation process.

The system's modular design allows for future enhancements while maintaining compatibility with existing code, making it a solid foundation for the CURSED language's compilation infrastructure.
