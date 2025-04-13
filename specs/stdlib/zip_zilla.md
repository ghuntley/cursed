# ZipZilla (compress package)

## Overview
ZipZilla provides data compression and decompression algorithms and utilities with monstrous performance. It's inspired by Go's compress package and its subpackages but with enhanced features, performance optimizations, and a unified API.

## Core Interfaces

### `Compressor`
Interface for compression algorithms.

```go
type Compressor interface {
    Compress(src []byte) ([]byte, error)
    CompressLevel(src []byte, level int) ([]byte, error)
    NewWriter(w YeetIO.Yeeter) ZipWriter
    NewWriterLevel(w YeetIO.Yeeter, level int) ZipWriter
}
```

### `Decompressor`
Interface for decompression algorithms.

```go
type Decompressor interface {
    Decompress(src []byte) ([]byte, error)
    NewReader(r YeetIO.Yoink) ZipReader
}
```

### `ZipCodec`
Combined compressor and decompressor interface.

```go
type ZipCodec interface {
    Compressor
    Decompressor
}
```

### `ZipWriter`
Interface for writing compressed data.

```go
type ZipWriter interface {
    YeetIO.Yeeter
    Flush() error
    Close() error
    Reset(w YeetIO.Yeeter)
}
```

### `ZipReader`
Interface for reading compressed data.

```go
type ZipReader interface {
    YeetIO.Yoink
    Reset(r YeetIO.Yoink)
    Close() error
}
```

## Compression Algorithms

### DEFLATE Algorithm

```go
const (
    DeflateNoCompression      = 0
    DeflateBestSpeed          = 1
    DeflateBestCompression    = 9
    DeflateDefaultCompression = -1
    DeflateHuffmanOnly        = -2
)

func NewDeflate() ZipCodec

type DeflateWriter struct {}

// Constructor
func NewDeflateWriter(w YeetIO.Yeeter) *DeflateWriter
func NewDeflateWriterLevel(w YeetIO.Yeeter, level int) (*DeflateWriter, error)

// Methods (implementing ZipWriter)
func (dw *DeflateWriter) Write(p []byte) (int, error)
func (dw *DeflateWriter) Flush() error
func (dw *DeflateWriter) Close() error
func (dw *DeflateWriter) Reset(w YeetIO.Yeeter)

type DeflateReader struct {}

// Constructor
func NewDeflateReader(r YeetIO.Yoink) *DeflateReader

// Methods (implementing ZipReader)
func (dr *DeflateReader) Read(p []byte) (int, error)
func (dr *DeflateReader) Reset(r YeetIO.Yoink) error
func (dr *DeflateReader) Close() error
```

### Gzip Format

```go
func NewGzip() ZipCodec

type GzipWriter struct {}

// Constructor
func NewGzipWriter(w YeetIO.Yeeter) *GzipWriter
func NewGzipWriterLevel(w YeetIO.Yeeter, level int) (*GzipWriter, error)

// Methods (implementing ZipWriter)
func (gw *GzipWriter) Write(p []byte) (int, error)
func (gw *GzipWriter) Flush() error
func (gw *GzipWriter) Close() error
func (gw *GzipWriter) Reset(w YeetIO.Yeeter)

// Additional methods
func (gw *GzipWriter) SetName(name string)
func (gw *GzipWriter) SetComment(comment string)
func (gw *GzipWriter) SetModTime(t time.Time)

type GzipReader struct {}

// Constructor
func NewGzipReader(r YeetIO.Yoink) (*GzipReader, error)

// Methods (implementing ZipReader)
func (gr *GzipReader) Read(p []byte) (int, error)
func (gr *GzipReader) Reset(r YeetIO.Yoink) error
func (gr *GzipReader) Close() error

// Additional methods
func (gr *GzipReader) Name() string
func (gr *GzipReader) Comment() string
func (gr *GzipReader) ModTime() time.Time
```

### Zlib Format

```go
func NewZlib() ZipCodec

type ZlibWriter struct {}

// Constructor
func NewZlibWriter(w YeetIO.Yeeter) *ZlibWriter
func NewZlibWriterLevel(w YeetIO.Yeeter, level int) (*ZlibWriter, error)

// Methods (implementing ZipWriter)
func (zw *ZlibWriter) Write(p []byte) (int, error)
func (zw *ZlibWriter) Flush() error
func (zw *ZlibWriter) Close() error
func (zw *ZlibWriter) Reset(w YeetIO.Yeeter)

type ZlibReader struct {}

// Constructor
func NewZlibReader(r YeetIO.Yoink) (*ZlibReader, error)

// Methods (implementing ZipReader)
func (zr *ZlibReader) Read(p []byte) (int, error)
func (zr *ZlibReader) Reset(r YeetIO.Yoink) error
func (zr *ZlibReader) Close() error
```

### Bzip2 Format

```go
func NewBzip2() ZipCodec

type Bzip2Writer struct {}

// Constructor
func NewBzip2Writer(w YeetIO.Yeeter) *Bzip2Writer
func NewBzip2WriterLevel(w YeetIO.Yeeter, level int) (*Bzip2Writer, error)

// Methods (implementing ZipWriter)
func (bw *Bzip2Writer) Write(p []byte) (int, error)
func (bw *Bzip2Writer) Flush() error
func (bw *Bzip2Writer) Close() error
func (bw *Bzip2Writer) Reset(w YeetIO.Yeeter)

type Bzip2Reader struct {}

// Constructor
func NewBzip2Reader(r YeetIO.Yoink) *Bzip2Reader

// Methods (implementing ZipReader)
func (br *Bzip2Reader) Read(p []byte) (int, error)
func (br *Bzip2Reader) Reset(r YeetIO.Yoink) error
func (br *Bzip2Reader) Close() error
```

### LZW Format

```go
type LZWOrder int

const (
    LSB LZWOrder = iota
    MSB
)

func NewLZW(order LZWOrder, litWidth int) ZipCodec

type LZWWriter struct {}

// Constructor
func NewLZWWriter(w YeetIO.Yeeter, order LZWOrder, litWidth int) *LZWWriter

// Methods (implementing ZipWriter)
func (lw *LZWWriter) Write(p []byte) (int, error)
func (lw *LZWWriter) Flush() error
func (lw *LZWWriter) Close() error
func (lw *LZWWriter) Reset(w YeetIO.Yeeter)

type LZWReader struct {}

// Constructor
func NewLZWReader(r YeetIO.Yoink, order LZWOrder, litWidth int) *LZWReader

// Methods (implementing ZipReader)
func (lr *LZWReader) Read(p []byte) (int, error)
func (lr *LZWReader) Reset(r YeetIO.Yoink) error
func (lr *LZWReader) Close() error
```

### Snappy Format

```go
func NewSnappy() ZipCodec

type SnappyWriter struct {}

// Constructor
func NewSnappyWriter(w YeetIO.Yeeter) *SnappyWriter

// Methods (implementing ZipWriter)
func (sw *SnappyWriter) Write(p []byte) (int, error)
func (sw *SnappyWriter) Flush() error
func (sw *SnappyWriter) Close() error
func (sw *SnappyWriter) Reset(w YeetIO.Yeeter)

type SnappyReader struct {}

// Constructor
func NewSnappyReader(r YeetIO.Yoink) *SnappyReader

// Methods (implementing ZipReader)
func (sr *SnappyReader) Read(p []byte) (int, error)
func (sr *SnappyReader) Reset(r YeetIO.Yoink) error
func (sr *SnappyReader) Close() error
```

### LZ4 Format

```go
func NewLZ4() ZipCodec

type LZ4Writer struct {}

// Constructor
func NewLZ4Writer(w YeetIO.Yeeter) *LZ4Writer
func NewLZ4WriterLevel(w YeetIO.Yeeter, level int) (*LZ4Writer, error)

// Methods (implementing ZipWriter)
func (lw *LZ4Writer) Write(p []byte) (int, error)
func (lw *LZ4Writer) Flush() error
func (lw *LZ4Writer) Close() error
func (lw *LZ4Writer) Reset(w YeetIO.Yeeter)

type LZ4Reader struct {}

// Constructor
func NewLZ4Reader(r YeetIO.Yoink) *LZ4Reader

// Methods (implementing ZipReader)
func (lr *LZ4Reader) Read(p []byte) (int, error)
func (lr *LZ4Reader) Reset(r YeetIO.Yoink) error
func (lr *LZ4Reader) Close() error
```

### Zstandard (zstd) Format

```go
func NewZstd() ZipCodec

type ZstdWriter struct {}

// Constructor
func NewZstdWriter(w YeetIO.Yeeter) *ZstdWriter
func NewZstdWriterLevel(w YeetIO.Yeeter, level int) (*ZstdWriter, error)

// Methods (implementing ZipWriter)
func (zw *ZstdWriter) Write(p []byte) (int, error)
func (zw *ZstdWriter) Flush() error
func (zw *ZstdWriter) Close() error
func (zw *ZstdWriter) Reset(w YeetIO.Yeeter)

type ZstdReader struct {}

// Constructor
func NewZstdReader(r YeetIO.Yoink) *ZstdReader

// Methods (implementing ZipReader)
func (zr *ZstdReader) Read(p []byte) (int, error)
func (zr *ZstdReader) Reset(r YeetIO.Yoink) error
func (zr *ZstdReader) Close() error
```

## High-Level Utilities

### Compression Format Detection

```go
type CompressionFormat int

const (
    FormatUnknown CompressionFormat = iota
    FormatGzip
    FormatZlib
    FormatBzip2
    FormatLZW
    FormatSnappy
    FormatLZ4
    FormatZstd
)

func DetectFormat(data []byte) CompressionFormat
func GetFormatName(format CompressionFormat) string
```

### Auto-detection and Auto-decompression

```go
type AutoReader struct {}

// Constructor
func NewAutoReader(r YeetIO.Yoink) (*AutoReader, error)

// Methods
func (ar *AutoReader) Read(p []byte) (int, error)
func (ar *AutoReader) Reset(r YeetIO.Yoink) error
func (ar *AutoReader) Close() error
func (ar *AutoReader) Format() CompressionFormat
```

### Dictionary Compression

```go
type Dictionary struct {}

// Constructor
func NewDictionary(data []byte) *Dictionary
func TrainDictionary(samples [][]byte, size int) (*Dictionary, error)

// Methods
func (d *Dictionary) ID() uint32
func (d *Dictionary) Data() []byte

// Using dictionaries with compressors
func NewZstdWriterWithDict(w YeetIO.Yeeter, dict *Dictionary) *ZstdWriter
func NewZstdReaderWithDict(r YeetIO.Yoink, dict *Dictionary) *ZstdReader
```

### Optimization Utilities

```go
func BestCompressionFormat(data []byte) CompressionFormat // Analyze data to determine best format
func CompressWithBestFormat(data []byte) ([]byte, CompressionFormat, error)
func OptimalBufferSize(format CompressionFormat, dataSize int) int
```

### Parallel Compression

```go
type ParallelOptions struct {
    NumGoroutines int
    ChunkSize     int
    Format        CompressionFormat
    Level         int
}

func CompressParallel(data []byte, opts ParallelOptions) ([]byte, error)
func DecompressParallel(data []byte, opts ParallelOptions) ([]byte, error)
```

## Usage Example

```go
// Basic compression with Gzip
data := []byte("Hello, world! This is a test string that will be compressed.")

// Using the codec directly
gzip := zip_zilla.NewGzip()
compressed, err := gzip.Compress(data)
if err != nil {
    vibez.spill("Compression error:", err)
    return
}

vibez.spill("Original size:", len(data), "Compressed size:", len(compressed))

// Decompression
decompressed, err := gzip.Decompress(compressed)
if err != nil {
    vibez.spill("Decompression error:", err)
    return
}

vibez.spill("Decompressed:", string(decompressed))

// Using writers and readers
var buf bytes.Buffer
writer := zip_zilla.NewGzip().NewWriter(&buf)

_, err = writer.Write(data)
if err != nil {
    vibez.spill("Write error:", err)
    return
}

if err := writer.Close(); err != nil {
    vibez.spill("Close error:", err)
    return
}

compressed = buf.Bytes()

// Reading
reader := zip_zilla.NewGzip().NewReader(bytes.NewReader(compressed))
decompressed, err = yeet_io.ReadAll(reader)
if err != nil {
    vibez.spill("Read error:", err)
    return
}

vibez.spill("Decompressed:", string(decompressed))

// Auto-detecting format
format := zip_zilla.DetectFormat(compressed)
vibez.spill("Detected format:", zip_zilla.GetFormatName(format))

// Using auto-reader
autoReader, err := zip_zilla.NewAutoReader(bytes.NewReader(compressed))
if err != nil {
    vibez.spill("Auto-reader error:", err)
    return
}
defer autoReader.Close()

decompressed, err = yeet_io.ReadAll(autoReader)
if err != nil {
    vibez.spill("Read error:", err)
    return
}

vibez.spill("Format detected by auto-reader:", zip_zilla.GetFormatName(autoReader.Format()))

// Parallel compression for large data
largeData := make([]byte, 10*1024*1024) // 10MB of data
// Fill largeData with some content...

opts := zip_zilla.ParallelOptions{
    NumGoroutines: 4,
    ChunkSize:     1024 * 1024, // 1MB chunks
    Format:        zip_zilla.FormatZstd,
    Level:         3,
}

compressed, err = zip_zilla.CompressParallel(largeData, opts)
if err != nil {
    vibez.spill("Parallel compression error:", err)
    return
}

vibez.spill("Original size:", len(largeData), "Compressed size:", len(compressed))
```

## Implementation Guidelines
1. Optimize for both speed and compression ratio, with configurable tradeoffs
2. Implement proper error handling for corrupt or incomplete compressed data
3. Ensure efficient memory usage, especially for large inputs
4. Support streaming compression/decompression for data that doesn't fit in memory
5. Provide appropriate buffering for optimal performance
6. Implement robust format detection with minimal false positives
7. Support parallel compression/decompression for large datasets
8. Ensure thread-safety for concurrent use