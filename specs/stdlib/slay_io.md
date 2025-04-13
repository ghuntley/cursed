# SlayIO (bufio package)

## Overview
SlayIO implements buffered I/O operations, building on the YeetIO package. It provides efficient reading and writing with internal buffering for optimal performance, just like Go's bufio but with a slaying (excellent) implementation.

## Reader

### `SlayReader`
A buffered reader that reads from an io.Reader (YeetIO.Yoink).

```go
type SlayReader struct {
    // contains buffered data and internal state
}

// Constructor
func NewSlayReader(r YeetIO.Yoink) *SlayReader
func NewSlayReaderSize(r YeetIO.Yoink, size int) *SlayReader

// Methods
func (b *SlayReader) Peek(n int) ([]byte, error)
func (b *SlayReader) Read(p []byte) (n int, err error)
func (b *SlayReader) ReadByte() (byte, error)
func (b *SlayReader) UnreadByte() error
func (b *SlayReader) ReadRune() (rune, int, error)
func (b *SlayReader) UnreadRune() error
func (b *SlayReader) ReadLine() (line []byte, isPrefix bool, err error)
func (b *SlayReader) ReadSlice(delim byte) (line []byte, err error)
func (b *SlayReader) ReadBytes(delim byte) ([]byte, error)
func (b *SlayReader) ReadString(delim byte) (string, error)
func (b *SlayReader) Reset(r YeetIO.Yoink)
func (b *SlayReader) Discard(n int) (discarded int, err error)
func (b *SlayReader) Buffered() int
func (b *SlayReader) Size() int
```

## Writer

### `SlayWriter`
A buffered writer that writes to an io.Writer (YeetIO.Yeeter).

```go
type SlayWriter struct {
    // contains buffered data and internal state
}

// Constructor
func NewSlayWriter(w YeetIO.Yeeter) *SlayWriter
func NewSlayWriterSize(w YeetIO.Yeeter, size int) *SlayWriter

// Methods
func (b *SlayWriter) Flush() error
func (b *SlayWriter) Write(p []byte) (nn int, err error)
func (b *SlayWriter) WriteString(s string) (int, error)
func (b *SlayWriter) WriteByte(c byte) error
func (b *SlayWriter) WriteRune(r rune) (size int, err error)
func (b *SlayWriter) Available() int
func (b *SlayWriter) Buffered() int
func (b *SlayWriter) Reset(w YeetIO.Yeeter)
func (b *SlayWriter) Size() int
```

## Scanner

### `SlayScanner`
A scanner for reading tokens from a stream.

```go
type SlayScanner struct {
    // contains scanning state
}

// Constructor
func NewSlayScanner(r YeetIO.Yoink) *SlayScanner

// ScanFunc defines a function that processes tokens
type ScanFunc func(data []byte, atEOF bool) (advance int, token []byte, err error)

// Methods
func (s *SlayScanner) Scan() bool
func (s *SlayScanner) Bytes() []byte
func (s *SlayScanner) Text() string
func (s *SlayScanner) Err() error
func (s *SlayScanner) Buffer(buf []byte, max int)
func (s *SlayScanner) Split(split ScanFunc)
```

## Predefined Scanner Split Functions

```go
func ScanLines(data []byte, atEOF bool) (advance int, token []byte, err error)
func ScanWords(data []byte, atEOF bool) (advance int, token []byte, err error)
func ScanRunes(data []byte, atEOF bool) (advance int, token []byte, err error)
func ScanBytes(data []byte, atEOF bool) (advance int, token []byte, err error)
```

## ReadWriter

### `SlayReadWriter`
Combines a SlayReader and SlayWriter.

```go
type SlayReadWriter struct {
    *SlayReader
    *SlayWriter
}

// Constructor
func NewSlayReadWriter(r *SlayReader, w *SlayWriter) *SlayReadWriter
```

## Special Features

### `SlayPhraseReader`
A specialized reader that recognizes popular Gen Z phrases and expands them.

```go
type SlayPhraseReader struct {
    *SlayReader
}

// Constructor
func NewSlayPhraseReader(r YeetIO.Yoink) *SlayPhraseReader

// Methods
func (p *SlayPhraseReader) ReadPhrase() (phrase string, err error)
```

## Implementation Guidelines
1. Default buffer sizes should be optimized for common use cases (4096 bytes)
2. Implementation should focus on minimizing allocations
3. All write operations should buffer until explicit Flush or buffer is full
4. Error handling should be consistent with YeetIO package
5. Scanner implementation should handle very long tokens gracefully