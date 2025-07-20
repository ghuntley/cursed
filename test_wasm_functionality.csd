// WASM compatibility test for CURSED
// This program tests basic functionality that should work in WASM

yeet "testz"

// Test basic variable declarations
sus message tea = "Hello WASM!"
sus count drip = 42
sus flag lit = based

// Test basic output (should work in WASM with console API)
vibez.spill(message)
vibez.spill("Count: " + count.(tea))

// Test basic arithmetic
sus result drip = count + 8
vibez.spill("Result: " + result.(tea))

// Test basic conditionals
chk (flag == based) {
    vibez.spill("Flag is true")
} sus {
    vibez.spill("Flag is false")
}

// Test basic loops
sus i drip = 0
bestie (i < 3) {
    vibez.spill("Loop iteration: " + i.(tea))
    i = i + 1
}

// Test basic functions
slay greet(name tea) nothing {
    vibez.spill("Hello, " + name + "!")
}

greet("WASM World")

vibez.spill("WASM compatibility test completed!")
