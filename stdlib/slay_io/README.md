# Slay IO Module

The `slay_io` module provides efficient buffered I/O operations for the CURSED language. It offers high-performance reading and writing with internal buffering, building on the core I/O functionality with advanced features.

## Features

### Buffered Reader
- **NewSlayReader**: Create buffered readers with default buffer size
- **NewSlayReaderSize**: Create buffered readers with custom buffer size
- **ReadData**: Read specified amount of data efficiently
- **ReadByte**: Read single bytes from the stream
- **ReadLine**: Read complete lines with delimiter detection
- **ReadString**: Read strings until specified delimiter
- **PeekData**: Preview data without consuming from buffer
- **BufferedBytes**: Get count of buffered data

### Buffered Writer
- **NewSlayWriter**: Create buffered writers with default buffer size
- **NewSlayWriterSize**: Create buffered writers with custom buffer size
- **WriteData**: Write data to the buffered stream
- **WriteString**: Write strings efficiently
- **WriteByte**: Write single bytes
- **FlushWriter**: Force buffer flush to underlying stream
- **AvailableSpace**: Get remaining buffer space
- **WriterBuffered**: Get count of buffered data

### Scanner
- **NewSlayScanner**: Create scanners for token-based reading
- **ScanNext**: Advance to next token
- **ScanBytes**: Get current token as bytes
- **ScanText**: Get current token as text
- **ScanError**: Check for scanning errors
- **SetScanBuffer**: Configure scanner buffer

### Scanner Split Functions
- **ScanLines**: Split input by newlines
- **ScanWords**: Split input by whitespace
- **ScanRunes**: Split input by Unicode characters
- **ScanBytesData**: Split input by individual bytes

### ReadWriter
- **NewSlayReadWriter**: Combine reader and writer functionality
- **ReadWriteData**: Perform combined read-write operations

### Special Features
- **SlayPhraseReader**: Specialized reader for Gen Z phrases
- **ReadPhrase**: Extract and expand popular Gen Z expressions
- **ExpandPhrase**: Convert slang to formal explanations

## Usage Examples

```cursed
fr fr Buffered reading
sus reader tea = NewSlayReader("input.txt")
sus data tea = ReadData(reader, 1024)
sus line tea = ReadLine(reader)

fr fr Buffered writing
sus writer tea = NewSlayWriter("output.txt")
sus bytesWritten normie = WriteData(writer, "Hello World")
sus flushResult tea = FlushWriter(writer)

fr fr Scanner usage
sus scanner tea = NewSlayScanner("data.txt")
lowkey ScanNext(scanner) {
    sus token tea = ScanText(scanner)
    vibez.spill("Token: " + token)
}

fr fr Custom buffer sizes
sus largeReader tea = NewSlayReaderSize("bigfile.txt", 8192)
sus smallWriter tea = NewSlayWriterSize("output.txt", 2048)

fr fr Gen Z phrase processing
sus phraseReader tea = NewSlayPhraseReader("social.txt")
sus phrase tea = ReadPhrase(phraseReader)
vibez.spill("Found phrase: " + phrase)

fr fr Combined operations
sus readWriter tea = NewSlayReadWriter(reader, writer)
sus processed tea = ReadWriteData(readWriter, "transform this")
```

## Buffer Management

### Default Settings
- Default buffer size: 4096 bytes
- Optimal for most file operations
- Automatically managed memory

### Custom Buffer Sizes
- Small files: 1024-2048 bytes
- Large files: 8192-16384 bytes
- Network operations: 4096-8192 bytes
- Memory-constrained: 512-1024 bytes

### Performance Tips
- Use larger buffers for sequential access
- Use smaller buffers for random access
- Flush frequently for real-time applications
- Monitor buffer efficiency with stats functions

## Scanner Configuration

### Split Functions
- **Lines**: Best for text processing
- **Words**: Ideal for tokenization
- **Runes**: Unicode-aware character processing
- **Bytes**: Low-level binary processing

### Custom Delimiters
Configure scanners for specific use cases:
- CSV parsing: comma delimiters
- Log processing: timestamp patterns
- Code analysis: syntax elements

## Gen Z Features

### Phrase Recognition
The SlayPhraseReader recognizes common Gen Z expressions:
- "no cap" → "seriously/honestly"
- "fr fr" → "for real, for real"
- "bussin" → "excellent/amazing"
- "slay" → "succeed/excel"

### Automatic Expansion
Convert slang to formal language for documentation or translation purposes.

## Implementation Notes

This is a pure CURSED implementation providing:
- High-performance buffered I/O operations
- Memory-efficient buffer management
- Unicode-aware text processing
- Cross-platform compatibility
- Gen Z culture integration
- Production-ready error handling

The module serves as a foundation for efficient data processing while maintaining the CURSED language's unique personality and modern appeal.
