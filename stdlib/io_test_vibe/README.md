# io_test_vibe

I/O testing utilities for comprehensive testing of I/O interfaces.

## Overview

The `io_test_vibe` module provides specialized readers and writers that help test error handling, partial reads/writes, and other edge cases that are difficult to reproduce with normal I/O operations.

## Core Test Readers

### OneByteReader
```cursed
reader := io_test_vibe.NewOneByteReader("hello world")
buffer := make([]byte, 5)
n, err := reader.Read(buffer)  // Reads exactly 1 byte
```

### HalfReader
```cursed
reader := io_test_vibe.NewHalfReader("hello world")
buffer := make([]byte, 8)
n, err := reader.Read(buffer)  // Reads 4 bytes (half of requested)
```

### TimeoutReader
```cursed
reader := io_test_vibe.NewTimeoutReader("hello world")
reader.SetTimeout(5, io_test_vibe.ErrTimeout)  // Timeout after 5 bytes
```

### ErrReader
```cursed
reader := io_test_vibe.NewErrReader("custom error")
n, err := reader.Read(buffer)  // Always returns the specified error
```

## Core Test Writers

### TruncateWriter
```cursed
writer := io_test_vibe.NewTruncateWriter(10, io_test_vibe.ErrShortWrite)
n, err := writer.Write(data)  // Errors after writing 10 bytes
```

## Enhanced Testing Features

### Network Condition Simulation
```cursed
flaky := io_test_vibe.NewNetworkCondition(reader, 0.2, 500)  // 20% packet loss, 500ms latency
```

### Random Failure Testing
```cursed
randomFail := io_test_vibe.NewRandomFailReader(reader, 0.1, "random error")  // 10% failure rate
```

### Bandwidth Limiting
```cursed
limited := io_test_vibe.NewBandwidthLimitedReader(reader, 1024)  // 1KB/s limit
```

### I/O Metrics Collection
```cursed
metered := io_test_vibe.NewMeteredReader(reader)
// ... use reader ...
stats := metered.Stats()  // Get detailed statistics
```

### Buffering Validation
```cursed
validator := io_test_vibe.NewBufferingValidator(reader, 4096)  // Expected 4KB buffer
result := validator.Validate()  // Check buffering behavior
```

## Utility Functions

- `ReadAll(reader)` - Read all data from any test reader
- `TestReader(input, expected)` - Test reader implementation
- `TestWriter(data)` - Test writer implementation
- `CreateTestData(size)` - Generate test data patterns
- `VerifyRead(reader, expected)` - Verify read operations
- `VerifyWrite(writer, data)` - Verify write operations

## Error Constants

- `ErrTimeout` - Simulates operation timeout
- `ErrNoProgress` - Multiple reads with no progress
- `ErrShortWrite` - Write accepted fewer bytes than requested

## Testing Patterns

Use these utilities to test:
- Error handling in I/O operations
- Partial read/write scenarios
- Timeout and network conditions
- Buffer management and optimization
- Concurrent I/O operations
- I/O performance characteristics
