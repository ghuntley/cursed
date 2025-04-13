# Pathing (path/filepath package)

## Overview
Pathing provides functions for manipulating file paths in a platform-independent way, as well as pattern matching for files. It combines functionality from Go's path and filepath packages with an emphasis on modern, intuitive path operations.

## Path Manipulation

### Basic Path Operations

```go
// Join joins any number of path elements into a single path
func Join(elem ...string) string

// Split splits path into directory and file components
func Split(path string) (dir, file string)

// Base returns the last element of path
func Base(path string) string

// Ext returns the file name extension used by path
func Ext(path string) string

// Dir returns all but the last element of path
func Dir(path string) string
```

### Path Cleaning and Normalization

```go
// Clean returns the shortest path equivalent to path
func Clean(path string) string

// ToSlash returns the result of replacing each separator character with a slash
func ToSlash(path string) string

// FromSlash returns the result of replacing each slash with a separator character
func FromSlash(path string) string

// Abs returns an absolute representation of path
func Abs(path string) (string, error)

// Rel returns a relative path that is lexically equivalent to targpath
func Rel(basepath, targpath string) (string, error)

// EvalSymlinks returns the path name after evaluating symlinks
func EvalSymlinks(path string) (string, error)
```

### Path Components

```go
// IsAbs reports whether the path is absolute
func IsAbs(path string) bool

// VolumeName returns leading volume name from path
func VolumeName(path string) string

// HasPrefix tests whether the path begins with prefix
func HasPrefix(path, prefix string) bool

// IsRoot determines if the path is a root directory
func IsRoot(path string) bool

// GetRoot returns the root component of the path
func GetRoot(path string) string
```

## File Path Operations

### Path Manipulation with OS Awareness

```go
// PathJoin joins path elements using the platform-specific separator
func PathJoin(elem ...string) string

// PathSplit splits path into directory and file components using platform-specific separator
func PathSplit(path string) (dir, file string)

// PathBase returns the last element of path using platform-specific separator
func PathBase(path string) string

// PathExt returns the file name extension used by path
func PathExt(path string) string

// PathDir returns all but the last element of path using platform-specific separator
func PathDir(path string) string

// PathClean returns the shortest path equivalent to path using platform-specific rules
func PathClean(path string) string
```

### Path Matching and Globbing

```go
// Match reports whether name matches the pattern
func Match(pattern, name string) (matched bool, err error)

// Glob returns the names of all files matching pattern
func Glob(pattern string) (matches []string, err error)

// PathMatches tests whether a path matches any of the provided patterns
func PathMatches(path string, patterns []string) bool

// FindByPattern walks the file tree and returns paths matching the pattern
func FindByPattern(root, pattern string) ([]string, error)
```

### Path Walking

```go
// Walk walks the file tree rooted at root, calling walkFn for each file or directory
func Walk(root string, walkFn func(path string, info os.FileInfo, err error) error) error

// WalkDir walks the file tree rooted at root, calling walkDirFn for each file or directory
func WalkDir(root string, walkDirFn func(path string, d os.DirEntry, err error) error) error

// FastWalk provides a faster alternative to Walk with fewer stat calls
func FastWalk(root string, walkFn func(path string, info os.FileInfo, err error) error) error
```

## Path Building and Manipulation

```go
// BuildPath creates a path from components with proper separators
func BuildPath(components ...string) string

// AddExt adds the extension to the path if it doesn't already have it
func AddExt(path, ext string) string

// ChangeExt changes the extension of the path
func ChangeExt(path, ext string) string

// RemoveExt removes the extension from the path
func RemoveExt(path string) string

// UpDir returns a path that is n directories up from the given path
func UpDir(path string, n int) string

// Siblings returns all siblings of a path (files in the same directory)
func Siblings(path string) ([]string, error)
```

## URL Path Handling

```go
// URLToPath converts a file URL to a file path
func URLToPath(fileURL string) (string, error)

// PathToURL converts a file path to a file URL
func PathToURL(path string) string

// JoinURLPath joins URL path components
func JoinURLPath(elem ...string) string
```

## Special Directories

```go
// HomeDir returns the home directory for the current user
func HomeDir() (string, error)

// TempDir returns the default directory for temporary files
func TempDir() string

// ConfigDir returns the appropriate configuration directory for the current user
func ConfigDir() (string, error)

// CacheDir returns the appropriate cache directory for the current user
func CacheDir() (string, error)

// DataDir returns the appropriate data directory for the current user
func DataDir() (string, error)

// ExecutableDir returns the directory containing the current executable
func ExecutableDir() (string, error)
```

## Usage Example

```go
// Joining paths
fullPath := pathing.PathJoin("usr", "local", "bin")
// On Unix: "/usr/local/bin", on Windows: "usr\local\bin"

// Getting path components
dir, file := pathing.PathSplit("/home/user/document.txt")
// dir = "/home/user/", file = "document.txt"

base := pathing.PathBase("/home/user/document.txt")
// base = "document.txt"

extension := pathing.PathExt("/home/user/document.txt")
// extension = ".txt"

// Cleaning paths
cleanPath := pathing.PathClean("/home//user/../user/./document.txt")
// cleanPath = "/home/user/document.txt"

// Absolute paths
absPath, _ := pathing.Abs("./document.txt")
// absPath might be "/current/working/dir/document.txt"

// Relative paths
relPath, _ := pathing.Rel("/home/user", "/home/user/docs/document.txt")
// relPath = "docs/document.txt"

// Finding files
matches, _ := pathing.Glob("/home/user/*.txt")
// matches = ["/home/user/document.txt", "/home/user/notes.txt", ...]

// Walking directories
pathing.Walk("/home/user", func(path string, info os.FileInfo, err error) error {
    if err != nil {
        return err
    }
    if info.IsDir() {
        vibez.spill("Directory:", path)
    } else {
        vibez.spill("File:", path)
    }
    return nil
})

// Using path building utilities
newPath := pathing.BuildPath("projects", "myapp", "src")
vibez.spill(newPath) // platform-specific path to src directory

home, _ := pathing.HomeDir()
configPath := pathing.PathJoin(home, ".config")
vibez.spill(configPath) // path to user's config directory
```

## Implementation Guidelines
1. Platform-independent implementation with platform-specific optimizations
2. Efficient string manipulation to minimize allocations
3. Proper error handling for file system operations
4. Comprehensive test coverage across different operating systems
5. Clear documentation with examples for all functions
6. Thread-safe implementation for concurrent use