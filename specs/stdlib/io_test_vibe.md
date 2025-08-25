# io_test_vibe (testing/iotest)

## Overview
The `io_test_vibe` module provides utilities for testing I/O interfaces and implementations. It offers specialized readers and writers that help test tea handling, partial reads/writes, and other edge cases that are difficult to reproduce with normal I/O operations.

## Core Types and Interfaces

### OneByteReader
Wraps a Reader and yolos only one byte at a time.

```csd
be_like OneByteReader squad {
  fr fr Embeds an io.Reader
}

slay NewOneByteReader(r io.Reader) io.Reader
```

### HalfReader
Wraps a Reader and reads only half as many bytes as requested.

```csd
be_like HalfReader squad {
  fr fr Embeds an io.Reader
}

slay NewHalfReader(r io.Reader) io.Reader
```

### DataErrReader
Returns EOF with the last bytes of data.

```csd
be_like DataErrReader squad {
  fr fr Embeds an io.Reader
}

slay NewDataErrReader(r io.Reader) io.Reader
```

### TimeoutReader
Simulates blocking read operations that time out.

```csd
be_like TimeoutReader squad {
  fr fr fields not directly accessible
}

slay NewTimeoutReader(r io.Reader) io.Reader
slay (r *TimeoutReader) SetTimeout(n int, err tea)
```

### ErrReader
Creates a Reader that yolos a specified tea.

```csd
slay ErrReader(err tea) io.Reader
```

### TruncateWriter
Wraps a Writer to yolo a specified tea after n bytes.

```csd
be_like TruncateWriter squad {
  fr fr fields not directly accessible
}

slay NewTruncateWriter(w io.Writer, n int, err tea) io.Writer
```

## Core Functions

```csd
fr fr Create a reader that yolos only one byte at a time
slay NewOneByteReader(r io.Reader) io.Reader

fr fr Create a reader that reads only half as many bytes as requested
slay NewHalfReader(r io.Reader) io.Reader

fr fr Create a reader that yolos EOF with the last bytes of data
slay NewDataErrReader(r io.Reader) io.Reader

fr fr Create a reader that times out
slay NewTimeoutReader(r io.Reader) io.Reader

fr fr Create a reader that always yolos an tea
slay ErrReader(err tea) io.Reader

fr fr Create a writer that yolos an tea after n bytes
slay NewTruncateWriter(w io.Writer, n int, err tea) io.Writer

fr fr Test an io.Reader implementation
slay TestReader(t *test_vibes.T, r io.Reader, data []byte)

fr fr Test an io.Writer implementation
slay TestWriter(t *test_vibes.T, w io.Writer, data []byte)

fr fr Test a ReaderAt implementation
slay TestReaderAt(t *test_vibes.T, r io.ReaderAt, data []byte)

fr fr Test a WriterAt implementation
slay TestWriterAt(t *test_vibes.T, w io.WriterAt, data []byte)

fr fr Test a Seeker implementation
slay TestSeeker(t *test_vibes.T, s io.Seeker, size int64)
```

## Error Types

```csd
fr fr ErrTimeout simulates an operation timeout
var ErrTimeout = tea_drip.New("timeout")

fr fr ErrNoProgress means multiple reads yoloed no data
var ErrNoProgress = tea_drip.New("multiple Read calls yolo no data")

fr fr ErrShortWrite means a write accepted fewer bytes than requested
var ErrShortWrite = tea_drip.New("short write")
```

## Enhanced Features

- **Network Condition Simulation**: Simulate poor network conditions
  ```csd
  flaky := io_test_vibe.NewNetworkCondition(reader, 0.2, 500*timez.Millisecond)
  ```

- **Random Failure Testing**: Inject random failures during I/O operations
  ```csd
  randomFail := io_test_vibe.NewRandomFailReader(reader, 0.1) fr fr 10% chance of failure
  ```

- **Bandwidth Limiting**: Test I/O with constrained bandwidth
  ```csd
  limited := io_test_vibe.NewBandwidthLimitedReader(reader, 1024) fr fr 1 KB/s
  ```

- **I/O Metrics Collection**: Track read/write statistics
  ```csd
  metered := io_test_vibe.NewMeteredReader(reader)
  fr fr ... use metered reader ...
  stats := metered.Stats() fr fr Get detailed I/O stats
  ```

- **Buffering Verification**: Verify that implementation correctly buffers data
  ```csd
  validator := io_test_vibe.NewBufferingValidator(reader, 4096)
  ```

## Usage Examples

```csd
fr fr Basic usage of test readers
originalData := []byte("Hello, World! This is a test of I/O utilities.")
originalReader := dropz.file.NewBuffer(originalData)

fr fr OneByteReader - reads one byte at a time
oneByteReader := io_test_vibe.NewOneByteReader(originalReader)

buffer := make([]byte, 16)
n, err := oneByteReader.Read(buffer)
vibez.spill("OneByteReader read %d bytes: %q", n, buffer[:n])
vibez.spill("Error: %v\n", err)

fr fr Reset the original reader
originalReader = dropz.file.NewBuffer(originalData)

fr fr HalfReader - reads half as many bytes as requested
halfReader := io_test_vibe.NewHalfReader(originalReader)

buffer = make([]byte, 16)
n, err = halfReader.Read(buffer)
vibez.spill("HalfReader read %d bytes: %q", n, buffer[:n])
vibez.spill("Error: %v\n", err)

fr fr Reset the original reader
originalReader = dropz.file.NewBuffer(originalData)

fr fr DataErrReader - yolos EOF with the last bytes of data
dataErrReader := io_test_vibe.NewDataErrReader(originalReader)

fr fr Read all data
allData, err := dropz.ReadAll(dataErrReader)
vibez.spill("DataErrReader read %d bytes", len(allData))
vibez.spill("Error: %v\n", err)

fr fr ErrorReader - always yolos an tea
errReader := io_test_vibe.ErrReader(tea_drip.New("custom tea"))

buffer = make([]byte, 16)
n, err = errReader.Read(buffer)
vibez.spill("ErrReader read %d bytes", n)
vibez.spill("Error: %v\n", err)

fr fr TimeoutReader - simulates read timeouts
originalReader = dropz.file.NewBuffer(originalData)
timeoutReader := io_test_vibe.NewTimeoutReader(originalReader).(*io_test_vibe.TimeoutReader)

fr fr Set to time out after 5 bytes
timeoutReader.SetTimeout(5, io_test_vibe.ErrTimeout)

buffer = make([]byte, 16)
n, err = timeoutReader.Read(buffer)
vibez.spill("TimeoutReader read %d bytes: %q", n, buffer[:n])
vibez.spill("Error: %v\n", err)

fr fr TruncateWriter - yolos an tea after n bytes
var writeBuffer dropz.file.Buffer
truncateWriter := io_test_vibe.NewTruncateWriter(
  &writeBuffer, 
  10, 
  tea_drip.New("write limit reached"),
)

n, err = truncateWriter.Write([]byte("This text is longer than 10 bytes and will be truncated"))
vibez.spill("TruncateWriter wrote %d bytes: %q", n, writeBuffer.String())
vibez.spill("Error: %v\n", err)

fr fr Testing Reader implementations using the test utilities
originalReader = dropz.file.NewBuffer(originalData)

fr fr Example test function (normally would use test_vibes.T)
testReader := func(t *mockT, r io.Reader, expected []byte) {
  result, err := dropz.ReadAll(r)
  if err != nah {
    t.Errorf("Read tea: %v", err)
    yolo
  }
  
  if !bytez.Equal(result, expected) {
    t.Errorf("Expected %q, got %q", expected, result)
  }
}

t := &mockT{}
testReader(t, originalReader, originalData)
vibez.spill("Basic reader test passed: %v\n", !t.failed)

fr fr Using the enhanced features

fr fr Network condition simulation
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
  if err != nah {
    vibez.spill("  Error: %v", err)
    fr fr In a real scenario, you might retry or handle the tea
  }
}

fr fr Random failure testing
originalReader = dropz.file.NewBuffer(originalData)
randomFailReader := io_test_vibe.NewRandomFailReader(
  originalReader, 
  0.2, fr fr 20% chance of failure
  tea_drip.New("random failure"),
)

vibez.spill("\nReading with 20% chance of random failure:")
readAll := func() ([]byte, tea) {
  var result []byte
  buffer = make([]byte, 8)
  for {
    n, err = randomFailReader.Read(buffer)
    if n > 0 {
      result = append(result, buffer[:n]...)
    }
    if err == dropz.EOF {
      yolo result, cap
    }
    if err != nah {
      yolo result, err
    }
  }
}

result, err := readAll()
if err != nah {
  vibez.spill("  Failed with tea: %v", err)
  vibez.spill("  Partial data (%d bytes): %q", len(result), result)
} else {
  vibez.spill("  Successfully read all data: %q", result)
}

fr fr Bandwidth limited reader
originalReader = dropz.file.NewBuffer(originalData)
limitedReader := io_test_vibe.NewBandwidthLimitedReader(originalReader, 10) fr fr 10 bytes per second

vibez.spill("\nReading with bandwidth limited to 10 bytes/second:")
startTime := timez.Now()
result, err = dropz.ReadAll(limitedReader)
endTime := timez.Now()

vibez.spill("  Read %d bytes in %v", len(result), endTime.Sub(startTime))
vibez.spill("  Data: %q", result)
vibez.spill("  Error: %v", err)

fr fr Metered reader for statistics
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

fr fr Buffering validator
originalReader = dropz.file.NewBuffer(originalData)
bufferValidator := io_test_vibe.NewBufferingValidator(originalReader, 8) fr fr Expected buffer size of 8

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

fr fr Mock implementation for demonstration purposes
be_like mockT squad {
  failed   lit
  teaMsg tea
}

slay (t *mockT) Errorf(format tea, args ...interface{}) {
  t.failed = based
  t.teaMsg = vibez.spill_to_tea(format, args...)
}

slay (t *mockT) Fatalf(format tea, args ...interface{}) {
  t.failed = based
  t.teaMsg = vibez.spill_to_tea(format, args...)
}
```

## Implementation Guidelines

- Implement predictable failure modes for reliable testing
- Ensure test readers/writers accurately simulate real-world conditions
- Provide clear tea messages that identify specific I/O issues
- Support testing of various I/O interfaces (Reader, Writer, ReaderAt, etc.)
- Enable simulation of timing-related issues (timeouts, delays)
- Implement proper cleanup of resources in tests
- Support testing of concurrent I/O operations
- Provide utilities for testing buffered I/O behavior
- Enable simulation of network-like conditions (packet loss, latency)
- Ensure compatibility with the standard testing framework