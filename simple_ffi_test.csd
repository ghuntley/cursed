// Simple FFI integration test
// Tests basic extern function calls

// Basic enum for testing
enum Color {
    Red = 0,
    Green = 1,
    Blue = 2
}

slay main() vibes {
    vibez.spill("=== FFI Integration Test ===")
    
    // Test set_pixel_color function call
    vibez.spill("Testing set_pixel_color(100, 50, 1)")
    set_pixel_color(100, 50, 1)
    
    // Test get_pixel_color function call
    vibez.spill("Testing get_pixel_color(100, 50)")
    sus result normie = get_pixel_color(100, 50)
    vibez.spill("Retrieved color:", result)
    
    // Test get_system_status function call
    vibez.spill("Testing get_system_status()")
    sus status normie = get_system_status()
    vibez.spill("System status:", status)
    
    // Test set_log_priority function call
    vibez.spill("Testing set_log_priority(10)")
    set_log_priority(10)
    
    vibez.spill("=== FFI Test Complete ===")
}
