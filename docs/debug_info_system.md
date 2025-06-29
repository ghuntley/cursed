# CURSED Debug Information System

## Overview

The CURSED debug information system provides comprehensive debugging capabilities for CURSED programs, including:

- **DWARF debug information parsing and generation**
- **Function parameter extraction**
- **Local variable tracking**
- **Inline function information**
- **Stack trace capture with full debugging context**
- **Symbol resolution and caching**
- **Source location mapping**

## Architecture

### Core Components

1. **StackTraceCapture**: Main interface for capturing stack traces with debugging information
2. **DwarfDebugDatabase**: DWARF parser and debug information storage
3. **LlvmDebugInfo**: Integration with LLVM debug metadata
4. **SymbolResolver**: Symbol address resolution and caching

### Debug Information Flow

```
CURSED Source Code
        ↓
   LLVM IR + Debug Metadata
        ↓
   Binary with DWARF Sections
        ↓
   DwarfDebugDatabase Parser
        ↓
   Runtime Debug Information
```

## Key Features

### 1. Function Parameter Extraction

The system can extract function parameters from DWARF debug information:

```rust
let debug_info = LlvmDebugInfo::new();
let parameters = debug_info.extract_function_parameters(function_address)?;

for param in parameters {
    println!("Parameter: {} (type: {})", param.name, param.param_type);
    if let Some(value) = param.value {
        println!("  Value: {}", value);
    }
}
```

**Features:**
- Parameter names and types
- Location information (register, stack, memory)
- Value extraction when possible
- Support for pass-by-reference parameters

### 2. Local Variable Tracking

Complete local variable information with scope tracking:

```rust
let variables = debug_info.extract_local_variables(address)?;

for var in variables {
    println!("Variable: {} (type: {})", var.name, var.var_type);
    println!("  Scope: {}", var.scope);
    if let Some(value) = var.value {
        println!("  Value: {}", value);
    }
}
```

**Features:**
- Variable names, types, and values
- Scope boundary tracking
- Memory location information
- Declaration line numbers

### 3. Inline Function Information

Track inlined functions with call site information:

```rust
let inline_info = debug_info.extract_inline_info(address)?;

for info in inline_info {
    println!("Inlined function: {}", info.function_name);
    println!("  Called from: {}:{}", info.inline_site.file, info.inline_site.line);
    println!("  Original: {}:{}", info.original_location.file, info.original_location.line);
}
```

**Features:**
- Inlined function identification
- Call site location tracking
- Original function location
- Nested inline function support

### 4. Enhanced Stack Traces

Rich stack traces with full debugging context:

```rust
let config = EnhancedStackTraceConfig {
    max_depth: 100,
    resolve_symbols: true,
    include_source: true,
    show_parameters: true,
    show_locals: true,
    expand_inlines: true,
    format: StackTraceFormat::Verbose,
    ..Default::default()
};

let capture = StackTraceCapture::new(config);
let frames = capture.capture_stack_trace()?;

for frame in frames {
    println!("Frame {}: {}", frame.depth, frame.symbol_info.name);
    
    if let Some(source) = frame.source_location {
        println!("  at {}:{}:{}", source.file, source.line, source.column);
    }
    
    for param in &frame.parameters {
        println!("  param {}: {} = {:?}", param.name, param.param_type, param.value);
    }
    
    for var in &frame.local_variables {
        println!("  local {}: {} = {:?}", var.name, var.var_type, var.value);
    }
}
```

## DWARF Integration

### Supported DWARF Features

- **DWARF 4 and 5** format support
- **Compilation unit parsing**
- **Function information** (DW_TAG_subprogram)
- **Variable information** (DW_TAG_variable, DW_TAG_formal_parameter)
- **Type information** (DW_TAG_base_type, DW_TAG_structure_type, etc.)
- **Inline function data** (DW_TAG_inlined_subroutine)
- **Line number information**
- **Location expressions** (DW_OP_* opcodes)

### DWARF Generation

The system can generate DWARF information for CURSED modules:

```rust
let functions = vec![
    FunctionInfo {
        name: "main".to_string(),
        start_address: 0x1000,
        end_address: 0x1200,
        start_line: 1,
        end_line: 50,
        parameter_count: 2,
        local_count: 5,
    }
];

let dwarf_data = debug_info.generate_dwarf_info("my_module", &functions)?;
```

## Performance Optimizations

### Symbol Caching

The symbol resolver includes an LRU cache for frequently accessed symbols:

```rust
let resolver = SymbolResolver::new();
// First lookup - cache miss
let symbol1 = resolver.resolve_address(0x1000)?;
// Second lookup - cache hit (much faster)
let symbol2 = resolver.resolve_address(0x1000)?;
```

### Efficient Data Structures

- **BTreeMap** for address range lookups
- **HashMap** for O(1) symbol access
- **Optimized parsing** with lazy loading
- **Memory-efficient** storage

## Integration with CURSED Tools

### Debugger Integration

The debug information system integrates with CURSED's debugging tools:

```bash
cursed-debug program.csd
> bt                    # Show stack trace with debug info
> info locals           # Show local variables
> info args             # Show function parameters
> info inline           # Show inline function information
```

### Profiler Integration

Stack traces include performance metrics:

```rust
let config = EnhancedStackTraceConfig {
    capture_performance: true,
    ..Default::default()
};

let stats = capture.get_statistics()?;
println!("Average capture time: {:?}", stats.avg_capture_time);
```

### Language Server Integration

The Language Server Protocol (LSP) server uses debug information for:
- **Hover information**
- **Go to definition**
- **Variable inspection**
- **Inline value display**

## Error Handling

The debug information system provides comprehensive error handling:

```rust
match debug_info.extract_function_parameters(address) {
    Ok(params) => {
        // Process parameters
    }
    Err(CursedError::RuntimeError(msg)) => {
        eprintln!("Debug info error: {}", msg);
    }
    Err(e) => {
        eprintln!("Unexpected error: {:?}", e);
    }
}
```

## Configuration Options

### Stack Trace Configuration

```rust
pub struct EnhancedStackTraceConfig {
    pub max_depth: usize,           // Maximum stack depth
    pub resolve_symbols: bool,      // Enable symbol resolution
    pub include_source: bool,       // Include source file info
    pub include_line_numbers: bool, // Include line numbers
    pub include_columns: bool,      // Include column numbers
    pub llvm_debug_info: bool,      // Use LLVM debug info
    pub show_parameters: bool,      // Show function parameters
    pub show_locals: bool,          // Show local variables
    pub expand_inlines: bool,       // Expand inline functions
    pub include_addresses: bool,    // Include memory addresses
    pub async_stack_traces: bool,   // Support async stack traces
    pub capture_performance: bool,  // Capture performance metrics
    pub format: StackTraceFormat,   // Output format
}
```

### Output Formats

- **Standard**: Basic function and location info
- **Compact**: Minimal information for space efficiency
- **Verbose**: Full debugging information
- **JSON**: Machine-readable format
- **Custom**: User-defined templates

## Testing

### Unit Tests

```bash
cargo test debug_info
```

### Integration Tests

```bash
cargo test --test debug_info_integration_tests
```

### Performance Tests

```bash
cargo test test_performance_with_large_debug_info
```

## Example Usage

### Basic Stack Trace

```rust
use cursed::runtime::debug_info::*;

let config = EnhancedStackTraceConfig::default();
let capture = StackTraceCapture::new(config);

match capture.capture_stack_trace() {
    Ok(frames) => {
        for frame in frames {
            println!("{}: {}", frame.depth, frame.symbol_info.name);
        }
    }
    Err(e) => eprintln!("Failed to capture stack trace: {}", e),
}
```

### Full Debug Information

```rust
let config = EnhancedStackTraceConfig {
    show_parameters: true,
    show_locals: true,
    expand_inlines: true,
    format: StackTraceFormat::Verbose,
    ..Default::default()
};

let capture = StackTraceCapture::new(config);
let frames = capture.capture_stack_trace()?;

for frame in frames {
    println!("=== Frame {} ===", frame.depth);
    println!("Function: {}", frame.symbol_info.name);
    
    if let Some(source) = &frame.source_location {
        println!("Source: {}:{}:{}", source.file, source.line, source.column);
    }
    
    if !frame.parameters.is_empty() {
        println!("Parameters:");
        for param in &frame.parameters {
            println!("  {}: {} = {:?}", param.name, param.param_type, param.value);
        }
    }
    
    if !frame.local_variables.is_empty() {
        println!("Local Variables:");
        for var in &frame.local_variables {
            println!("  {}: {} = {:?} (scope: {})", 
                     var.name, var.var_type, var.value, var.scope);
        }
    }
    
    if !frame.inline_info.is_empty() {
        println!("Inlined Functions:");
        for inline in &frame.inline_info {
            println!("  {} (from {}:{})", 
                     inline.function_name, 
                     inline.inline_site.file, 
                     inline.inline_site.line);
        }
    }
}
```

## Future Enhancements

1. **DWARF Expression Evaluation**: Full support for DW_OP_* location expressions
2. **Remote Debugging**: Debug information for distributed CURSED programs
3. **Hot Reloading**: Update debug information for live code changes
4. **Machine Learning**: AI-powered debugging assistance
5. **Visualization**: Graphical debug information browsers
6. **Performance Analysis**: Integrated profiling with debug info

## Conclusion

The CURSED debug information system provides comprehensive debugging capabilities that rival those of traditional compiled languages while maintaining the flexibility and expressiveness of CURSED. The system's integration with DWARF standards ensures compatibility with existing debugging tools while providing CURSED-specific enhancements for modern development workflows.
