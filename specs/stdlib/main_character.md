# MainCharacter (os package)

## Overview
MainCharacter provides a platform-independent interface to operating system functionality, positioning the application as the "main character" in its environment. It's inspired by Go's os package but with expanded capabilities and a user-centric design.

## File and Directory Operations

### `OpenVibe`
Opens a file for reading/writing (equivalent to os.Open/Create).

```go
func OpenVibe(name string) (*VibeFile, error)
func CreateVibe(name string) (*VibeFile, error)
func OpenVibeFile(name string, flag int, perm FileMode) (*VibeFile, error)
```

### `VibeFile`
Represents an open file descriptor.

```go
type VibeFile struct {}

// Methods
func (f *VibeFile) Read(b []byte) (n int, err error)
func (f *VibeFile) Write(b []byte) (n int, err error)
func (f *VibeFile) Close() error
func (f *VibeFile) Seek(offset int64, whence int) (int64, error)
func (f *VibeFile) ReadAt(b []byte, off int64) (n int, err error)
func (f *VibeFile) WriteAt(b []byte, off int64) (n int, err error)
func (f *VibeFile) Stat() (FileInfo, error)
func (f *VibeFile) Sync() error
func (f *VibeFile) Truncate(size int64) error
func (f *VibeFile) Chmod(mode FileMode) error
func (f *VibeFile) Chown(uid, gid int) error
```

### Directory Operations

```go
func MkdirVibe(name string, perm FileMode) error
func MkdirVibeAll(path string, perm FileMode) error
func RemoveVibe(name string) error
func RemoveVibeAll(path string) error
func ReadFolderVibe(name string) ([]DirEntry, error)
func CheckVibe(name string) error // Checks if a file or directory exists
```

### File Info and Permissions

```go
type FileMode uint32

const (
    // Permission bits
    VibePerms = 0777 // Unix permission bits
    
    // File mode bits
    VibeDirMode  = 1 << (32 - 1 - iota) // d: is a directory
    VibeSymMode                         // L: symbolic link
    VibeExclMode                        // exclusive use
    VibeTPipeMode                       // p: named pipe (FIFO)
    VibeDevMode                         // S: Unix domain socket
    VibeCharDevMode                     // c: Unix character device
    VibeBlockDevMode                    // b: Unix block device
)

type FileInfo interface {
    Name() string       // base name of the file
    Size() int64       // length in bytes
    Mode() FileMode    // file mode bits
    ModTime() time.Time // modification time
    IsDir() bool       // is a directory
    Sys() interface{}  // underlying data source
}

type DirEntry interface {
    Name() string      // base name of the directory entry
    IsDir() bool      // is the entry a directory
    Type() FileMode   // file mode bits
    Info() (FileInfo, error) // file info
}
```

## Process and Environment

### Process Management

```go
func VibeOut(code int) // Exits with status code (like os.Exit)
func NoVibeCheck(err error) // Exits if err is non-nil (like os.Exit)
func GetVibeID() int // Returns process ID (like os.Getpid)
func StartVibe(name string, args ...string) (*VibeProcess, error) // Starts a new process

type VibeProcess struct {}

// Methods
func (p *VibeProcess) Kill() error
func (p *VibeProcess) Wait() (*VibeProcessState, error)
func (p *VibeProcess) Signal(sig Signal) error

type VibeProcessState struct {}

// Methods
func (p *VibeProcessState) ExitCode() int
func (p *VibeProcessState) Success() bool
func (p *VibeProcessState) Sys() interface{}
```

### Environment Variables

```go
func GetEnvVibe(key string) string // Returns environment variable
func SetEnvVibe(key, value string) error // Sets environment variable
func UnsetEnvVibe(key string) error // Unsets environment variable
func VibeEnviron() []string // Returns all environment variables
func ExpandEnvVibe(s string) string // Expands environment variables in string
```

## Input/Output

```go
var (
    StandardVibe = NewVibeFile(uintptr(syscall.Stdin), "/dev/stdin") // Standard input
    VibeOutput   = NewVibeFile(uintptr(syscall.Stdout), "/dev/stdout") // Standard output
    ErrorVibe    = NewVibeFile(uintptr(syscall.Stderr), "/dev/stderr") // Standard error
)

func ReadVibe(file *VibeFile, b []byte) (n int, err error) // Read from file
func WriteVibe(file *VibeFile, b []byte) (n int, err error) // Write to file
```

## Working Directory

```go
func GetVibeWD() (string, error) // Gets working directory
func SetVibeWD(dir string) error // Sets working directory
```

## User Information

```go
func GetVibeHostname() (string, error) // Gets hostname
func IsVibeRoot() bool // Checks if process has root/admin privileges
func GetVibeUser() (*VibeUser, error) // Gets current user info

type VibeUser struct {
    Uid      string // user ID
    Gid      string // primary group ID
    Username string // login name
    Name     string // display name
    HomeDir  string // home directory
}
```

## Error Handling

```go
type VibeError struct {
    Op   string
    Path string
    Err  error
}

func (e *VibeError) Error() string

var (
    ErrVibeNotFound = errors.New("file not found")
    ErrVibeExist    = errors.New("file already exists")
    ErrVibeClosed   = errors.New("file already closed")
    ErrNoPerm       = errors.New("permission denied")
)

func IsNotVibeExist(err error) bool // Checks if error is ErrVibeNotFound
func IsVibeExist(err error) bool // Checks if error is ErrVibeExist
func IsVibePermission(err error) bool // Checks if error is permission-related
```

## File System Operations

```go
func WalkVibeDir(root string, fn WalkDirFunc) error // Walks directory tree
type WalkDirFunc func(path string, d DirEntry, err error) error

func RenameVibe(oldpath, newpath string) error // Renames file or directory
func SymlinkVibe(oldname, newname string) error // Creates symbolic link
func CopyVibe(src, dst string) error // Copies file (not in standard library)
func MoveVibe(src, dst string) error // Moves file (not in standard library)
func IsVibePathSeparator(c uint8) bool // Checks if character is path separator
```

## Operating System Detection

```go
const (
    IsWindows = runtime.GOOS == "windows"
    IsLinux   = runtime.GOOS == "linux"
    IsMac     = runtime.GOOS == "darwin"
)

func GetVibeOS() string // Returns operating system name
func GetVibeArch() string // Returns system architecture
```

## Usage Example

```go
// Creating and writing to a file
file, err := main_character.CreateVibe("example.txt")
if err != nil {
    main_character.ErrorVibe.WriteString("Failed to create file: " + err.Error())
    main_character.VibeOut(1)
}
defer file.Close()

_, err = file.WriteString("Hello, I'm the main character!")
if err != nil {
    vibez.spill("Error writing to file: ", err)
}

// Reading directory contents
entries, err := main_character.ReadFolderVibe(".")
if err != nil {
    vibez.spill("Error reading directory: ", err)
}

for _, entry := range entries {
    info, _ := entry.Info()
    vibez.spill(entry.Name(), info.Size())
}

// Getting environment variables
home := main_character.GetEnvVibe("HOME")
vibez.spill("Home directory: ", home)

// Starting a new process
proc, err := main_character.StartVibe("ls", "-la")
if err != nil {
    vibez.spill("Error starting process: ", err)
}

state, _ := proc.Wait()
vibez.spill("Exit code: ", state.ExitCode())
```

## Implementation Guidelines
1. Platform-independent interface with platform-specific optimizations
2. Consistent error handling across all operations
3. Thread-safe implementation for concurrent use
4. Efficient resource management with proper cleanup
5. Clear documentation for all functions and methods
6. Comprehensive test coverage for all platforms