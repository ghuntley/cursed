# squish_core (compress)

## Overview
The `squish_core` module provides compression and decompression functionality. It includes interfaces and implementations for various compression algorithms like gzip, zlib, bzip2, and more. The module is designed to be both efficient and easy to use.

## Core Types and Interfaces

### Reader/Writer
Interfaces for reading from and writing to compressed data streams.

```csd
type Reader interface {
  Read(p []byte) (n int, err error)
  Close() error
}

type Writer interface {
  Write(p []byte) (n int, err error)
  Close() error
  Flush() error
}
```

### Compressor/Decompressor
Generic interfaces for compression operations.

```csd
type Compressor interface {
  Compress(dst, src []byte) (int, error)
  CompressLevel(dst, src []byte, level int) (int, error)
}

type Decompressor interface {
  Decompress(dst, src []byte) (int, error)
}
```

## Core Modules

### GZIP
Implements the gzip compression format.

```csd
func NewReader(r io.Reader) (*Reader, error)
func NewWriter(w io.Writer) *Writer
func NewWriterLevel(w io.Writer, level int) (*Writer, error)

type Reader struct {
  // fields not directly accessible
}

type Writer struct {
  // fields not directly accessible
}

func (w *Writer) Write(p []byte) (int, error)
func (w *Writer) Close() error
func (w *Writer) Flush() error
func (w *Writer) Reset(dst io.Writer)

func (r *Reader) Read(p []byte) (int, error)
func (r *Reader) Close() error
```

### ZLIB
Implements the zlib compression format.

```csd
func NewReader(r io.Reader) (*Reader, error)
func NewWriter(w io.Writer) *Writer
func NewWriterLevel(w io.Writer, level int) (*Writer, error)

type Reader struct {
  // fields not directly accessible
}

type Writer struct {
  // fields not directly accessible
}
```

### FLATE
Implements the raw DEFLATE compressed data format.

```csd
func NewReader(r io.Reader) io.ReadCloser
func NewWriter(w io.Writer, level int) (*Writer, error)

type Reader struct {
  // fields not directly accessible
}

type Writer struct {
  // fields not directly accessible
}
```

### BZIP2
Implements the bzip2 compression algorithm.

```csd
func NewReader(r io.Reader) io.ReadCloser
func NewWriter(w io.Writer) (*Writer, error)
func NewWriterLevel(w io.Writer, level int) (*Writer, error)

type Reader struct {
  // fields not directly accessible
}

type Writer struct {
  // fields not directly accessible
}
```

### LZW
Implements the Lempel-Ziv-Welch compression algorithm.

```csd
func NewReader(r io.Reader, order Order, litWidth int) io.ReadCloser
func NewWriter(w io.Writer, order Order, litWidth int) io.WriteCloser
```

## Compression Levels and Constants

```csd
const (
  NoCompression      = 0
  BestSpeed          = 1
  BestCompression    = 9
  DefaultCompression = -1
  HuffmanOnly        = -2
)
```

## Enhanced Features

- **Adaptive Compression**: Automatically selects the best algorithm
  ```csd
  compressor := squish_core.NewAdaptiveCompressor(data)
  compressed := compressor.Compress()
  ```

- **Dictionary-Based Compression**: Improved compression with predefined dictionaries
  ```csd
  dictionary := []byte{...} // Common patterns in your data
  writer := squish_core.NewWriterWithDict(out, dictionary)
  ```

- **Parallel Compression**: Multi-threaded compression for large datasets
  ```csd
  opts := squish_core.ParallelOptions{NumGoroutines: 4}
  compressed := squish_core.CompressParallel(data, opts)
  ```

- **Compression Statistics**: Detailed metrics on compression performance
  ```csd
  stats := compressor.Stats() // Returns compression ratio, speed, etc.
  ```

- **Progressive Compression**: Stream processing with incremental updates
  ```csd
  stream := squish_core.NewProgressiveCompressor()
  stream.AddChunk(chunk1)
  stream.AddChunk(chunk2)
  result := stream.Finalize()
  ```

## Usage Examples

```csd
// GZIP compression example
var buffer bytes_drip.Buffer

// Create a gzip writer
writer := squish_core.gzip.NewWriter(&buffer)

// Write data to the gzip writer
data := "Hello, World! This is a test of the gzip compression algorithm."
_, err := writer.Write([]byte(data))
if err != nil {
  vibez.spill("Write error: %v", err)
  return
}

// Close the writer to flush any pending data
err = writer.Close()
if err != nil {
  vibez.spill("Close error: %v", err)
  return
}

vibez.spill("Original size: %d bytes", len(data))
vibez.spill("Compressed size: %d bytes", buffer.Len())
vibez.spill("Compression ratio: %.2f%%", float64(buffer.Len()) / float64(len(data)) * 100)

// Decompress the data
reader, err := squish_core.gzip.NewReader(&buffer)
if err != nil {
  vibez.spill("NewReader error: %v", err)
  return
}

// Read the decompressed data
decompressed, err := dropz.ReadAll(reader)
if err != nil {
  vibez.spill("Read error: %v", err)
  return
}

// Close the reader
err = reader.Close()
if err != nil {
  vibez.spill("Reader close error: %v", err)
  return
}

vibez.spill("Decompressed data: %s", string(decompressed))
vibez.spill("Decompressed size: %d bytes", len(decompressed))

// Using the higher-level compress/decompress functions
input := []byte("This is another test string for compression.")

// Compress with different levels
fastCompressed := squish_core.Compress(input, squish_core.BestSpeed)
vibez.spill("Fast compression size: %d bytes", len(fastCompressed))

bestCompressed := squish_core.Compress(input, squish_core.BestCompression)
vibez.spill("Best compression size: %d bytes", len(bestCompressed))

// Decompress
decompressed, err = squish_core.Decompress(bestCompressed)
if err != nil {
  vibez.spill("Decompress error: %v", err)
  return
}

vibez.spill("Decompressed correctly: %v", string(decompressed) == string(input))

// Working with files
inputFile, err := dropz.file.Open("large_file.txt")
if err != nil {
  vibez.spill("File open error: %v", err)
  return
}
defer inputFile.Close()

outputFile, err := dropz.file.Create("large_file.txt.gz")
if err != nil {
  vibez.spill("File create error: %v", err)
  return
}
defer outputFile.Close()

// Create a gzip writer for the file
gzWriter := squish_core.gzip.NewWriter(outputFile)
defer gzWriter.Close()

// Copy data from input file to gzip writer
bytesWritten, err := dropz.Copy(gzWriter, inputFile)
if err != nil {
  vibez.spill("Copy error: %v", err)
  return
}

vibez.spill("Compressed %d bytes from file", bytesWritten)
```

## Implementation Guidelines

- Optimize for both speed and compression ratio with sensible defaults
- Use buffer pooling to minimize memory allocations
- Ensure all compression methods properly flush and close streams
- Provide detailed error information for troubleshooting
- Support streaming interfaces for large data processing
- Include buffer size tuning options for performance optimization
- Implement proper handling of edge cases (empty input, very large input)
- Allow custom dictionaries for domain-specific compression improvements
- Ensure thread safety for parallel compression/decompression