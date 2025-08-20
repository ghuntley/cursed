// Direct FFI test at top level

vibez.spill("=== Direct FFI Test ===")

// Test extern function calls directly
set_pixel_color(100, 50, 1)
sus result normie = get_pixel_color(100, 50)
vibez.spill("Retrieved color:", result)

sus status normie = get_system_status()  
vibez.spill("System status:", status)

set_log_priority(10)

vibez.spill("=== Direct FFI test complete ===")
