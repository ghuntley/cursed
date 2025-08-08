// Test short declaration compilation
// Single variable short declaration
x := 42
vibez.spill("x:", x)

// String short declaration
name := "CURSED"
vibez.spill("name:", name)

// Multiple variable short declaration  
a, b := 10, 20
vibez.spill("a:", a, "b:", b)

// Boolean short declaration
flag := based
vibez.spill("flag:", flag)

vibez.spill("Short declarations completed")
