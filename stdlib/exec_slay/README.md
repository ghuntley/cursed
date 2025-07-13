# exec_slay - Process Execution Module

**Critical self-hosting module for CURSED compiler pipeline execution**

The exec_slay module provides comprehensive process execution capabilities essential for the CURSED compiler's self-hosting functionality. It enables the compiler to execute LLVM tools, system commands, and manage complex compilation pipelines.

## Core Features

### Process Execution
- **Command Execution**: Execute system commands with arguments and capture results
- **Background Processes**: Launch and monitor long-running background processes  
- **Error Handling**: Robust error handling with detailed exit codes and error messages
- **Output Capture**: Capture stdout/stderr for processing and debugging

### Pipeline Operations
- **Command Chaining**: Chain multiple commands in sequential execution pipelines
- **Environment Management**: Set environment variables and working directories
- **Timeout Handling**: Execute commands with configurable timeouts
- **Pipeline Builder**: Fluent API for constructing complex command pipelines

### Compiler Integration
- **LLVM Integration**: Direct integration with LLVM toolchain (llc, opt, llvm-as)
- **Compilation Pipeline**: Complete source-to-executable compilation workflows
- **Optimization Passes**: Support for LLVM optimization levels (-O1, -O2, -O3)
- **Linking**: Object file linking with runtime library integration

## API Reference

### Core Execution Functions

```cursed
// Execute command with arguments and return detailed result
slay exec_command(cmd tea, args []tea) ProcessResult

// Launch background process and return handle for monitoring
slay exec_background(cmd tea) ProcessHandle

// Execute command with timeout protection
slay exec_with_timeout(cmd tea, args []tea, timeout_seconds normie) ProcessResult
```

### Pipeline Building

```cursed
// Create new pipeline for command chaining
slay create_pipeline() Pipeline

// Add command to pipeline sequence
slay pipe_command(pipeline *Pipeline, cmd tea) lit

// Execute all commands in pipeline
slay execute_pipeline(pipeline Pipeline) ProcessResult
```

### Command Builder

```cursed
// Create fluent command builder
slay build_command(program tea) CommandBuilder

// Add argument to command
slay add_argument(builder *CommandBuilder, arg tea) lit

// Set environment variable
slay set_env(builder *CommandBuilder, key tea, value tea) lit

// Execute built command
slay execute_command(builder CommandBuilder) ProcessResult
```

### Process Monitoring

```cursed
// Wait for process completion
slay wait_for_process(handle ProcessHandle) ProcessResult

// Terminate running process
slay kill_process(handle ProcessHandle) lit

// Check process status
slay process_status(handle ProcessHandle) tea

// Check if process is still running
slay is_process_running(handle ProcessHandle) lit
```

### Compiler Integration

```cursed
// Compile CURSED source to LLVM IR
slay compile_file(source_file tea, output_file tea) ProcessResult

// Run LLVM optimization passes
slay run_llvm_opt(input_file tea, output_file tea, opt_level normie) ProcessResult

// Compile LLVM IR to object file
slay run_llvm_compile(ir_file tea, obj_file tea) ProcessResult

// Link object files to executable
slay link_objects(obj_files []tea, output_exe tea) ProcessResult

// Complete compilation pipeline (source -> executable)
slay compile_pipeline(source_file tea, executable tea, optimize lit) ProcessResult
```

## Data Types

### ProcessResult
```cursed
vibe ProcessResult = smash {
    exit_code normie,    // Process exit code (0 = success)
    stdout tea,          // Standard output capture
    stderr tea,          // Standard error capture  
    success lit          // Overall success flag
}
```

### ProcessHandle
```cursed
vibe ProcessHandle = smash {
    pid normie,          // Process ID
    command tea,         // Executed command
    started_at normie,   // Start timestamp
    status tea           // Current status ("running", "completed", etc.)
}
```

### Pipeline
```cursed
vibe Pipeline = smash {
    commands []tea,           // Command sequence
    env_vars map[tea]tea,     // Environment variables
    working_dir tea,          // Working directory
    timeout normie            // Execution timeout
}
```

### CommandBuilder
```cursed
vibe CommandBuilder = smash {
    program tea,              // Program to execute
    args []tea,               // Command arguments
    env map[tea]tea,          // Environment variables
    cwd tea,                  // Working directory
    stdin_data tea            // Standard input data
}
```

## Usage Examples

### Basic Command Execution

```cursed
yeet "exec_slay"

// Execute simple command
sus result ProcessResult = exec_slay.exec_command("ls", []tea{"-la", "/tmp"})
sketchy result.success {
    vibez.spill("Directory listing:")
    vibez.spill(result.stdout)
} yikes {
    vibez.spill("Error:", result.stderr)
}
```

### Pipeline Construction

```cursed
// Build compilation pipeline
sus pipeline Pipeline = exec_slay.create_pipeline()
exec_slay.pipe_command(&pipeline, "cursed --emit-llvm program.csd")
exec_slay.pipe_command(&pipeline, "opt -O2 program.ll -o program_opt.ll")
exec_slay.pipe_command(&pipeline, "llc program_opt.ll -o program.o")
exec_slay.pipe_command(&pipeline, "clang program.o -o program")

sus result ProcessResult = exec_slay.execute_pipeline(pipeline)
```

### Fluent Command Building

```cursed
// Build complex compilation command
sus builder CommandBuilder = exec_slay.build_command("clang")
exec_slay.add_argument(&builder, "-std=c11")
exec_slay.add_argument(&builder, "-O3")
exec_slay.add_argument(&builder, "-Wall")
exec_slay.add_argument(&builder, "source.c")
exec_slay.add_argument(&builder, "-o")
exec_slay.add_argument(&builder, "output")

exec_slay.set_env(&builder, "CC", "clang")
exec_slay.set_cwd(&builder, "/build")

sus result ProcessResult = exec_slay.execute_command(builder)
```

### Complete Compilation Pipeline

```cursed
// Compile CURSED program with optimization
sus source_file tea = "my_program.csd"
sus executable tea = "my_program"
sus optimize lit = based

sus result ProcessResult = exec_slay.compile_pipeline(source_file, executable, optimize)
sketchy result.success {
    vibez.spill("Compilation successful!")
    vibez.spill("Executable:", executable)
} yikes {
    vibez.spill("Compilation failed:", result.stderr)
}
```

### Background Process Management

```cursed
// Launch background compiler service
sus handle ProcessHandle = exec_slay.exec_background("cursed --daemon")

// Monitor process
bestie exec_slay.is_process_running(handle) {
    sus status tea = exec_slay.process_status(handle)
    vibez.spill("Daemon status:", status)
    
    // Do other work...
    
    // Check again later
    core.sleep(1000)
}

// Clean shutdown
exec_slay.kill_process(handle)
sus final_result ProcessResult = exec_slay.wait_for_process(handle)
```

## Self-Hosting Integration

The exec_slay module is critical for CURSED's self-hosting capabilities:

### Compiler Pipeline Integration
- **Source Compilation**: Compile .csd files to LLVM IR
- **Optimization**: Apply LLVM optimization passes
- **Code Generation**: Generate native object files
- **Linking**: Link with runtime library to create executables

### Tool Integration
- **LLVM Tools**: llc, opt, llvm-as, llvm-link
- **System Tools**: clang, gcc, ld, ar
- **Build Tools**: make, ninja, cmake (future)

### Runtime Execution
- **JIT Compilation**: On-demand compilation and execution
- **Native Execution**: Launch compiled executables
- **Process Monitoring**: Track compilation and execution status

## Testing

```bash
# Test exec_slay module
cargo run --bin cursed stdlib/exec_slay/test_exec_slay.csd

# Test both interpretation and compilation modes
cargo run --bin cursed stdlib/exec_slay/test_exec_slay.csd
cargo run --bin cursed -- compile stdlib/exec_slay/test_exec_slay.csd
./test_exec_slay
```

## Performance Considerations

- **Process Overhead**: Minimize process spawning for performance-critical operations
- **Pipeline Efficiency**: Use pipelines for command sequences to reduce overhead
- **Timeout Management**: Set appropriate timeouts to prevent hanging
- **Resource Cleanup**: Always clean up process handles and temporary files

## Security Notes

- **Command Injection**: Always validate command arguments to prevent injection attacks
- **Path Validation**: Validate file paths to prevent directory traversal
- **Environment Isolation**: Use clean environments for untrusted code compilation
- **Resource Limits**: Implement resource limits for long-running processes

## Future Enhancements

- **Parallel Execution**: Support for parallel command execution
- **Remote Execution**: Execute commands on remote systems
- **Container Integration**: Docker/container support for isolated builds
- **Build Cache**: Intelligent caching for incremental compilation
- **Progress Reporting**: Real-time progress updates for long operations

---

**Status**: Production-ready, critical for self-hosting compiler functionality
**Dependencies**: core, stringz, io, error_drip modules
**Integration**: Essential for CURSED compiler toolchain and self-hosting
