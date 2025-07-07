// Simple self-hosting validation test
sus message tea = "Self-hosting test successful!"
sus count normie = 42
sus flag lit = based

// Test function definition and call
slay greet(name tea) normie {
    vibez.spill("Hello from CURSED compiler!")
    vibez.spill(name)
    damn 100
}

// Test basic output
vibez.spill(message)
vibez.spill("Count: " + count.(tea))
vibez.spill("Flag: " + flag.(tea))

// Test function call
sus result normie = greet("Self-hosting compiler")
vibez.spill("Function returned: " + result.(tea))

// Test type assertions
sus small_num smol = count.(smol)
sus large_num thicc = count.(thicc)

vibez.spill("Type assertions work:")
vibez.spill("Small: " + small_num.(tea))
vibez.spill("Large: " + large_num.(tea))

// Test short variable declarations
name := "CURSED"
version := 1

vibez.spill("Short declarations work:")
vibez.spill("Name: " + name)
vibez.spill("Version: " + version.(tea))

vibez.spill("")
vibez.spill("🎉 Self-hosting validation completed successfully!")
vibez.spill("The CURSED compiler can handle its own syntax.")
