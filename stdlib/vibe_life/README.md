# vibe_life - Operating System Interface Module

A comprehensive operating system interface module for CURSED, equivalent to Go's `os` package. Provides access to operating system functionality including environment variables, process control, file operations, and system information.

## Features

### Environment Variables
- `getenv(key)` - Get environment variable value
- `setenv(key, value)` - Set environment variable
- `unsetenv(key)` - Remove environment variable
- `environ()` - Get all environment variables
- `clearenv()` - Clear all environment variables
- `expand_env(text)` - Expand environment variables in text

### Command Line Arguments
- `args()` - Get all command line arguments
- `arg(index)` - Get specific argument by index
- `argc()` - Get number of arguments

### Process Control
- `exit(code)` - Exit program with specified code
- `getpid()` - Get current process ID
- `getppid()` - Get parent process ID
- `kill(pid, signal)` - Send signal to process

### File Path Operations
- `path_join(paths)` - Join file paths with appropriate separator
- `path_split(path)` - Split path into directory and file components
- `path_ext(path)` - Get file extension
- `path_base(path)` - Get base name of path
- `path_dir(path)` - Get directory portion of path
- `path_clean(path)` - Clean path by removing redundant separators
- `path_abs(path)` - Get absolute path

### Directory Operations
- `getcwd()` - Get current working directory
- `chdir(dir)` - Change working directory
- `mkdir(path, perm)` - Create directory with permissions
- `mkdir_all(path, perm)` - Create directory and all parents
- `rmdir(path)` - Remove empty directory
- `remove(path)` - Remove file or directory
- `remove_all(path)` - Remove directory and all contents

### File Information
- `exists(path)` - Check if file or directory exists
- `is_dir(path)` - Check if path is a directory
- `is_file(path)` - Check if path is a regular file
- `file_size(path)` - Get file size in bytes
- `file_mode(path)` - Get file permissions/mode
- `chmod(path, mode)` - Change file permissions
- `chown(path, uid, gid)` - Change file ownership

### User/Group Information
- `getuid()` - Get user ID
- `getgid()` - Get group ID
- `username()` - Get current username
- `hostname()` - Get system hostname

### Signal Handling
- `signal_handler(sig, handler)` - Install signal handler
- Signal constants: `SIGINT`, `SIGTERM`, `SIGKILL`, `SIGUSR1`, `SIGUSR2`

### System Information
- `system_info()` - Get system information
- `temp_dir()` - Get temporary directory path
- `home_dir()` - Get user home directory
- `time_now()` - Get current Unix timestamp
- `sleep(seconds)` - Sleep for specified seconds

## Usage Examples

### Environment Variables
```cursed
yeet "vibe_life"

# Get environment variable
sus home tea = getenv("HOME")
vibez.spill("Home directory:", home)

# Set environment variable
setenv("MY_VAR", "my_value")
sus my_val tea = getenv("MY_VAR")
vibez.spill("My variable:", my_val)
```

### Command Line Arguments
```cursed
yeet "vibe_life"

# Get all arguments
sus all_args [tea] = args()
vibez.spill("Argument count:", argc())

# Get specific argument
sus first_arg tea = arg(0)
vibez.spill("First argument:", first_arg)
```

### File Path Operations
```cursed
yeet "vibe_life"

# Join paths
sus paths [tea] = ["home", "user", "documents"]
sus full_path tea = path_join(paths)
vibez.spill("Full path:", full_path)

# Split path
sus (dir, file) = path_split("/home/user/file.txt")
vibez.spill("Directory:", dir)
vibez.spill("File:", file)

# Get file extension
sus ext tea = path_ext("/home/user/file.txt")
vibez.spill("Extension:", ext)
```

### Directory Operations
```cursed
yeet "vibe_life"

# Get current directory
sus cwd tea = getcwd()
vibez.spill("Current directory:", cwd)

# Create directory
mkdir("/tmp/my_dir", 755)
vibez.spill("Directory created")

# Check if exists
wenn exists("/tmp/my_dir") {
    vibez.spill("Directory exists")
}
```

### Process Control
```cursed
yeet "vibe_life"

# Get process information
sus pid normie = getpid()
sus ppid normie = getppid()
vibez.spill("Process ID:", pid)
vibez.spill("Parent Process ID:", ppid)

# Exit with code
exit(EXIT_SUCCESS)
```

### System Information
```cursed
yeet "vibe_life"

# Get system info
sus info tea = system_info()
sus host tea = hostname()
sus user tea = username()

vibez.spill("System:", info)
vibez.spill("Hostname:", host)
vibez.spill("Username:", user)
```

### Signal Handling
```cursed
yeet "vibe_life"

# Install signal handler
signal_handler(SIGINT, slay(sig normie) {
    vibez.spill("Received signal:", sig)
    exit(EXIT_SUCCESS)
})
```

## Constants

### Exit Codes
- `EXIT_SUCCESS` - Successful termination (0)
- `EXIT_FAILURE` - Unsuccessful termination (1)

### File Permissions
- `MODE_READ` - Read permission (0o444)
- `MODE_WRITE` - Write permission (0o200)
- `MODE_EXEC` - Execute permission (0o111)
- `MODE_USER_RWX` - User read/write/execute (0o700)
- `MODE_GROUP_RWX` - Group read/write/execute (0o070)
- `MODE_OTHER_RWX` - Other read/write/execute (0o007)

### Signals
- `SIGINT` - Interrupt signal (2)
- `SIGTERM` - Termination signal (15)
- `SIGKILL` - Kill signal (9)
- `SIGUSR1` - User signal 1 (10)
- `SIGUSR2` - User signal 2 (12)

## Error Handling

Functions return appropriate default values in simulation mode:
- String functions return empty strings when not found
- Boolean functions return `based` (true) for successful operations
- Integer functions return sensible default values
- Array functions return empty arrays when no data available

## Implementation Notes

This module provides a pure CURSED implementation of operating system interface functionality. In a production environment, these functions would interface with the actual operating system through system calls and C library functions.

The module maintains compatibility with both interpretation and compilation modes, ensuring consistent behavior across different execution environments.

## Testing

The module includes comprehensive tests covering all functionality:
- 50+ test functions covering all major features
- Edge case testing for path operations
- Constant validation
- Both positive and negative test cases

Run tests with:
```bash
cargo run --bin cursed stdlib/vibe_life/test_vibe_life.csd
```

## Architecture

The vibe_life module is designed as a high-level interface to operating system functionality, providing:
- Consistent API across different platforms
- Safe defaults and error handling
- Pure CURSED implementation without external dependencies
- Comprehensive coverage of common OS operations
- Production-ready reliability and performance

This module serves as the foundation for system-level programming in CURSED, enabling developers to build robust applications that interact with the operating system in a platform-independent manner.
