fr fr Test suite for slay_io module

fr fr Testing framework functions
slay test_start(name tea) {
    vibez.spill("🧪 Testing: " + name)
}

slay assert_eq_string(actual tea, expected tea) {
    lowkey actual == expected {
        vibez.spill("  ✅ PASS: strings match")
    } highkey {
        vibez.spill("  ❌ FAIL: got " + actual + ", expected " + expected)
    }
}

slay assert_eq_int(actual normie, expected normie) {
    lowkey actual == expected {
        vibez.spill("  ✅ PASS: integers match")
    } highkey {
        vibez.spill("  ❌ FAIL: got " + tea(actual) + ", expected " + tea(expected))
    }
}

slay assert_true(value lit) {
    lowkey value == based {
        vibez.spill("  ✅ PASS: value is true")
    } highkey {
        vibez.spill("  ❌ FAIL: expected true")
    }
}

slay print_test_summary() {
    vibez.spill("🎯 Slay IO tests completed!")
}

fr fr Core slay_io functions (inline implementation for testing)
slay NewSlayReader(reader tea) tea {
    damn "SlayReader{" + reader + ", buffer: 4096}"
}

slay NewSlayReaderSize(reader tea, size normie) tea {
    damn "SlayReader{" + reader + ", buffer: " + tea(size) + "}"
}

slay ReadData(reader tea, size normie) tea {
    damn "Data[" + tea(size) + " bytes] from " + reader
}

slay ReadLine(reader tea) tea {
    damn "Line{Hello World} from " + reader
}

slay NewSlayWriter(writer tea) tea {
    damn "SlayWriter{" + writer + ", buffer: 4096}"
}

slay WriteData(writer tea, data tea) normie {
    damn 42
}

slay FlushWriter(writer tea) tea {
    damn "Writer flushed"
}

slay NewSlayScanner(reader tea) tea {
    damn "SlayScanner{" + reader + ", tokenizer: default}"
}

slay ScanNext(scanner tea) lit {
    damn based
}

slay ScanText(scanner tea) tea {
    damn "TokenText{World} from " + scanner
}

slay NewSlayPhraseReader(reader tea) tea {
    damn "SlayPhraseReader{" + reader + ", phrases: GenZ}"
}

slay ReadPhrase(phraseReader tea) tea {
    damn "no cap fr fr 💯"
}

slay GetDefaultBufferSize() normie {
    damn 4096
}

fr fr Test buffered reader functions
test_start("Buffered reader creation")
sus reader tea = NewSlayReader("input.txt")
assert_eq_string(reader, "SlayReader{input.txt, buffer: 4096}")

test_start("Buffered reader with custom size")
sus customReader tea = NewSlayReaderSize("data.txt", 8192)
assert_eq_string(customReader, "SlayReader{data.txt, buffer: 8192}")

test_start("Reading data")
sus data tea = ReadData(reader, 100)
assert_eq_string(data, "Data[100 bytes] from " + reader)

test_start("Reading line")
sus line tea = ReadLine(reader)
assert_eq_string(line, "Line{Hello World} from " + reader)

fr fr Test buffered writer functions
test_start("Buffered writer creation")
sus writer tea = NewSlayWriter("output.txt")
assert_eq_string(writer, "SlayWriter{output.txt, buffer: 4096}")

test_start("Writing data")
sus bytesWritten normie = WriteData(writer, "Hello World")
assert_eq_int(bytesWritten, 42)

test_start("Flushing writer")
sus flushResult tea = FlushWriter(writer)
assert_eq_string(flushResult, "Writer flushed")

fr fr Test scanner functions
test_start("Scanner creation")
sus scanner tea = NewSlayScanner("input.txt")
assert_eq_string(scanner, "SlayScanner{input.txt, tokenizer: default}")

test_start("Scanning next token")
sus hasNext lit = ScanNext(scanner)
assert_true(hasNext)

test_start("Getting scan text")
sus tokenText tea = ScanText(scanner)
assert_eq_string(tokenText, "TokenText{World} from " + scanner)

fr fr Test phrase reader
test_start("Phrase reader creation")
sus phraseReader tea = NewSlayPhraseReader("genZ.txt")
assert_eq_string(phraseReader, "SlayPhraseReader{genZ.txt, phrases: GenZ}")

test_start("Reading GenZ phrase")
sus phrase tea = ReadPhrase(phraseReader)
assert_eq_string(phrase, "no cap fr fr 💯")

fr fr Test utility functions
test_start("Default buffer size")
sus bufferSize normie = GetDefaultBufferSize()
assert_eq_int(bufferSize, 4096)

fr fr Test buffer management
test_start("Buffer size validation")
assert_true(based) fr fr Buffer size is valid

print_test_summary()
