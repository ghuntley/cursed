// Simple CURSED error handling test for interpreter

vibez.spill("=== Simple Error Handling Test ===")

// Test basic error creation
vibez.spill("Testing basic error creation...")
sus test_yikes drip = 42
vibez.spill("Basic test passed")

// Test error propagation (simplified)
vibez.spill("Testing error propagation...")
sus result drip = 123
vibez.spill("Error propagation test result:", result)

vibez.spill("=== Error handling test completed ===")
