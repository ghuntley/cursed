yeet "testz"

# Test method resolution for inherited interfaces
collab Writable {
    slay write(data tea) normie
}

collab Readable {
    slay read() tea
}

collab FileIO : Writable, Readable {
    slay close() normie
}

# Implementation type
struct FileHandle {
    filename tea
    is_open lit
}

# Test method resolution
slay test_method_resolution() {
    vibez.spill("Method resolution test")
    
    # FileHandle should implement all methods from:
    # - Writable.write()
    # - Readable.read() 
    # - FileIO.close()
    
    sus handle := FileHandle{
        filename: "test.txt",
        is_open: based
    }
    
    vibez.spill("FileHandle created successfully")
}

test_start("Interface method resolution test")
test_method_resolution()
print_test_summary()
