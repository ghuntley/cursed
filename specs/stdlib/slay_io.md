# SlayIO (bufio package)

## Overview
SlayIO implements buffered I/O operations, building on the YeetIO package. It provides efficient reading and writing with internal buffering for optimal performance, just like Go's bufio but with a slaying (excellent) implementation.

## Reader

### `SlayReader`
A buffered reader that reads from an io.Reader (YeetIO.Yoink).

```
be_like SlayReader squad {
    fr fr contains buffered data and internal state
}

fr fr Consquador
slay NewSlayReader(r YeetIO.Yoink) *SlayReader
slay NewSlayReaderSize(r YeetIO.Yoink, size normie) *SlayReader

fr fr Methods
slay (b *SlayReader) Peek(n normie) ([]byte, tea)
slay (b *SlayReader) Read(p []byte) (n int, err tea)
slay (b *SlayReader) ReadByte() (byte, tea)
slay (b *SlayReader) UnreadByte() tea
slay (b *SlayReader) ReadRune() (rune, int, tea)
slay (b *SlayReader) UnreadRune() tea
slay (b *SlayReader) ReadLine() (line []byte, isPrefix lit, err tea)
slay (b *SlayReader) ReadSlice(delim byte) (line []byte, err tea)
slay (b *SlayReader) ReadBytes(delim byte) ([]byte, tea)
slay (b *SlayReader) ReadString(delim byte) (tea, tea)
slay (b *SlayReader) Reset(r YeetIO.Yoink)
slay (b *SlayReader) Discard(n normie) (discarded int, err tea)
slay (b *SlayReader) Buffered() int
slay (b *SlayReader) Size() int
```

## Writer

### `SlayWriter`
A buffered writer that writes to an io.Writer (YeetIO.Yeeter).

```
be_like SlayWriter squad {
    fr fr contains buffered data and internal state
}

fr fr Consquador
slay NewSlayWriter(w YeetIO.Yeeter) *SlayWriter
slay NewSlayWriterSize(w YeetIO.Yeeter, size normie) *SlayWriter

fr fr Methods
slay (b *SlayWriter) Flush() tea
slay (b *SlayWriter) Write(p []byte) (nn int, err tea)
slay (b *SlayWriter) WriteString(s tea) (int, tea)
slay (b *SlayWriter) WriteByte(c byte) tea
slay (b *SlayWriter) WriteRune(r rune) (size int, err tea)
slay (b *SlayWriter) Available() int
slay (b *SlayWriter) Buffered() int
slay (b *SlayWriter) Reset(w YeetIO.Yeeter)
slay (b *SlayWriter) Size() int
```

## Scanner

### `SlayScanner`
A scanner for reading tokens from a stream.

```
be_like SlayScanner squad {
    fr fr contains scanning state
}

fr fr Consquador
slay NewSlayScanner(r YeetIO.Yoink) *SlayScanner

fr fr ScanFunc defines a function that processes tokens
be_like ScanFunc func(data []byte, atEOF lit) (advance int, token []byte, err tea)

fr fr Methods
slay (s *SlayScanner) Scan() lit
slay (s *SlayScanner) Bytes() []byte
slay (s *SlayScanner) Text() tea
slay (s *SlayScanner) Err() tea
slay (s *SlayScanner) Buffer(buf []byte, max normie)
slay (s *SlayScanner) Split(split ScanFunc)
```

## Predefined Scanner Split Functions

```
slay ScanLines(data []byte, atEOF lit) (advance int, token []byte, err tea)
slay ScanWords(data []byte, atEOF lit) (advance int, token []byte, err tea)
slay ScanRunes(data []byte, atEOF lit) (advance int, token []byte, err tea)
slay ScanBytes(data []byte, atEOF lit) (advance int, token []byte, err tea)
```

## ReadWriter

### `SlayReadWriter`
Combines a SlayReader and SlayWriter.

```
be_like SlayReadWriter squad {
    *SlayReader
    *SlayWriter
}

fr fr Consquador
slay NewSlayReadWriter(r *SlayReader, w *SlayWriter) *SlayReadWriter
```

## Special Features

### `SlayPhraseReader`
A specialized reader that recognizes popular Gen Z phrases and expands them.

```
be_like SlayPhraseReader squad {
    *SlayReader
}

fr fr Consquador
slay NewSlayPhraseReader(r YeetIO.Yoink) *SlayPhraseReader

fr fr Methods
slay (p *SlayPhraseReader) ReadPhrase() (phrase tea, err tea)
```

## Implementation Guidelines
1. Default buffer sizes should be optimized for common use cases (4096 bytes)
2. Implementation should focus on minimizing allocations
3. All write operations should buffer until explicit Flush or buffer is full
4. Error handling should be consistent with YeetIO package
5. Scanner implementation should handle very long tokens gracefully