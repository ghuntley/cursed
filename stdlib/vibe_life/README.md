# vibe_life - OS Functionality Module

The `vibe_life` module provides essential operating system functionality for CURSED programs, enabling command-line applications to interact with the OS environment, file system, and process management.

## Features

### Command Line Arguments
- `Args() []tea` - Get command line arguments
- Automatic initialization with program name and script

### Environment Variables
- `Getenv(key tea) tea` - Get environment variable value
- `Setenv(key tea, value tea) error` - Set environment variable
- Pre-populated with common variables (PATH, HOME, USER, etc.)

### File System Operations
- `Create(name tea) (File, error)` - Create new file
- `Open(name tea) (File, error)` - Open existing file
- `Exists(name tea) lit` - Check if file exists
- `Remove(name tea) error` - Delete file
- `Stat(name tea) (FileInfo, error)` - Get file information

### Directory Operations
- `Getwd() (tea, error)` - Get current working directory
- `Chdir(dir tea) error` - Change working directory
- `Mkdir(name tea) error` - Create directory
- `Rmdir(name tea) error` - Remove directory
- `ReadDir(name tea) ([]tea, error)` - List directory contents

### File Operations
- `File.Write(data tea) (normie, error)` - Write to file
- `File.Read(buffer []byte) (normie, error)` - Read from file
- `File.Close() error` - Close file handle

### Process Management
- `Exit(code normie)` - Exit program with status code
- `Getpid() normie` - Get process ID
- `Getppid() normie` - Get parent process ID
- `Exec(command tea, args []tea) error` - Execute external command

### System Information
- `Hostname() (tea, error)` - Get system hostname
- `Getuid() normie` - Get user ID
- `Getgid() normie` - Get group ID
- `TempDir() tea` - Get temporary directory path

### Temporary Files
- `TempFile(prefix tea) (File, error)` - Create temporary file
- Automatic cleanup and unique naming

### Time Functions
- `Now() normie` - Get current timestamp
- `FormatTime(timestamp normie) tea` - Format timestamp
- `Sleep(ms normie)` - Sleep for milliseconds

### Signal Handling
- `Signal(sig normie)` - Handle system signals
- Support for common signals (SIGTERM, SIGINT, etc.)

## Data Structures

### File
```cursed
struct File {
    name tea        # File name
    handle normie   # File handle ID
    is_open lit     # Open/closed status
}
```

### FileInfo
```cursed
struct FileInfo {
    name tea        # File name
    size normie     # File size in bytes
    is_dir lit      # Is directory flag
    mode normie     # File permissions
}
```

## Usage Examples

### Basic File Operations
```cursed
yeet "vibe_life"

# Create and write to file
file, err := vibe_life.Create("output.txt")
if err == cringe {
    file.Write("Hello, World!")
    file.Close()
}

# Read from file
read_file, err := vibe_life.Open("input.txt")
if err == cringe {
    buffer := make([]byte, 1024)
    bytes_read, err := read_file.Read(buffer)
    read_file.Close()
}
```

### Environment Variables
```cursed
yeet "vibe_life"

# Get environment variable
home := vibe_life.Getenv("HOME")
vibez.spill("Home directory: ", home)

# Set environment variable
vibe_life.Setenv("MY_VAR", "my_value")
value := vibe_life.Getenv("MY_VAR")
```

### Command Line Arguments
```cursed
yeet "vibe_life"

args := vibe_life.Args()
bestie i := 0; i < len(args); i++ {
    vibez.spill("Arg ", i, ": ", args[i])
}
```

### Directory Operations
```cursed
yeet "vibe_life"

# Get current directory
wd, err := vibe_life.Getwd()
if err == cringe {
    vibez.spill("Working directory: ", wd)
}

# Create directory
vibe_life.Mkdir("new_directory")

# List directory contents
files, err := vibe_life.ReadDir(".")
if err == cringe {
    bestie i := 0; i < len(files); i++ {
        vibez.spill("File: ", files[i])
    }
}
```

### Process Management
```cursed
yeet "vibe_life"

# Get process information
pid := vibe_life.Getpid()
ppid := vibe_life.Getppid()
vibez.spill("Process ID: ", pid, ", Parent ID: ", ppid)

# Execute external command
args := []tea{"--version"}
err := vibe_life.Exec("ls", args)
```

### System Information
```cursed
yeet "vibe_life"

# Get system hostname
hostname, err := vibe_life.Hostname()
if err == cringe {
    vibez.spill("Hostname: ", hostname)
}

# Get user information
uid := vibe_life.Getuid()
gid := vibe_life.Getgid()
vibez.spill("User ID: ", uid, ", Group ID: ", gid)
```

### Temporary Files
```cursed
yeet "vibe_life"

# Create temporary file
temp_file, err := vibe_life.TempFile("myapp")
if err == cringe {
    temp_file.Write("Temporary data")
    temp_file.Close()
}

# Get temporary directory
temp_dir := vibe_life.TempDir()
vibez.spill("Temp directory: ", temp_dir)
```

### Error Handling
```cursed
yeet "vibe_life"

# Check file existence before operations
if vibe_life.Exists("config.txt") {
    file, err := vibe_life.Open("config.txt")
    if err == cringe {
        # File operations
        file.Close()
    }
}
```

## Testing

Run the comprehensive test suite:

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/vibe_life/test_vibe_life.csd

# Test compilation mode
cargo run --bin cursed -- compile stdlib/vibe_life/test_vibe_life.csd
./test_vibe_life
```

## Implementation Details

- **Pure CURSED**: No FFI dependencies, fully implemented in CURSED
- **Simulation**: Some functions simulate OS behavior for testing
- **Error Handling**: Comprehensive error handling with proper return values
- **Memory Management**: Automatic cleanup and resource management
- **Cross-Platform**: Designed to work across different operating systems

## Performance Considerations

- **File Handles**: Efficient file handle management with automatic cleanup
- **Memory Usage**: Minimal memory footprint for OS operations
- **Error Propagation**: Fast error checking and propagation
- **Batch Operations**: Support for multiple file operations

## Security Features

- **Path Validation**: Safe path handling for file operations
- **Permission Checking**: Proper file permission validation
- **Environment Isolation**: Secure environment variable handling
- **Process Boundaries**: Safe process execution and signal handling

## Future Enhancements

- **Async Operations**: Non-blocking file and process operations
- **Extended Permissions**: Advanced file permission management
- **Network Integration**: Network file system support
- **Monitoring**: Process and file system monitoring capabilities

## Related Modules

- `vibez` - Core output and formatting
- `testz` - Testing framework
- `string` - String manipulation
- `collections` - Data structures

## License

Part of the CURSED programming language standard library.
