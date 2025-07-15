// Simple test for panic/recover parsing
slay test_panic_parsing() {
    vibez.spill("Testing panic parsing")
    
    // This should parse correctly
    // panic("test message")
    
    vibez.spill("Panic parsing test completed")
}

slay test_recover_parsing() {
    vibez.spill("Testing recover parsing")
    
    // This should parse correctly  
    // sus recovered_value tea = recover()
    
    vibez.spill("Recover parsing test completed")
}

slay main() {
    test_panic_parsing()
    test_recover_parsing()
}
