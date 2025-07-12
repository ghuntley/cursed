# Process Module

The Process module provides essential system operations for process management, environment variables, command line arguments, and process execution. This module is critical for self-hosting capabilities, enabling the CURSED compiler to interact with the operating system.

## Features

- **Environment Variable Access**: Get, set, and manage environment variables
- **Command Line Argument Parsing**: Parse and access command line arguments
- **Process Execution**: Execute external commands and processes
- **Working Directory Operations**: Get and set current working directory
- **Process Information**: Access process ID, user information, and system details
- **Self-Hosting Support**: Special functions for compiler self-hosting

## Installation

```cursed
yeet "process"
```

## Basic Usage

### Environment Variables

```cursed
yeet "process"

# Get environment variable
home := process.get_env("HOME")
vibez.spill("Home directory: " + home)

# Set environment variable
process.set_env("MY_VAR", "my_value")

# Get all environment variables
all_env := process.get_all_env()
bestie key, value := range all_env {
    vibez.spill(key + "=" + value)
}

# Unset environment variable
process.unset_env("MY_VAR")
```

### Command Line Arguments

```cursed
yeet "process"

# Get all command line arguments
args := process.get_args()
vibez.spill("Program: " + args[0])

# Get specific argument
filename := process.get_arg(1)
vibez.spill("Input file: " + filename)

# Parse arguments with flags
parsed := process.parse_args(args)
if optimize := parsed["optimize"]; optimize != "" {
    vibez.spill("Optimization enabled")
}
```

### Process Execution

```cursed
yeet "process"

# Run a command
exit_code := process.run_command("ls -la")
if exit_code == 0 {
    vibez.spill("Command executed successfully")
}

# Spawn process with arguments
args := []tea{"-la", "/home/user"}
exit_code := process.spawn_process("ls", args)

# Check if command exists
if process.command_exists("gcc") {
    vibez.spill("GCC is available")
}
```

### Working Directory Operations

```cursed
yeet "process"

# Get current working directory
cwd := process.get_cwd()
vibez.spill("Current directory: " + cwd)

# Change directory
process.set_cwd("/tmp")
vibez.spill("Changed to: " + process.get_cwd())
```

### Process Information

```cursed
yeet "process"

# Get process information
pid := process.get_pid()
user := process.get_user()
hostname := process.get_hostname()
platform := process.get_platform()
arch := process.get_arch()

vibez.spill("PID: " + stringz.from_int(pid))
vibez.spill("User: " + user)
vibez.spill("Host: " + hostname)
vibez.spill("Platform: " + platform + "/" + arch)
```

## Self-Hosting Functions

The Process module includes special functions designed for compiler self-hosting:

### Compiler Environment Setup

```cursed
yeet "process"

# Setup environment for Stage 2 compiler
process.setup_compiler_environment()

# Check if we're in self-hosting mode
if process.get_env("CURSED_SELF_HOSTING") == "based" {
    vibez.spill("Self-hosting mode enabled")
}
```

### Build Tool Integration

```cursed
yeet "process"

# Check if build tools are available
if process.check_build_tools() {
    vibez.spill("All build tools available")
} else {
    vibez.spill("Missing build tools")
}

# Execute LLVM IR compilation
exit_code := process.execute_llc("program.ll", "program.o")
if exit_code == 0 {
    # Link executable
    exit_code := process.execute_gcc("program.o", "program")
}
```

### Compiler Argument Processing

```cursed
yeet "process"

# Get compiler-specific arguments
compiler_args := process.get_compiler_args()
bestie i := 0; i < len(compiler_args); i++ {
    arg := compiler_args[i]
    if stringz.has_suffix(arg, ".csd") {
        vibez.spill("Source file: " + arg)
    }
}
```

## Advanced Usage

### Environment Debugging

```cursed
yeet "process"

# Debug entire environment
process.debug_environment()
```

### Process Exit Handling

```cursed
yeet "process"

# Normal exit
process.exit()

# Exit with specific code
process.exit_with_code(1)
```

### Command Line Parsing

```cursed
yeet "process"

# Parse complex command line arguments
args := []tea{"cursed", "program.csd", "--optimize", "--output=binary", "-v", "--debug"}
parsed := process.parse_args(args)

# Check for flags
if parsed["optimize"] == "based" {
    vibez.spill("Optimization enabled")
}

# Get named parameters
output_file := parsed["output"]
if output_file != "" {
    vibez.spill("Output file: " + output_file)
}

# Check for short flags
if parsed["v"] == "based" {
    vibez.spill("Verbose mode enabled")
}
```

## Self-Hosting Example

Here's a complete example of using the Process module for compiler self-hosting:

```cursed
yeet "process"
yeet "stringz"

slay main() {
    # Setup compiler environment
    process.setup_compiler_environment()
    
    # Get compiler arguments
    args := process.get_compiler_args()
    if len(args) == 0 {
        vibez.spill("No input file specified")
        process.exit_with_code(1)
    }
    
    input_file := args[0]
    if !stringz.has_suffix(input_file, ".csd") {
        vibez.spill("Input file must be a .csd file")
        process.exit_with_code(1)
    }
    
    # Parse compiler options
    parsed := process.parse_args(process.get_args())
    output_file := parsed["output"]
    if output_file == "" {
        output_file = stringz.trim_suffix(input_file, ".csd")
    }
    
    # Check build tools
    if !process.check_build_tools() {
        vibez.spill("Required build tools not available")
        process.exit_with_code(1)
    }
    
    # Compile to LLVM IR
    ir_file := output_file + ".ll"
    vibez.spill("Compiling " + input_file + " to " + ir_file)
    
    # Execute LLVM compilation
    exit_code := process.execute_llc(ir_file, output_file + ".o")
    if exit_code != 0 {
        vibez.spill("LLVM compilation failed")
        process.exit_with_code(exit_code)
    }
    
    # Link executable
    exit_code := process.execute_gcc(output_file + ".o", output_file)
    if exit_code != 0 {
        vibez.spill("Linking failed")
        process.exit_with_code(exit_code)
    }
    
    vibez.spill("Compilation successful: " + output_file)
    process.exit()
}
```

## API Reference

### Environment Variables

- `get_env(key tea) tea` - Get environment variable value
- `set_env(key tea, value tea) lit` - Set environment variable
- `unset_env(key tea) lit` - Remove environment variable
- `get_all_env() map[tea]tea` - Get all environment variables

### Command Line Arguments

- `get_args() []tea` - Get all command line arguments
- `set_args(args []tea) lit` - Set command line arguments
- `parse_args(args []tea) map[tea]tea` - Parse arguments into map
- `get_arg(index normie) tea` - Get argument by index

### Process Execution

- `run_command(cmd tea) normie` - Execute command and return exit code
- `spawn_process(cmd tea, args []tea) normie` - Spawn process with arguments
- `command_exists(cmd tea) lit` - Check if command exists in PATH
- `exit()` - Exit process with code 0
- `exit_with_code(code normie)` - Exit process with specific code

### Working Directory

- `get_cwd() tea` - Get current working directory
- `set_cwd(path tea) lit` - Set working directory
- `change_dir(path tea) lit` - Change directory (alias for set_cwd)

### Process Information

- `get_pid() normie` - Get process ID
- `get_user() tea` - Get current user
- `get_hostname() tea` - Get hostname
- `get_platform() tea` - Get platform (e.g., "linux")
- `get_arch() tea` - Get architecture (e.g., "x86_64")

### Self-Hosting Functions

- `setup_compiler_environment() lit` - Setup compiler environment
- `get_compiler_args() []tea` - Get compiler-specific arguments
- `execute_llc(ir_file tea, output_file tea) normie` - Execute LLVM compiler
- `execute_gcc(obj_file tea, output_file tea) normie` - Execute GCC linker
- `check_build_tools() lit` - Check if build tools are available
- `debug_environment()` - Debug entire environment state

## Testing

Run the comprehensive test suite:

```bash
cargo run --bin cursed stdlib/process/test_process.csd
```

Run tests in both interpretation and compilation modes:

```bash
cargo run --bin cursed stdlib/process/test_process.csd
cargo run --bin cursed -- compile stdlib/process/test_process.csd
./test_process
```

## Implementation Notes

- This module is implemented in pure CURSED without FFI dependencies
- Process execution is simulated for essential self-hosting commands (llc, gcc)
- Environment variables are stored in global state for consistency
- Command line argument parsing supports both GNU-style (--key=value) and short (-k) formats
- Working directory operations maintain state across function calls
- Build tool integration is specifically designed for CURSED compiler self-hosting

## Self-Hosting Support

The Process module is specifically designed to support the CURSED compiler's self-hosting capabilities:

1. **Environment Setup**: Configures necessary environment variables for Stage 2 compilation
2. **Argument Processing**: Parses compiler command line arguments correctly
3. **Build Tool Integration**: Interfaces with LLVM (llc) and GCC for native compilation
4. **Process Management**: Handles process execution and exit codes appropriately
5. **Development Support**: Provides debugging and environment inspection capabilities

This module is essential for the Stage 2 compiler to successfully compile CURSED programs to native executables.
