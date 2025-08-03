# CURSED DWARF Debug Information Implementation

## Overview

This implementation provides comprehensive DWARF debug information generation for the CURSED programming language, enabling full debugging support with gdb, lldb, and other debuggers.

## Features Implemented

### 1. Core Debug Information Generator (`debug_info.zig`)
- **DWARF Debug Information Generator**: Complete implementation of LLVM DIBuilder integration
- **Compile Unit Creation**: Proper source file and directory metadata
- **Basic Type Support**: All CURSED types (normie, tea, drip, lit, meal, smol, thicc, sip)
- **Function Debug Info**: Function signatures, parameters, and return types
- **Variable Debug Info**: Local variables, parameters, and global variables
- **Struct Debug Info**: Complete struct layout with field information
- **Pointer and Array Types**: Debug info for complex data structures
- **Source Location Tracking**: Line and column information for all constructs
- **Lexical Scope Management**: Proper scope nesting for blocks and functions

### 2. Advanced Code Generator Integration (`advanced_codegen.zig`)
- **Debug Info Initialization**: `enableDebugInfo(source_file)` function
- **CURSED-Specific Debug Functions**: Tailored debug info for CURSED language features
- **Function Debug Info**: `generateFunctionDebugInfo()` with CURSED type support
- **Variable Debug Info**: `generateVariableDebugInfo()` with CURSED type mapping
- **Struct Debug Info**: `generateStructDebugInfo()` with field layout
- **Interface Debug Info**: `generateInterfaceDebugInfo()` with vtable representation
- **Instruction Location Tracking**: `setInstructionDebugLocation()` for precise debugging
- **Scope Management**: `pushDebugScope()` and `popDebugScope()` for lexical blocks

### 3. CURSED Type System Debug Support
```zig
// CURSED type to DWARF encoding mapping
normie -> 32-bit signed integer (LLVMDWARFTypeEncodingSigned)
tea    -> 64-bit UTF string (LLVMDWARFTypeEncodingUTF)
drip   -> 64-bit signed integer (LLVMDWARFTypeEncodingSigned)
lit    -> 1-bit boolean (LLVMDWARFTypeEncodingBoolean)
meal   -> 64-bit float (LLVMDWARFTypeEncodingFloat)
smol   -> 8-bit signed integer (LLVMDWARFTypeEncodingSigned)
thicc  -> 64-bit signed integer (LLVMDWARFTypeEncodingSigned)
sip    -> 8-bit unsigned integer (LLVMDWARFTypeEncodingUnsigned)
```

### 4. Debug Information Features

#### Function Debug Information
```zig
// Generate debug info for CURSED functions
const di_function = try codegen.generateFunctionDebugInfo(
    function,           // LLVM function value
    "function_name",    // Function name
    line_number,        // Source line
    param_types,        // Parameter debug types
    return_type         // Return debug type
);
```

#### Variable Debug Information
```zig
// Generate debug info for CURSED variables
try codegen.generateVariableDebugInfo(
    alloca,            // LLVM alloca instruction
    "variable_name",   // Variable name
    line_number,       // Source line
    "normie"           // CURSED type name
);
```

#### Struct Debug Information
```zig
// Generate debug info for CURSED structs
const struct_debug = try codegen.generateStructDebugInfo(
    "Point",                           // Struct name
    &[_][]const u8{"x", "y", "label"}, // Field names
    &[_][]const u8{"meal", "meal", "tea"} // Field types
);
```

#### Interface Debug Information
```zig
// Generate debug info for CURSED interfaces
const interface_debug = try codegen.generateInterfaceDebugInfo(
    "Drawable",                        // Interface name
    &[_][]const u8{"draw", "area"}     // Method names
);
```

## Usage Instructions

### 1. Enable Debug Information
```zig
var codegen = try AdvancedCodeGen.init(allocator);
defer codegen.deinit();

// Enable debug info for source file
try codegen.enableDebugInfo("program.csd");
```

### 2. Compile with Debug Information
```bash
# Build CURSED compiler
zig build

# Compile CURSED program with debug info
./zig-out/bin/cursed-zig --debug program.csd

# Or use the integrated debug compilation
zig build && ./zig-out/bin/cursed-zig program.csd
```

### 3. Debug with GDB
```bash
# Start debugging session
gdb ./program

# GDB commands for CURSED debugging
(gdb) info functions          # List all functions
(gdb) info variables          # List all variables
(gdb) break main             # Set breakpoint on main function
(gdb) run                    # Run program
(gdb) print variable_name    # Print CURSED variable
(gdb) step                   # Step through CURSED code
(gdb) backtrace              # Show call stack
```

### 4. Debug with LLDB
```bash
# Start LLDB debugging session
lldb ./program

# LLDB commands for CURSED debugging
(lldb) target create program
(lldb) breakpoint set --name main
(lldb) run
(lldb) frame variable        # Show local variables
(lldb) step                  # Step through code
(lldb) bt                    # Show backtrace
```

## Implementation Details

### 1. Debug Information Architecture
- **DIBuilder Integration**: Uses LLVM's DIBuilder for standard DWARF generation
- **Metadata Caching**: Efficient caching of debug types to avoid duplication
- **Scope Stack Management**: Proper lexical scope tracking for nested blocks
- **Source Location Mapping**: Maps LLVM instructions to source locations

### 2. CURSED Language Feature Support
- **Slang Type Names**: Debug info preserves CURSED type names (normie, tea, etc.)
- **Struct Layout**: Accurate field offset and size information
- **Interface Vtables**: Debug representation of virtual method tables
- **Function Signatures**: Complete parameter and return type information
- **Variable Lifetime**: Proper scope tracking for local variables

### 3. Testing Infrastructure
- **Unit Tests**: Complete test suite in `debug_compilation_test.zig`
- **Integration Tests**: Full compilation pipeline testing
- **Debugger Validation**: Automated testing with gdb/lldb
- **Comprehensive Test Program**: `debug_info_comprehensive_test.csd`

## Files Implemented

### Core Implementation
- `src-zig/debug_info.zig` - DWARF debug information generator
- `src-zig/advanced_codegen.zig` - Debug info integration with code generator
- `src-zig/debug_compilation_test.zig` - Unit tests for debug functionality

### Test Programs
- `debug_info_comprehensive_test.csd` - Comprehensive debug feature testing
- `test_debug_info.sh` - Automated testing script

### Documentation
- `DWARF_DEBUG_IMPLEMENTATION_SUMMARY.md` - This implementation summary

## Debugging CURSED Programs

### Example Debug Session
```bash
# Compile CURSED program with debug info
./zig-out/bin/cursed-zig debug_test.csd

# Start GDB session
gdb ./debug_test

# Set breakpoint on CURSED function
(gdb) break calculate_distance

# Run program
(gdb) run

# When breakpoint hits, examine CURSED variables
(gdb) print p1      # Shows Point struct with x, y, label fields
(gdb) print dx      # Shows meal (float) variable
(gdb) info locals   # Shows all local variables with CURSED types

# Step through CURSED code
(gdb) step          # Steps through CURSED statements
(gdb) next          # Steps over CURSED function calls

# Examine CURSED data structures
(gdb) print coordinates[0].x    # Access struct field
(gdb) print *drawable_ptr       # Dereference interface pointer
```

### Debug Information Validation
```bash
# Check for debug sections in executable
objdump -h program | grep debug

# Examine DWARF debug information
dwarfdump program

# Verify debug symbols
nm --debug program

# Check LLVM IR debug metadata
llvm-dis program.ll | grep -E "!dbg|DICompileUnit|DISubprogram"
```

## Benefits

1. **Full Debugging Support**: Complete integration with standard debuggers
2. **CURSED Type Awareness**: Debuggers understand CURSED type system
3. **Source-Level Debugging**: Step through CURSED source code line by line
4. **Variable Inspection**: Examine CURSED variables with proper type information
5. **Struct and Interface Debugging**: Full support for complex CURSED data structures
6. **Production Ready**: Robust implementation suitable for production debugging

## Future Enhancements

1. **Custom CURSED Language ID**: Register CURSED as a custom DWARF language
2. **Optimized Debug Info**: Debug info that survives optimization passes
3. **Inlined Function Support**: Debug info for inlined CURSED functions
4. **Template/Generic Debug Info**: Enhanced debug support for generic types
5. **Remote Debugging**: Debug info compatible with remote debugging protocols

## Conclusion

The DWARF debug information implementation provides comprehensive debugging support for CURSED programs, enabling developers to use familiar debugging tools (gdb, lldb) with full visibility into CURSED language constructs, types, and execution flow.
