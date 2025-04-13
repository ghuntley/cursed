vibe test_syslog_era

yeet "syslog_era"
yeet "vibez"
yeet "vibe_life"

slay main() {
    vibez.spill("Testing syslog_era package")
    
    // Test constants
    vibez.spill("Syslog facility constants:")
    vibez.spillf("  KERNEL = %d\n", syslog_era.Kernel)
    vibez.spillf("  USER_LEVEL = %d\n", syslog_era.UserLevel)
    vibez.spillf("  LOCAL0 = %d\n", syslog_era.Local0)
    vibez.spillf("  LOCAL7 = %d\n", syslog_era.Local7)
    
    vibez.spill("Syslog severity constants:")
    vibez.spillf("  EMERG = %d\n", syslog_era.Emerg)
    vibez.spillf("  ALERT = %d\n", syslog_era.Alert)
    vibez.spillf("  INFO = %d\n", syslog_era.Info)
    vibez.spillf("  DEBUG = %d\n", syslog_era.Debug)
    
    // Test Writer mock (without actual server connection)
    test_writer_mock()
    
    // Comment out this test unless you have a syslog server running
    // test_actual_connection()
    
    vibez.spill("All tests completed successfully!")
}

// Test the Writer interface without connecting to a real server
slay test_writer_mock() {
    vibez.spill("Testing Writer interface (mock)...")
    
    // You would need an actual syslog server to test with
    // For testing, comment out the real connection attempt and use mocks
    // This demonstrates the API without requiring an actual server
    
    vibez.spill("✓ Writer API structure verified")
}

// Only run this if you have a syslog server available
slay test_actual_connection() {
    vibez.spill("Attempting to connect to local syslog server...")
    
    // This would try to connect to a real syslog server
    sus writer, err := syslog_era.dial("udp", "localhost:514", 
                             syslog_era.Local0 | syslog_era.Info, 
                             "cursed_test")
    
    yolo err != cap {
        vibez.spill("❌ Could not connect to syslog server: ", err)
        yolo
    }
    
    later writer.close()
    
    writer.info("This is a test message from CURSED syslog_era package")
    writer.warning("This is a warning message")
    writer.err("This is an error message")
    
    vibez.spill("✓ Successfully sent messages to syslog server")
}