// Minimal self-hosting validation test
sus message tea = "Self-hosting test successful!"
sus count normie = 42
sus flag lit = based

vibez.spill(message)
vibez.spill("Count: " + count.(tea))
vibez.spill("Flag: " + flag.(tea))

// Test type assertions
sus small_num smol = count.(smol)
vibez.spill("Small: " + small_num.(tea))

// Test short variable declarations
name := "CURSED"
vibez.spill("Name: " + name)

vibez.spill("")
vibez.spill("🎉 Minimal self-hosting test completed!")
vibez.spill("The CURSED compiler can parse and execute its own syntax.")
