# Test specifically for the parentheses memory leak fix
sus a drip = (((5 + 3) * 2) + 1)
sus b drip = ((a + 10) * (2 + 3))
sus c drip = (((b - 5) / 2) + ((a * 3) - 10))
vibez.spill("Nested parentheses result:", c)

# Multiple complex expressions to stress test memory management
sus d drip = ((1 + 2) * (3 + 4)) + ((5 - 1) * (6 / 2))
sus e drip = (((d + a) - b) * 2) + (((c + 10) / 5) - 3)
vibez.spill("Complex calculation:", e)
