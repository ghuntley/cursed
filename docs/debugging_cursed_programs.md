# Debugging CURSED Programs

This guide covers debugging CURSED programs using various debugging tools and techniques. The CURSED compiler generates comprehensive debug information compatible with standard debuggers like GDB, LLDB, and IDE debugging interfaces.

## Table of Contents

1. [Debug Information Generation](#debug-information-generation)
2. [Debugging Tools](#debugging-tools)
3. [Configuration](#configuration)
4. [Command Line Usage](#command-line-usage)
5. [IDE Integration](#ide-integration)
6. [Common Debugging Scenarios](#common-debugging-scenarios)
7. [Troubleshooting](#troubleshooting)

## Debug Information Generation

The CURSED compiler automatically generates debug information when compiling your programs. This includes:

- **DWARF debug information**: Industry-standard debug format
- **Source location mapping**: Maps compiled code back to source lines
- **Symbol information**: Function names, variable names, and types
- **Stack frame information**: For accurate stack traces
- **Scope tracking**: Variable visibility and lifetime information

### Debug Levels

The compiler supports multiple debug information levels:

- **Level 0**: No debug information
- **Level 1**: Line tables only (minimal overhead)
- **Level 2**: Full debug information (default)
- **Level 3**: Enhanced debug information with additional metadata

## Debugging Tools

### GDB (GNU Debugger)

GDB is the most common debugger for CURSED programs on Linux and macOS.

#### Basic GDB Commands

```bash
# Compile with debug information
cursed build --debug-level 2 myprogram.csd

# Start debugging
gdb ./myprogram

# Common GDB commands
(gdb) break main           # Set breakpoint at main function
(gdb) run                  # Start program execution
(gdb) step                 # Step into functions
(gdb) next                 # Step over functions
(gdb) print variable_name  # Print variable value
(gdb) backtrace           # Show stack trace
(gdb) info locals         # Show local variables
(gdb) continue            # Continue execution
```

#### Generated GDB Scripts

The CURSED debug tool can generate GDB scripts for your programs:

```bash
cursed-debug myprogram.csd --format gdb-script --output debug/
gdb -x debug/myprogram.gdb ./myprogram
```

### LLDB (LLVM Debugger)

LLDB is the preferred debugger on macOS and works well with CURSED programs.

#### Basic LLDB Commands

```bash
# Start debugging
lldb ./myprogram

# Common LLDB commands
(lldb) breakpoint set --name main    # Set breakpoint
(lldb) run                          # Start program
(lldb) step                         # Step into
(lldb) next                         # Step over
(lldb) print variable_name          # Print variable
(lldb) bt                          # Show backtrace
(lldb) frame variable              # Show local variables
(lldb) continue                    # Continue execution
```

#### Generated LLDB Scripts

```bash
cursed-debug myprogram.csd --format lldb-script --output debug/
lldb -s debug/myprogram.lldb ./myprogram
```

## Configuration

### Build Configuration

Configure debug information in your `CursedBuild.toml`:

```toml
[tools.compiler.debug]
enabled = true
level = 2
include_source = false
optimized = false
compress = true
split_debug_info = false
dwarf_version = 4
inline_debug = true
type_debug = true
variable_debug = true
parameter_debug = true
scope_debug = true
line_debug = true
column_debug = false
```

### Debug Profiles

Set up different debug profiles for different use cases:

```toml
[profiles.debug]
debug = true
optimization = "none"
debug_assertions = true

[profiles.debug.compiler.debug]
level = 3
include_source = true
column_debug = true

[profiles.release-with-debug]
debug = true
optimization = "speed"
debug_assertions = false

[profiles.release-with-debug.compiler.debug]
level = 1
optimized = true
compress = true
split_debug_info = true
```

### Command Line Options

The `cursed` compiler supports debug-related command line options:

```bash
# Debug level
cursed build --debug-level 2

# Include source in debug info
cursed build --debug-include-source

# Optimized debug info
cursed build --debug-optimized

# Compress debug sections
cursed build --debug-compress

# Split debug info
cursed build --debug-split

# DWARF version
cursed build --debug-dwarf-version 4
```

## Command Line Usage

### CURSED Debug Tool

The `cursed-debug` tool provides comprehensive debug information generation:

```bash
# Generate LLVM IR with debug info
cursed-debug myprogram.csd --format llvm-ir --output debug/

# Generate DWARF information
cursed-debug myprogram.csd --format dwarf --output debug/

# Generate GDB script
cursed-debug myprogram.csd --format gdb-script --output debug/

# Generate LLDB script
cursed-debug myprogram.csd --format lldb-script --output debug/

# Generate VS Code configuration
cursed-debug myprogram.csd --format vscode-config --output .vscode/

# Generate comprehensive debug report
cursed-debug myprogram.csd --format report --output debug/

# Validate debug information
cursed-debug myprogram.csd --validate --stats
```

### Advanced Options

```bash
# High debug level with source inclusion
cursed-debug myprogram.csd -g 3 --include-source --format llvm-ir

# Optimized debug for release builds
cursed-debug myprogram.csd --optimized --compress --split-debug

# Custom DWARF version
cursed-debug myprogram.csd --dwarf-version 5 --format dwarf

# Verbose output with statistics
cursed-debug myprogram.csd --verbose --stats --validate
```

## IDE Integration

### Visual Studio Code

1. Generate VS Code debug configuration:

```bash
cursed-debug myprogram.csd --format vscode-config --output .vscode/
```

2. The generated `.vscode/launch.json` will contain debugging configuration for the C/C++ extension.

3. Install the C/C++ extension if not already installed.

4. Set breakpoints in your CURSED source files and start debugging with F5.

### CLion / IntelliJ IDEA

1. Import your CURSED project
2. Configure the debugger to use GDB or LLDB
3. Set the executable path to your compiled CURSED program
4. Add source directories for your CURSED files

### Vim/Neovim

Use plugins like `vimspector` or `nvim-dap` with the generated GDB/LLDB scripts:

```vim
" In your .vimrc or init.lua
let g:vimspector_enable_mappings = 'HUMAN'

" Use generated GDB script
:VimspectorLoadSession debug/myprogram.gdb
```

## Common Debugging Scenarios

### Setting Breakpoints

```bash
# Function breakpoint
(gdb) break my_function
(lldb) breakpoint set --name my_function

# Line breakpoint
(gdb) break myfile.csd:42
(lldb) breakpoint set --file myfile.csd --line 42

# Conditional breakpoint
(gdb) break my_function if x == 5
(lldb) breakpoint set --name my_function --condition 'x == 5'
```

### Inspecting Variables

```bash
# Print variable value
(gdb) print my_variable
(lldb) print my_variable

# Print with formatting
(gdb) print/x my_variable    # Hexadecimal
(lldb) print/x my_variable

# Watch variable changes
(gdb) watch my_variable
(lldb) watchpoint set variable my_variable
```

### Analyzing Stack Traces

```bash
# Show full stack trace
(gdb) backtrace
(lldb) bt

# Show stack trace with local variables
(gdb) backtrace full
(lldb) bt all

# Move between stack frames
(gdb) frame 2
(lldb) frame select 2
```

### Memory Debugging

```bash
# Examine memory
(gdb) x/10x $rsp            # Examine 10 hex words from stack pointer
(lldb) memory read $rsp     # Read memory from stack pointer

# Set memory watchpoints
(gdb) watch *0x12345678
(lldb) watchpoint set expression -- 0x12345678
```

## Troubleshooting

### Debug Information Not Available

**Problem**: Debugger shows "No debugging symbols found"

**Solutions**:
1. Ensure debug information is enabled in build configuration
2. Check that debug level is > 0
3. Verify the executable wasn't stripped of debug symbols
4. Use `cursed-debug --validate` to check debug information

### Optimized Code Debugging Issues

**Problem**: Variables showing `<optimized out>` or stepping behaves strangely

**Solutions**:
1. Use debug builds for debugging (optimization = "none")
2. Enable optimized debug information: `optimized = true`
3. Use higher debug levels that preserve more information
4. Set compiler to preserve frame pointers

### Source File Not Found

**Problem**: Debugger can't find source files

**Solutions**:
1. Ensure source paths are correct in debug information
2. Use absolute paths when compiling
3. Set source directories in debugger configuration
4. Check file permissions and accessibility

### Incomplete Stack Traces

**Problem**: Stack traces are truncated or missing frames

**Solutions**:
1. Ensure debug information includes scope information
2. Check that all libraries were compiled with debug info
3. Increase stack trace depth limits in debugger
4. Use `parameter_debug = true` for better parameter tracking

### Performance Impact

**Problem**: Debug builds are too slow

**Solutions**:
1. Use optimized debug builds for better performance
2. Enable debug compression to reduce binary size
3. Use split debug information for faster loading
4. Consider level 1 debug info for minimal overhead

## Debug Information Validation

Use the validation features to ensure debug information quality:

```bash
# Validate debug information
cursed-debug myprogram.csd --validate

# Show detailed statistics
cursed-debug myprogram.csd --stats

# Generate comprehensive report
cursed-debug myprogram.csd --format report --validate --stats
```

The validation checks for:
- Consistent symbol information
- Valid source location mappings
- Proper scope nesting
- Complete type information
- Correct line number mappings

## Best Practices

1. **Always compile with debug information during development**
2. **Use appropriate debug levels** - level 2 for most cases, level 3 for complex debugging
3. **Enable source inclusion** for production debugging when source isn't available
4. **Use optimized debug info** for release builds that need debugging capability
5. **Validate debug information** as part of your build process
6. **Generate debug scripts** for consistent debugging workflows
7. **Configure IDEs properly** using generated configuration files
8. **Keep debug symbols** even in release builds when possible

## Advanced Features

### Custom Debug Utilities

CURSED programs include built-in debug utilities:

```cursed
// Built-in debug functions (available when compiled with debug info)
cursed_debug_print_int(42)
cursed_debug_print_string("Hello, debugging!")
cursed_debug_breakpoint()  // Triggers debugger breakpoint
```

### Debug Information Analysis

Analyze debug information with external tools:

```bash
# View DWARF information
objdump --dwarf=info myprogram
readelf --debug-dump myprogram

# Analyze debug sections
llvm-dwarfdump myprogram
```

For more information, see the [CURSED Compiler Reference](compiler_reference.md) and [Build System Documentation](build_system.md).
