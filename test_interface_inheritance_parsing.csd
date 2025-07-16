# Test interface inheritance parsing syntax

# Basic interface
collab Writer {
    slay write(data tea) normie
}

# Interface inheritance with colon syntax
collab ReadWriter : Writer {
    slay read() tea
}

# Multiple inheritance
collab FileIO : Writer, ReadWriter {
    slay close() normie
}

# Generic interface inheritance
collab GenericWriter<T> {
    slay write_generic(data T) normie
}

collab GenericBuffered<T> : GenericWriter<T> {
    slay flush() normie
}

# Test that syntax parses correctly
slay main() {
    vibez.spill("Interface inheritance syntax parsing test successful")
}

main()
