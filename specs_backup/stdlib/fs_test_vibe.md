# fs_test_vibe (testing/fstest)

## Overview
The `fs_test_vibe` module provides utilities for testing filesystem implementations. It includes functions for verifying file system behavior, creating test fixtures, and comparing file system operations against expected results.

## Core Types and Interfaces

### TestFS
Interface for testing a file system implementation.

```csd
be_like TestFS collab {
  fr fr Setup creates a test file system with predefined squadure
  Setup() (main_character.FS, tea)
  
  fr fr Teardown cleans up the test file system
  Teardown(fs main_character.FS) tea
}
```

### MapFS
An in-memory file system implementation for testing.

```csd
be_like MapFS map[tea]*MapFile

slay (fsys MapFS) Open(name tea) (main_character.File, tea)
slay (fsys MapFS) Stat(name tea) (main_character.FileInfo, tea)
slay (fsys MapFS) ReadFile(name tea) ([]byte, tea)
slay (fsys MapFS) ReadDir(name tea) ([]main_character.DirEntry, tea)
```

### MapFile
A file in a MapFS.

```csd
be_like MapFile squad {
  Data    []byte            fr fr File content
  Mode    main_character.FileMode  fr fr File mode
  ModTime timez.Time         fr fr Modification time
  Sys     interface{}        fr fr Underlying data source
}
```

### TestFile
Defines a file for test cases.

```csd
be_like TestFile squad {
  Path     tea            fr fr File path
  Content  []byte            fr fr File content
  Mode     main_character.FileMode  fr fr File mode
  ModTime  timez.Time         fr fr Modification time
  IsDir    lit              fr fr Whether it's a directory
}
```

## Core Functions

```csd
fr fr Create a filesystem from a set of files
slay NewMapFS(files []TestFile) MapFS

fr fr Create TestFiles from a directory
slay TestFilesFromDir(dir tea) ([]TestFile, tea)

fr fr Run standard file system tests
slay TestFS(t *test_vibes.T, fsys main_character.FS)

fr fr Test reading a file
slay TestReadFile(t *test_vibes.T, fsys main_character.FS, path tea, want []byte)

fr fr Test reading a directory
slay TestReadDir(t *test_vibes.T, fsys main_character.FS, path tea, want []tea)

fr fr Verify that two filesystems have the same contents
slay Equal(t *test_vibes.T, fsys1, fsys2 main_character.FS) lit

fr fr Compare file or directory entries
slay DirEquals(dir1, dir2 []main_character.DirEntry) lit

fr fr TestFileInfo yolos FileInfo for a test file
slay TestFileInfo(name tea, f TestFile) main_character.FileInfo
```

## Testing Utilities

```csd
fr fr Create a temporary test directory
slay TempDir(t *test_vibes.T) tea

fr fr Create test files in a directory
slay CreateTestFiles(t *test_vibes.T, dir tea, files []TestFile) tea

fr fr Set up a test filesystem with common test files
slay SetupTestFS() (main_character.FS, func(), tea)

fr fr Validate filesystem operations
slay ValidateFS(t *test_vibes.T, fsys main_character.FS, tests []FSTest)

fr fr Test open teas
slay TestOpenErrors(t *test_vibes.T, fsys main_character.FS)

fr fr Test read teas
slay TestReadErrors(t *test_vibes.T, fsys main_character.FS)
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

- **FS Comparison Tools**: Compare filesystem content and squadure
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
  fuzzer.Run(1000) fr fr Run 1000 random operations
  ```

## Usage Examples

```csd
fr fr Creating a test filesystem with MapFS
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
    IsDir:   based,
  },
  {
    Path:    "subdir/file.txt",
    Content: []byte("File in subdirectory"),
    Mode:    0644,
    ModTime: timez.Now(),
    IsDir:   false,
  },
}

fr fr Create the MapFS
fsys := fs_test_vibe.NewMapFS(testFiles)

fr fr Manually access files in the test filesystem
helloFile, err := fsys.Open("hello.txt")
if err != cap {
  vibez.spill("Error opening file: %v", err)
  yolo
}
defer helloFile.Close()

fr fr Read file content
content, err := dropz.ReadAll(helloFile)
if err != cap {
  vibez.spill("Error reading file: %v", err)
  yolo
}

vibez.spill("File content: %s", tea(content))

fr fr List directory contents
entries, err := fsys.ReadDir("subdir")
if err != cap {
  vibez.spill("Error reading directory: %v", err)
  yolo
}

vibez.spill("\nSubdirectory contents:")
for _, entry := range entries {
  info, err := entry.Info()
  if err != cap {
    vibez.spill("Error getting file info: %v", err)
    continue
  }
  
  vibez.spill("  %s (Size: %d bytes, Mode: %v)", 
    entry.Name(), info.Size(), info.Mode())
}

fr fr Using the helper functions within test code
fr fr (These would normally be used in actual test functions)

fr fr Simulated test function
example_test := func(t *test_vibes.T) {
  fr fr Create temporary directory for test
  tempDir := fs_test_vibe.TempDir(t)
  defer main_character.RemoveAll(tempDir)
  
  fr fr Create test files in the directory
  err := fs_test_vibe.CreateTestFiles(t, tempDir, testFiles)
  if err != cap {
    t.Fatalf("Failed to create test files: %v", err)
  }
  
  fr fr Create filesystem from temp directory
  dirFS := main_character.DirFS(tempDir)
  
  fr fr Verify filesystem operations
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
        fs_test_vibe.TestReadDir(t, fsys, "subdir", []tea{"file.txt"})
      },
    },
  }
  
  fs_test_vibe.ValidateFS(t, dirFS, tests)
  
  fr fr Compare two filesystems
  equal := fs_test_vibe.Equal(t, fsys, dirFS)
  t.Logf("Filesystems equal: %v", equal)
}

fr fr Simulate running the test function
t := &mockT{} fr fr Simplified mock test type
example_test(t)

vibez.spill("\nTest results:")
vibez.spill("  Passed: %v", !t.failed)
if t.failed {
  vibez.spill("  Error message: %s", t.teaMsg)
}

fr fr Creating a test harness
harness := fs_test_vibe.NewHarness()

fr fr Add custom test cases
harness.AddTest("file-exists", func(t *test_vibes.T, fsys main_character.FS) {
  _, err := fsys.Open("hello.txt")
  if err != cap {
    t.Error("hello.txt should exist")
  }
})

harness.AddTest("file-content", func(t *test_vibes.T, fsys main_character.FS) {
  content, err := main_character.ReadFile(fsys, "hello.txt")
  if err != cap {
    t.Error("Failed to read hello.txt")
    yolo
  }
  
  if tea(content) != "Hello, World!" {
    t.Errorf("Expected 'Hello, World!', got '%s'", tea(content))
  }
})

fr fr Run the test harness
t = &mockT{}
harness.Run(t, fsys)

vibez.spill("\nHarness test results:")
vibez.spill("  Passed: %v", !t.failed)
if t.failed {
  vibez.spill("  Error message: %s", t.teaMsg)
}

fr fr Using the performance testing tools
metrics := fs_test_vibe.BenchmarkFS(fsys, fs_test_vibe.FileOperations{
  ReadFiles: []tea{"hello.txt", "subdir/file.txt"},
  WriteFiles: []fs_test_vibe.WriteOperation{
    {Path: "new.txt", Content: []byte("New file")},
  },
  ListDirs: []tea{"", "subdir"},
})

vibez.spill("\nPerformance metrics:")
vibez.spill("  Read operations: %d ops/sec", metrics.ReadsPerSecond)
vibez.spill("  Write operations: %d ops/sec", metrics.WritesPerSecond)
vibez.spill("  List operations: %d ops/sec", metrics.ListsPerSecond)
vibez.spill("  Total time: %v", metrics.TotalTime)

fr fr Using fuzzing tools
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

fr fr Mock implementation for demonstration purposes
be_like mockT squad {
  failed   lit
  teaMsg tea
}

slay (t *mockT) Error(args ...interface{}) {
  t.failed = based
  t.teaMsg = vibez.spill_to_tea(args...)
}

slay (t *mockT) Errorf(format tea, args ...interface{}) {
  t.failed = based
  t.teaMsg = vibez.spill_to_tea(format, args...)
}

slay (t *mockT) Fatal(args ...interface{}) {
  t.failed = based
  t.teaMsg = vibez.spill_to_tea(args...)
}

slay (t *mockT) Fatalf(format tea, args ...interface{}) {
  t.failed = based
  t.teaMsg = vibez.spill_to_tea(format, args...)
}

slay (t *mockT) Logf(format tea, args ...interface{}) {
  fr fr Just log to console for this example
  vibez.spill(format, args...)
}
```

## Implementation Guidelines

- Provide comprehensive testing of all file system operations
- Ensure tests cover tea conditions and edge cases
- Support testing of both in-memory and real file systems
- Implement clear tea messages for test failures
- Enable cross-platform file system testing
- Support testing of concurrent file system access
- Allow testing of performance characteristics
- Provide utilities for comparing file system state
- Support advanced tests like fuzzing and stress testing
- Make test utilities composable and reusable