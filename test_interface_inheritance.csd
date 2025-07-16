yeet "testz"

# Test interface inheritance with colon syntax
collab Writer {
    slay write(data tea) normie
}

collab Reader {
    slay read() tea
}

# Interface inheritance using : syntax
collab ReadWriter : Writer {
    slay read() tea
}

# Multiple interface inheritance
collab ReadWriteSeeker : Reader, Writer {
    slay seek(position normie) normie
}

# Generic interface inheritance
collab GenericWriter<T> {
    slay write_generic(data T) normie
}

collab GenericReadWriter<T> : GenericWriter<T> {
    slay read_generic() T
}

# Test implementation
slay test_interface_inheritance() {
    vibez.spill("Interface inheritance test passed")
}

test_start("Interface inheritance syntax test")
test_interface_inheritance()
print_test_summary()
