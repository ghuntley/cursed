// Simple FFI test that should work with the current binary

slay main() vibes {
    vibez.spill("=== Simple FFI Test ===")
    
    // Test simple extern function call
    set_pixel_color(100, 50, 1)
    
    vibez.spill("=== FFI test complete ===")
}
