# Test all three tuple destructuring patterns

# Let statement tuple destructuring
let (a, b) = (1, 2)

# Assignment tuple destructuring  
sus (c, d) := (3, 4)
(c, d) = (a + 2, b + 3)

# Short declaration tuple destructuring
sus (e, f) := (c + a, d + b)

# Test variable access after destructuring
vibez.spill("Let destructuring:")
vibez.spill(a)
vibez.spill(b)

vibez.spill("Assignment destructuring:")  
vibez.spill(c)
vibez.spill(d)

vibez.spill("Short declaration destructuring:")
vibez.spill(e)
vibez.spill(f)

# Test nested access
sus total := a + b + c + d + e + f
vibez.spill("Total:")
vibez.spill(total)
