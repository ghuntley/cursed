# Simple type test for the CURSED compiler

# Variable declarations with type inference
sus x := 42                    # Should infer drip (int)
sus y := 3.14                  # Should infer meal (float)
sus name := "CURSED"           # Should infer tea (string)
sus flag := based              # Should infer lit (bool)

# Function with explicit types
slay add_numbers(a drip, b drip) drip {
    damn a + b
}

# Simple struct
squad Point {
    spill x drip
    spill y drip
}

# Use the functions and types
sus result := add_numbers(10, 20)
sus origin := Point { x: 0, y: 0 }

vibez.spill("Type checking test complete!")
