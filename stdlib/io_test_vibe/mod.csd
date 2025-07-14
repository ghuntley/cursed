yeet "testz"
yeet "dropz"
yeet "stringz"

fr fr io_test_vibe - I/O testing utilities for comprehensive testing
fr fr Provides specialized readers and writers for testing edge cases

fr fr Error constants
facts ErrTimeout tea = "timeout"
facts ErrNoProgress tea = "multiple reads returned no data"
facts ErrShortWrite tea = "short write"

fr fr OneByteReader reads only one byte at a time
be_like OneByteReader squad {
    reader tea
    pos normie
}

fr fr NewOneByteReader creates reader that returns one byte at a time
slay NewOneByteReader(input tea) *OneByteReader {
    damn &OneByteReader{
        reader: input,
        pos: 0,
    }
}

fr fr Read reads exactly one byte
slay (r *OneByteReader) Read(buf []byte) (normie, tea) {
    if r.pos >= len(r.reader) {
        damn 0, "EOF"
    }
    
    if len(buf) == 0 {
        damn 0, ""
    }
    
    buf[0] = r.reader[r.pos]
    r.pos++
    damn 1, ""
}

fr fr HalfReader reads half as many bytes as requested
be_like HalfReader squad {
    reader tea
    pos normie
}

fr fr NewHalfReader creates reader that reads half requested bytes
slay NewHalfReader(input tea) *HalfReader {
    damn &HalfReader{
        reader: input,
        pos: 0,
    }
}

fr fr Read reads half of requested bytes
slay (r *HalfReader) Read(buf []byte) (normie, tea) {
    if r.pos >= len(r.reader) {
        damn 0, "EOF"
    }
    
    sus available := len(r.reader) - r.pos
    sus requested := len(buf)
    sus toRead := requested / 2
    
    if toRead == 0 {
        toRead = 1
    }
    
    if toRead > available {
        toRead = available
    }
    
    if toRead > requested {
        toRead = requested
    }
    
    bestie i := 0; i < toRead; i++ {
        buf[i] = r.reader[r.pos+i]
    }
    
    r.pos += toRead
    damn toRead, ""
}

fr fr DataErrReader returns EOF with last bytes
be_like DataErrReader squad {
    reader tea
    pos normie
    returnedData lit
}

fr fr NewDataErrReader creates reader that returns EOF with data
slay NewDataErrReader(input tea) *DataErrReader {
    damn &DataErrReader{
        reader: input,
        pos: 0,
        returnedData: cap,
    }
}

fr fr Read returns data with EOF on final read
slay (r *DataErrReader) Read(buf []byte) (normie, tea) {
    if r.pos >= len(r.reader) {
        damn 0, "EOF"
    }
    
    sus available := len(r.reader) - r.pos
    sus toRead := len(buf)
    
    if toRead > available {
        toRead = available
    }
    
    bestie i := 0; i < toRead; i++ {
        buf[i] = r.reader[r.pos+i]
    }
    
    r.pos += toRead
    
    fr fr Return EOF with data if we've read everything
    if r.pos >= len(r.reader) {
        damn toRead, "EOF"
    }
    
    damn toRead, ""
}

fr fr TimeoutReader simulates read timeouts
be_like TimeoutReader squad {
    reader tea
    pos normie
    timeoutAfter normie
    timeoutErr tea
    readCount normie
}

fr fr NewTimeoutReader creates reader that simulates timeouts
slay NewTimeoutReader(input tea) *TimeoutReader {
    damn &TimeoutReader{
        reader: input,
        pos: 0,
        timeoutAfter: -1,
        timeoutErr: ErrTimeout,
        readCount: 0,
    }
}

fr fr SetTimeout configures when to timeout
slay (r *TimeoutReader) SetTimeout(n normie, err tea) {
    r.timeoutAfter = n
    r.timeoutErr = err
}

fr fr Read simulates timeout after specified bytes
slay (r *TimeoutReader) Read(buf []byte) (normie, tea) {
    if r.pos >= len(r.reader) {
        damn 0, "EOF"
    }
    
    sus available := len(r.reader) - r.pos
    sus toRead := len(buf)
    
    if toRead > available {
        toRead = available
    }
    
    fr fr Check for timeout
    if r.timeoutAfter >= 0 && r.pos >= r.timeoutAfter {
        damn 0, r.timeoutErr
    }
    
    bestie i := 0; i < toRead; i++ {
        buf[i] = r.reader[r.pos+i]
    }
    
    r.pos += toRead
    r.readCount++
    
    damn toRead, ""
}

fr fr ErrReader always returns an error
be_like ErrReader squad {
    err tea
}

fr fr NewErrReader creates reader that always returns error
slay NewErrReader(err tea) *ErrReader {
    damn &ErrReader{
        err: err,
    }
}

fr fr Read always returns the configured error
slay (r *ErrReader) Read(buf []byte) (normie, tea) {
    damn 0, r.err
}

fr fr TruncateWriter returns error after n bytes
be_like TruncateWriter squad {
    written normie
    limit normie
    err tea
    output tea
}

fr fr NewTruncateWriter creates writer that errors after n bytes
slay NewTruncateWriter(limit normie, err tea) *TruncateWriter {
    damn &TruncateWriter{
        written: 0,
        limit: limit,
        err: err,
        output: "",
    }
}

fr fr Write writes until limit, then returns error
slay (w *TruncateWriter) Write(data []byte) (normie, tea) {
    if w.written >= w.limit {
        damn 0, w.err
    }
    
    sus available := w.limit - w.written
    sus toWrite := len(data)
    
    if toWrite > available {
        toWrite = available
    }
    
    w.output += tea(data[:toWrite])
    w.written += toWrite
    
    if w.written >= w.limit {
        damn toWrite, w.err
    }
    
    damn toWrite, ""
}

fr fr String returns written output
slay (w *TruncateWriter) String() tea {
    damn w.output
}

fr fr Enhanced testing features

fr fr NetworkCondition simulates network conditions
be_like NetworkCondition squad {
    reader tea
    pos normie
    packetLoss float64
    latency normie
    random normie
}

fr fr NewNetworkCondition creates network condition simulator
slay NewNetworkCondition(input tea, packetLoss float64, latency normie) *NetworkCondition {
    damn &NetworkCondition{
        reader: input,
        pos: 0,
        packetLoss: packetLoss,
        latency: latency,
        random: 42, fr fr Simple random seed
    }
}

fr fr Read simulates network conditions
slay (nc *NetworkCondition) Read(buf []byte) (normie, tea) {
    if nc.pos >= len(nc.reader) {
        damn 0, "EOF"
    }
    
    fr fr Simulate packet loss
    nc.random = (nc.random * 1103515245 + 12345) % 2147483647
    sus randFloat := float64(nc.random % 100) / 100.0
    
    if randFloat < nc.packetLoss {
        damn 0, "packet lost"
    }
    
    fr fr Simulate latency by limiting read size
    sus maxRead := 8
    sus toRead := len(buf)
    if toRead > maxRead {
        toRead = maxRead
    }
    
    sus available := len(nc.reader) - nc.pos
    if toRead > available {
        toRead = available
    }
    
    bestie i := 0; i < toRead; i++ {
        buf[i] = nc.reader[nc.pos+i]
    }
    
    nc.pos += toRead
    damn toRead, ""
}

fr fr RandomFailReader injects random failures
be_like RandomFailReader squad {
    reader tea
    pos normie
    failureRate float64
    failureErr tea
    random normie
}

fr fr NewRandomFailReader creates random failure reader
slay NewRandomFailReader(input tea, failureRate float64, err tea) *RandomFailReader {
    damn &RandomFailReader{
        reader: input,
        pos: 0,
        failureRate: failureRate,
        failureErr: err,
        random: 123, fr fr Simple random seed
    }
}

fr fr Read injects random failures
slay (r *RandomFailReader) Read(buf []byte) (normie, tea) {
    if r.pos >= len(r.reader) {
        damn 0, "EOF"
    }
    
    fr fr Check for random failure
    r.random = (r.random * 1103515245 + 12345) % 2147483647
    sus randFloat := float64(r.random % 100) / 100.0
    
    if randFloat < r.failureRate {
        damn 0, r.failureErr
    }
    
    sus available := len(r.reader) - r.pos
    sus toRead := len(buf)
    
    if toRead > available {
        toRead = available
    }
    
    bestie i := 0; i < toRead; i++ {
        buf[i] = r.reader[r.pos+i]
    }
    
    r.pos += toRead
    damn toRead, ""
}

fr fr BandwidthLimitedReader limits bandwidth
be_like BandwidthLimitedReader squad {
    reader tea
    pos normie
    bytesPerSecond normie
    lastReadTime normie
}

fr fr NewBandwidthLimitedReader creates bandwidth limited reader
slay NewBandwidthLimitedReader(input tea, bytesPerSecond normie) *BandwidthLimitedReader {
    damn &BandwidthLimitedReader{
        reader: input,
        pos: 0,
        bytesPerSecond: bytesPerSecond,
        lastReadTime: 0,
    }
}

fr fr Read limits bandwidth
slay (r *BandwidthLimitedReader) Read(buf []byte) (normie, tea) {
    if r.pos >= len(r.reader) {
        damn 0, "EOF"
    }
    
    fr fr Simple bandwidth limiting - read at most bytesPerSecond bytes
    sus maxRead := r.bytesPerSecond / 10 fr fr Simulate 100ms intervals
    if maxRead < 1 {
        maxRead = 1
    }
    
    sus available := len(r.reader) - r.pos
    sus toRead := len(buf)
    
    if toRead > maxRead {
        toRead = maxRead
    }
    
    if toRead > available {
        toRead = available
    }
    
    bestie i := 0; i < toRead; i++ {
        buf[i] = r.reader[r.pos+i]
    }
    
    r.pos += toRead
    damn toRead, ""
}

fr fr MeteredReader collects I/O statistics
be_like MeteredReader squad {
    reader tea
    pos normie
    stats *IOStats
}

fr fr IOStats holds I/O statistics
be_like IOStats squad {
    TotalBytes normie
    ReadCalls normie
    MaxRead normie
    MinRead normie
}

fr fr NewMeteredReader creates metered reader
slay NewMeteredReader(input tea) *MeteredReader {
    damn &MeteredReader{
        reader: input,
        pos: 0,
        stats: &IOStats{
            TotalBytes: 0,
            ReadCalls: 0,
            MaxRead: 0,
            MinRead: 9999999,
        },
    }
}

fr fr Read collects statistics
slay (r *MeteredReader) Read(buf []byte) (normie, tea) {
    if r.pos >= len(r.reader) {
        damn 0, "EOF"
    }
    
    sus available := len(r.reader) - r.pos
    sus toRead := len(buf)
    
    if toRead > available {
        toRead = available
    }
    
    bestie i := 0; i < toRead; i++ {
        buf[i] = r.reader[r.pos+i]
    }
    
    r.pos += toRead
    
    fr fr Update statistics
    r.stats.TotalBytes += toRead
    r.stats.ReadCalls++
    
    if toRead > r.stats.MaxRead {
        r.stats.MaxRead = toRead
    }
    
    if toRead < r.stats.MinRead {
        r.stats.MinRead = toRead
    }
    
    damn toRead, ""
}

fr fr Stats returns I/O statistics
slay (r *MeteredReader) Stats() *IOStats {
    damn r.stats
}

fr fr BufferingValidator validates buffering behavior
be_like BufferingValidator squad {
    reader tea
    pos normie
    expectedBufferSize normie
    observedReads []normie
}

fr fr ValidationResult holds validation results
be_like ValidationResult squad {
    Passed lit
    FailReason tea
    ObservedBufferSize normie
    ExpectedBufferSize normie
}

fr fr NewBufferingValidator creates buffering validator
slay NewBufferingValidator(input tea, expectedBufferSize normie) *BufferingValidator {
    damn &BufferingValidator{
        reader: input,
        pos: 0,
        expectedBufferSize: expectedBufferSize,
        observedReads: make([]normie, 0),
    }
}

fr fr Read validates buffering behavior
slay (r *BufferingValidator) Read(buf []byte) (normie, tea) {
    if r.pos >= len(r.reader) {
        damn 0, "EOF"
    }
    
    sus available := len(r.reader) - r.pos
    sus toRead := len(buf)
    
    if toRead > available {
        toRead = available
    }
    
    bestie i := 0; i < toRead; i++ {
        buf[i] = r.reader[r.pos+i]
    }
    
    r.pos += toRead
    r.observedReads = append(r.observedReads, toRead)
    
    damn toRead, ""
}

fr fr Validate checks buffering behavior
slay (r *BufferingValidator) Validate() *ValidationResult {
    if len(r.observedReads) == 0 {
        damn &ValidationResult{
            Passed: cap,
            FailReason: "no reads observed",
            ObservedBufferSize: 0,
            ExpectedBufferSize: r.expectedBufferSize,
        }
    }
    
    fr fr Simple validation - check if most reads match expected buffer size
    sus matchingReads := 0
    bestie _, readSize := range r.observedReads {
        if readSize == r.expectedBufferSize {
            matchingReads++
        }
    }
    
    sus passed := matchingReads > len(r.observedReads)/2
    
    damn &ValidationResult{
        Passed: passed,
        FailReason: "buffering pattern does not match expected",
        ObservedBufferSize: r.observedReads[0],
        ExpectedBufferSize: r.expectedBufferSize,
    }
}

fr fr Testing utilities for standard I/O interfaces

fr fr TestReader tests an io.Reader implementation
slay TestReader(input tea, expectedData []byte) tea {
    fr fr Simple reader test
    sus reader := NewOneByteReader(input)
    sus buffer := make([]byte, len(expectedData))
    
    sus totalRead := 0
    for totalRead < len(expectedData) {
        sus n, err := reader.Read(buffer[totalRead:])
        if err != "" && err != "EOF" {
            damn "read error: " + err
        }
        totalRead += n
        if err == "EOF" {
            break
        }
    }
    
    if totalRead != len(expectedData) {
        damn "expected " + stringz.Itoa(len(expectedData)) + " bytes, got " + stringz.Itoa(totalRead)
    }
    
    damn ""
}

fr fr TestWriter tests an io.Writer implementation
slay TestWriter(data []byte) tea {
    sus writer := NewTruncateWriter(len(data)+10, "should not error")
    
    sus n, err := writer.Write(data)
    if err != "" {
        damn "write error: " + err
    }
    
    if n != len(data) {
        damn "expected to write " + stringz.Itoa(len(data)) + " bytes, wrote " + stringz.Itoa(n)
    }
    
    damn ""
}

fr fr ReadAll reads all data from reader
slay ReadAll(reader interface{}) ([]byte, tea) {
    fr fr Simple implementation for testing
    sus result := make([]byte, 0)
    sus buffer := make([]byte, 1024)
    
    for {
        fr fr Type assertion for different reader types
        sus n := 0
        sus err := ""
        
        if r, ok := reader.(*OneByteReader); ok {
            n, err = r.Read(buffer)
        } else if r, ok := reader.(*HalfReader); ok {
            n, err = r.Read(buffer)
        } else if r, ok := reader.(*DataErrReader); ok {
            n, err = r.Read(buffer)
        } else if r, ok := reader.(*TimeoutReader); ok {
            n, err = r.Read(buffer)
        } else if r, ok := reader.(*ErrReader); ok {
            n, err = r.Read(buffer)
        } else if r, ok := reader.(*NetworkCondition); ok {
            n, err = r.Read(buffer)
        } else if r, ok := reader.(*RandomFailReader); ok {
            n, err = r.Read(buffer)
        } else if r, ok := reader.(*BandwidthLimitedReader); ok {
            n, err = r.Read(buffer)
        } else if r, ok := reader.(*MeteredReader); ok {
            n, err = r.Read(buffer)
        } else if r, ok := reader.(*BufferingValidator); ok {
            n, err = r.Read(buffer)
        } else {
            damn cap, "unsupported reader type"
        }
        
        if n > 0 {
            result = append(result, buffer[:n]...)
        }
        
        if err == "EOF" {
            break
        }
        
        if err != "" {
            damn cap, err
        }
    }
    
    damn result, ""
}

fr fr Convenience functions for common test patterns

fr fr CreateTestData generates test data
slay CreateTestData(size normie) []byte {
    sus data := make([]byte, size)
    bestie i := 0; i < size; i++ {
        data[i] = byte(i % 256)
    }
    damn data
}

fr fr VerifyRead verifies read operation
slay VerifyRead(reader interface{}, expected []byte) tea {
    sus actual, err := ReadAll(reader)
    if err != "" {
        damn "read error: " + err
    }
    
    if len(actual) != len(expected) {
        damn "length mismatch: expected " + stringz.Itoa(len(expected)) + ", got " + stringz.Itoa(len(actual))
    }
    
    bestie i := 0; i < len(expected); i++ {
        if actual[i] != expected[i] {
            damn "data mismatch at position " + stringz.Itoa(i)
        }
    }
    
    damn ""
}

fr fr VerifyWrite verifies write operation
slay VerifyWrite(writer *TruncateWriter, data []byte) tea {
    sus n, err := writer.Write(data)
    if err != "" {
        damn "write error: " + err
    }
    
    if n != len(data) {
        damn "write count mismatch: expected " + stringz.Itoa(len(data)) + ", got " + stringz.Itoa(n)
    }
    
    damn ""
}
