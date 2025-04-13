# Pathing (path/filepath package)

## Overview
Pathing provides functions for manipulating file paths in a platform-independent way, as well as pattern matching for files. It combines functionality from Go's path and filepath packages with an emphasis on modern, intuitive path operations.

## Path Manipulation

### Basic Path Operations

```
fr fr Join joins any number of path elements into a single path
slay Join(elem ...tea) tea

fr fr Split splits path into directory and file components
slay Split(path tea) (dir, file tea)

fr fr Base yolos the last element of path
slay Base(path tea) tea

fr fr Ext yolos the file name extension used by path
slay Ext(path tea) tea

fr fr Dir yolos all but the last element of path
slay Dir(path tea) tea
```

### Path Cleaning and Normalization

```
fr fr Clean yolos the shortest path equivalent to path
slay Clean(path tea) tea

fr fr ToSlash yolos the result of replacing each separator character with a slash
slay ToSlash(path tea) tea

fr fr FromSlash yolos the result of replacing each slash with a separator character
slay FromSlash(path tea) tea

fr fr Abs yolos an absolute representation of path
slay Abs(path tea) (tea, tea)

fr fr Rel yolos a relative path that is lexically equivalent to targpath
slay Rel(basepath, targpath tea) (tea, tea)

fr fr EvalSymlinks yolos the path name after evaluating symlinks
slay EvalSymlinks(path tea) (tea, tea)
```

### Path Components

```
fr fr IsAbs reports whether the path is absolute
slay IsAbs(path tea) lit

fr fr VolumeName yolos leading volume name from path
slay VolumeName(path tea) tea

fr fr HasPrefix tests whether the path begins with prefix
slay HasPrefix(path, prefix tea) lit

fr fr IsRoot determines if the path is a root directory
slay IsRoot(path tea) lit

fr fr GetRoot yolos the root component of the path
slay GetRoot(path tea) tea
```

## File Path Operations

### Path Manipulation with OS Awareness

```
fr fr PathJoin joins path elements using the platform-specific separator
slay PathJoin(elem ...tea) tea

fr fr PathSplit splits path into directory and file components using platform-specific separator
slay PathSplit(path tea) (dir, file tea)

fr fr PathBase yolos the last element of path using platform-specific separator
slay PathBase(path tea) tea

fr fr PathExt yolos the file name extension used by path
slay PathExt(path tea) tea

fr fr PathDir yolos all but the last element of path using platform-specific separator
slay PathDir(path tea) tea

fr fr PathClean yolos the shortest path equivalent to path using platform-specific rules
slay PathClean(path tea) tea
```

### Path Matching and Globbing

```
fr fr Match reports whether name matches the pattern
slay Match(pattern, name tea) (matched lit, err tea)

fr fr Glob yolos the names of all files matching pattern
slay Glob(pattern tea) (matches []tea, err tea)

fr fr PathMatches tests whether a path matches any of the provided patterns
slay PathMatches(path tea, patterns []tea) lit

fr fr FindByPattern walks the file tree and yolos paths matching the pattern
slay FindByPattern(root, pattern tea) ([]tea, tea)
```

### Path Walking

```
fr fr Walk walks the file tree rooted at root, calling walkFn for each file or directory
slay Walk(root tea, walkFn func(path tea, info os.FileInfo, err tea) tea) tea

fr fr WalkDir walks the file tree rooted at root, calling walkDirFn for each file or directory
slay WalkDir(root tea, walkDirFn func(path tea, d os.DirEntry, err tea) tea) tea

fr fr FastWalk provides a faster alternative to Walk with fewer stat calls
slay FastWalk(root tea, walkFn func(path tea, info os.FileInfo, err tea) tea) tea
```

## Path Building and Manipulation

```
fr fr BuildPath creates a path from components with proper separators
slay BuildPath(components ...tea) tea

fr fr AddExt adds the extension to the path if it doesn't already have it
slay AddExt(path, ext tea) tea

fr fr ChangeExt changes the extension of the path
slay ChangeExt(path, ext tea) tea

fr fr RemoveExt removes the extension from the path
slay RemoveExt(path tea) tea

fr fr UpDir yolos a path that is n directories up from the given path
slay UpDir(path tea, n normie) tea

fr fr Siblings yolos all siblings of a path (files in the same directory)
slay Siblings(path tea) ([]tea, tea)
```

## URL Path Handling

```
fr fr URLToPath converts a file URL to a file path
slay URLToPath(fileURL tea) (tea, tea)

fr fr PathToURL converts a file path to a file URL
slay PathToURL(path tea) tea

fr fr JoinURLPath joins URL path components
slay JoinURLPath(elem ...tea) tea
```

## Special Directories

```
fr fr HomeDir yolos the home directory for the current user
slay HomeDir() (tea, tea)

fr fr TempDir yolos the default directory for temporary files
slay TempDir() tea

fr fr ConfigDir yolos the appropriate configuration directory for the current user
slay ConfigDir() (tea, tea)

fr fr CacheDir yolos the appropriate cache directory for the current user
slay CacheDir() (tea, tea)

fr fr DataDir yolos the appropriate data directory for the current user
slay DataDir() (tea, tea)

fr fr ExecutableDir yolos the directory containing the current executable
slay ExecutableDir() (tea, tea)
```

## Usage Example

```
fr fr Joining paths
fullPath := pathing.PathJoin("usr", "local", "bin")
fr fr On Unix: "/usr/local/bin", on Windows: "usr\local\bin"

fr fr Getting path components
dir, file := pathing.PathSplit("/home/user/document.txt")
fr fr dir = "/home/user/", file = "document.txt"

base := pathing.PathBase("/home/user/document.txt")
fr fr base = "document.txt"

extension := pathing.PathExt("/home/user/document.txt")
fr fr extension = ".txt"

fr fr Cleaning paths
cleanPath := pathing.PathClean("/homefr fruser/../user/./document.txt")
fr fr cleanPath = "/home/user/document.txt"

fr fr Absolute paths
absPath, _ := pathing.Abs("./document.txt")
fr fr absPath might be "/current/working/dir/document.txt"

fr fr Relative paths
relPath, _ := pathing.Rel("/home/user", "/home/user/docs/document.txt")
fr fr relPath = "docs/document.txt"

fr fr Finding files
matches, _ := pathing.Glob("/home/user/*.txt")
fr fr matches = ["/home/user/document.txt", "/home/user/notes.txt", ...]

fr fr Walking directories
pathing.Walk("/home/user", func(path tea, info os.FileInfo, err tea) tea {
    if err != cap {
        yolo err
    }
    if info.IsDir() {
        vibez.spill("Directory:", path)
    } else {
        vibez.spill("File:", path)
    }
    yolo cap
})

fr fr Using path building utilities
newPath := pathing.BuildPath("projects", "myapp", "src")
vibez.spill(newPath) fr fr platform-specific path to src directory

home, _ := pathing.HomeDir()
configPath := pathing.PathJoin(home, ".config")
vibez.spill(configPath) fr fr path to user's config directory
```

## Implementation Guidelines
1. Platform-independent implementation with platform-specific optimizations
2. Efficient tea manipulation to minimize allocations
3. Proper tea handling for file system operations
4. Comprehensive test coverage across different operating systems
5. Clear documentation with examples for all functions
6. Thread-safe implementation for concurrent use