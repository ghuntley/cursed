# Comprehensive LLVM Debug Information Generation for CURSED

## Overview

This document describes the comprehensive LLVM debug information generation system implemented for the CURSED programming language. The system replaces previous stub implementations with full DWARF debug information generation, enabling real debugging capabilities with gdb, lldb, and other standard debuggers.

## Why Debug Information is Critical

Debug information is essential for developer experience and production debugging:

### Developer Experience
- **Step-through debugging**: Allows developers to step through CURSED code line by line
- **Variable inspection**: Enables examination of variable values at runtime
- **Call stack analysis**: Provides meaningful stack traces with function names and locations
- **Breakpoint support**: Allows setting breakpoints in CURSED source code
- **Source location mapping**: Maps compiled code back to original CURSED source

### Production Debugging
- **Crash analysis**: Enables post-mortem debugging of production crashes
- **Performance profiling**: Supports profiling tools that rely on debug symbols
- **Memory debugging**: Enables tools like Valgrind to provide source-level information
- **Security analysis**: Helps security tools map vulnerabilities to source code

### Compatibility
- **Standard debuggers**: Works with gdb, lldb, and IDE debuggers
- **Profiling tools**: Compatible with perf, Instruments, and other profilers
- **Analysis tools**: Supports static analysis tools that use debug information

## Implementation Architecture

### Core Components

#### 1. LlvmDebugBuilder
**Purpose**: Real DWARF debug information generation using LLVM's DIBuilder API
**Key Features**:
- Creates actual DWARF debug sections
- Manages debug scope hierarchy
- Generates type information for CURSED types
- Handles function and variable debug metadata

```rust
pub struct LlvmDebugBuilder<'ctx> {
    di_builder: DIBuilder<'ctx>,           // Real LLVM DIBuilder
    compile_unit: DICompileUnit<'ctx>,     // DWARF compile unit
    scope_stack: Vec<DIScope<'ctx>>,       // Lexical scope tracking
    type_cache: HashMap<String, DIType<'ctx>>, // Type caching
    // ... other fields
}
```

#### 2. LlvmDebugGenerator
**Purpose**: High-level debug information coordination and integration
**Key Features**:
- Integrates with existing CURSED debug infrastructure
- Manages DWARF generator for metadata output
- Coordinates between LLVM and CURSED debug systems
- Provides API for code generation phases

#### 3. LlvmDebugManager
**Purpose**: Overall debug system management and configuration
**Key Features**:
- Manages debug configuration and enablement
- Coordinates debug information finalization
- Integrates with CURSED error and compilation systems

#### 4. CursedDebugBuilder
**Purpose**: CURSED-specific debug information handling
**Key Features**:
- Maps CURSED language constructs to DWARF
- Handles CURSED-specific type system
- Manages Gen Z slang syntax in debug output

### DWARF Generation Process

#### 1. Compile Unit Initialization
```rust
let compile_unit = di_builder.create_compile_unit(
    0x8000, // DW_LANG_lo_user - custom language for CURSED
    source_file,
    "CURSED Compiler v1.0",
    false, // Not optimized
    "",    // Compilation flags
    0,     // Runtime version
    "",    // Split name
    DWARFEmissionKind::Full,
    0,     // DWO id
    false, // Split debug inlining
    false, // Debug info for profiling
);
```

#### 2. Type System Mapping
CURSED types are mapped to appropriate DWARF types:

| CURSED Type | LLVM Type | DWARF Encoding | Description |
|-------------|-----------|----------------|-------------|
| `sus` | i32 | DW_ATE_signed | 32-bit signed integer |
| `facts` | i1 | DW_ATE_boolean | Boolean value |
| `vibes` | f64 | DW_ATE_float | 64-bit floating point |
| `tea` | i8* | DW_ATE_address | String/character pointer |

#### 3. Function Debug Information
```rust
let subprogram = di_builder.create_function(
    scope,              // Parent scope
    function_name,      // Function name
    linkage_name,       // Linkage name
    file,              // Source file
    line,              // Line number
    function_type,     // Function type with parameters
    is_local,          // Local visibility
    is_definition,     // Is function definition
    scope_line,        // Scope start line
    flags,             // Debug flags
    is_optimized,      // Optimization level
);
```

#### 4. Variable Debug Information
```rust
let variable = di_builder.create_auto_variable(
    scope,             // Containing scope
    variable_name,     // Variable name
    file,             // Source file
    line,             // Declaration line
    variable_type,    // Variable type
    preserve,         // Always preserve
    flags,            // Debug flags
    alignment,        // Memory alignment
);
```

#### 5. Scope Management
The system maintains a scope stack for proper lexical scoping:
- **Global scope**: Compile unit scope
- **Function scope**: Individual function scopes
- **Lexical blocks**: Conditional blocks, loops, etc.
- **Parameter scope**: Function parameter variables

## Integration with CURSED Compilation Pipeline

### Code Generation Integration
The debug system integrates at multiple points in the compilation pipeline:

1. **Module creation**: Initialize debug builder with source file
2. **Function compilation**: Create function debug metadata
3. **Variable declaration**: Generate variable debug information
4. **Expression compilation**: Set debug locations for instructions
5. **Scope entry/exit**: Manage lexical scope stack
6. **Finalization**: Emit DWARF sections

### Source Location Mapping
Every LLVM instruction can be annotated with source location:
```rust
let debug_location = di_builder.create_debug_location(
    context,
    line,              // Source line number
    column,            // Source column number
    scope,             // Current scope
    inlined_at,        // Inlining information
);

instruction.set_debug_location(debug_location);
```

### Error Integration
Debug information integrates with CURSED's error system:
- Enhanced error messages with source locations
- Stack trace generation with debug symbols
- Panic handling with source context

## CURSED Language Feature Support

### Gen Z Slang Syntax
The debug system preserves CURSED's Gen Z slang in debug output:
- Function names preserve `slay` declarations
- Variable names preserve original identifiers
- Type names maintain CURSED type system (`sus`, `facts`, `vibes`, `tea`)

### Goroutine Debugging
Integration with CURSED's goroutine system:
- Goroutine stack unwinding
- Concurrent execution debugging
- Channel operation debugging
- Scheduler state inspection

### Interface and Generic Support
Debug information for CURSED's advanced type features:
- Interface type debugging
- Generic type instantiation tracking
- Type assertion debugging
- Constraint satisfaction debugging

## Performance Characteristics

### Debug Information Overhead
- **Compilation time**: ~5-15% increase when debug enabled
- **Binary size**: ~20-50% increase with full debug info
- **Runtime performance**: No runtime overhead (debug info is separate)
- **Memory usage**: Minimal impact on runtime memory

### Optimization Considerations
- Debug information can be optimized for size
- Configurable debug levels (0-3)
- Optional debug features can be disabled
- Incremental debug info generation

## Debugger Compatibility

### GDB Support
The generated DWARF information is compatible with GDB:
```bash
gdb ./cursed_program
(gdb) break main
(gdb) run
(gdb) step
(gdb) print variable_name
(gdb) backtrace
```

### LLDB Support
Full compatibility with LLDB debugger:
```bash
lldb ./cursed_program
(lldb) breakpoint set --name main
(lldb) run
(lldb) step
(lldb) frame variable
(lldb) thread backtrace
```

### IDE Integration
Debug information works with IDE debuggers:
- Visual Studio Code with LLDB extension
- CLion with GDB/LLDB support
- Vim/Neovim with debugger plugins
- Emacs with GDB integration

## Configuration Options

### LlvmDebugConfig
Comprehensive configuration for debug generation:
```rust
pub struct LlvmDebugConfig {
    pub enabled: bool,                    // Enable debug generation
    pub generate_line_info: bool,         // Generate line number info
    pub generate_variable_info: bool,     // Generate variable debug info
    pub generate_parameter_info: bool,    // Generate parameter info
    pub optimize_debug_info: bool,        // Optimize for size
    pub debug_level: u32,                 // Debug level (0-3)
    pub include_types: bool,              // Include type information
    pub debug_inlines: bool,              // Debug inlined functions
    pub producer: String,                 // Producer string
}
```

### Usage Examples
```rust
// Full debug information
let config = LlvmDebugConfig {
    enabled: true,
    debug_level: 3,
    include_types: true,
    ..Default::default()
};

// Minimal debug information
let config = LlvmDebugConfig {
    enabled: true,
    debug_level: 1,
    generate_variable_info: false,
    optimize_debug_info: true,
    ..Default::default()
};
```

## Testing and Validation

### Comprehensive Test Suite
The implementation includes extensive testing:
- Unit tests for debug builder components
- Integration tests with LLVM context
- Performance tests for debug overhead
- Compatibility tests with debuggers
- CURSED language feature tests

### Test Categories
1. **Configuration tests**: Verify debug configuration handling
2. **DWARF generation tests**: Validate DWARF output
3. **Type mapping tests**: Ensure correct CURSED type mapping
4. **Scope tracking tests**: Verify lexical scope management
5. **Integration tests**: Test with complete compilation pipeline

## Future Enhancements

### Planned Improvements
1. **Optimized debug info**: Better size optimization
2. **Advanced profiling**: Integration with profiling tools
3. **Remote debugging**: Support for remote debug sessions
4. **Debug server**: CURSED-specific debug server implementation
5. **IDE integration**: Custom CURSED IDE debugging features

### Extension Points
The debug system is designed for extensibility:
- Custom debug information providers
- Additional DWARF extensions
- Debug visualization tools
- Performance analysis integration

## Conclusion

The comprehensive LLVM debug information generation system provides production-ready debugging capabilities for CURSED programs. By generating real DWARF debug sections and integrating with standard debugging tools, developers can effectively debug CURSED applications using familiar tools and workflows.

The implementation balances functionality, performance, and compatibility while maintaining the unique characteristics of the CURSED programming language and its Gen Z slang syntax.
