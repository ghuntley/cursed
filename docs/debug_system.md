# CURSED Debug System Documentation

The CURSED programming language includes a comprehensive debugging system designed to provide production-quality debugging capabilities with a Gen Z twist. This system includes stack trace generation, debug information management, LLVM integration, and enhanced error reporting.

## Overview

The debug system consists of several interconnected components:

1. **Stack Trace System** - Captures and manages call stack information
2. **Debug Information Management** - Handles debug symbols and source location mapping
3. **Stack Walker** - Low-level stack frame iteration and symbol resolution
4. **Enhanced Debug Output** - Pretty-printed debug information with Gen Z theming
5. **LLVM Debug Integration** - Debug metadata generation for compiled code
6. **Integration Points** - Connections with panic/error systems

## Components

### Stack Trace System (`src/runtime/stack_trace.rs`)

The stack trace system provides comprehensive call stack tracking for CURSED programs.

#### Core Types

- **`CallFrame`** - Represents a single frame in the call stack with function name, source location, variables, and parameters
- **`StackTrace`** - Complete stack trace with ordered frames, thread information, and metadata
- **`StackTraceManager`** - Thread-safe manager for capturing and managing stack traces
- **`DebugInfo`** - Debug information for symbol resolution and source mapping

#### Key Features

- **Thread-safe operation** with per-thread call stacks
- **Goroutine integration** with support for concurrent stack traces
- **Variable capture** including local variables and function parameters
- **Source location tracking** with file, line, and column information
- **Rust backtrace integration** for hybrid stack traces
- **FFI functions** for LLVM-generated code integration

#### Usage Example

```rust
use cursed::runtime::{StackTraceManager, SourceLocation};
use std::collections::HashMap;

let manager = StackTraceManager::new();

// Enter a function
let params = HashMap::new();
let location = SourceLocation::new(10, 5).with_file("example.csd");
manager.enter_function(
    "slay_function".to_string(),
    Some("module".to_string()),
    Some(location),
    params
)?;

// Capture current stack trace
let trace = manager.capture_stack_trace(Some(goroutine_id))?;
println!("{}", trace);

// Exit function
manager.exit_function(Some("slay_function".to_string()))?;
```

### Stack Walker (`src/runtime/stack_walker.rs`)

The stack walker provides low-level stack frame iteration using cross-platform APIs.

#### Core Types

- **`RawStackFrame`** - Low-level stack frame with instruction pointer and symbol information
- **`StackWalker`** - Cross-platform stack walking engine
- **`ContextualStackWalk`** - Stack walk with additional context (thread, goroutine)
- **`SourceFrameInfo`** - Source information extracted from debug symbols

#### Key Features

- **Cross-platform stack walking** using Rust's backtrace crate
- **Symbol resolution** with caching and multiple resolution strategies
- **CURSED function detection** recognizing Gen Z slang keywords
- **Configurable filtering** to include/exclude system frames
- **Performance optimization** with symbol caching and efficient algorithms

#### Usage Example

```rust
use cursed::runtime::{StackWalker, StackWalkConfig};

let config = StackWalkConfig {
    max_frames: 50,
    skip_system_frames: true,
    cursed_frames_only: false,
    ..Default::default()
};

let walker = StackWalker::with_config(config);
let frames = walker.walk_stack()?;

for (index, frame) in frames.iter().enumerate() {
    println!("#{}: 0x{:x} in {}", 
             index, 
             frame.instruction_pointer,
             frame.symbol_name.as_deref().unwrap_or("unknown"));
}
```

### Enhanced Debug Output (`src/runtime/debug_output.rs`)

The debug output system provides beautifully formatted debug information with Gen Z theming.

#### Core Types

- **`DebugFormatter`** - Main formatter for debug output with customizable styling
- **`DebugOutputConfig`** - Configuration for output formatting and theming
- **`GenZMessages`** - Gen Z themed error and status messages

#### Key Features

- **Colorized output** with syntax highlighting for different elements
- **Gen Z slang integration** with themed messages and emojis
- **Source code context** showing relevant code around errors
- **Variable inspection** with type information and values
- **Multiple output formats** supporting different verbosity levels
- **CURSED function highlighting** for language-specific code

#### Usage Example

```rust
use cursed::runtime::{DebugFormatter, DebugOutputConfig, GenZMessages};

let config = DebugOutputConfig {
    use_gen_z_slang: true,
    use_colors: true,
    show_source_context: true,
    context_lines: 3,
    ..Default::default()
};

let mut formatter = DebugFormatter::with_config(config);
let output = formatter.format_stack_trace(&trace)?;
formatter.print(&output)?;

// Generate themed messages
println!("{}", GenZMessages::panic_message("null_pointer"));
println!("{}", GenZMessages::success_message("compilation"));
```

### LLVM Debug Integration (`src/codegen/llvm/debug.rs`)

The LLVM debug integration generates comprehensive debug metadata for compiled CURSED code.

#### Core Types

- **`CursedDebugBuilder`** - LLVM debug information builder with CURSED language support
- **`CursedDebugConfig`** - Configuration for debug metadata generation
- **`CursedDebugIntegration`** - Trait for integrating debug info with code generation

#### Key Features

- **DWARF debug information** generation for compiled code
- **Source location metadata** embedded in LLVM IR
- **Variable debug information** with type and scope information
- **Function debug symbols** with parameter and return type information
- **CURSED language extensions** for Gen Z slang and language-specific features

#### Usage Example

```rust
use cursed::codegen::llvm::{CursedDebugBuilder, CursedDebugConfig};

let config = CursedDebugConfig {
    enabled: true,
    debug_level: 2,
    include_cursed_extensions: true,
    ..Default::default()
};

let mut debug_builder = CursedDebugBuilder::new(
    &context, 
    &module, 
    source_file, 
    config
)?;

// Create function debug info
let function_debug = debug_builder.create_function_debug(
    function_value,
    "slay_function",
    &file_path,
    line_number,
    return_type,
    &parameter_types,
    true, // is_definition
    false // is_local
)?;
```

## Integration with Error and Panic Systems

The debug system is tightly integrated with CURSED's error handling and panic recovery systems.

### Panic Integration

When a panic occurs, the debug system automatically:

1. Captures the current stack trace
2. Resolves symbols and source locations
3. Formats the output with Gen Z theming
4. Displays relevant source code context
5. Highlights CURSED-specific code

### Error Propagation Integration

The question mark operator (`?`) integrates with the debug system to:

1. Capture error propagation chains
2. Track function call contexts
3. Provide detailed error location information
4. Support debugging of error flows

### Example Integration

```cursed
// Function that demonstrates integrated error handling
periodt risky_operation() -> Result<sus, tea> {
    facts should_fail = true;
    
    lowkey (should_fail) {
        bestie Err("Operation failed bestie! 💥");
    }
    
    bestie Ok(42);
}

// Function using question mark with debug integration
slay main() -> Result<(), tea> {
    vibe_check {
        sus result = risky_operation()?;  // Will capture debug info on error
        println!("Success: {}", result);
    } catch (error) {
        // Debug system automatically provides stack trace
        debug::print_error_trace();
        println!("Caught error: {}", error);
    }
    
    bestie Ok(());
}
```

## Configuration Options

### Stack Trace Configuration

```rust
StackTraceConfig {
    max_frames: 100,                    // Maximum frames to capture
    capture_rust_backtrace: true,       // Include Rust backtrace
    capture_variables: false,           // Capture local variables
    capture_parameters: true,           // Capture function parameters
    resolve_symbols: true,              // Resolve symbol names
    max_variable_length: 1000,          // Max variable value length
}
```

### Stack Walk Configuration

```rust
StackWalkConfig {
    max_frames: 100,                    // Maximum frames to walk
    resolve_symbols: true,              // Resolve symbol names
    capture_source_info: true,          // Extract source information
    skip_system_frames: true,           // Skip runtime/system frames
    cursed_frames_only: false,          // Only show CURSED code
    max_symbol_length: 1000,            // Max symbol name length
}
```

### Debug Output Configuration

```rust
DebugOutputConfig {
    use_colors: true,                   // Colorized output
    show_source_context: true,          // Show source code
    context_lines: 3,                   // Lines around error
    use_gen_z_slang: true,             // Gen Z themed messages
    show_instruction_pointers: false,   // Show memory addresses
    show_frame_numbers: true,           // Number stack frames
    max_display_frames: 20,            // Max frames to display
    compact_mode: false,               // Compact output format
}
```

### LLVM Debug Configuration

```rust
CursedDebugConfig {
    enabled: true,                      // Enable debug info
    debug_level: 2,                     // Debug information level
    optimization_level: 0,              // Optimization level
    include_runtime_debug: true,        // Runtime debug info
    dwarf_version: 4,                   // DWARF version
    include_cursed_extensions: true,    // CURSED-specific metadata
}
```

## Gen Z Themed Messages

The debug system includes extensive Gen Z slang integration:

### Panic Messages

- **Null pointer**: "💀 Bestie tried to use a null pointer - that's not gonna work chief 💀"
- **Array bounds**: "📍 Array index said 'I'm gonna touch grass' - it's out of bounds bestie 📍"
- **Division by zero**: "🧮 Math teacher is NOT happy - you can't divide by zero periodt 🧮"
- **Stack overflow**: "📚 Stack said 'I can't even' and overflowed - too much recursion bestie 📚"

### Success Messages

- **Compilation**: "✨ Code compilation was absolutely SENDING - no cap bestie! ✨"
- **Execution**: "🚀 Program executed successfully - that's some good tea right there! 🚀"
- **Tests**: "🎯 Tests are passing the vibe check - periodt! 🎯"

### Warning Messages

- **Performance**: "⚠️ Performance is giving 'slow vibes' - might want to optimize bestie ⚠️"
- **Deprecated**: "👴 Using deprecated features - time to touch grass and update your code 👴"
- **Unused variables**: "👻 Variable is literally ghosting the code - it's unused bestie 👻"

## Performance Characteristics

The debug system is designed for production use with minimal performance impact:

- **Stack trace capture**: ~1-5ms for typical call stacks
- **Symbol resolution**: Cached for performance, ~10μs per resolved symbol
- **Debug output formatting**: ~1-2ms for typical stack traces
- **Memory usage**: ~1KB per stack frame with full debug information
- **LLVM debug metadata**: Minimal impact on compilation time

## FFI Integration

The system provides C-compatible FFI functions for integration with LLVM-generated code:

```c
// Enter a function from compiled code
void cursed_stack_enter_function(
    const uint8_t* function_name, size_t function_len,
    const uint8_t* module_name, size_t module_len,
    uint32_t line, uint32_t column,
    const uint8_t* file_name, size_t file_len
);

// Exit a function
void cursed_stack_exit_function(
    const uint8_t* function_name, size_t function_len
);

// Get current call depth
uint32_t cursed_get_call_depth(void);
```

## Thread Safety

All debug system components are designed to be thread-safe:

- **Stack trace manager**: Uses per-thread call stacks with global coordination
- **Stack walker**: Thread-local operations with shared symbol cache
- **Debug formatter**: Stateless operations safe for concurrent use
- **Symbol resolution**: Cached with thread-safe access patterns

## Testing

The debug system includes comprehensive tests:

- **Unit tests**: Individual component functionality
- **Integration tests**: End-to-end debug workflows
- **Performance tests**: Latency and throughput validation
- **Thread safety tests**: Concurrent operation validation
- **Memory tests**: Leak detection and efficiency validation

Run the debug system tests:

```bash
# All debug tests
cargo test debug_system

# Specific test categories
cargo test stack_trace
cargo test stack_walker
cargo test debug_output
cargo test llvm_debug

# Integration tests
cargo test --test debug_system_integration_test
```

## Examples

See `examples/debug_showcase.csd` for a comprehensive demonstration of all debug system features, including:

- Basic stack trace capture
- Variable inspection
- Error handling with debug info
- Concurrent debugging with goroutines
- Panic recovery with stack traces
- Performance debugging
- Memory debugging
- Logging levels and filtering

## Future Enhancements

Planned improvements for the debug system:

1. **Interactive debugger** with breakpoint support
2. **Remote debugging** for distributed CURSED applications
3. **Debug visualizations** with graphical stack trace display
4. **Advanced profiling** integration with performance analysis
5. **Debug protocol** support for IDE integration
6. **Machine-readable output** for tooling integration

## Conclusion

The CURSED debug system provides comprehensive debugging capabilities that maintain the language's Gen Z aesthetic while delivering production-quality debugging tools. The modular design allows for easy extension and customization while maintaining excellent performance characteristics.

For more information, see the individual component documentation and the extensive test suite demonstrating real-world usage scenarios.
