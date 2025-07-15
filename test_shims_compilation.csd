vibez.spill("Testing minimal shims in compilation mode...")

# Test basic functionality that should work with minimal shims
sus x := 42
sus y := 100
sus result := x + y

vibez.spill("Result:", result)
vibez.spill("✅ Minimal shims working in compilation mode")
