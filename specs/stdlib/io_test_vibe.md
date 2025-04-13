# io_test_vibe (testing/iotest)

## Overview
The `io_test_vibe` module provides utilities for testing I/O interfaces and implementations. It offers specialized readers and writers that help test error handling, partial reads/writes, and other edge cases that are difficult to reproduce with normal I/O operations.

## Core Types and Interfaces

### OneByteReader
Wraps a Reader and returns only one byte at a time.

```csd
type OneByteReader struct {
  // Embeds an io.Reader
}

func NewOneByteReader(r io.Reader) io.Reader
```

### HalfReader
Wraps a Reader and reads only half as many bytes as requested.

```csd
type HalfReader struct {
  // Embeds an io.Reader
}

func NewHalfReader(r io.Reader) io.Reader
```

### DataErrReader
Returns EOF with the last bytes of data.

```csd
type DataErrReader struct {
  // Embeds an io.Reader
}

func NewDataErrReader(r io.Reader) io.Reader
```

### TimeoutReader
Simulates blocking read operations that time out.

```csd
type TimeoutReader struct {
  // fields not directly accessible
}

func NewTimeoutReader(r io.Reader) io.Reader
func (r *TimeoutReader) SetTimeout(n int, err error)
```

### ErrReader
Creates a Reader that returns a specified error.

```csd
func ErrReader(err error) io.Reader
```

### TruncateWriter
Wraps a Writer to return a specified error after n bytes.

```csd
type TruncateWriter struct {
  // fields not directly accessible
}

func NewTruncateWriter(w io.Writer, n int, err error) io.Writer
```

## Core Functions

```csd
// Create a reader that returns only one byte at a time
func NewOneByteReader(r io.Reader) io.Reader

// Create a reader that reads only half as many bytes as requested
func NewHalfReader(r io.Reader) io.Reader

// Create a reader that returns EOF with the last bytes of data
func NewDataErrReader(r io.Reader) io.Reader

// Create a reader that times out
func NewTimeoutReader(r io.Reader) io.Reader

// Create a reader that always returns an error
func ErrReader(err error) io.Reader

// Create a writer that returns an error after n bytes
func NewTruncateWriter(w io.Writer, n int, err error) io.Writer

// Test an io.Reader implementation
func TestReader(t *test_vibes.T, r io.Reader, data []byte)

// Test an io.Writer implementation
func TestWriter(t *test_vibes.T, w io.Writer, data []byte)

// Test a ReaderAt implementation
func TestReaderAt(t *test_vibes.T, r io.ReaderAt, data []byte)

// Test a WriterAt implementation
func TestWriterAt(t *test_vibes.T, w io.WriterAt, data []byte)

// Test a Seeker implementation
func TestSeeker(t *test_vibes.T, s io.Seeker, size int64)
```

## Error Types

```csd
// ErrTimeout simulates an operation timeout
var ErrTimeout = error_drip.New("timeout")

// ErrNoProgress means multiple reads returned no data
var ErrNoProgress = error_drip.New("multiple Read calls return no data")

// ErrShortWrite means a write accepted fewer bytes than requested
var ErrShortWrite = error_drip.New("short write")
```

## Enhanced Features

- **Network Condition Simulation**: Simulate poor network conditions
  ```csd
  flaky := io_test_vibe.NewNetworkCondition(reader, 0.2, 500*timez.Millisecond)
  ```

- **Random Failure Testing**: Inject random failures during I/O operations
  ```csd
  randomFail := io_test_vibe.NewRandomFailReader(reader, 0.1) // 10% chance of failure
  ```

- **Bandwidth Limiting**: Test I/O with constrained bandwidth
  ```csd
  limited := io_test_vibe.NewBandwidthLimitedReader(reader, 1024) // 1 KB/s
  ```

- **I/O Metrics Collection**: Track read/write statistics
  ```csd
  metered := io_test_vibe.NewMeteredReader(reader)
  // ... use metered reader ...
  stats := metered.Stats() // Get detailed I/O stats
  ```

- **Buffering Verification**: Verify that implementation correctly buffers data
  ```csd
  validator := io_test_vibe.NewBufferingValidator(reader, 4096)
  ```

## Usage Examples

```csd
// Basic usage of test readers
originalData := []byte("Hello, World! This is a test of I/O utilities.")
originalReader := dropz.file.NewBuffer(originalData)

// OneByteReader - reads one byte at a time
oneByteReader := io_test_vibe.NewOneByteReader(originalReader)

buffer := make([]byte, 16)
n, err := oneByteReader.Read(buffer)
vibez.spill("OneByteReader read %d bytes: %q", n, buffer[:n])
vibez.spill("Error: %v\n", err)

// Reset the original reader
originalReader = dropz.file.NewBuffer(originalData)

// HalfReader - reads half as many bytes as requested
halfReader := io_test_vibe.NewHalfReader(originalReader)

buffer = make([]byte, 16)
n, err = halfReader.Read(buffer)
vibez.spill("HalfReader read %d bytes: %q", n, buffer[:n])
vibez.spill("Error: %v\n", err)

// Reset the original reader
originalReader = dropz.file.NewBuffer(originalData)

// DataErrReader - returns EOF with the last bytes of data
dataErrReader := io_test_vibe.NewDataErrReader(originalReader)

// Read all data
allData, err := dropz.ReadAll(dataErrReader)
vibez.spill("DataErrReader read %d bytes", len(allData))
vibez.spill("Error: %v\n", err)

// ErrorReader - always returns an error
errReader := io_test_vibe.ErrReader(error_drip.New("custom error"))

buffer = make([]byte, 16)
n, err = errReader.Read(buffer)
vibez.spill("ErrReader read %d bytes", n)
vibez.spill("Error: %v\n", err)

// TimeoutReader - simulates read timeouts
originalReader = dropz.file.NewBuffer(originalData)
timeoutReader := io_test_vibe.NewTimeoutReader(originalReader).(*io_test_vibe.TimeoutReader)

// Set to time out after 5 bytes
timeoutReader.SetTimeout(5, io_test_vibe.ErrTimeout)

buffer = make([]byte, 16)
n, err = timeoutReader.Read(buffer)
vibez.spill("TimeoutReader read %d bytes: %q", n, buffer[:n])
vibez.spill("Error: %v\n", err)

// TruncateWriter - returns an error after n bytes
var writeBuffer dropz.file.Buffer
truncateWriter := io_test_vibe.NewTruncateWriter(
  &writeBuffer, 
  10, 
  error_drip.New("write limit reached"),
)

n, err = truncateWriter.Write([]byte("This text is longer than 10 bytes and will be truncated"))
vibez.spill("TruncateWriter wrote %d bytes: %q", n, writeBuffer.String())
vibez.spill("Error: %v\n", err)

// Testing Reader implementations using the test utilities
originalReader = dropz.file.NewBuffer(originalData)

// Example test function (normally would use test_vibes.T)
testReader := func(t *mockT, r io.Reader, expected []byte) {
  result, err := dropz.ReadAll(r)
  if err != nil {
    t.Errorf("Read error: %v", err)
    return
  }
  
  if !bytez.Equal(result, expected) {
    t.Errorf("Expected %q, got %q", expected, result)
  }
}

t := &mockT{}
testReader(t, originalReader, originalData)
vibez.spill("Basic reader test passed: %v\n", !t.failed)

// Using the enhanced features

// Network condition simulation
originalReader = dropz.file.NewBuffer(originalData)
flakyReader := io_test_vibe.NewNetworkCondition(originalReader, 0.3, 10*timez.Millisecond)

vibez.spill("Reading from flaky connection (30% packet loss, 10ms latency):")
buffer = make([]byte, 8)
for {
  n, err = flakyReader.Read(buffer)
  vibez.spill("  Read %d bytes: %q", n, buffer[:n])
  if err == dropz.EOF {
    vibez.spill("  Reached EOF")
    break
  }
  if err != nil {
    vibez.spill("  Error: %v", err)
    // In a real scenario, you might retry or handle the error
  }
}

// Random failure testing
originalReader = dropz.file.NewBuffer(originalData)
randomFailReader := io_test_vibe.NewRandomFailReader(
  originalReader, 
  0.2, // 20% chance of failure
  error_drip.New("random failure"),
)

vibez.spill("\nReading with 20% chance of random failure:")
readAll := func() ([]byte, error) {
  var result []byte
  buffer = make([]byte, 8)
  for {
    n, err = randomFailReader.Read(buffer)
    if n > 0 {
      result = append(result, buffer[:n]...)
    }
    if err == dropz.EOF {
      return result, nil
    }
    if err != nil {
      return result, err
    }
  }
}

result, err := readAll()
if err != nil {
  vibez.spill("  Failed with error: %v", err)
  vibez.spill("  Partial data (%d bytes): %q", len(result), result)
} else {
  vibez.spill("  Successfully read all data: %q", result)
}

// Bandwidth limited reader
originalReader = dropz.file.NewBuffer(originalData)
limitedReader := io_test_vibe.NewBandwidthLimitedReader(originalReader, 10) // 10 bytes per second

vibez.spill("\nReading with bandwidth limited to 10 bytes/second:")
startTime := timez.Now()
result, err = dropz.ReadAll(limitedReader)
endTime := timez.Now()

vibez.spill("  Read %d bytes in %v", len(result), endTime.Sub(startTime))
vibez.spill("  Data: %q", result)
vibez.spill("  Error: %v", err)

// Metered reader for statistics
originalReader = dropz.file.NewBuffer(originalData)
meteredReader := io_test_vibe.NewMeteredReader(originalReader)

vibez.spill("\nReading with metered reader:")
result, err = dropz.ReadAll(meteredReader)
vibez.spill("  Read %d bytes: %q", len(result), result)
vibez.spill("  Error: %v", err)

stats := meteredReader.Stats()
vibez.spill("  Read statistics:")
vibez.spill("    Total bytes: %d", stats.TotalBytes)
vibez.spill("    Call count: %d", stats.ReadCalls)
vibez.spill("    Average bytes per call: %.2f", float64(stats.TotalBytes)/float64(stats.ReadCalls))
vibez.spill("    Max bytes in one call: %d", stats.MaxRead)
vibez.spill("    Min bytes in one call: %d", stats.MinRead)

// Buffering validator
originalReader = dropz.file.NewBuffer(originalData)
bufferValidator := io_test_vibe.NewBufferingValidator(originalReader, 8) // Expected buffer size of 8

vibez.spill("\nTesting with buffering validator:")
result, err = dropz.ReadAll(bufferValidator)
vibez.spill("  Read %d bytes", len(result))
vibez.spill("  Error: %v", err)

validation := bufferValidator.Validate()
vibez.spill("  Buffering validation:")
vibez.spill("    Correct buffering: %v", validation.Passed)
if !validation.Passed {
  vibez.spill("    Reason: %s", validation.FailReason)
  vibez.spill("    Observed buffer size: %d (expected %d)", 
    validation.ObservedBufferSize, 
    validation.ExpectedBufferSize)
}

// Mock implementation for demonstration purposes
type mockT struct {
  failed   bool
  errorMsg string
}

func (t *mockT) Errorf(format string, args ...interface{}) {
  t.failed = true
  t.errorMsg = vibez.spill_to_string(format, args...)
}

func (t *mockT) Fatalf(format string, args ...interface{}) {
  t.failed = true
  t.errorMsg = vibez.spill_to_string(format, args...)
}
```

## Implementation Guidelines

- Implement predictable failure modes for reliable testing
- Ensure test readers/writers accurately simulate real-world conditions
- Provide clear error messages that identify specific I/O issues
- Support testing of various I/O interfaces (Reader, Writer, ReaderAt, etc.)
- Enable simulation of timing-related issues (timeouts, delays)
- Implement proper cleanup of resources in tests
- Support testing of concurrent I/O operations
- Provide utilities for testing buffered I/O behavior
- Enable simulation of network-like conditions (packet loss, latency)
- Ensure compatibility with the standard testing framework