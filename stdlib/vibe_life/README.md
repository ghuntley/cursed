# vibe_life - Essential OS Functionality for CURSED Self-Hosting

The `vibe_life` module provides essential operating system functionality needed for CURSED's self-hosting capabilities. This module implements pure CURSED solutions for command line processing, environment management, process control, and file system operations without requiring FFI bridges.

## Features

### Command Line Processing
- Access to command line arguments
- Argument counting and indexing
- Dynamic argument modification for testing

### Environment Variables
- Get/set environment variables
- Environment variable existence checking
- Environment cleanup and listing

### Process Control
- Process exit with custom codes
- Process ID access (simulated)
- Exit code management

### Working Directory Operations
- Current working directory access
- Directory changing
- Path manipulation utilities

### File System Operations
- File creation, reading, writing, deletion
- Directory operations
- File existence checking
- Path utilities (dirname, basename, join)

## API Reference

### Command Line Functions

```cursed
# Get all command line arguments
slay get_args() [tea]

# Get number of arguments
slay get_arg_count() normie

# Get specific argument by index
slay get_arg(index normie) tea

# Set arguments (for testing)
slay set_args(args [tea])
```

### Environment Functions

```cursed
# Get environment variable
slay get_env(key tea) tea

# Set environment variable
slay set_env(key tea, value tea) lit

# Check if environment variable exists
slay has_env(key tea) lit

# Get all environment variable keys
slay get_env_keys() [tea]

# Remove environment variable
slay unset_env(key tea) lit
```

### Process Functions

```cursed
# Exit process with code
slay exit(code normie)

# Get current exit code
slay get_exit_code() normie

# Get process ID (simulated)
slay get_pid() normie

# Get parent process ID (simulated)
slay get_ppid() normie
```

### Working Directory Functions

```cursed
# Get current working directory
slay get_cwd() tea

# Change working directory
slay set_cwd(path tea) lit

# Join path components
slay join_path(base tea, component tea) tea

# Get directory name from path
slay dirname(path tea) tea

# Get base name from path
slay basename(path tea) tea
```

### File System Functions

```cursed
# Check if file exists
slay file_exists(path tea) lit

# Create file with content
slay create_file(path tea, content tea) lit

# Read file content
slay read_file(path tea) tea

# Write content to file
slay write_file(path tea, content tea) lit

# Append content to file
slay append_file(path tea, content tea) lit

# Delete file
slay delete_file(path tea) lit

# Get file size
slay get_file_size(path tea) normie

# List all files
slay list_files() [tea]

# Create directory
slay create_dir(path tea) lit

# Check if path is directory
slay is_dir(path tea) lit
```

### Utility Functions

```cursed
# Get current timestamp
slay get_timestamp() normie

# Sleep for specified seconds
slay sleep(seconds normie)

# Initialize module
slay init_vibe_life()

# Get module information
slay get_module_info() tea
```

## Usage Examples

### Basic Command Line Processing
```cursed
yeet "vibe_life"

# Initialize the module
vibe_life.init_vibe_life()

# Get command line arguments
sus args [tea] = vibe_life.get_args()
vibez.spill("Program: " + args[0])
vibez.spill("Script: " + args[1])

# Process arguments
sus i normie = 0
while i < vibe_life.get_arg_count() {
    vibez.spill("Arg " + core.tea(i) + ": " + vibe_life.get_arg(i))
    i = i + 1
}
```

### Environment Variable Management
```cursed
yeet "vibe_life"

# Set up development environment
vibe_life.set_env("CURSED_MODE", "development")
vibe_life.set_env("DEBUG_LEVEL", "2")

# Get environment information
sus home tea = vibe_life.get_env("HOME")
sus path tea = vibe_life.get_env("PATH")

vibez.spill("Home directory: " + home)
vibez.spill("Debug mode: " + vibe_life.get_env("CURSED_MODE"))

# Check environment
if vibe_life.has_env("DEBUG_LEVEL") {
    vibez.spill("Debug level: " + vibe_life.get_env("DEBUG_LEVEL"))
}
```

### File System Operations
```cursed
yeet "vibe_life"

# Create project structure
vibe_life.create_dir("/home/user/myproject")
vibe_life.set_cwd("/home/user/myproject")

# Create source file
sus code tea = "slay main() {\n    vibez.spill(\"Hello, World!\")\n}\n"
vibe_life.create_file("main.💀", code)

# Read and verify
if vibe_life.file_exists("main.💀") {
    sus content tea = vibe_life.read_file("main.💀")
    vibez.spill("File size: " + core.tea(vibe_life.get_file_size("main.💀")))
    vibez.spill("Content: " + content)
}

# List project files
sus files [tea] = vibe_life.list_files()
sus i normie = 0
while i < files.length() {
    vibez.spill("File: " + files[i])
    i = i + 1
}
```

### Path Manipulation
```cursed
yeet "vibe_life"

# Working with paths
sus base_path tea = "/home/user/projects"
sus file_name tea = "main.💀"
sus full_path tea = vibe_life.join_path(base_path, file_name)

vibez.spill("Full path: " + full_path)
vibez.spill("Directory: " + vibe_life.dirname(full_path))
vibez.spill("Filename: " + vibe_life.basename(full_path))

# Create nested structure
sus project_dir tea = vibe_life.join_path(base_path, "myapp")
sus src_dir tea = vibe_life.join_path(project_dir, "src")
vibe_life.create_dir(project_dir)
vibe_life.create_dir(src_dir)
```

### Process Management
```cursed
yeet "vibe_life"

# Get process information
vibez.spill("Process ID: " + core.tea(vibe_life.get_pid()))
vibez.spill("Parent ID: " + core.tea(vibe_life.get_ppid()))

# Handle exit conditions
sus error_occurred lit = cap
if error_occurred {
    vibez.spill("Error detected, exiting...")
    vibe_life.exit(1)
} else {
    vibez.spill("Success!")
    vibe_life.exit(0)
}
```

## Implementation Details

### Pure CURSED Design
The `vibe_life` module is implemented entirely in CURSED without FFI dependencies:
- **Simulated File System**: Uses in-memory maps to simulate file operations
- **Environment Storage**: Uses maps to store environment variables
- **Process Simulation**: Provides simulated process IDs and exit codes
- **Path Operations**: Implements string-based path manipulation

### Self-Hosting Support
This module provides the essential OS interface needed for CURSED self-hosting:
- Command line argument processing for compiler flags
- Environment variable access for configuration
- File system operations for source code management
- Working directory management for build processes

### Testing Strategy
Comprehensive test coverage includes:
- **Unit Tests**: Individual function verification
- **Integration Tests**: Cross-module functionality
- **Edge Cases**: Boundary conditions and error handling
- **Both-Mode Testing**: Interpretation and compilation verification

## Testing

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/vibe_life/test_vibe_life.💀

# Test compilation mode
cargo run --bin cursed -- compile stdlib/vibe_life/test_vibe_life.💀
./test_vibe_life

# Verify both modes produce identical output
test_both_modes() {
    cargo run --bin cursed stdlib/vibe_life/test_vibe_life.💀 > interp.txt
    cargo run --bin cursed -- compile stdlib/vibe_life/test_vibe_life.💀
    ./test_vibe_life > comp.txt
    diff interp.txt comp.txt
}
```

## Integration with Other Modules

### Required Dependencies
- `testz`: Testing framework
- `core`: Type conversion utilities

### Module Interactions
- **Build System**: Provides file operations for compilation
- **Package Manager**: Supports environment and filesystem access
- **Self-Hosting**: Essential OS interface for compiler bootstrap

## Performance Characteristics

- **Memory Usage**: Efficient map-based storage for file system simulation
- **Startup Time**: Fast initialization with default environment setup
- **Scalability**: Handles large file content and extensive environment variables
- **Cross-Platform**: Pure CURSED implementation ensures portability

## Future Enhancements

1. **Real OS Integration**: Replace simulation with actual OS calls
2. **Permission System**: Add file permission management
3. **Async Operations**: Support for asynchronous file operations
4. **Network Integration**: Extend to network file systems
5. **Security Features**: Add access control and sandboxing

The `vibe_life` module represents a critical step toward CURSED's self-hosting capability, providing the essential OS functionality needed for a complete programming environment.
