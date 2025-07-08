# SysCore Module

Low-level interface to operating system functionality with improved organization and Gen Z themed system operations.

## Features

- File operations (open, read, write, seek, close)
- Process management (PID, signals, resource usage)
- System information and error handling
- GenZ themed system operations

## Core Operations

### File Operations
- `Open(path, mode, perm)` - Open file with permissions
- `Close(fd)` - Close file descriptor
- `Read(fd, buf)` - Read from file
- `Write(fd, buf)` - Write to file
- `Seek(fd, offset, whence)` - Seek in file

### Process Management
- `Getpid()` - Get process ID
- `Getppid()` - Get parent process ID
- `Kill(pid, sig)` - Send signal to process
- `Getuid()/Getgid()` - Get user/group IDs

### Error Handling
- `IsPermissionDenied(err)` - Check permission errors
- `IsNotExist(err)` - Check file not found errors
- `IsTimeout(err)` - Check timeout errors

### GenZ Features
- `NoCap(fn)` - Execute function without restrictions
- `YeetProcess(pid, reason)` - Terminate process dramatically
- `VibeCheckSystem()` - Check system health
- `FlexResourceUsage()` - Get resource usage stats
- `SusIOPoll(fds, timeout)` - Efficient I/O polling

## Usage Examples

```cursed
// File operations
sus fd, err := sys_core.Open("/tmp/test.txt", 
    sys_core.O_RDWR | sys_core.O_CREAT, 
    sys_core.S_DEFAULT_FILE)
if err == cringe {
    sus data := []byte("Hello, syscalls!")
    sys_core.Write(fd, data)
    sys_core.Close(fd)
}

// Process info
sus pid := sys_core.Getpid()
sus uid := sys_core.Getuid()
vibez.spill("Process:", pid, "User:", uid)

// GenZ operations
sus healthy, issues := sys_core.VibeCheckSystem()
if !healthy {
    vibez.spill("System issues:", issues)
}

sus usage, err := sys_core.FlexResourceUsage()
if err == cringe {
    vibez.spill("Memory usage:", usage.Maxrss, "KB")
}
```
