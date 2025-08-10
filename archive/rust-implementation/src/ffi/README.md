# CURSED FFI System

The CURSED Foreign Function Interface (FFI) system provides comprehensive capabilities for interoperating with foreign languages and libraries. This system enables CURSED programs to call functions written in other languages and vice versa, with full type safety and memory management.

## Features

### 🔧 Core Capabilities

- **Automatic C Header Parsing**: Parse C header files and generate bindings automatically
- **Multi-Language Support**: C/C++, Python, Go, JavaScript/WASM support
- **Type Safety**: Comprehensive type mapping and validation
- **Memory Safety**: Advanced memory management with leak detection
- **Performance Optimization**: Zero-copy transfers, bulk operations, caching
- **Developer Tools**: Debug tools, profiling, and binding generators

### 🚀 Advanced Features

- **Struct Marshalling**: Automatic conversion of complex data structures
- **Callback Support**: Bi-directional function calls between languages
- **Error Recovery**: Intelligent error handling with recovery strategies
- **Performance Profiling**: Detailed performance analysis and optimization
- **Safety Checks**: Buffer overflow protection, null pointer validation
- **Memory Pools**: Efficient memory allocation for frequent operations

## Quick Start

### Basic Usage

```rust
use cursed::ffi::FfiSystem;

// Create FFI system
let ffi = FfiSystem::new()?;

// Parse C header and generate bindings
let header_info = ffi.parse_header_file("library.h")?;

// Call a C function
let args = vec![
    FfiValue::SignedInteger(42),
    FfiValue::String("hello".to_string()),
];
let result = ffi.call_function("my_function", &args, "c")?;
```

### Python Integration

```rust
// Call Python function
let python_result = ffi.call_function("numpy.sum", &args, "python")?;
```

### JavaScript/WASM Integration

```rust
// Call WASM function
let wasm_result = ffi.call_function("calculate", &args, "wasm")?;
```

## Architecture

### System Components

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   FfiSystem     │    │  TypeMapper     │    │MemorySafetyMgr  │
│   (Main API)    │◄──►│(Type Conversion)│◄──►│(Memory Safety)  │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│LanguageBridges  │    │ CallbackManager │    │PerformanceOpt  │
│(C/Python/Go/JS) │    │(Callbacks)      │    │(Optimization)   │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│  HeaderParser   │    │  DebugTools     │    │  SafetyChecker  │
│(C Header Parse) │    │(Debug/Profile)  │    │(Safety Checks)  │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

### Language Bridges

Each language bridge implements the `LanguageBridge` trait:

```rust
pub trait LanguageBridge: Send + Sync {
    fn language_name(&self) -> &str;
    fn call_function(&self, function_name: &str, args: &[FfiValue]) -> Result<FfiValue, CursedError>;
    fn load_library(&self, path: &str) -> Result<LibraryHandle, CursedError>;
    fn get_available_functions(&self) -> Result<Vec<String>, CursedError>;
    fn generate_bindings(&self, library_path: &str, output_path: &str) -> Result<(), CursedError>;
    fn cleanup(&self) -> Result<(), CursedError>;
}
```

## Module Documentation

### Core Modules

#### `FfiSystem` - Main API
- Central FFI system manager
- Coordinates all FFI operations
- Provides high-level API for foreign function calls

#### `TypeMapper` - Type Conversion
- Automatic type mapping between languages
- Custom type converters
- Struct marshalling/unmarshalling
- Type validation

#### `MemorySafetyManager` - Memory Management
- Safe memory allocation/deallocation
- Leak detection and prevention
- Memory pooling for performance
- Cross-language memory sharing

#### `LanguageBridge` - Language Support
- C/C++ bridge with dynamic library loading
- Python bridge with CPython API integration
- Go bridge with CGO support
- JavaScript/WASM bridge for web environments

### Advanced Modules

#### `HeaderParser` - C Header Parsing
- Automatic C header file parsing
- Function signature extraction
- Struct and enum definitions
- Constant and typedef handling

#### `BindingGenerator` - Code Generation
- Automatic binding generation
- CURSED wrapper functions
- Type-safe interfaces
- Error handling integration

#### `CallbackManager` - Callback Support
- Bidirectional function calls
- Callback registration and management
- Lifetime management
- Thread-safe operations

#### `PerformanceOptimizer` - Performance
- Zero-copy data transfers
- Memory pooling
- Call caching
- Bulk operations

#### `DebugTools` - Development Tools
- Function call tracing
- Memory debugging
- Performance profiling
- Interactive debugging

#### `SafetyChecker` - Safety Validation
- Type safety checks
- Memory safety validation
- Buffer overflow protection
- Null pointer checks

## Examples

### 1. C Library Integration

```rust
use cursed::ffi::FfiSystem;

// Initialize FFI system
let ffi = FfiSystem::new()?;

// Parse C header file
let header_info = ffi.parse_header_file("/usr/include/math.h")?;

// Call math function
let args = vec![FfiValue::Float(2.0)];
let result = ffi.call_function("sqrt", &args, "c")?;

if let FfiValue::Float(sqrt_result) = result {
    println!("sqrt(2.0) = {}", sqrt_result);
}
```

### 2. Python Integration

```rust
// Call Python function
let python_code = r#"
import numpy as np
def calculate_mean(data):
    return np.mean(data)
"#;

// Execute Python code and call function
let data = vec![
    FfiValue::Float(1.0),
    FfiValue::Float(2.0),
    FfiValue::Float(3.0),
];
let result = ffi.call_function("calculate_mean", &data, "python")?;
```

### 3. Struct Marshalling

```rust
use std::collections::HashMap;

// Define struct fields
let mut point_fields = HashMap::new();
point_fields.insert("x".to_string(), FfiValue::Float(10.0));
point_fields.insert("y".to_string(), FfiValue::Float(20.0));

// Marshal struct to C
let c_struct = ffi.marshal_struct("Point", &point_fields)?;

// Call C function with struct
let args = vec![FfiValue::Struct(point_fields)];
let result = ffi.call_function("process_point", &args, "c")?;
```

### 4. Callback Registration

```rust
// Register callback function
let callback = |args: &[FfiValue]| -> Result<FfiValue, CursedError> {
    if let [FfiValue::SignedInteger(x), FfiValue::SignedInteger(y)] = args {
        Ok(FfiValue::SignedInteger(x + y))
    } else {
        Err(CursedError::General("Invalid arguments".to_string()))
    }
};

let signature = FunctionSignature {
    name: "add_callback".to_string(),
    return_type: FfiType::SignedInteger(32),
    parameters: vec![
        Parameter {
            name: "x".to_string(),
            param_type: FfiType::SignedInteger(32),
            is_const: false,
            is_nullable: false,
        },
        Parameter {
            name: "y".to_string(),
            param_type: FfiType::SignedInteger(32),
            is_const: false,
            is_nullable: false,
        },
    ],
    is_variadic: false,
};

let callback_handle = ffi.create_callback(callback, &signature)?;
```

### 5. Performance Profiling

```rust
// Enable performance profiling
ffi.enable_debug_mode()?;

// Perform FFI operations
let result = ffi.call_function("expensive_operation", &args, "c")?;

// Get performance statistics
let stats = ffi.get_performance_stats()?;
println!("Total calls: {}", stats.total_calls);
println!("Average call time: {:?}", stats.average_call_time);
```

## Configuration

### FFI System Configuration

```rust
use cursed::ffi::{FfiSystem, PerformanceConfig, DebugConfig, SafetyConfig};

// Configure performance settings
let perf_config = PerformanceConfig {
    enable_call_cache: true,
    enable_memory_pools: true,
    enable_zero_copy: true,
    enable_bulk_operations: true,
    cache_ttl: Duration::from_secs(300),
    ..Default::default()
};

// Configure debug settings
let debug_config = DebugConfig {
    enable_debug: true,
    enable_tracing: true,
    enable_memory_debug: true,
    enable_profiling: true,
    output_directory: "debug_output".to_string(),
    ..Default::default()
};

// Configure safety settings
let safety_config = SafetyConfig {
    leak_detection: true,
    buffer_protection: true,
    pointer_validation: true,
    automatic_cleanup: true,
    allocation_limit: Some(1024 * 1024 * 1024), // 1GB
    ..Default::default()
};

// Create configured FFI system
let ffi = FfiSystem::new()?;
```

## Error Handling

The FFI system provides comprehensive error handling:

```rust
use cursed::ffi::FfiError;

match ffi.call_function("my_function", &args, "c") {
    Ok(result) => println!("Success: {:?}", result),
    Err(error) => {
        match error {
            FfiError::TypeConversion { from_type, to_type, reason } => {
                println!("Type conversion error: {} -> {}: {}", from_type, to_type, reason);
            }
            FfiError::MemoryAllocation { size, reason } => {
                println!("Memory allocation failed for {} bytes: {}", size, reason);
            }
            FfiError::FunctionCall { function_name, error_code, message } => {
                println!("Function {} failed with code {}: {}", function_name, error_code, message);
            }
            _ => println!("Other FFI error: {}", error),
        }
    }
}
```

## Memory Management

### Memory Safety Features

- **Automatic Cleanup**: Resources are automatically cleaned up when no longer needed
- **Leak Detection**: Memory leaks are detected and reported
- **Buffer Protection**: Buffer overflows are prevented with guard pages
- **Pointer Validation**: All pointers are validated before use
- **Reference Counting**: Shared resources use reference counting

### Memory Pool Configuration

```rust
// Configure memory pools for different allocation sizes
let pool_config = MemoryPoolConfig {
    small_pool_size: 1024,
    medium_pool_size: 64 * 1024,
    large_pool_size: 1024 * 1024,
    pool_count: 16,
    cleanup_interval: Duration::from_secs(60),
};
```

## Performance Optimization

### Zero-Copy Operations

```rust
// Enable zero-copy transfers
let mut buffer = vec![0u8; 1024];
let result = ffi.zero_copy_transfer(source_ptr, buffer.as_mut_ptr(), 1024)?;
```

### Bulk Operations

```rust
// Submit bulk operations for better performance
let operations = vec![
    FfiValue::SignedInteger(1),
    FfiValue::SignedInteger(2),
    FfiValue::SignedInteger(3),
];

ffi.submit_bulk_operation(BulkOperationType::TypeConversion, operations, None)?;
```

### Call Caching

```rust
// Function calls are automatically cached for performance
let result1 = ffi.call_function("expensive_function", &args, "c")?; // Slow
let result2 = ffi.call_function("expensive_function", &args, "c")?; // Fast (cached)
```

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ffi_system_creation() {
        let ffi = FfiSystem::new().unwrap();
        assert!(ffi.get_supported_languages().contains(&"c".to_string()));
    }
    
    #[test]
    fn test_type_conversion() {
        let ffi = FfiSystem::new().unwrap();
        let value = FfiValue::SignedInteger(42);
        let result = ffi.marshal_to_foreign(&value, &FfiType::SignedInteger(32), "c").unwrap();
        assert_eq!(result.size, 4);
    }
}
```

### Integration Tests

```rust
#[test]
fn test_c_integration() {
    let ffi = FfiSystem::new().unwrap();
    
    // Load math library
    let lib = ffi.load_library("libm.so").unwrap();
    
    // Call sqrt function
    let args = vec![FfiValue::Float(4.0)];
    let result = ffi.call_function("sqrt", &args, "c").unwrap();
    
    if let FfiValue::Float(sqrt_result) = result {
        assert!((sqrt_result - 2.0).abs() < 0.001);
    }
}
```

## Best Practices

### 1. Error Handling
- Always handle FFI errors gracefully
- Use appropriate error recovery strategies
- Log errors for debugging

### 2. Memory Management
- Use RAII patterns for resource management
- Validate all pointers before use
- Monitor memory usage in production

### 3. Performance
- Use bulk operations for multiple calls
- Enable caching for frequently called functions
- Profile FFI operations to identify bottlenecks

### 4. Safety
- Enable all safety checks in development
- Use type validation for all conversions
- Test with memory sanitizers

### 5. Debugging
- Enable debug mode during development
- Use profiling tools to optimize performance
- Export debug data for analysis

## Troubleshooting

### Common Issues

1. **Type Conversion Errors**
   - Verify type mappings are correct
   - Check for endianness issues
   - Validate struct alignment

2. **Memory Errors**
   - Check for memory leaks
   - Validate pointer lifetimes
   - Ensure proper cleanup

3. **Performance Issues**
   - Enable profiling
   - Use memory pools
   - Optimize frequent calls

4. **Library Loading Errors**
   - Check library paths
   - Verify dependencies
   - Check symbol visibility

### Debug Commands

```bash
# Enable debug mode
export CURSED_FFI_DEBUG=1

# Run with memory checking
valgrind --tool=memcheck ./program

# Profile FFI operations
perf record -g ./program
perf report
```

## Contributing

### Development Setup

1. Install dependencies:
   ```bash
   cargo install bindgen
   pip install pycparser
   ```

2. Run tests:
   ```bash
   cargo test --features ffi
   ```

3. Generate documentation:
   ```bash
   cargo doc --features ffi --open
   ```

### Adding New Language Support

1. Implement the `LanguageBridge` trait
2. Add type mappings
3. Create marshalling functions
4. Add comprehensive tests
5. Update documentation

### Code Style

- Follow Rust naming conventions
- Use comprehensive error handling
- Add detailed documentation
- Include unit tests for all functions
- Use RAII patterns for resource management

## License

This FFI system is part of the CURSED language project and is licensed under the same terms as the main project.

## Changelog

### Version 1.0.0
- Initial FFI system implementation
- C language bridge support
- Basic type mapping and marshalling
- Memory safety features

### Version 1.1.0
- Added Python integration
- Performance optimization system
- Enhanced error handling
- Debug tools implementation

### Version 1.2.0
- Go language bridge support
- WebAssembly integration
- Callback management system
- Advanced profiling tools

### Version 1.3.0
- Enhanced memory safety
- Zero-copy operations
- Bulk operation support
- Production-ready stability
