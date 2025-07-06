# Test interface compliance checking in CURSED

# Define a Writer interface
collab Writer {
    slay write(data tea) normie
    slay flush() normie
}

# Define a Reader interface  
collab Reader {
    slay read() tea
    slay close() normie
}

# Define a FileWriter struct that implements Writer
squad FileWriter {
    path tea
    
    slay write(data tea) normie {
        vibez.spill("Writing to file: " + data)
        damn 1
    }
    
    slay flush() normie {
        vibez.spill("Flushing file")
        damn 0
    }
}

# Define a ConsoleWriter struct that implements Writer
squad ConsoleWriter {
    slay write(data tea) normie {
        vibez.spill("Console: " + data)
        damn 1
    }
    
    slay flush() normie {
        vibez.spill("Console flushed")
        damn 0
    }
}

# Define a NetworkReader struct that implements Reader
squad NetworkReader {
    url tea
    
    slay read() tea {
        damn "network data"
    }
    
    slay close() normie {
        vibez.spill("Connection closed")
        damn 0
    }
}

# Define an incomplete struct (missing methods)
squad IncompleteWriter {
    name tea
    
    slay write(data tea) normie {
        vibez.spill("Incomplete: " + data)
        damn 1
    }
    # Missing flush() method - should fail interface compliance
}

# Test basic interface compliance
slay testInterfaceCompliance() {
    vibez.spill("Testing interface compliance...")
    
    # These should work - types implement interfaces
    sus fileWriter FileWriter = FileWriter{path: "test.txt"}
    sus consoleWriter ConsoleWriter = ConsoleWriter{}
    sus networkReader NetworkReader = NetworkReader{url: "http://example.com"}
    
    # Test method calls
    fileWriter.write("Hello, World!")
    fileWriter.flush()
    
    consoleWriter.write("Console message")
    consoleWriter.flush()
    
    sus data tea = networkReader.read()
    vibez.spill("Read: " + data)
    networkReader.close()
    
    vibez.spill("Interface compliance test completed")
}

# Test interface assignability
slay testInterfaceAssignability() {
    vibez.spill("Testing interface assignability...")
    
    # These should work - structs can be assigned to interfaces they implement
    sus writer Writer = FileWriter{path: "output.txt"}
    sus reader Reader = NetworkReader{url: "http://api.example.com"}
    
    # Test interface method calls
    writer.write("Interface message")
    writer.flush()
    
    sus content tea = reader.read()
    vibez.spill("Interface read: " + content)
    reader.close()
    
    vibez.spill("Interface assignability test completed")
}

# Main function to run tests
slay main() {
    vibez.spill("Starting interface compliance tests...")
    
    testInterfaceCompliance()
    testInterfaceAssignability()
    
    vibez.spill("All interface tests completed")
}
