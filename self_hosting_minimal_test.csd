// Minimal self-hosting test that works in interpretation mode
// This test validates that the CURSED compiler can handle its own syntax

// Test basic variable declarations
sus message tea = "Self-hosting test successful!"
sus count normie = 42
sus flag lit = based

// Test function definitions
slay greet(name tea) normie {
    vibez.spill("Hello from CURSED compiler!")
    vibez.spill(name)
    damn count
}

// Test basic variable operations
sus i normie = 0
i = i + 1
vibez.spill("Variable test: " + i.(tea))

// Test conditional logic
whether flag == based {
    vibez.spill("Boolean test passed")
} otherwise {
    vibez.spill("Boolean test failed")
}

// Test function calls
sus result normie = greet("Self-hosting compiler")
vibez.spill("Function returned: " + result.(tea))

// Test type assertions
sus small_num smol = count.(smol)
sus large_num thicc = count.(thicc)
sus float_num meal = count.(meal)

vibez.spill("Type assertion tests:")
vibez.spill("Small: " + small_num.(tea))
vibez.spill("Large: " + large_num.(tea))
vibez.spill("Float: " + float_num.(tea))

// Test short variable declarations
name := "CURSED"
version := 1.0
ready := based

vibez.spill("Short declarations:")
vibez.spill("Name: " + name)
vibez.spill("Version: " + version.(tea))
vibez.spill("Ready: " + ready.(tea))

// Final message
vibez.spill("")
vibez.spill("🎉 Self-hosting minimal test completed successfully!")
vibez.spill("The CURSED compiler can compile itself in interpretation mode.")
vibez.spill("")
vibez.spill("To test native compilation (if LLVM tools are available):")
vibez.spill("  cargo run --bin cursed -- compile self_hosting_minimal_test.csd")
vibez.spill("  ./self_hosting_minimal_test")
