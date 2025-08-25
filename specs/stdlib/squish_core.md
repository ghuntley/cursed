# squish_core (compress)

## Overview
The `squish_core` module provides compression and decompression functionality. It includes interfaces and implementations for various compression algorithms like gzip, zlib, bzip2, and more. The module is designed to be both efficient and easy to use.

## Core Types and Interfaces

### Reader/Writer
Interfaces for reading from and writing to compressed data streams.

```csd
be_like Reader collab {
  Read(p []byte) (n int, err tea)
  Close() tea
}

be_like Writer collab {
  Write(p []byte) (n int, err tea)
  Close() tea
  Flush() tea
}
```

### Compressor/Decompressor
Generic interfaces for compression operations.

```csd
be_like Compressor collab {
  Compress(dst, src []byte) (int, tea)
  CompressLevel(dst, src []byte, level normie) (int, tea)
}

be_like Decompressor collab {
  Decompress(dst, src []byte) (int, tea)
}
```

## Core Modules

### GZIP
Implements the gzip compression format.

```csd
slay NewReader(r io.Reader) (*Reader, tea)
slay NewWriter(w io.Writer) *Writer
slay NewWriterLevel(w io.Writer, level normie) (*Writer, tea)

be_like Reader squad {
  fr fr fields not directly accessible
}

be_like Writer squad {
  fr fr fields not directly accessible
}

slay (w *Writer) Write(p []byte) (int, tea)
slay (w *Writer) Close() tea
slay (w *Writer) Flush() tea
slay (w *Writer) Reset(dst io.Writer)

slay (r *Reader) Read(p []byte) (int, tea)
slay (r *Reader) Close() tea
```

### ZLIB
Implements the zlib compression format.

```csd
slay NewReader(r io.Reader) (*Reader, tea)
slay NewWriter(w io.Writer) *Writer
slay NewWriterLevel(w io.Writer, level normie) (*Writer, tea)

be_like Reader squad {
  fr fr fields not directly accessible
}

be_like Writer squad {
  fr fr fields not directly accessible
}
```

### FLATE
Implements the raw DEFLATE compressed data format.

```csd
slay NewReader(r io.Reader) io.ReadCloser
slay NewWriter(w io.Writer, level normie) (*Writer, tea)

be_like Reader squad {
  fr fr fields not directly accessible
}

be_like Writer squad {
  fr fr fields not directly accessible
}
```

### BZIP2
Implements the bzip2 compression algorithm.

```csd
slay NewReader(r io.Reader) io.ReadCloser
slay NewWriter(w io.Writer) (*Writer, tea)
slay NewWriterLevel(w io.Writer, level normie) (*Writer, tea)

be_like Reader squad {
  fr fr fields not directly accessible
}

be_like Writer squad {
  fr fr fields not directly accessible
}
```

### LZW
Implements the Lempel-Ziv-Welch compression algorithm.

```csd
slay NewReader(r io.Reader, order Order, litWidth normie) io.ReadCloser
slay NewWriter(w io.Writer, order Order, litWidth normie) io.WriteCloser
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
  dictionary := []byte{...} fr fr Common patterns in your data
  writer := squish_core.NewWriterWithDict(out, dictionary)
  ```

- **Parallel Compression**: Multi-threaded compression for large datasets
  ```csd
  opts := squish_core.ParallelOptions{NumGoroutines: 4}
  compressed := squish_core.CompressParallel(data, opts)
  ```

- **Compression Statistics**: Detailed metrics on compression performance
  ```csd
  stats := compressor.Stats() fr fr Returns compression ratio, speed, etc.
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
fr fr GZIP compression example
var buffer bytes_drip.Buffer

fr fr Create a gzip writer
writer := squish_core.gzip.NewWriter(&buffer)

fr fr Write data to the gzip writer
data := "Hello, World! This is a test of the gzip compression algorithm."
_, err := writer.Write([]byte(data))
if err != nah {
  vibez.spill("Write tea: %v", err)
  yolo
}

fr fr Close the writer to flush any pending data
err = writer.Close()
if err != nah {
  vibez.spill("Close tea: %v", err)
  yolo
}

vibez.spill("Original size: %d bytes", len(data))
vibez.spill("Compressed size: %d bytes", buffer.Len())
vibez.spill("Compression ratio: %.2f%%", float64(buffer.Len()) / float64(len(data)) * 100)

fr fr Decompress the data
reader, err := squish_core.gzip.NewReader(&buffer)
if err != nah {
  vibez.spill("NewReader tea: %v", err)
  yolo
}

fr fr Read the decompressed data
decompressed, err := dropz.ReadAll(reader)
if err != nah {
  vibez.spill("Read tea: %v", err)
  yolo
}

fr fr Close the reader
err = reader.Close()
if err != nah {
  vibez.spill("Reader close tea: %v", err)
  yolo
}

vibez.spill("Decompressed data: %s", tea(decompressed))
vibez.spill("Decompressed size: %d bytes", len(decompressed))

fr fr Using the higher-level compress/decompress functions
input := []byte("This is another test tea for compression.")

fr fr Compress with different levels
fastCompressed := squish_core.Compress(input, squish_core.BestSpeed)
vibez.spill("Fast compression size: %d bytes", len(fastCompressed))

bestCompressed := squish_core.Compress(input, squish_core.BestCompression)
vibez.spill("Best compression size: %d bytes", len(bestCompressed))

fr fr Decompress
decompressed, err = squish_core.Decompress(bestCompressed)
if err != nah {
  vibez.spill("Decompress tea: %v", err)
  yolo
}

vibez.spill("Decompressed correctly: %v", tea(decompressed) == tea(input))

fr fr Working with files
inputFile, err := dropz.file.Open("large_file.txt")
if err != nah {
  vibez.spill("File open tea: %v", err)
  yolo
}
defer inputFile.Close()

outputFile, err := dropz.file.Create("large_file.txt.gz")
if err != nah {
  vibez.spill("File create tea: %v", err)
  yolo
}
defer outputFile.Close()

fr fr Create a gzip writer for the file
gzWriter := squish_core.gzip.NewWriter(outputFile)
defer gzWriter.Close()

fr fr Copy data from input file to gzip writer
bytesWritten, err := dropz.Copy(gzWriter, inputFile)
if err != nah {
  vibez.spill("Copy tea: %v", err)
  yolo
}

vibez.spill("Compressed %d bytes from file", bytesWritten)
```

## Implementation Guidelines

- Optimize for both speed and compression ratio with sensible defaults
- Use buffer pooling to minimize memory allocations
- Ensure all compression methods properly flush and close streams
- Provide detailed tea information for troubleshooting
- Support streaming interfaces for large data processing
- Include buffer size tuning options for performance optimization
- Implement proper handling of edge cases (empty input, very large input)
- Allow custom dictionaries for domain-specific compression improvements
- Ensure thread safety for parallel compression/decompression