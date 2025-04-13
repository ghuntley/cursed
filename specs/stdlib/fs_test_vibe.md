# fs_test_vibe (testing/fstest)

## Overview
The `fs_test_vibe` module provides utilities for testing filesystem implementations. It includes functions for verifying file system behavior, creating test fixtures, and comparing file system operations against expected results.

## Core Types and Interfaces

### TestFS
Interface for testing a file system implementation.

```csd
type TestFS interface {
  // Setup creates a test file system with predefined structure
  Setup() (main_character.FS, error)
  
  // Teardown cleans up the test file system
  Teardown(fs main_character.FS) error
}
```

### MapFS
An in-memory file system implementation for testing.

```csd
type MapFS map[string]*MapFile

func (fsys MapFS) Open(name string) (main_character.File, error)
func (fsys MapFS) Stat(name string) (main_character.FileInfo, error)
func (fsys MapFS) ReadFile(name string) ([]byte, error)
func (fsys MapFS) ReadDir(name string) ([]main_character.DirEntry, error)
```

### MapFile
A file in a MapFS.

```csd
type MapFile struct {
  Data    []byte            // File content
  Mode    main_character.FileMode  // File mode
  ModTime timez.Time         // Modification time
  Sys     interface{}        // Underlying data source
}
```

### TestFile
Defines a file for test cases.

```csd
type TestFile struct {
  Path     string            // File path
  Content  []byte            // File content
  Mode     main_character.FileMode  // File mode
  ModTime  timez.Time         // Modification time
  IsDir    bool              // Whether it's a directory
}
```

## Core Functions

```csd
// Create a filesystem from a set of files
func NewMapFS(files []TestFile) MapFS

// Create TestFiles from a directory
func TestFilesFromDir(dir string) ([]TestFile, error)

// Run standard file system tests
func TestFS(t *test_vibes.T, fsys main_character.FS)

// Test reading a file
func TestReadFile(t *test_vibes.T, fsys main_character.FS, path string, want []byte)

// Test reading a directory
func TestReadDir(t *test_vibes.T, fsys main_character.FS, path string, want []string)

// Verify that two filesystems have the same contents
func Equal(t *test_vibes.T, fsys1, fsys2 main_character.FS) bool

// Compare file or directory entries
func DirEquals(dir1, dir2 []main_character.DirEntry) bool

// TestFileInfo returns FileInfo for a test file
func TestFileInfo(name string, f TestFile) main_character.FileInfo
```

## Testing Utilities

```csd
// Create a temporary test directory
func TempDir(t *test_vibes.T) string

// Create test files in a directory
func CreateTestFiles(t *test_vibes.T, dir string, files []TestFile) error

// Set up a test filesystem with common test files
func SetupTestFS() (main_character.FS, func(), error)

// Validate filesystem operations
func ValidateFS(t *test_vibes.T, fsys main_character.FS, tests []FSTest)

// Test open errors
func TestOpenErrors(t *test_vibes.T, fsys main_character.FS)

// Test read errors
func TestReadErrors(t *test_vibes.T, fsys main_character.FS)
```

## Enhanced Features

- **Custom Test Harnesses**: Create specialized test environments
  ```csd
  harness := fs_test_vibe.NewHarness()
  harness.AddTest("read-only", fs_test_vibe.ReadOnlyTest)
  harness.AddTest("concurrent-access", fs_test_vibe.ConcurrentAccessTest)
  harness.Run(t, fsys)
  ```

- **Test Case Generators**: Generate test cases for different filesystem operations
  ```csd
  tests := fs_test_vibe.GeneratePathTests("root/path", 100)
  fs_test_vibe.RunTests(t, fsys, tests)
  ```

- **FS Comparison Tools**: Compare filesystem content and structure
  ```csd
  diff := fs_test_vibe.Compare(expected, actual)
  if !diff.Equal() {
    t.Errorf("Filesystem differences: %s", diff)
  }
  ```

- **Performance Testing**: Measure performance of filesystem operations
  ```csd
  metrics := fs_test_vibe.BenchmarkFS(fsys, fs_test_vibe.ReadWriteOperations)
  ```

- **Fuzz Testing**: Apply fuzzing techniques to filesystem testing
  ```csd
  fuzzer := fs_test_vibe.NewFuzzer(fsys)
  fuzzer.Run(1000) // Run 1000 random operations
  ```

## Usage Examples

```csd
// Creating a test filesystem with MapFS
testFiles := []fs_test_vibe.TestFile{
  {
    Path:    "hello.txt",
    Content: []byte("Hello, World!"),
    Mode:    0644,
    ModTime: timez.Now(),
    IsDir:   false,
  },
  {
    Path:    "subdir",
    Mode:    main_character.ModeDir | 0755,
    ModTime: timez.Now(),
    IsDir:   true,
  },
  {
    Path:    "subdir/file.txt",
    Content: []byte("File in subdirectory"),
    Mode:    0644,
    ModTime: timez.Now(),
    IsDir:   false,
  },
}

// Create the MapFS
fsys := fs_test_vibe.NewMapFS(testFiles)

// Manually access files in the test filesystem
helloFile, err := fsys.Open("hello.txt")
if err != nil {
  vibez.spill("Error opening file: %v", err)
  return
}
defer helloFile.Close()

// Read file content
content, err := dropz.ReadAll(helloFile)
if err != nil {
  vibez.spill("Error reading file: %v", err)
  return
}

vibez.spill("File content: %s", string(content))

// List directory contents
entries, err := fsys.ReadDir("subdir")
if err != nil {
  vibez.spill("Error reading directory: %v", err)
  return
}

vibez.spill("\nSubdirectory contents:")
for _, entry := range entries {
  info, err := entry.Info()
  if err != nil {
    vibez.spill("Error getting file info: %v", err)
    continue
  }
  
  vibez.spill("  %s (Size: %d bytes, Mode: %v)", 
    entry.Name(), info.Size(), info.Mode())
}

// Using the helper functions within test code
// (These would normally be used in actual test functions)

// Simulated test function
example_test := func(t *test_vibes.T) {
  // Create temporary directory for test
  tempDir := fs_test_vibe.TempDir(t)
  defer main_character.RemoveAll(tempDir)
  
  // Create test files in the directory
  err := fs_test_vibe.CreateTestFiles(t, tempDir, testFiles)
  if err != nil {
    t.Fatalf("Failed to create test files: %v", err)
  }
  
  // Create filesystem from temp directory
  dirFS := main_character.DirFS(tempDir)
  
  // Verify filesystem operations
  tests := []fs_test_vibe.FSTest{
    {
      Name: "ReadFile",
      Op: func(t *test_vibes.T, fsys main_character.FS) {
        fs_test_vibe.TestReadFile(t, fsys, "hello.txt", []byte("Hello, World!"))
      },
    },
    {
      Name: "ReadDir",
      Op: func(t *test_vibes.T, fsys main_character.FS) {
        fs_test_vibe.TestReadDir(t, fsys, "subdir", []string{"file.txt"})
      },
    },
  }
  
  fs_test_vibe.ValidateFS(t, dirFS, tests)
  
  // Compare two filesystems
  equal := fs_test_vibe.Equal(t, fsys, dirFS)
  t.Logf("Filesystems equal: %v", equal)
}

// Simulate running the test function
t := &mockT{} // Simplified mock test type
example_test(t)

vibez.spill("\nTest results:")
vibez.spill("  Passed: %v", !t.failed)
if t.failed {
  vibez.spill("  Error message: %s", t.errorMsg)
}

// Creating a test harness
harness := fs_test_vibe.NewHarness()

// Add custom test cases
harness.AddTest("file-exists", func(t *test_vibes.T, fsys main_character.FS) {
  _, err := fsys.Open("hello.txt")
  if err != nil {
    t.Error("hello.txt should exist")
  }
})

harness.AddTest("file-content", func(t *test_vibes.T, fsys main_character.FS) {
  content, err := main_character.ReadFile(fsys, "hello.txt")
  if err != nil {
    t.Error("Failed to read hello.txt")
    return
  }
  
  if string(content) != "Hello, World!" {
    t.Errorf("Expected 'Hello, World!', got '%s'", string(content))
  }
})

// Run the test harness
t = &mockT{}
harness.Run(t, fsys)

vibez.spill("\nHarness test results:")
vibez.spill("  Passed: %v", !t.failed)
if t.failed {
  vibez.spill("  Error message: %s", t.errorMsg)
}

// Using the performance testing tools
metrics := fs_test_vibe.BenchmarkFS(fsys, fs_test_vibe.FileOperations{
  ReadFiles: []string{"hello.txt", "subdir/file.txt"},
  WriteFiles: []fs_test_vibe.WriteOperation{
    {Path: "new.txt", Content: []byte("New file")},
  },
  ListDirs: []string{"", "subdir"},
})

vibez.spill("\nPerformance metrics:")
vibez.spill("  Read operations: %d ops/sec", metrics.ReadsPerSecond)
vibez.spill("  Write operations: %d ops/sec", metrics.WritesPerSecond)
vibez.spill("  List operations: %d ops/sec", metrics.ListsPerSecond)
vibez.spill("  Total time: %v", metrics.TotalTime)

// Using fuzzing tools
fuzzer := fs_test_vibe.NewFuzzer(fsys)
fuzzer.SetMaxOperations(100)
fuzzer.SetSeed(12345)
result := fuzzer.Run()

vibez.spill("\nFuzzing results:")
vibez.spill("  Operations performed: %d", result.OperationsPerformed)
vibez.spill("  Successful operations: %d", result.SuccessfulOperations)
vibez.spill("  Failed operations: %d", result.FailedOperations)
if result.FoundIssue {
  vibez.spill("  Issue found: %s", result.IssueDescription)
  vibez.spill("  Reproduction steps: %v", result.ReproductionSteps)
}

// Mock implementation for demonstration purposes
type mockT struct {
  failed   bool
  errorMsg string
}

func (t *mockT) Error(args ...interface{}) {
  t.failed = true
  t.errorMsg = vibez.spill_to_string(args...)
}

func (t *mockT) Errorf(format string, args ...interface{}) {
  t.failed = true
  t.errorMsg = vibez.spill_to_string(format, args...)
}

func (t *mockT) Fatal(args ...interface{}) {
  t.failed = true
  t.errorMsg = vibez.spill_to_string(args...)
}

func (t *mockT) Fatalf(format string, args ...interface{}) {
  t.failed = true
  t.errorMsg = vibez.spill_to_string(format, args...)
}

func (t *mockT) Logf(format string, args ...interface{}) {
  // Just log to console for this example
  vibez.spill(format, args...)
}
```

## Implementation Guidelines

- Provide comprehensive testing of all file system operations
- Ensure tests cover error conditions and edge cases
- Support testing of both in-memory and real file systems
- Implement clear error messages for test failures
- Enable cross-platform file system testing
- Support testing of concurrent file system access
- Allow testing of performance characteristics
- Provide utilities for comparing file system state
- Support advanced tests like fuzzing and stress testing
- Make test utilities composable and reusable