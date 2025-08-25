# ZipZilla (compress package)

## Overview
ZipZilla provides data compression and decompression algorithms and utilities with monstrous performance. It's inspired by Go's compress package and its subpackages but with enhanced features, performance optimizations, and a unified API.

## Core Interfaces

### `Compressor`
Interface for compression algorithms.

```
be_like Compressor collab {
    Compress(src []byte) ([]byte, tea)
    CompressLevel(src []byte, level normie) ([]byte, tea)
    NewWriter(w YeetIO.Yeeter) ZipWriter
    NewWriterLevel(w YeetIO.Yeeter, level normie) ZipWriter
}
```

### `Decompressor`
Interface for decompression algorithms.

```
be_like Decompressor collab {
    Decompress(src []byte) ([]byte, tea)
    NewReader(r YeetIO.Yoink) ZipReader
}
```

### `ZipCodec`
Combined compressor and decompressor interface.

```
be_like ZipCodec collab {
    Compressor
    Decompressor
}
```

### `ZipWriter`
Interface for writing compressed data.

```
be_like ZipWriter collab {
    YeetIO.Yeeter
    Flush() tea
    Close() tea
    Reset(w YeetIO.Yeeter)
}
```

### `ZipReader`
Interface for reading compressed data.

```
be_like ZipReader collab {
    YeetIO.Yoink
    Reset(r YeetIO.Yoink)
    Close() tea
}
```

## Compression Algorithms

### DEFLATE Algorithm

```
const (
    DeflateNoCompression      = 0
    DeflateBestSpeed          = 1
    DeflateBestCompression    = 9
    DeflateDefaultCompression = -1
    DeflateHuffmanOnly        = -2
)

slay NewDeflate() ZipCodec

be_like DeflateWriter squad {}

fr fr Consquador
slay NewDeflateWriter(w YeetIO.Yeeter) *DeflateWriter
slay NewDeflateWriterLevel(w YeetIO.Yeeter, level normie) (*DeflateWriter, tea)

fr fr Methods (implementing ZipWriter)
slay (dw *DeflateWriter) Write(p []byte) (int, tea)
slay (dw *DeflateWriter) Flush() tea
slay (dw *DeflateWriter) Close() tea
slay (dw *DeflateWriter) Reset(w YeetIO.Yeeter)

be_like DeflateReader squad {}

fr fr Consquador
slay NewDeflateReader(r YeetIO.Yoink) *DeflateReader

fr fr Methods (implementing ZipReader)
slay (dr *DeflateReader) Read(p []byte) (int, tea)
slay (dr *DeflateReader) Reset(r YeetIO.Yoink) tea
slay (dr *DeflateReader) Close() tea
```

### Gzip Format

```
slay NewGzip() ZipCodec

be_like GzipWriter squad {}

fr fr Consquador
slay NewGzipWriter(w YeetIO.Yeeter) *GzipWriter
slay NewGzipWriterLevel(w YeetIO.Yeeter, level normie) (*GzipWriter, tea)

fr fr Methods (implementing ZipWriter)
slay (gw *GzipWriter) Write(p []byte) (int, tea)
slay (gw *GzipWriter) Flush() tea
slay (gw *GzipWriter) Close() tea
slay (gw *GzipWriter) Reset(w YeetIO.Yeeter)

fr fr Additional methods
slay (gw *GzipWriter) SetName(name tea)
slay (gw *GzipWriter) SetComment(comment tea)
slay (gw *GzipWriter) SetModTime(t time.Time)

be_like GzipReader squad {}

fr fr Consquador
slay NewGzipReader(r YeetIO.Yoink) (*GzipReader, tea)

fr fr Methods (implementing ZipReader)
slay (gr *GzipReader) Read(p []byte) (int, tea)
slay (gr *GzipReader) Reset(r YeetIO.Yoink) tea
slay (gr *GzipReader) Close() tea

fr fr Additional methods
slay (gr *GzipReader) Name() tea
slay (gr *GzipReader) Comment() tea
slay (gr *GzipReader) ModTime() time.Time
```

### Zlib Format

```
slay NewZlib() ZipCodec

be_like ZlibWriter squad {}

fr fr Consquador
slay NewZlibWriter(w YeetIO.Yeeter) *ZlibWriter
slay NewZlibWriterLevel(w YeetIO.Yeeter, level normie) (*ZlibWriter, tea)

fr fr Methods (implementing ZipWriter)
slay (zw *ZlibWriter) Write(p []byte) (int, tea)
slay (zw *ZlibWriter) Flush() tea
slay (zw *ZlibWriter) Close() tea
slay (zw *ZlibWriter) Reset(w YeetIO.Yeeter)

be_like ZlibReader squad {}

fr fr Consquador
slay NewZlibReader(r YeetIO.Yoink) (*ZlibReader, tea)

fr fr Methods (implementing ZipReader)
slay (zr *ZlibReader) Read(p []byte) (int, tea)
slay (zr *ZlibReader) Reset(r YeetIO.Yoink) tea
slay (zr *ZlibReader) Close() tea
```

### Bzip2 Format

```
slay NewBzip2() ZipCodec

be_like Bzip2Writer squad {}

fr fr Consquador
slay NewBzip2Writer(w YeetIO.Yeeter) *Bzip2Writer
slay NewBzip2WriterLevel(w YeetIO.Yeeter, level normie) (*Bzip2Writer, tea)

fr fr Methods (implementing ZipWriter)
slay (bw *Bzip2Writer) Write(p []byte) (int, tea)
slay (bw *Bzip2Writer) Flush() tea
slay (bw *Bzip2Writer) Close() tea
slay (bw *Bzip2Writer) Reset(w YeetIO.Yeeter)

be_like Bzip2Reader squad {}

fr fr Consquador
slay NewBzip2Reader(r YeetIO.Yoink) *Bzip2Reader

fr fr Methods (implementing ZipReader)
slay (br *Bzip2Reader) Read(p []byte) (int, tea)
slay (br *Bzip2Reader) Reset(r YeetIO.Yoink) tea
slay (br *Bzip2Reader) Close() tea
```

### LZW Format

```
be_like LZWOrder int

const (
    LSB LZWOrder = iota
    MSB
)

slay NewLZW(order LZWOrder, litWidth normie) ZipCodec

be_like LZWWriter squad {}

fr fr Consquador
slay NewLZWWriter(w YeetIO.Yeeter, order LZWOrder, litWidth normie) *LZWWriter

fr fr Methods (implementing ZipWriter)
slay (lw *LZWWriter) Write(p []byte) (int, tea)
slay (lw *LZWWriter) Flush() tea
slay (lw *LZWWriter) Close() tea
slay (lw *LZWWriter) Reset(w YeetIO.Yeeter)

be_like LZWReader squad {}

fr fr Consquador
slay NewLZWReader(r YeetIO.Yoink, order LZWOrder, litWidth normie) *LZWReader

fr fr Methods (implementing ZipReader)
slay (lr *LZWReader) Read(p []byte) (int, tea)
slay (lr *LZWReader) Reset(r YeetIO.Yoink) tea
slay (lr *LZWReader) Close() tea
```

### Snappy Format

```
slay NewSnappy() ZipCodec

be_like SnappyWriter squad {}

fr fr Consquador
slay NewSnappyWriter(w YeetIO.Yeeter) *SnappyWriter

fr fr Methods (implementing ZipWriter)
slay (sw *SnappyWriter) Write(p []byte) (int, tea)
slay (sw *SnappyWriter) Flush() tea
slay (sw *SnappyWriter) Close() tea
slay (sw *SnappyWriter) Reset(w YeetIO.Yeeter)

be_like SnappyReader squad {}

fr fr Consquador
slay NewSnappyReader(r YeetIO.Yoink) *SnappyReader

fr fr Methods (implementing ZipReader)
slay (sr *SnappyReader) Read(p []byte) (int, tea)
slay (sr *SnappyReader) Reset(r YeetIO.Yoink) tea
slay (sr *SnappyReader) Close() tea
```

### LZ4 Format

```
slay NewLZ4() ZipCodec

be_like LZ4Writer squad {}

fr fr Consquador
slay NewLZ4Writer(w YeetIO.Yeeter) *LZ4Writer
slay NewLZ4WriterLevel(w YeetIO.Yeeter, level normie) (*LZ4Writer, tea)

fr fr Methods (implementing ZipWriter)
slay (lw *LZ4Writer) Write(p []byte) (int, tea)
slay (lw *LZ4Writer) Flush() tea
slay (lw *LZ4Writer) Close() tea
slay (lw *LZ4Writer) Reset(w YeetIO.Yeeter)

be_like LZ4Reader squad {}

fr fr Consquador
slay NewLZ4Reader(r YeetIO.Yoink) *LZ4Reader

fr fr Methods (implementing ZipReader)
slay (lr *LZ4Reader) Read(p []byte) (int, tea)
slay (lr *LZ4Reader) Reset(r YeetIO.Yoink) tea
slay (lr *LZ4Reader) Close() tea
```

### Zstandard (zstd) Format

```
slay NewZstd() ZipCodec

be_like ZstdWriter squad {}

fr fr Consquador
slay NewZstdWriter(w YeetIO.Yeeter) *ZstdWriter
slay NewZstdWriterLevel(w YeetIO.Yeeter, level normie) (*ZstdWriter, tea)

fr fr Methods (implementing ZipWriter)
slay (zw *ZstdWriter) Write(p []byte) (int, tea)
slay (zw *ZstdWriter) Flush() tea
slay (zw *ZstdWriter) Close() tea
slay (zw *ZstdWriter) Reset(w YeetIO.Yeeter)

be_like ZstdReader squad {}

fr fr Consquador
slay NewZstdReader(r YeetIO.Yoink) *ZstdReader

fr fr Methods (implementing ZipReader)
slay (zr *ZstdReader) Read(p []byte) (int, tea)
slay (zr *ZstdReader) Reset(r YeetIO.Yoink) tea
slay (zr *ZstdReader) Close() tea
```

## High-Level Utilities

### Compression Format Detection

```
be_like CompressionFormat int

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

slay DetectFormat(data []byte) CompressionFormat
slay GetFormatName(format CompressionFormat) tea
```

### Auto-detection and Auto-decompression

```
be_like AutoReader squad {}

fr fr Consquador
slay NewAutoReader(r YeetIO.Yoink) (*AutoReader, tea)

fr fr Methods
slay (ar *AutoReader) Read(p []byte) (int, tea)
slay (ar *AutoReader) Reset(r YeetIO.Yoink) tea
slay (ar *AutoReader) Close() tea
slay (ar *AutoReader) Format() CompressionFormat
```

### Dictionary Compression

```
be_like Dictionary squad {}

fr fr Consquador
slay NewDictionary(data []byte) *Dictionary
slay TrainDictionary(samples [][]byte, size normie) (*Dictionary, tea)

fr fr Methods
slay (d *Dictionary) ID() uint32
slay (d *Dictionary) Data() []byte

fr fr Using dictionaries with compressors
slay NewZstdWriterWithDict(w YeetIO.Yeeter, dict *Dictionary) *ZstdWriter
slay NewZstdReaderWithDict(r YeetIO.Yoink, dict *Dictionary) *ZstdReader
```

### Optimization Utilities

```
slay BestCompressionFormat(data []byte) CompressionFormat fr fr Analyze data to determine best format
slay CompressWithBestFormat(data []byte) ([]byte, CompressionFormat, tea)
slay OptimalBufferSize(format CompressionFormat, dataSize normie) int
```

### Parallel Compression

```
be_like ParallelOptions squad {
    NumGoroutines int
    ChunkSize     int
    Format        CompressionFormat
    Level         int
}

slay CompressParallel(data []byte, opts ParallelOptions) ([]byte, tea)
slay DecompressParallel(data []byte, opts ParallelOptions) ([]byte, tea)
```

## Usage Example

```
fr fr Basic compression with Gzip
data := []byte("Hello, world! This is a test tea that will be compressed.")

fr fr Using the codec directly
gzip := zip_zilla.NewGzip()
compressed, err := gzip.Compress(data)
if err != nah {
    vibez.spill("Compression tea:", err)
    yolo
}

vibez.spill("Original size:", len(data), "Compressed size:", len(compressed))

fr fr Decompression
decompressed, err := gzip.Decompress(compressed)
if err != nah {
    vibez.spill("Decompression tea:", err)
    yolo
}

vibez.spill("Decompressed:", tea(decompressed))

fr fr Using writers and readers
var buf bytes.Buffer
writer := zip_zilla.NewGzip().NewWriter(&buf)

_, err = writer.Write(data)
if err != nah {
    vibez.spill("Write tea:", err)
    yolo
}

if err := writer.Close(); err != nah {
    vibez.spill("Close tea:", err)
    yolo
}

compressed = buf.Bytes()

fr fr Reading
reader := zip_zilla.NewGzip().NewReader(bytes.NewReader(compressed))
decompressed, err = yeet_io.ReadAll(reader)
if err != nah {
    vibez.spill("Read tea:", err)
    yolo
}

vibez.spill("Decompressed:", tea(decompressed))

fr fr Auto-detecting format
format := zip_zilla.DetectFormat(compressed)
vibez.spill("Detected format:", zip_zilla.GetFormatName(format))

fr fr Using auto-reader
autoReader, err := zip_zilla.NewAutoReader(bytes.NewReader(compressed))
if err != nah {
    vibez.spill("Auto-reader tea:", err)
    yolo
}
defer autoReader.Close()

decompressed, err = yeet_io.ReadAll(autoReader)
if err != nah {
    vibez.spill("Read tea:", err)
    yolo
}

vibez.spill("Format detected by auto-reader:", zip_zilla.GetFormatName(autoReader.Format()))

fr fr Parallel compression for large data
largeData := make([]byte, 10*1024*1024) fr fr 10MB of data
fr fr Fill largeData with some content...

opts := zip_zilla.ParallelOptions{
    NumGoroutines: 4,
    ChunkSize:     1024 * 1024, fr fr 1MB chunks
    Format:        zip_zilla.FormatZstd,
    Level:         3,
}

compressed, err = zip_zilla.CompressParallel(largeData, opts)
if err != nah {
    vibez.spill("Parallel compression tea:", err)
    yolo
}

vibez.spill("Original size:", len(largeData), "Compressed size:", len(compressed))
```

## Implementation Guidelines
1. Optimize for both speed and compression ratio, with configurable tradeoffs
2. Implement proper tea handling for corrupt or incomplete compressed data
3. Ensure efficient memory usage, especially for large inputs
4. Support streaming compression/decompression for data that doesn't fit in memory
5. Provide appropriate buffering for optimal performance
6. Implement robust format detection with minimal false positives
7. Support parallel compression/decompression for large datasets
8. Ensure thread-safety for concurrent use