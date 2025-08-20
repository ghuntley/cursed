// Comprehensive FFI integration test with proper debugging

slay main() vibes {
    // Initialize the FFI system
    vibez.spill("Starting FFI integration test...")
    
    // Test 1: Call set_pixel_color
    vibez.spill("=== Test 1: set_pixel_color ===")
    
    // First try basic function call
    fam {
        set_pixel_color(100, 50, 1)
        vibez.spill("✅ set_pixel_color call succeeded")
    } shook (error) {
        vibez.spill("❌ set_pixel_color failed:", error)
    }
    
    // Test 2: Call get_pixel_color
    vibez.spill("=== Test 2: get_pixel_color ===")
    
    fam {
        sus result normie = get_pixel_color(100, 50)
        vibez.spill("✅ get_pixel_color returned:", result)
    } shook (error) {
        vibez.spill("❌ get_pixel_color failed:", error)
    }
    
    // Test 3: Call get_system_status
    vibez.spill("=== Test 3: get_system_status ===")
    
    fam {
        sus status normie = get_system_status()
        vibez.spill("✅ get_system_status returned:", status)
    } shook (error) {
        vibez.spill("❌ get_system_status failed:", error)
    }
    
    // Test 4: Call set_log_priority
    vibez.spill("=== Test 4: set_log_priority ===")
    
    fam {
        set_log_priority(10)
        vibez.spill("✅ set_log_priority call succeeded")
    } shook (error) {
        vibez.spill("❌ set_log_priority failed:", error)
    }
    
    vibez.spill("=== FFI Integration Test Complete ===")
}
