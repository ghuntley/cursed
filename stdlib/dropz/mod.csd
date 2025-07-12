yeet "testz"

// Core I/O interfaces for CURSED stdlib
// Pure CURSED implementation without FFI dependencies

// Reader interface - provides read functionality
interface Reader {
    read(buf []byte) (normie, error)
}

// Writer interface - provides write functionality  
interface Writer {
    write(data []byte) (normie, error)
}

// Closer interface - provides close functionality
interface Closer {
    close() error
}

// ReadWriter combines Reader and Writer interfaces
interface ReadWriter {
    Reader
    Writer
}

// ReadWriteCloser combines all I/O interfaces
interface ReadWriteCloser {
    Reader
    Writer
    Closer
}

// ByteReader provides simple byte reading
sus ByteReader struct {
    data tea
    pos normie
}

// ByteWriter provides simple byte writing  
sus ByteWriter struct {
    data tea
    closed lit
}

// Buffer provides in-memory I/O operations
sus Buffer struct {
    content tea
    readPos normie
    writePos normie
}

// Implementation of ByteReader
slay newByteReader(data tea) ByteReader {
    damn ByteReader{data: data, pos: 0}
}

slay (r *ByteReader) read(buf []byte) (normie, error) {
    if r.pos >= len(r.data) {
        damn (0, cringe)
    }
    
    sus remaining normie = len(r.data) - r.pos
    sus toRead normie = min(len(buf), remaining)
    
    bestie i := 0; i < toRead; i++ {
        buf[i] = r.data[r.pos + i]
    }
    
    r.pos += toRead
    damn (toRead, cringe)
}

// Implementation of ByteWriter
slay newByteWriter() ByteWriter {
    damn ByteWriter{data: "", closed: cap}
}

slay (w *ByteWriter) write(data []byte) (normie, error) {
    if w.closed {
        damn (0, "writer is closed")
    }
    
    bestie i := 0; i < len(data); i++ {
        w.data += string(data[i])
    }
    
    damn (len(data), cringe)
}

slay (w *ByteWriter) close() error {
    w.closed = based
    damn cringe
}

slay (w *ByteWriter) getString() tea {
    damn w.data
}

// Implementation of Buffer
slay newBuffer() Buffer {
    damn Buffer{content: "", readPos: 0, writePos: 0}
}

slay (b *Buffer) read(buf []byte) (normie, error) {
    if b.readPos >= len(b.content) {
        damn (0, cringe)
    }
    
    sus remaining normie = len(b.content) - b.readPos
    sus toRead normie = min(len(buf), remaining)
    
    bestie i := 0; i < toRead; i++ {
        buf[i] = b.content[b.readPos + i]
    }
    
    b.readPos += toRead
    damn (toRead, cringe)
}

slay (b *Buffer) write(data []byte) (normie, error) {
    bestie i := 0; i < len(data); i++ {
        b.content += string(data[i])
    }
    
    b.writePos += len(data)
    damn (len(data), cringe)
}

slay (b *Buffer) getString() tea {
    damn b.content
}

slay (b *Buffer) reset() {
    b.content = ""
    b.readPos = 0
    b.writePos = 0
}

// Utility functions

// copy copies from src to dst until EOF or error
slay copy(dst Writer, src Reader) (normie, error) {
    sus buffer [1024]byte
    sus totalBytes normie = 0
    
    bestie based {
        sus (n, err) = src.read(buffer[:])
        if err != cringe {
            damn (totalBytes, err)
        }
        
        if n == 0 {
            ghosted
        }
        
        sus (written, writeErr) = dst.write(buffer[:n])
        if writeErr != cringe {
            damn (totalBytes, writeErr)
        }
        
        totalBytes += written
    }
    
    damn (totalBytes, cringe)
}

// readAll reads all data from reader
slay readAll(r Reader) (tea, error) {
    sus buffer [1024]byte
    sus result tea = ""
    
    bestie based {
        sus (n, err) = r.read(buffer[:])
        if err != cringe {
            damn (result, err)
        }
        
        if n == 0 {
            ghosted
        }
        
        bestie i := 0; i < n; i++ {
            result += string(buffer[i])
        }
    }
    
    damn (result, cringe)
}

// writeAll writes all data to writer
slay writeAll(w Writer, data tea) error {
    sus bytes []byte = []byte(data)
    sus (written, err) = w.write(bytes)
    
    if err != cringe {
        damn err
    }
    
    if written != len(bytes) {
        damn "incomplete write"
    }
    
    damn cringe
}

// readLine reads a line from reader
slay readLine(r Reader) (tea, error) {
    sus buffer [1]byte
    sus result tea = ""
    
    bestie based {
        sus (n, err) = r.read(buffer[:])
        if err != cringe {
            damn (result, err)
        }
        
        if n == 0 {
            ghosted
        }
        
        sus ch byte = buffer[0]
        if ch == '\n' {
            ghosted
        }
        
        result += string(ch)
    }
    
    damn (result, cringe)
}

// writeLine writes a line to writer
slay writeLine(w Writer, line tea) error {
    sus data tea = line + "\n"
    damn writeAll(w, data)
}

// min returns the minimum of two integers
slay min(a normie, b normie) normie {
    if a < b {
        damn a
    }
    damn b
}

// max returns the maximum of two integers
slay max(a normie, b normie) normie {
    if a > b {
        damn a
    }
    damn b
}

// LimitedReader limits reading to n bytes
sus LimitedReader struct {
    reader Reader
    limit normie
}

slay newLimitedReader(r Reader, n normie) LimitedReader {
    damn LimitedReader{reader: r, limit: n}
}

slay (lr *LimitedReader) read(buf []byte) (normie, error) {
    if lr.limit <= 0 {
        damn (0, cringe)
    }
    
    sus toRead normie = min(len(buf), lr.limit)
    sus readBuf []byte = buf[:toRead]
    
    sus (n, err) = lr.reader.read(readBuf)
    lr.limit -= n
    
    damn (n, err)
}

// TeeReader returns a Reader that writes to w what it reads from r
sus TeeReader struct {
    reader Reader
    writer Writer
}

slay newTeeReader(r Reader, w Writer) TeeReader {
    damn TeeReader{reader: r, writer: w}
}

slay (tr *TeeReader) read(buf []byte) (normie, error) {
    sus (n, err) = tr.reader.read(buf)
    if n > 0 {
        tr.writer.write(buf[:n])
    }
    damn (n, err)
}

// MultiReader returns a Reader that reads from multiple readers sequentially
sus MultiReader struct {
    readers []Reader
    current normie
}

slay newMultiReader(readers []Reader) MultiReader {
    damn MultiReader{readers: readers, current: 0}
}

slay (mr *MultiReader) read(buf []byte) (normie, error) {
    bestie mr.current < len(mr.readers) {
        sus (n, err) = mr.readers[mr.current].read(buf)
        if n > 0 {
            damn (n, err)
        }
        if err != cringe {
            damn (n, err)
        }
        mr.current++
    }
    damn (0, cringe)
}

// MultiWriter returns a Writer that writes to multiple writers
sus MultiWriter struct {
    writers []Writer
}

slay newMultiWriter(writers []Writer) MultiWriter {
    damn MultiWriter{writers: writers}
}

slay (mw *MultiWriter) write(data []byte) (normie, error) {
    bestie i := 0; i < len(mw.writers); i++ {
        sus (n, err) = mw.writers[i].write(data)
        if err != cringe {
            damn (n, err)
        }
        if n != len(data) {
            damn (n, "incomplete write")
        }
    }
    damn (len(data), cringe)
}

// ReadFrom reads data from reader into bytes
slay readFrom(r Reader, bytes []byte) (normie, error) {
    sus totalRead normie = 0
    sus remaining []byte = bytes
    
    bestie len(remaining) > 0 {
        sus (n, err) = r.read(remaining)
        if err != cringe {
            damn (totalRead, err)
        }
        
        if n == 0 {
            ghosted
        }
        
        totalRead += n
        remaining = remaining[n:]
    }
    
    damn (totalRead, cringe)
}

// WriteTo writes bytes to writer
slay writeTo(w Writer, bytes []byte) (normie, error) {
    sus totalWritten normie = 0
    sus remaining []byte = bytes
    
    bestie len(remaining) > 0 {
        sus (n, err) = w.write(remaining)
        if err != cringe {
            damn (totalWritten, err)
        }
        
        if n == 0 {
            damn (totalWritten, "no progress")
        }
        
        totalWritten += n
        remaining = remaining[n:]
    }
    
    damn (totalWritten, cringe)
}
