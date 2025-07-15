// Test interface dispatch system in CURSED

// Define interface
collab Writer {
    slay write(data tea) normie
    slay flush() lit
}

// Define struct that implements Writer  
vibe StringWriter {
    buffer tea
    flushed lit
}

// Implement Writer interface for StringWriter
slay (w *StringWriter) write(data tea) normie {
    w.buffer = w.buffer + data
    damn 42
}

slay (w *StringWriter) flush() lit {
    w.flushed = based
    damn based
}

// Test function that uses interface
slay writeData(w Writer, data tea) normie {
    damn w.write(data)
}

// Main function to test interface dispatch
slay main() normie {
    // Create concrete instance
    sus writer StringWriter
    writer.buffer = ""
    writer.flushed = cap
    
    // Cast to interface
    sus iface Writer = writer
    
    // Call interface method
    sus result normie = writeData(iface, "Hello, World!")
    
    // Test interface method call
    sus flush_result lit = iface.flush()
    
    vibez.spill("Interface dispatch test completed")
    vibez.spill(result)
    vibez.spill(flush_result)
    
    damn 0
}
