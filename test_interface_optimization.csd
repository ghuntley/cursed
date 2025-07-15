// Test interface optimization features

// Define multiple interfaces for testing hierarchy
collab Readable {
    slay read() tea
}

collab Writable {
    slay write(data tea) lit
}

collab ReadWritable {
    extends Readable, Writable
    slay readWrite(data tea) tea
}

// Define structs implementing interfaces
vibe FileHandler {
    filename tea
    content tea
}

vibe NetworkHandler {
    url tea
    buffer tea
}

// Implement interfaces for FileHandler
slay (f *FileHandler) read() tea {
    damn f.content
}

slay (f *FileHandler) write(data tea) lit {
    f.content = data
    damn based
}

slay (f *FileHandler) readWrite(data tea) tea {
    f.content = data
    damn f.content
}

// Implement interfaces for NetworkHandler  
slay (n *NetworkHandler) read() tea {
    damn n.buffer
}

slay (n *NetworkHandler) write(data tea) lit {
    n.buffer = data
    damn based
}

slay (n *NetworkHandler) readWrite(data tea) tea {
    n.buffer = data
    damn n.buffer
}

// Test polymorphic function
slay processData[T ReadWritable](handler T, input tea) tea {
    handler.write(input)
    damn handler.read()
}

// Test interface casting
slay testInterfaceCasting() {
    // Create concrete objects
    sus file FileHandler
    file.filename = "test.txt"
    file.content = ""
    
    sus network NetworkHandler  
    network.url = "http://example.com"
    network.buffer = ""
    
    // Cast to interfaces
    sus readable Readable = file
    sus writable Writable = network
    sus readwritable ReadWritable = file
    
    // Test method calls through interfaces
    sus read_result tea = readable.read()
    sus write_result lit = writable.write("test data")
    sus readwrite_result tea = readwritable.readWrite("combined data")
    
    vibez.spill("Interface casting test:")
    vibez.spill(read_result)
    vibez.spill(write_result)
    vibez.spill(readwrite_result)
}

// Test interface assertions
slay testInterfaceAssertions() {
    sus file FileHandler
    file.content = "assertion test"
    
    // Test successful assertion
    sus readable, ok := file.(Readable)
    lowkey ok {
        sus result tea = readable.read()
        vibez.spill("Successful assertion:")
        vibez.spill(result)
    } else {
        vibez.spill("Assertion failed")
    }
    
    // Test interface hierarchy
    sus readwritable ReadWritable = file
    sus base_readable Readable = readwritable // Should work due to inheritance
    
    vibez.spill("Interface hierarchy test completed")
}

// Main function to test all interface features
slay main() normie {
    vibez.spill("Starting interface optimization tests...")
    
    // Test basic interface dispatch
    testInterfaceCasting()
    
    // Test interface assertions
    testInterfaceAssertions()
    
    // Test generic interface usage
    sus file FileHandler
    file.content = "generic test"
    sus result tea = processData(file, "processed data")
    vibez.spill("Generic interface result:")
    vibez.spill(result)
    
    vibez.spill("Interface optimization tests completed!")
    damn 0
}
