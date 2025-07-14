# Test nested destructuring assignments - P4 edge case
sus (a, b) := (1, 2)
sus (c, d) := (a + 1, b + 2)
sus (e, f) := ((c * 2), (d * 3))

vibez.spill("Nested destructuring test:")
vibez.spill(a)
vibez.spill(b)
vibez.spill(c)
vibez.spill(d)
vibez.spill(e)
vibez.spill(f)
