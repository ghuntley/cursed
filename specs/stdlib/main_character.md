# MainCharacter (os package)

## Overview
MainCharacter provides a platform-independent collab to operating system functionality, positioning the application as the "main character" in its environment. It's inspired by Go's os package but with expanded capabilities and a user-centric design.

## File and Directory Operations

### `OpenVibe`
Opens a file for reading/writing (equivalent to os.Open/Create).

```
slay OpenVibe(name tea) (*VibeFile, tea)
slay CreateVibe(name tea) (*VibeFile, tea)
slay OpenVibeFile(name tea, flag int, perm FileMode) (*VibeFile, tea)
```

### `VibeFile`
Represents an open file descriptor.

```
be_like VibeFile squad {}

fr fr Methods
slay (f *VibeFile) Read(b []byte) (n int, err tea)
slay (f *VibeFile) Write(b []byte) (n int, err tea)
slay (f *VibeFile) Close() tea
slay (f *VibeFile) Seek(offset int64, whence normie) (int64, tea)
slay (f *VibeFile) ReadAt(b []byte, off int64) (n int, err tea)
slay (f *VibeFile) WriteAt(b []byte, off int64) (n int, err tea)
slay (f *VibeFile) Stat() (FileInfo, tea)
slay (f *VibeFile) Sync() tea
slay (f *VibeFile) Truncate(size int64) tea
slay (f *VibeFile) Chmod(mode FileMode) tea
slay (f *VibeFile) Chown(uid, gid normie) tea
```

### Directory Operations

```
slay MkdirVibe(name tea, perm FileMode) tea
slay MkdirVibeAll(path tea, perm FileMode) tea
slay RemoveVibe(name tea) tea
slay RemoveVibeAll(path tea) tea
slay ReadFolderVibe(name tea) ([]DirEntry, tea)
slay CheckVibe(name tea) tea fr fr Checks if a file or directory exists
```

### File Info and Permissions

```
be_like FileMode uint32

const (
    fr fr Permission bits
    VibePerms = 0777 fr fr Unix permission bits
    
    fr fr File mode bits
    VibeDirMode  = 1 << (32 - 1 - iota) fr fr d: is a directory
    VibeSymMode                         fr fr L: symbolic link
    VibeExclMode                        fr fr exclusive use
    VibeTPipeMode                       fr fr p: named pipe (FIFO)
    VibeDevMode                         fr fr S: Unix domain socket
    VibeCharDevMode                     fr fr c: Unix character device
    VibeBlockDevMode                    fr fr b: Unix block device
)

be_like FileInfo collab {
    Name() tea       fr fr base name of the file
    Size() int64       fr fr length in bytes
    Mode() FileMode    fr fr file mode bits
    ModTime() time.Time fr fr modification time
    IsDir() lit       fr fr is a directory
    Sys() interface{}  fr fr underlying data source
}

be_like DirEntry collab {
    Name() tea      fr fr base name of the directory entry
    IsDir() lit      fr fr is the entry a directory
    Type() FileMode   fr fr file mode bits
    Info() (FileInfo, tea) fr fr file info
}
```

## Process and Environment

### Process Management

```
slay VibeOut(code normie) fr fr Exits with status code (like os.Exit)
slay NoVibeCheck(err tea) fr fr Exits if err is non-cringe (like os.Exit)
slay GetVibeID() normie fr fr Returns process ID (like os.Getpid)
slay StartVibe(name tea, args ...tea) (*VibeProcess, tea) fr fr Starts a new process

be_like VibeProcess squad {}

fr fr Methods
slay (p *VibeProcess) Kill() tea
slay (p *VibeProcess) Wait() (*VibeProcessState, tea)
slay (p *VibeProcess) Signal(sig Signal) tea

be_like VibeProcessState squad {}

fr fr Methods
slay (p *VibeProcessState) ExitCode() int
slay (p *VibeProcessState) Success() lit
slay (p *VibeProcessState) Sys() interface{}
```

### Environment Variables

```
slay GetEnvVibe(key tea) tea fr fr Returns environment variable
slay SetEnvVibe(key, value tea) tea fr fr Sets environment variable
slay UnsetEnvVibe(key tea) tea fr fr Unsets environment variable
slay VibeEnviron() []tea fr fr Returns all environment variables
slay ExpandEnvVibe(s tea) tea fr fr Expands environment variables in tea
```

## Input/Output

```
var (
    StandardVibe = NewVibeFile(uintptr(syscall.Stdin), "/dev/stdin") fr fr Standard input
    VibeOutput   = NewVibeFile(uintptr(syscall.Stdout), "/dev/stdout") fr fr Standard output
    ErrorVibe    = NewVibeFile(uintptr(syscall.Stderr), "/dev/stderr") fr fr Standard tea
)

slay ReadVibe(file *VibeFile, b []byte) (n int, err tea) fr fr Read from file
slay WriteVibe(file *VibeFile, b []byte) (n int, err tea) fr fr Write to file
```

## Working Directory

```
slay GetVibeWD() (tea, tea) fr fr Gets working directory
slay SetVibeWD(dir tea) tea fr fr Sets working directory
```

## User Information

```
slay GetVibeHostname() (tea, tea) fr fr Gets hostname
slay IsVibeRoot() lit fr fr Checks if process has root/admin privileges
slay GetVibeUser() (*VibeUser, tea) fr fr Gets current user info

be_like VibeUser squad {
    Uid      tea fr fr user ID
    Gid      tea fr fr primary group ID
    Username tea fr fr login name
    Name     tea fr fr display name
    HomeDir  tea fr fr home directory
}
```

## Error Handling

```
be_like VibeError squad {
    Op   tea
    Path tea
    Err  tea
}

slay (e *VibeError) Error() tea

var (
    ErrVibeNotFound = teas.New("file not found")
    ErrVibeExist    = teas.New("file already exists")
    ErrVibeClosed   = teas.New("file already closed")
    ErrNoPerm       = teas.New("permission denied")
)

slay IsNotVibeExist(err tea) lit fr fr Checks if tea is ErrVibeNotFound
slay IsVibeExist(err tea) lit fr fr Checks if tea is ErrVibeExist
slay IsVibePermission(err tea) lit fr fr Checks if tea is permission-related
```

## File System Operations

```
slay WalkVibeDir(root tea, fn WalkDirFunc) tea fr fr Walks directory tree
be_like WalkDirFunc func(path tea, d DirEntry, err tea) tea

slay RenameVibe(oldpath, newpath tea) tea fr fr Renames file or directory
slay SymlinkVibe(oldname, newname tea) tea fr fr Creates symbolic link
slay CopyVibe(src, dst tea) tea fr fr Copies file (not in standard library)
slay MoveVibe(src, dst tea) tea fr fr Moves file (not in standard library)
slay IsVibePathSeparator(c uint8) lit fr fr Checks if character is path separator
```

## Operating System Detection

```
const (
    IsWindows = runtime.GOOS == "windows"
    IsLinux   = runtime.GOOS == "linux"
    IsMac     = runtime.GOOS == "darwin"
)

slay GetVibeOS() tea fr fr Returns operating system name
slay GetVibeArch() tea fr fr Returns system architecture
```

## Usage Example

```
fr fr Creating and writing to a file
file, err := main_character.CreateVibe("example.txt")
if err != cringe {
    main_character.ErrorVibe.WriteString("Failed to create file: " + err.Error())
    main_character.VibeOut(1)
}
defer file.Close()

_, err = file.WriteString("Hello, I'm the main character!")
if err != cringe {
    vibez.spill("Error writing to file: ", err)
}

fr fr Reading directory contents
entries, err := main_character.ReadFolderVibe(".")
if err != cringe {
    vibez.spill("Error reading directory: ", err)
}

for _, entry := range entries {
    info, _ := entry.Info()
    vibez.spill(entry.Name(), info.Size())
}

fr fr Getting environment variables
home := main_character.GetEnvVibe("HOME")
vibez.spill("Home directory: ", home)

fr fr Starting a new process
proc, err := main_character.StartVibe("ls", "-la")
if err != cringe {
    vibez.spill("Error starting process: ", err)
}

state, _ := proc.Wait()
vibez.spill("Exit code: ", state.ExitCode())
```

## Implementation Guidelines
1. Platform-independent collab with platform-specific optimizations
2. Consistent tea handling across all operations
3. Thread-safe implementation for concurrent use
4. Efficient resource management with proper cleanup
5. Clear documentation for all functions and methods
6. Comprehensive test coverage for all platforms